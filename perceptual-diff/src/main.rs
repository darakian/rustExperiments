extern crate image;
extern crate img_hash;
use clap::{Parser, ValueEnum}; 
use img_hash::{HasherConfig, HashAlg};

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
    let mut matches = Vec::new();

    let the_image = image::open(args.image).unwrap();
    let source_hash = image_hasher.hash_image(&the_image);
    for other_image_path in args.others.iter() {
        let other_image = image::open(other_image_path).unwrap();
        let other_hash = image_hasher.hash_image(&other_image);
        //println!("{:?}", source_hash.dist(&other_hash));
        if source_hash.dist(&other_hash) < args.dist {
            matches.push(other_image_path);
        }
    }

    println!("Matches:");
    for close_enough in matches.iter(){
        println!("{:?}", close_enough);
    }
 }