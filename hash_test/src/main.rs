use std::time::{Duration, Instant};
use std::fs::File;
use std::io::Read;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use blake3;
use seahash;

const HASH_ITERATIONS: u32 = 100;

#[derive(Debug)]
struct Args {
    help: bool,
    file: Option<String>,
}

fn main() {
    let mut args = pico_args::Arguments::from_env();
    // Arguments can be parsed in any order.
    let args = Args {
        help: args.contains(["-h", "--help"]),
        file: args.opt_value_from_str(["-f", "--file"]).unwrap_or(None),
    };

    match args.file {
        Some(f) => {
            println!("Hashing {:?}", f);
            bench_sip_hash(f.clone());
            bench_sea_hash(f.clone());
            bench_blake3_hash(f.clone());
            },
        None => {
            println!("No file");
            return},
    }
}

fn bench_sip_hash(file: String) {
    let mut file = File::open(file).expect("Error opening file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Error reading file");
    let mut total_time = Duration::new(0, 0);
    for _i in 0..HASH_ITERATIONS{
        let mut sip_hasher = DefaultHasher::new();
        let sip_now = Instant::now();
        sip_hasher.write(&buffer);
        sip_hasher.finish();
        total_time += sip_now.elapsed();
    }
    println!("Sip Hash took {:?} on average", total_time/HASH_ITERATIONS);
}

fn bench_sea_hash(file: String) {
    let mut file = File::open(file).expect("Error opening file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Error reading file");
    let mut total_time = Duration::new(0, 0);
    for _i in 0..HASH_ITERATIONS{
        let mut hasher = seahash::SeaHasher::new();
        let now = Instant::now();
        hasher.write(&buffer);
        hasher.finish();
        total_time += now.elapsed();
    }
    println!("Sea Hash took {:?} on average", total_time/HASH_ITERATIONS);
}

fn bench_blake3_hash(file: String) {
    let mut file = File::open(file).expect("Error opening file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Error reading file");
    let mut total_time = Duration::new(0, 0);
    for _i in 0..HASH_ITERATIONS{
        let mut hasher = blake3::Hasher::new();
        let now = Instant::now();
        hasher.update(&buffer);
        hasher.finalize();
        total_time += now.elapsed();
    }
    println!("Blake3 took {:?} on average", total_time/HASH_ITERATIONS);
}
