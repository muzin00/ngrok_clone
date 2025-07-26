use std::error::Error;
use std::io::{Read, Write, copy};
use std::net::{TcpListener, TcpStream};

pub fn connect(address: &str) -> Result<Connection, Box<dyn Error>> {
    let stream = TcpStream::connect(address)?;
    Ok(Connection { stream })
}

pub struct TunnelServer {
    listener: TcpListener,
}

impl TunnelServer {
    pub fn listen(address: &str) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(address)?;
        Ok(TunnelServer { listener: listener })
    }

    pub fn accept(&self) -> Result<Connection, Box<dyn Error>> {
        let (stream, _) = self.listener.accept()?;
        Ok(Connection { stream })
    }
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

    pub fn read_message(&self) -> Result<String, Box<dyn Error>> {
        let mut buffer = [0; 1024];
        let mut stream = self.stream.try_clone()?;
        stream.read(&mut buffer)?;
        stream.flush()?;
        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }

    pub fn relay_stream(&mut self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        copy(&mut stream, &mut self.stream)?;
        Ok(())
    }

    pub fn try_clone(&self) -> Result<Connection, Box<dyn Error>> {
        let stream = self.stream.try_clone()?;
        Ok(Connection { stream })
    }
}
