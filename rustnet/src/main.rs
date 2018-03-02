//Std imports
use std::net::UdpSocket;
use std::time::{Duration, Instant};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:3401").expect("couldn't bind to address");
    //socket.connect("127.0.0.1:4242").expect("connect function failed");
    socket.set_nonblocking(true).unwrap();
    let ten_second = Duration::new(10, 0);
    let mut speed = 0;
    let now = Instant::now();
    loop {
        socket.send_to(&[0; 10], "127.0.0.1:4242").expect("couldn't send data");
        //socket.send(&[0; 10]).expect("couldn't send message");
        speed+=1;
        if now.elapsed()>=ten_second{break}
    }
    println!("{:?}", speed/10);
}
