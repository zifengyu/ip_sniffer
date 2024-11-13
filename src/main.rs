use clap::Parser;
use std::net::TcpStream;
use std::time::Duration;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short = 'j', default_value_t = 8)]
    threads: u16,

    ip: String,
}

const MAX_PORT: u16 = 65535;

fn main() {
    let args = Arguments::parse();
    scan(&args.ip, 1, MAX_PORT);
}

fn scan(ip: &str, from_port: u16, to_port: u16) {
    for port in from_port..=to_port {
        match format!("{}:{}", ip, port).parse::<SocketAddr>() {
            Ok(address) => {
                if TcpStream::connect_timeout(&address, Duration::from_millis(100)).is_ok() {
                    println!("Port {} is open", port);
                }
            },
            Err(err) => println!("Invalid IP address: {}", err),
        }
        // if let Ok(address) = format!("{}:{}", ip, port).parse() {
        //     println!("Scanning port {}", port);
        // }

        // match TcpStream::connect_timeout(&address.parse().expect("Invalid IP address"), Duration::from_millis(100)) {
        //     Ok(_) => println!("Port {} is open", port),
        //     Err(_) => continue,
        // }
    }
}