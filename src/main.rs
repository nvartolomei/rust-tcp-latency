use std::net::SocketAddr;

use clap::Parser;

mod connecter;
mod listener;

use connecter::Connecter;
use listener::Listener;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, help = "ip:port to listen on for incoming connections")]
    listen: Option<SocketAddr>,

    #[arg(long, help = "target ip:port to connect to")]
    connect: Option<SocketAddr>,

    #[arg(
        long,
        help = "seconds between sending messages",
        default_value = "0.05"
    )]
    send_interval: Option<f64>,

    #[arg(long, help = "number of bytes to send in each message (on top of the 16 bytes header)", default_value = "0")]
    send_bytes: usize,
}

fn main() {
    let args = Args::parse();

    if args.listen.is_some() && args.connect.is_some() {
        eprintln!("Error: --listen and --connect are mutually exclusive");
        std::process::exit(1);
    }

    if let Some(listen) = args.listen {
        println!("Starting rust-tcp-latency in listen mode on {}", listen);
        Listener::new(listen).run().unwrap();
    } else if let Some(connect) = args.connect {
        println!("Starting rust-tcp-latency in connect mode to {}", connect);
        Connecter::new(
            connect,
            std::time::Duration::from_secs_f64(args.send_interval.unwrap()),
            args.send_bytes,
        )
        .run()
        .unwrap();
    } else {
        eprintln!("Error: --listen or --connect is required");
        std::process::exit(1);
    }
}
