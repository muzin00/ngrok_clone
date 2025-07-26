use ngrok_clone::tunnel::TunnelServer;

use std::net::TcpListener;
use std::thread;

fn main() {
    let server = TunnelServer::listen("127.0.0.1:8080").unwrap();
    let web_listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    loop {
        let conn = server.accept().unwrap();

        for stream in web_listener.incoming() {
            let mut conn = conn.try_clone().unwrap();
            thread::spawn(move || {
                println!("TCP通信を中継します");
                conn.relay_stream(stream.unwrap()).unwrap();
            });
        }
    }
}
