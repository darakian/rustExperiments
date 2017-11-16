use std::fs;
use blake2::{Blake2b, Digest};

fn main() {
    let first_dir = String::from(".");
    recurse_on_dir(first_dir);
}

fn recurse_on_dir(current_dir: String) -> std::io::Result<()>{
    let mut files: Vec<String> = Vec::new();
    let mut sub_directories: Vec<std::fs::DirEntry> = Vec::new();

    //Read files and directories
    for entry in fs::read_dir(current_dir)? {
        let item = entry?;
        if (item.file_type()?.is_file()){
            //println!("{:?} is a file", item.path());
            files.push(item.file_name().into_string().unwrap());
        } else{
            //println!("{:?} is a dir", item.path());
            sub_directories.push(item);
        }
    }

    //Print current files and hashes
    for entry in files.iter() {
        let the_file = std::fs::File::open(entry);

        //let item = entry?;
        println!("{:?} is a file", entry);
    }

    for entry in sub_directories.iter(){
        println!("{:?} is a dir", entry.path());
    }
    Ok(())
}
