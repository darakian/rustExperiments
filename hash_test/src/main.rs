use std::time::{Duration, Instant};
use std::fs::File;
use std::io::Read;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

use blake3;
use seahash;

#[derive(Debug)]
struct Args {
    help: bool,
    iters: Option<u32>,
    file: Option<String>,
}

fn main() {
    let mut hash_iterations: u32 = 100;
    let mut args = pico_args::Arguments::from_env();
    // Arguments can be parsed in any order.
    let args = Args {
        help: args.contains(["-h", "--help"]),
        iters: args.opt_value_from_str(["-i", "--iters"]).unwrap_or(None),
        file: args.opt_value_from_str(["-f", "--file"]).unwrap_or(None),
    };
    match args.iters{
        Some(i) => hash_iterations = i,
        None => {},
    }
    let mut results = HashMap::new();

    match args.file {
        Some(f) => {
            println!("Hashing {:?}", f);
            results.insert("Sip",
                bench_sip_hash(f.clone(), hash_iterations));
            results.insert("Sea",
                bench_sea_hash(f.clone(), hash_iterations));
            results.insert("Blake3",
                bench_blake3_hash(f.clone(), hash_iterations));
            },
        None => {
            println!("No file");
            return},
    }
    for token in results.iter(){
        println!("{:?}", token);
    }
}

fn bench_sip_hash(file: String, hash_iterations: u32) -> Vec<Duration> {
    let mut results = Vec::with_capacity(hash_iterations as usize);
    let mut file = File::open(file).expect("Error opening file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Error reading file");
    for _i in 0..hash_iterations{
        let mut sip_hasher = DefaultHasher::new();
        let now = Instant::now();
        sip_hasher.write(&buffer);
        sip_hasher.finish();
        results.push(now.elapsed());
    }
    results
}

fn bench_sea_hash(file: String, hash_iterations: u32) -> Vec<Duration> {
    let mut results = Vec::with_capacity(hash_iterations as usize);
    let mut file = File::open(file).expect("Error opening file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Error reading file");
    for _i in 0..hash_iterations{
        let mut hasher = seahash::SeaHasher::new();
        let now = Instant::now();
        hasher.write(&buffer);
        hasher.finish();
        results.push(now.elapsed());
    }
    results
}

fn bench_blake3_hash(file: String, hash_iterations: u32) -> Vec<Duration> {
    let mut results = Vec::with_capacity(hash_iterations as usize);
    let mut file = File::open(file).expect("Error opening file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Error reading file");
    for _i in 0..hash_iterations{
        let mut hasher = blake3::Hasher::new();
        let now = Instant::now();
        hasher.update(&buffer);
        hasher.finalize();
        results.push(now.elapsed());
    }
    results
}
