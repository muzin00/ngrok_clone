use std::io::copy;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let web_listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    loop {
        let (stream, _) = listener.accept().unwrap();

        for web_stream in web_listener.incoming() {
            let tunnel_stream_clone = stream.try_clone().unwrap();
            let web_stream_clone = web_stream.unwrap().try_clone().unwrap();
            thread::spawn(move || {
                println!("通信を中継します");
                relay_connection(web_stream_clone, tunnel_stream_clone);
            });
        }
    }
}

fn relay_connection(mut from_stream: TcpStream, mut to_stream: TcpStream) {
    copy(&mut from_stream, &mut to_stream).unwrap();
}
