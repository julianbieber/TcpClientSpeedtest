#[macro_use]
extern crate structopt;
use structopt::StructOpt;

use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::time::Duration;

fn handle_client(mut stream: TcpStream) -> std::io::Result<usize> {
    let mut data: [u8; 1024] = [0; 1024];
    loop {
        stream.read(&mut data).unwrap();
    };
    Ok(0)
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "tcp-client")]
struct Options {
    #[structopt(short, long)]
    threads: i64,
    #[structopt(short, long)]
    address: String,
    #[structopt(short, long)]
    new_connections: bool
}


fn main() {
    let options: Options = Options::from_args();
    if options.new_connections {
        loop {
            let mut stream = TcpStream::connect(options.address.as_str()).unwrap();
            let mut data: [u8; 1024] = [0; 1024];
            for _ in 0..1024{
                stream.read(&mut data).unwrap();
            };
        }
    } else {
        for _ in 0..options.threads {
            let o = options.clone();
            std::thread::spawn(move || {
                let stream = TcpStream::connect(o.address.as_str()).unwrap();
                handle_client(stream).unwrap()
            });
        }
        loop{
            std::thread::sleep(Duration::from_secs(5));
        }
    }
}
