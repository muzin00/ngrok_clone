use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        thread::spawn(move || {
            handle_connection(stream.unwrap());
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut msg = String::new();
    stream.read_to_string(&mut msg).unwrap();
    println!("Received message: {}", msg);
}
