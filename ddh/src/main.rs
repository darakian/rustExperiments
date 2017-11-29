use std::io;
use std::io::Read;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashSet;
use std::fs::{self};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let arguments = App::new("Directory Difference hTool")
                          .version("0.1.0")
                          .author("Jon Moroney jmoroney@cs.ru.nl")
                          .about("Compare and contrast directories")
                          .arg(Arg::with_name("directories")
                               .short("d")
                               .long("directories")
                               .value_name("FILE")
                               .help("Directories to parse")
                               .min_values(2)
                               .required(true)
                               .takes_value(true)
                               .index(1))
                          .arg(Arg::with_name("Blocksize")
                               .short("b")
                               .long("blocksize")
                               .takes_value(true)
                               .possible_values(&["K", "M", "G"])
                               .help("Sets the display blocksize to Kilobyte, Megabyte, or Gigabyte. Default is Byte."))
                          .get_matches();

    let directories = arguments.values_of("directories").unwrap();
    let display_size = arguments.value_of("Blocksize").unwrap_or("");
    let display_power = match display_size{"K" => 1, "M" => 2, "G" => 3, _ => 0};
    let blocksize = match display_size{"K" => "Kilobytes", "M" => "Megabytes", "G" => "Gigabytes", _ => "Bytes"};
    let display_divisor =  1024u64.pow(display_power);
    let directory_results: Vec<_> = directories.into_iter().map(|x| recurse_on_dir(Path::new(&x)).unwrap()).collect();
    let complete_files = directory_results.iter().fold(HashSet::new(), |unity, element| unity.union(&element).cloned().collect());
    let common_files = directory_results.iter().fold(complete_files.clone(), |intersection_of_elements, element| intersection_of_elements.intersection(element).cloned().collect());
    println!("{:?} Total files with {:?} total {}", complete_files.len(), complete_files.iter().fold(0, |sum, x| sum+x.2)/display_divisor, blocksize);
    println!("{:?} Common files with {:?} common {}", common_files.len(), common_files.iter().fold(0, |sum, x| sum+x.2)/display_divisor, blocksize);
    println!("{:?} Unique files with {:?} unique {}", complete_files.len()-common_files.len(), (complete_files.iter().fold(0, |sum, x| sum+x.2)-common_files.iter().fold(0, |sum, x| sum+x.2))/display_divisor, blocksize);
}

fn recurse_on_dir(current_dir: &Path) -> Result<HashSet<(String, u64, u64)>, io::Error>{
    let mut file_set: HashSet<(String, u64, u64)> = HashSet::new();
    for entry in fs::read_dir(current_dir)? {
        let item = entry?;
        if item.file_type()?.is_dir(){
            let additional_files = recurse_on_dir(&item.path())?;
            file_set.extend(additional_files);
        } else if item.file_type()?.is_file(){
            let hash = hash_file(&item.path())?;
            file_set.insert((item.file_name().into_string().unwrap(), hash, item.metadata().unwrap().len()));
        }
    }
    return Ok(file_set)
}

fn hash_file(file_path: &Path) -> Result<u64, io::Error>{
    let mut hasher = DefaultHasher::new();
    let file = fs::File::open(file_path)?;
    let buffer_reader = BufReader::new(file);
    for byte in buffer_reader.bytes() {
        hasher.write(&[byte.unwrap()]);
    }
    let hash = hasher.finish();
    return Ok(hash);
}
