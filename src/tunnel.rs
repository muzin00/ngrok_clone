use std::error::Error;
use std::io::{BufReader, Read, Write, copy};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

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

    pub fn on_connect<F>(&self, callback: F)
    where
        F: Fn(Connection) + Send + Sync + 'static,
    {
        let callback = Arc::new(callback);
        for stream in self.listener.incoming() {
            let callback = Arc::clone(&callback);
            thread::spawn(move || {
                callback(Connection {
                    stream: stream.unwrap(),
                });
            });
        }
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
        let mut reader = BufReader::new(&self.stream);
        let mut buffer = [0; 1024];
        let bytes_read = reader.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).into_owned())
    }

    pub fn relay_stream(&mut self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        self.stream.set_nonblocking(true)?;
        stream.set_nonblocking(true)?;

        let mut buffer = [0; 1024];

        loop {
            // 方向1: stream → self.stream
            match stream.read(&mut buffer) {
                Ok(0) => break, // 接続終了
                Ok(n) => {
                    self.stream.write_all(&buffer[..n])?;
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock {
                        break; // その他のエラー
                    }
                }
            }

            // 方向2: self.stream → stream
            match self.stream.read(&mut buffer) {
                Ok(0) => break, // 接続終了
                Ok(n) => {
                    stream.write_all(&buffer[..n])?;
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock {
                        break; // その他のエラー
                    }
                }
            }
        }

        Ok(())
    }

    pub fn try_clone(&self) -> Result<Connection, Box<dyn Error>> {
        let stream = self.stream.try_clone()?;
        Ok(Connection { stream })
    }
}
