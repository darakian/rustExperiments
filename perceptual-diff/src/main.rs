extern crate image;
extern crate img_hash;
use rayon::prelude::*;
use walkdir::WalkDir;
use std::path::PathBuf;
use clap::{Parser, ValueEnum}; 
use img_hash::{HasherConfig, HashAlg};
use crossbeam_channel::unbounded;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, short, num_args(0..=1), required = true)]
    image: String,

    #[arg(short, long("distance"), default_value_t = 1)]
    dist: u32,

    #[arg(short, long("others"), value_delimiter(' '), num_args(1..), required = true)]
    others: Vec<String>,
}


 fn main() {
    let args = Args::parse();
    let image_hasher = HasherConfig::new().to_hasher();

    let the_image = image::open(args.image).unwrap();
    let source_hash = image_hasher.hash_image(&the_image);
    let (path_sender, path_recv) = unbounded();
    args.others.par_iter().for_each(|path| {
    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|x| x.path().to_owned())
        .collect();
        path_sender.send(files);
    });
    drop(path_sender);
    let paths: Vec<_> = path_recv.iter().flatten().collect();

    let (image_path_sender, image_path_recv) = unbounded();
    paths.into_par_iter().for_each(|path| {
        let image_hasher = HasherConfig::new().to_hasher();
        let other_image = image::open(&path).unwrap();
        let other_hash = image_hasher.hash_image(&other_image);
        if source_hash.dist(&other_hash) < args.dist {
            image_path_sender.send(path);
        }
    });
    drop(image_path_sender);

    println!("Matches:");
    for close_enough in image_path_recv.iter(){
        println!("{:?}", close_enough);
    }
 }