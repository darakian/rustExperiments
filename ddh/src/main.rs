use std::fs::{self, DirEntry};
use std::path::Path;
use std::io;
use std::env;

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
        println!("Arg1: {:?}", &args[1]);
    } else if args.len() == 3 {
        let first_path = Path::new(&args[1]);
        let second_path = Path::new(&args[2]);
        println!("Arg1: {:?} Arg2: {:?}", &args[1], &args[2]);
    } else {
        //Wtf? How are we here?
        println!("How are we here?");
        println!("Usage: ddh dir_1 {{dir_2}}");
        return;
    }
    //recurse_on_dir(first_dir);
}

fn recurse_on_dir(current_dir: &Path) -> Result<Vec<String>, io::Error>{
    println!("Entering directory: {:?}", current_dir.to_str());
    let mut files: Vec<String> = Vec::new();
    let mut sub_directories: Vec<String> = Vec::new();

    //Read files and directories
    for entry in fs::read_dir(current_dir)? {
        let item = entry?;
        if (item.file_type()?.is_file()){
            //println!("{:?} is a file", item.path());
            files.push(item.file_name().into_string().unwrap());
        } else{
            //println!("{:?} is a dir", item.path());
            //sub_directories.push();
        }
    }
    //Print current files and hashes
    for entry in files.iter() {
        let the_file = std::fs::File::open(entry);
        //let item = entry?;
        println!("File: {:?}", entry);

        // let mut file = fs::File::open(&path)?;
        // let hash = Blake2b::digest_reader(&mut file)?;
        // println!("{:x}\t{}", hash, path);
    }
    for sub_dir in sub_directories.iter(){
        println!("Dir: {:?}", sub_dir);
        //recurse_on_dir();
    }
    return Ok(files)
}

fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
