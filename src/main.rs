use std::cmp::Ordering;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};

fn main() {
    println!("Hello server.");
    start_server();
}

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("port listen failed.");
    for stream in listener.incoming() {
        stream.map(echo);
    }
}

fn echo(stream: TcpStream) {
    let mut stream = stream.try_clone().expect("stream clone failed.");
    let mut reader = BufReader::new(stream.try_clone().expect("stream clone failed."));
    'request: loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(len) => match len.cmp(&0) {
                Ordering::Equal => {
                    println!("len = 0");
                    break 'request;
                }
                Ordering::Greater => {
                    let _ = stream.write(&buf.into_bytes());
                }
                _ => {}
            },
            Err(_) => break 'request,
        }
    }
}
