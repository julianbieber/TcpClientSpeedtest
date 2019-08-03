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

fn main() {
    for _ in 0..10 {
        std::thread::spawn(|| {
            let stream = TcpStream::connect("127.0.0.1:1111").unwrap();
            let data = "1".repeat(1024);
            handle_client(stream).unwrap()
        });
    }
    loop{
        std::thread::sleep(Duration::from_secs(5));
    }
}
