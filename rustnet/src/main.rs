//Std imports
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:3401").expect("couldn't bind to address");
    socket.send_to(&[0; 10], "8.8.8.8:4242").expect("couldn't send data");
}
