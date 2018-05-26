use std::time::{Duration, Instant};


fn main() {
    let sixteen = Duration::new(0, 16666666);
    let start_time = Instant::now();
    while Instant::now().duration_since(start_time) < Duration::new(1,0){
            println!("Hello, world!");
    }
}
