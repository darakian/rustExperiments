use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let start_time = Instant::now();
    let (tx, rx) = channel();
    // Threaded one millisecond timer
    thread::spawn(move || {
    loop {
        thread::sleep(Duration::from_millis(1));
        tx.send("tick").unwrap();
    }
    });




    for entry in rx.iter() {
        println!("{:?} :: {:?}", Instant::now(), entry);
        if start_time.elapsed() >= Duration::from_secs(3) {break}
    }

}
