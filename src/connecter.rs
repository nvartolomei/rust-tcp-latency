use std::{
    io::{Read, Write},
    net::SocketAddr,
    time::Duration,
};

use anyhow::Context;

pub struct Connecter {
    remote_addr: SocketAddr,
    send_interval: Duration,

    seq_num: u64,
}

impl Connecter {
    pub fn new(remote_addr: SocketAddr, send_interval: Duration) -> Self {
        Connecter {
            remote_addr,
            send_interval,
            seq_num: 0,
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let stream = std::net::TcpStream::connect(self.remote_addr)?;
        stream
            .set_nodelay(true)
            .with_context(|| "failed to set_nodelay(true)")?;
        let mut ostream = std::io::BufWriter::new(&stream);
        let mut istream = std::io::BufReader::new(&stream);

        let mut read_buf = [0; std::mem::size_of::<u64>()];
        loop {
            let start = std::time::Instant::now();
            let b = self.seq_num.to_be_bytes();
            ostream.write_all(&b)?;
            ostream.flush()?;

            istream.read_exact(&mut read_buf)?;
            let seq_num = u64::from_be_bytes(read_buf);
            if seq_num != self.seq_num {
                eprintln!(
                    "Error: expected seq_num {} but got {}",
                    self.seq_num, seq_num
                );
                std::process::exit(1);
            }
            let elapsed = start.elapsed();

            println!("Latency: {}us", elapsed.as_micros());

            self.seq_num += 1;

            if elapsed < self.send_interval {
                std::thread::sleep(self.send_interval - elapsed);
            }
        }
    }
}
