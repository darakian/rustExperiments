use std::io;
use std::env;
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs::{self};
extern crate sha2;
use sha2::{Sha256, Digest};
extern crate generic_array;

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
        let first_directory_result: HashSet<(String, generic_array::GenericArray<u8, generic_array::typenum::U32>, u64)> = HashSet::from_iter(recurse_on_dir(first_path).unwrap());
        let second_directory_result: HashSet<(String, generic_array::GenericArray<u8, generic_array::typenum::U32>, u64)> = HashSet::from_iter(recurse_on_dir(first_path).unwrap());
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

fn recurse_on_dir(current_dir: &Path) -> Result<Vec<(String, generic_array::GenericArray<u8, generic_array::typenum::U32>, u64)>, io::Error>{
    let mut files: Vec<(String, generic_array::GenericArray<u8, generic_array::typenum::U32>, u64)> = Vec::new();
    let mut sub_directories: Vec<Box<Path>> = Vec::new();
    //Read files and directories
    for entry in fs::read_dir(current_dir)? {
        let item = entry?;
        if item.file_type()?.is_file(){
            let mut file = fs::File::open(item.path())?;
            let hash = Sha256::digest_reader(&mut file)?;
            files.push((item.file_name().into_string().unwrap(), hash, item.metadata().unwrap().len()));
        } else{
            sub_directories.push(item.path().into_boxed_path());
        }
    }
    for sub_dir in sub_directories.iter(){
        let additional_files = recurse_on_dir(&*sub_dir)?;
        files.extend(additional_files.iter().cloned());
    }
    return Ok(files)
}
