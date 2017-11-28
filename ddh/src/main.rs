use std::io;
use std::io::Read;
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
    let mut files: HashSet<(String, u64, u64)> = HashSet::new();

    //Read files and directories
    for entry in fs::read_dir(current_dir)? {
        let item = entry?;
        if item.file_type()?.is_dir(){
            let additional_files = recurse_on_dir(&item.path())?;
            files.extend(additional_files);
        } else if item.file_type()?.is_file(){
            let mut file = fs::File::open(item.path())?;
            let mut file_contents = Vec::with_capacity(file.metadata().unwrap().len() as usize);
            file.read_to_end(&mut file_contents)?;
            let mut hasher = DefaultHasher::new();
            hasher.write(&file_contents);
            let hash = hasher.finish();
            files.insert((item.file_name().into_string().unwrap(), hash, file.metadata().unwrap().len()));
        }
    }

    files.shrink_to_fit();
    return Ok(files)
}
