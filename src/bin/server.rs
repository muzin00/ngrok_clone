use std::io::Read;
use std::io::copy;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let web_listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    loop {
        let (stream, _) = listener.accept().unwrap();
        let tunnel_stream = stream.try_clone().unwrap();
        thread::spawn(move || {
            handle_connection(stream);
        });

        let (web_stream, _) = web_listener.accept().unwrap();
        thread::spawn(move || {
            relay_connection(web_stream, tunnel_stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut msg = String::new();
    stream.read_to_string(&mut msg).unwrap();
    println!("Received message: {}", msg);
}

fn relay_connection(mut from_stream: TcpStream, mut to_stream: TcpStream) {
    copy(&mut from_stream, &mut to_stream).unwrap();
}
