use std::io;
use std::io::Read;
use std::hash::Hash;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashSet;
use std::fs::{self};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

extern crate clap;
use clap::{Arg, App};

#[derive(Clone)]
struct Fileinfo{
    file_name: String,
    file_path: String,
    file_hash: u64,
    file_len: u64,
}
impl PartialEq for Fileinfo {
    fn eq(&self, other: &Fileinfo) -> bool {
        self.file_hash == other.file_hash
    }
}
impl Eq for Fileinfo {}

impl Hash for Fileinfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_hash.hash(state);
    }
}

fn main() {
    let arguments = App::new("Directory Difference hTool")
                          .version("0.1.0")
                          .author("Jon Moroney jmoroney@cs.ru.nl")
                          .about("Compare and contrast directories")
                          .arg(Arg::with_name("directories")
                               .short("d")
                               .long("directories")
                               .case_insensitive(true)
                               .value_name("FILE")
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
                               .possible_values(&["K", "M", "G"])
                               .help("Sets the display blocksize to Kilobytes, Megabytes or Gigabytes. Default is Bytes."))
                          .arg(Arg::with_name("Hidden")
                               .short("h")
                               .long("hidden")
                               .possible_values(&["true", "false"])
                               .case_insensitive(true)
                               .help("Searches hidden folders. NOT YET IMPLEMENTED. CURRENTLY TRUE"))
                          .get_matches();

    let display_power = match arguments.value_of("Blocksize").unwrap_or(""){"K" => 1, "M" => 2, "G" => 3, _ => 0};
    let blocksize = match arguments.value_of("Blocksize").unwrap_or(""){"K" => "Kilobytes", "M" => "Megabytes", "G" => "Gigabytes", _ => "Bytes"};
    let display_divisor =  1024u64.pow(display_power);
    let directory_results: Vec<_> = arguments.values_of("directories").unwrap().into_iter().map(|x| recurse_on_dir(Path::new(&x)).unwrap()).collect();
    let complete_files = directory_results.iter().fold(HashSet::new(), |unity, element| unity.union(&element).cloned().collect());
    let common_files = directory_results.iter().fold(complete_files.clone(), |intersection_of_elements, element| intersection_of_elements.intersection(element).cloned().collect());
    let unique_files = directory_results.iter().fold(complete_files.clone(), |sym_diff, element| sym_diff.symmetric_difference(element).cloned().collect());
    println!("{:?} Total unique files: {:?} {}", complete_files.len(), complete_files.iter().fold(0, |sum, x| sum+x.file_len)/display_divisor, blocksize);
    println!("{:?} Files in the intersection: {:?} {}", common_files.len(), common_files.iter().fold(0, |sum, x| sum+x.file_len)/display_divisor, blocksize);
    println!("{:?} Files in the symmetric difference: {:?} {}", unique_files.len(), (unique_files.iter().fold(0, |sum, x| sum+x.file_len))/display_divisor, blocksize);
    for item in common_files {
        println!("{}", item.file_name);
    }
}

fn recurse_on_dir(current_dir: &Path) -> Result<HashSet<Fileinfo>, io::Error>{
    let mut file_set: HashSet<Fileinfo> = HashSet::new();
    for entry in fs::read_dir(current_dir)? {
        let item = entry?;
        if item.file_type()?.is_dir(){
            let additional_files = recurse_on_dir(&item.path())?;
            file_set.extend(additional_files);
        } else if item.file_type()?.is_file(){
            let hash = hash_file(&item.path())?;
            file_set.insert(Fileinfo{file_name:item.file_name().into_string().unwrap(), file_path: String::from(item.path().to_str().unwrap()), file_hash: hash, file_len: item.metadata().unwrap().len()});
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
