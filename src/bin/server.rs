use ngrok_clone::tunnel::TunnelServer;

use std::net::TcpListener;

fn main() {
    let server = TunnelServer::listen("127.0.0.1:8080").unwrap();
    let web_listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    server.on_connect(move |mut conn| {
        println!("新しい接続が確立されました");

        for stream in web_listener.incoming() {
            println!("TCP通信を中継します");
            conn.relay_stream(stream.unwrap()).unwrap();
            println!("TCP通信を中継しました");
        }
    });
}
