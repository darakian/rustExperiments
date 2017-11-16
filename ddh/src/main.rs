use std::fs;

fn main() {
    println!("Hello, world!");
    let first_dir = String::from(".");
    read_dir(first_dir);
}

fn read_dir(current_dir: String) -> std::io::Result<()>{
    let mut files: Vec<std::fs::File> = Vec::new();
    let mut sub_directories: Vec<std::fs::DirEntry> = Vec::new();

    //Read files and directories
    for entry in fs::read_dir(current_dir)? {
        let item = entry?;
        println!("{:?}", item.path());
        sub_directories.push(item);
    }

    for entry in sub_directories::drain().collect()? {
        let item = entry?;
        println!("{:?}", item.path());
    }
    Ok(())
}
