use std::io;
use std::io::Read;
use std::hash::Hash;
use std::io::BufReader;
use std::path::Path;
use std::thread;
use std::collections::HashSet;
use std::path::PathBuf;

use std::fs::{self};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

extern crate clap;
use clap::{Arg, App};

#[derive(Clone)]
struct Fileinfo{
    file_hash: u64,
    file_len: u64,
    file_paths: Vec<PathBuf>,
}
impl PartialEq for Fileinfo{
    fn eq(&self, other: &Fileinfo) -> bool {
        (self.file_hash==other.file_hash)
    }
}
impl Eq for Fileinfo{}

impl Hash for Fileinfo{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_hash.hash(state);
    }
}

impl Fileinfo{
    fn add_path(&mut self, new_path: PathBuf){
        self.file_paths.push(new_path);
    }
}

fn main() {
    let arguments = App::new("Directory Difference hTool")
                          .version("0.5.0")
                          .author("Jon Moroney jmoroney@cs.ru.nl")
                          .about("Compare and contrast directories")
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
                          .arg(Arg::with_name("Blocksize")
                               .short("b")
                               .long("blocksize")
                               .case_insensitive(true)
                               .takes_value(true)
                               .max_values(1)
                               .possible_values(&["K", "M", "G"])
                               .help("Sets the display blocksize to Kilobytes, Megabytes or Gigabytes. Default is Bytes."))
                          .arg(Arg::with_name("Hidden")
                               .short("h")
                               .long("hidden")
                               .possible_values(&["true", "false"])
                               .case_insensitive(true)
                               .help("Searches hidden folders. NOT YET IMPLEMENTED. CURRENTLY TRUE."))
                          .arg(Arg::with_name("Print")
                                .short("p")
                                .long("print")
                                .possible_values(&["U", "S"])
                                .case_insensitive(true)
                                .takes_value(true)
                                .help("Print Unique or Shared files.")
                            )
                          .get_matches();

    let display_power = match arguments.value_of("Blocksize").unwrap_or(""){"K" => 1, "M" => 2, "G" => 3, _ => 0};
    let blocksize = match arguments.value_of("Blocksize").unwrap_or(""){"K" => "Kilobytes", "M" => "Megabytes", "G" => "Gigabytes", _ => "Bytes"};
    let display_divisor =  1024u64.pow(display_power);
    let mut directory_results = Vec::new();
    let mut thread_handles = Vec::new();
    for arg in arguments.values_of("directories").unwrap().into_iter(){
        let arg_str = String::from(arg);
        thread_handles.push(thread::spawn(move|| -> Result<HashSet<Fileinfo>, io::Error> {
            recurse_on_dir(Path::new(&arg_str), HashSet::new())
        }));
    }
    for handle in thread_handles {
        directory_results.push(handle.join().unwrap().unwrap());
    }
    let complete_files: HashSet<Fileinfo> = directory_results.to_vec().into_iter().fold(HashSet::new(), |unifier, element| additive_union(unifier, element));
    let shared_files: HashSet<Fileinfo> = directory_results.to_vec().into_iter().fold(complete_files.clone(), |intersector, element| additive_intersection(intersector, element));
    println!("{} Total files: {} {}", complete_files.iter().fold(0, |sum, x| sum+x.file_paths.len()), complete_files.iter().fold(0, |sum, x| sum+(x.file_len*x.file_paths.len() as u64))/display_divisor, blocksize);
    println!("{} Total unique files: {} {}", complete_files.len(), complete_files.iter().fold(0, |sum, x| sum+x.file_len)/display_divisor, blocksize);
    println!("{} Total shared files: {} {}", shared_files.len(), shared_files.iter().fold(0, |sum, x| sum+x.file_len)/display_divisor, blocksize);
    match arguments.value_of("Print").unwrap_or(""){
        "U" => {println!("Unique Files"); complete_files.iter().for_each(|x| if(x.file_paths.len())==1{x.file_paths.iter().for_each(|y| println!("{}", y.to_str().unwrap()))});},
        "S" => {println!("Shared Files"); shared_files.iter().for_each(|x| println!("{}", x.file_paths[0].file_name().unwrap().to_str().unwrap()));},
        _ => {}};
    //println!("{:?} Files in the symmetric difference: {:?} {}", unique_files.len(), (unique_files.iter().fold(0, |sum, x| sum+x.file_len))/display_divisor, blocksize);
}

fn recurse_on_dir(current_dir: &Path, mut file_set: HashSet<Fileinfo>) -> Result<HashSet<Fileinfo>, io::Error>{
    for entry in fs::read_dir(current_dir)? {
        let item = entry?;
        if item.file_type()?.is_dir(){
            file_set = recurse_on_dir(&item.path(), file_set)?;
        } else if item.file_type()?.is_file(){
            let hash = hash_file(&item.path())?;
            match file_set.replace(Fileinfo{file_paths: vec![item.path()], file_hash: hash, file_len: item.metadata().unwrap().len()}) {
                Some(mut v) => {v.add_path(item.path()); file_set.replace(v);},
                None => {},
            }
        }
    }
    Ok(file_set)
}

fn hash_file(file_path: &Path) -> Result<u64, io::Error>{
    let mut hasher = DefaultHasher::new();
    match fs::File::open(file_path) {
        Ok(f) => {
            let buffer_reader = BufReader::new(f);
            for byte in buffer_reader.bytes() {
                hasher.write(&[byte.unwrap()]);
            }
            let hash = hasher.finish();
            Ok(hash)
        }
        Err(e) => {println!("Error:{} when opening {:?}. Skipping.", e, file_path); Err(e)}
    }
}

fn additive_union(mut output_hash: HashSet<Fileinfo>, burnable_hash: HashSet<Fileinfo>) -> HashSet<Fileinfo>{
    for new_record in burnable_hash.into_iter(){
        let new_paths = new_record.file_paths.to_vec();
        match output_hash.replace(new_record) {
            Some(mut old_record) => {new_paths.into_iter().for_each(|new_path| old_record.add_path(new_path)); output_hash.replace(old_record);},
            None => {},
        }
    }
    return output_hash;
}

fn additive_intersection(mut output_hash: HashSet<Fileinfo>, burnable_hash: HashSet<Fileinfo>) -> HashSet<Fileinfo>{
    let difference: HashSet<_> = burnable_hash.symmetric_difference(&output_hash).cloned().collect();
    for unique_record in difference.iter() {
        output_hash.remove(&unique_record);
    }
    //output_hash = additive_union(output_hash, burnable_hash);
    return output_hash;
}
