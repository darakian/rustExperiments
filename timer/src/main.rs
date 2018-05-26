use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    let sixteen = Duration::from_millis(50);
    let start_time = Instant::now();
    while Instant::now().duration_since(start_time) <= sixteen{
            println!("Hello, world!");
            sleep(Duration::from_millis(1));

    }
}
