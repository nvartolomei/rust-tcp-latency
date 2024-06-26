use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

use anyhow::Context;

pub struct Listener {
    addr: SocketAddr,
}

impl Listener {
    pub fn new(addr: SocketAddr) -> Self {
        Listener { addr }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.addr)
            .with_context(|| format!("failed to bind to {}", self.addr))?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Accepted connection from {}", stream.peer_addr()?);
                    thread::spawn(move || {
                        handle_connection(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }

        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut read = [0; std::mem::size_of::<i64>()];
        match stream.read_exact(&mut read) {
            Ok(_n) => {
                let mut payload_len = [0; std::mem::size_of::<usize>()];
                stream.read_exact(&mut payload_len).unwrap();
                let payload_len = usize::from_be_bytes(payload_len);
                let mut payload = vec![0; payload_len];
                stream.read_exact(&mut payload).unwrap();

                stream.write_all(&read).unwrap();
            }
            Err(err) => {
                eprintln!("Failed to read from stream: {}", err);
                break;
            }
        }
    }
}
