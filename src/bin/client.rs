use ngrok_clone::tunnel;

fn main() {
    let conn = tunnel::connect("127.0.0.1:8080");

    loop {
        let msg = conn.read_message().unwrap();
        println!("Received message: {}", msg);
    }
}
