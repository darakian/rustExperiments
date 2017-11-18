use std::fs;
use std::env;
use blake2::{Blake2b, Digest};

fn main() {
    let first_dir = env::args().nth(1).expect("Missing argument");
    let second_dir = env::args().nth(1).expect("Missing argument");
    recurse_on_dir(first_dir);
}

fn recurse_on_dir(current_dir: String) -> std::io::Result<()>{
    println!("Entering directory: {:?}", current_dir);
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
            sub_directories.push();
        }
    }
    //Print current files and hashes
    for entry in files.iter() {
        let the_file = std::fs::File::open(entry);
        //let item = entry?;
        println!("File: {:?}", entry.file_name().into_string().unwrap());

        // let mut file = fs::File::open(&path)?;
        // let hash = Blake2b::digest_reader(&mut file)?;
        // println!("{:x}\t{}", hash, path);
    }
    for sub_dir in sub_directories.iter(){
        println!("Dir: {:?}", sub_dir.file_name().into_string().unwrap());
        recurse_on_dir();
    }
    Ok(())
}

fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
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
