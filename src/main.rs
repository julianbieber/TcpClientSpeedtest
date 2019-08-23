#![feature(cfg_target_has_atomic)]
#[cfg(target_has_atomic)]

#[macro_use]
extern crate structopt;
use structopt::StructOpt;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

fn handle_client(mut stream: TcpStream) -> std::io::Result<usize> {
    let mut data: [u8; 1024] = [0; 1024];
    loop {
        stream.read(&mut data)?;
    }
    Ok(0)
}

fn short_read(mut stream: TcpStream, mb: usize) -> std::io::Result<usize> {
    let mut data: [u8; 1024] = [0; 1024];
    for _ in 0..1024 * mb {
        stream.read(&mut data)?;
    }

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
    new_connections: bool,
    #[structopt(short, long, default_value = "1")]
    mb: usize
}

fn main() {
    let options: Options = Options::from_args();
    static mb_counter: AtomicUsize = AtomicUsize::new(0);
    let mut threads = Vec::new();
    if options.new_connections {
        for _ in 0..options.threads {
            let o = options.clone();
            threads.push(std::thread::spawn(move || loop {
                TcpStream::connect(o.address.as_str()).map(|stream| {
                    short_read(stream, o.mb);
                    let c = mb_counter.fetch_add(1, Ordering::SeqCst);
                    println!("{}", c);
                });
            }));
        }
    } else {
        for _ in 0..options.threads {
            let o = options.clone();
            threads.push(std::thread::spawn(move || {
                let stream = TcpStream::connect(o.address.as_str()).unwrap();
                handle_client(stream).unwrap()
            }));
        }
    }

    for thread in threads.into_iter() {
        thread.join();
    }
}
