use ngrok_clone::tunnel;

fn main() {
    let conn = tunnel::connect("127.0.0.1:8080").unwrap();

    loop {
        let msg = conn.read_message().unwrap();
        conn.send_message("HTTP/1.1 204 No Content\r\n\r\n")
            .unwrap();
        if msg.is_empty() {
            break;
        }
        println!("Received message: {}", msg);
    }
}
