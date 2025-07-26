use std::error::Error;
use std::io::Write;
use std::net::TcpStream;

pub fn connect(address: &str) -> Connection {
    let stream = TcpStream::connect(address).unwrap();
    Connection { stream }
}

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn send_message(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        let mut stream = self.stream.try_clone()?;
        stream.write_all(msg.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
