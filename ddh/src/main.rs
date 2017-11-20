use std::io;
use std::env;
use std::path::Path;
use std::fs::{self, DirEntry};

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
        let directory_vector = recurse_on_dir(first_path);
        for entry in directory_vector.unwrap().iter(){
            println!("{:x}\t > {}", entry.1, entry.0);
        }
    } else if args.len() == 3 {
        let first_path = Path::new(&args[1]);
        let second_path = Path::new(&args[2]);
        let first_directory_vector = recurse_on_dir(first_path);
        let second_directory_vector = recurse_on_dir(first_path);
    } else {
        //Wtf? How are we here?
        println!("How are we here?");
        println!("Usage: ddh dir_1 {{dir_2}}");
        return;
    }

    //recurse_on_dir(first_dir);
}

fn recurse_on_dir(current_dir: &Path) -> Result<Vec<(String, generic_array::GenericArray<u8, generic_array::typenum::U32>)>, io::Error>{
    //println!("Entering directory: {:?}", current_dir.to_str().unwrap());
    let mut files: Vec<(String, generic_array::GenericArray<u8, generic_array::typenum::U32>)> = Vec::new();
    let mut sub_directories: Vec<Box<Path>> = Vec::new();

    //Read files and directories
    for entry in fs::read_dir(current_dir)? {
        let item = entry?;
        if item.file_type()?.is_file(){
            let mut file = fs::File::open(item.path())?;
            let hash = Sha256::digest_reader(&mut file)?;
            files.push((item.file_name().into_string().unwrap(), hash));
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
