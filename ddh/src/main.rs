use std::io;
use std::io::Read;
use std::io::BufReader;
use std::env;
use std::path::Path;
use std::collections::HashSet;
use std::fs::{self};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

fn main() {
    let args: Vec<String> = env::args().collect();
    //Check Args length
    if args.len() == 1 {
        //Print usage
        println!("{:?}: Missing Argument", env::args().nth(0).unwrap());
        println!("Usage: ddh dir_1 {{dir_2}}");
        return;
    } else if args.len() == 2 {
        let first_path = Path::new(&args[1]);
        let directory_result = recurse_on_dir(first_path);
        for entry in directory_result.unwrap().iter(){
            println!("{:x} >> {} bytes", entry.1, entry.2);
        }
    } else if args.len() == 3 {
        let first_path = Path::new(&args[1]);
        let second_path = Path::new(&args[2]);
        let first_directory_result = recurse_on_dir(first_path).unwrap();
        let second_directory_result = recurse_on_dir(second_path).unwrap();
        let common_files = first_directory_result.intersection(&second_directory_result);
        let symmetric_difference = first_directory_result.symmetric_difference(&second_directory_result);
        let common_files_size = common_files.fold(0, |sum, x| sum+x.2);
        let difference_size = symmetric_difference.fold(0, |sum, x| sum+x.2);
        println!("{} bytes in common\n{} bytes difference", common_files_size, difference_size);
    } else {
        //Wtf? How are we here?
        println!("How are we here?");
        println!("Usage: ddh dir_1 {{dir_2}}");
        return;
    }
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
