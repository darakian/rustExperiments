//Std imports
use std::io::{Read, BufReader, Write};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Sender, channel};
use std::collections::hash_map::{DefaultHasher, HashMap, Entry};
use std::fs::{self, File, DirEntry};

//External imports
extern crate stacker;
extern crate clap;
extern crate rayon;
use clap::{Arg, App};
use rayon::prelude::*;

#[derive(Clone)]
struct Fileinfo{
    file_hash: u64,
    file_len: u64,
    file_paths: Vec<PathBuf>,
    da_4k: [u8;4096]
}

impl Fileinfo{
    fn new(hash: u64, length: u64, path: PathBuf, input_buf: [u8;4096]) -> Self{
        let mut set = Vec::<PathBuf>::new();
        set.push(path);
        Fileinfo{file_hash: hash, file_len: length, file_paths: set, da_4k: input_buf}
    }
}


impl Hash for Fileinfo{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_hash.hash(state);
    }
}

fn main() {
    let arguments = App::new("File Compare")
                        .version("0.2.0")
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
        stacker::maybe_grow(32 * 1024, 1024 * 1024, || {
            traverse_and_spawn(Path::new(&search_dir), s.clone());
        });
    });

    drop(sender);
    let mut files_of_lengths: HashMap<u64, Vec<Fileinfo>> = HashMap::new();
    for entry in receiver.iter(){
    match files_of_lengths.entry(entry.file_len) {
        Entry::Vacant(e) => { e.insert(vec![entry]); },
        Entry::Occupied(mut e) => { e.get_mut().push(entry); }
        }
    }

    let mut sizes = File::create(format!("filesizes")).unwrap();
    //println!("{:?}", sizes);
    let mut complete_files: Vec<Fileinfo> = files_of_lengths.into_par_iter().map(|x|
        x.1
    ).flatten().collect();
    complete_files.iter().for_each(|x| write!(sizes, "{}\n", x.file_len).unwrap());

    for i in 1..4096{
        let mut hashesfile = File::create(format!("data/{:05}",i)).unwrap();
        //write!(file, "Step {}\n", i).unwrap();
        complete_files.par_iter_mut().for_each(|x| hash_and_update(x, i));
        complete_files.par_sort_unstable_by(|a, b| b.file_hash.cmp(&a.file_hash));
        let mut tmp_files = complete_files.clone();
        tmp_files.dedup_by(|a, b| if a.file_hash==b.file_hash{ //O(n)
            b.file_paths.extend(a.file_paths.drain(0..));
            true
        }else{false});
        tmp_files.iter().for_each(|x| write!(hashesfile, "{} {}\n", x.file_hash, x.file_paths.len()).unwrap());
    }
}

fn hash_and_update(input: &mut Fileinfo, length: u64) -> (){
    let mut hasher = DefaultHasher::new();
    hasher.write(&input.da_4k[0..length as usize]);
    input.file_hash=hasher.finish();
}

fn traverse_and_spawn(current_path: &Path, sender: Sender<Fileinfo>) -> (){
    if !current_path.exists(){
        return
    }

    if current_path.symlink_metadata().expect("Error getting Symlink Metadata").file_type().is_dir(){
        let mut paths: Vec<DirEntry> = Vec::new();
        match fs::read_dir(current_path) {
                Ok(read_dir_results) => read_dir_results.filter(|x| x.is_ok()).for_each(|x| paths.push(x.unwrap())),
                Err(e) => println!("Skipping {:?}. {:?}", current_path, e.kind()),
            }
        paths.into_par_iter().for_each_with(sender, |s, dir_entry| {
            stacker::maybe_grow(32 * 1024, 1024 * 1024, || {
                traverse_and_spawn(dir_entry.path().as_path(), s.clone());
            });
        });
    } else if current_path.symlink_metadata().expect("Error getting Symlink Metadata").file_type().is_file(){
        match fs::File::open(current_path) {
            Ok(f) => {
                let mut buffer_reader = BufReader::new(f);
                let mut hash_buffer = [0;4096];
                buffer_reader.read(&mut hash_buffer).unwrap();
                sender.send(Fileinfo::new(0, current_path.metadata().expect("Error with current path length").len(), current_path.to_path_buf(), hash_buffer)).unwrap();
            }
            Err(e) => {println!("Error:{} when opening {:?}. Skipping.", e, current_path)}
        }
    } else {}
}
