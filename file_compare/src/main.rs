//Std imports
use std::io::{Read, BufReader, Write};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Sender, channel};
use std::collections::hash_map::{DefaultHasher, HashMap, Entry};
use std::cmp::Ordering;
use std::fs::{self, File};

//External imports
extern crate clap;
extern crate rayon;
use clap::{Arg, App};
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Fileinfo{
    file_hash: u64,
    file_len: u64,
    file_paths: Vec<PathBuf>,
    hashed: bool,
    to_hash: bool,
}

impl Fileinfo{
    fn new(hash: u64, length: u64, path: PathBuf) -> Self{
        let mut set = Vec::<PathBuf>::new();
        set.push(path);
        Fileinfo{file_hash: hash, file_len: length, file_paths: set, hashed: false, to_hash: false}
    }
}

impl PartialEq for Fileinfo{
    fn eq(&self, other: &Fileinfo) -> bool {
        (self.file_hash==other.file_hash)&&(self.file_len==other.file_len)
    }
}
impl Eq for Fileinfo{}

impl PartialOrd for Fileinfo{
    fn partial_cmp(&self, other: &Fileinfo) -> Option<Ordering>{
        self.file_len.partial_cmp(&other.file_len)
    }
}

impl Ord for Fileinfo{
    fn cmp(&self, other: &Fileinfo) -> Ordering {
        self.file_len.cmp(&other.file_len)
    }
}

impl Hash for Fileinfo{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_hash.hash(state);
    }
}

fn main() {
    let arguments = App::new("File Compare")
                        .version("0.9.4")
                        .author("Jon Moroney jmoroney@hawaii.edu")
                        .about("Compares file differences in steps")
                        .arg(Arg::with_name("directories")
                               .short("d")
                               .long("directories")
                               .case_insensitive(true)
                               .value_name("Directories")
                               .help("Directories to parse")
                               .min_values(1)
                               .required(true)
                               .takes_value(true)
                               .index(1))
                        .get_matches();


    let (sender, receiver) = channel();
    let search_dirs: Vec<_> = arguments.values_of("directories").unwrap().collect();

    search_dirs.par_iter().for_each_with(sender.clone(), |s, search_dir| {
        traverse_and_spawn(Path::new(&search_dir), s.clone());
    });

    drop(sender);
    let mut files_of_lengths: HashMap<u64, Vec<Fileinfo>> = HashMap::new();
    for entry in receiver.iter(){
    match files_of_lengths.entry(entry.file_len) {
        Entry::Vacant(e) => { e.insert(vec![entry]); },
        Entry::Occupied(mut e) => { e.get_mut().push(entry); }
        }
    }

    let mut complete_files: Vec<Fileinfo> = files_of_lengths.into_par_iter().map(|x|
        x.1
    ).flatten().collect();

    for i in 1..1000{
        let mut file = File::create(format!("data/{:05}",i*10)).unwrap();
        //write!(file, "Step {}\n", i).unwrap();
        complete_files.par_iter_mut().for_each(|x| hash_and_update(x, i));
        complete_files.par_sort_unstable_by(|a, b| b.file_hash.cmp(&a.file_hash));
        let mut tmp_files = complete_files.clone();
        tmp_files.dedup_by(|a, b| if a.file_hash==b.file_hash{ //O(n)
            b.file_paths.extend(a.file_paths.drain(0..));
            true
        }else{false});
        tmp_files.iter().for_each(|x| write!(file, "{} {}\n", x.file_hash, x.file_paths.len()).unwrap());
    }

}

fn hash_and_update(input: &mut Fileinfo, length: u64) -> (){
    let mut hasher = DefaultHasher::new();
    match fs::File::open(input.file_paths.iter().next().expect("Error opening file for hashing")) {
        Ok(f) => {
            let mut buffer_reader = BufReader::new(f);
            let mut hash_buffer = [0;100];
            for _i in 1..length {
                match buffer_reader.read(&mut hash_buffer) {
                    Ok(n) if n>0 => hasher.write(&hash_buffer[0..n]),
                    Ok(n) if n==0 => break,
                    Err(e) => println!("{:?} reading {:?}", e, input.file_paths.iter().next().expect("Error opening file for hashing")),
                    _ => println!("Should not be here"),
                }
            }
            input.file_hash=hasher.finish();
        }
        Err(e) => {println!("Error:{} when opening {:?}. Skipping.", e, input.file_paths.iter().next().expect("Error opening file for hashing"))}
    }
}

fn traverse_and_spawn(current_path: &Path, sender: Sender<Fileinfo>) -> (){
    if current_path.is_dir(){
        let paths: Vec<_> = fs::read_dir(current_path).unwrap().map(|a| a.ok().expect("Unable to open directory for traversal")).collect();
        paths.par_iter().for_each_with(sender, |s, dir_entry| {
            traverse_and_spawn(dir_entry.path().as_path(), s.clone());
        });
    } else if current_path.is_file() {
        sender.send(Fileinfo::new(0, current_path.metadata().unwrap().len(), current_path.to_path_buf())).unwrap();
    } else {println!("Cannot open {:?}. Skipping.", current_path);}
}
