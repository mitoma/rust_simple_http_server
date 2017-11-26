use std::cmp::Ordering;
use std::net::TcpListener;
use std::io::{BufRead, BufReader};

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = stream;
                let mut buf = String::new();
                let mut stream = BufReader::new(stream);
                'request: loop {
                    let len = stream.read_line(&mut buf);
                    match len {
                        Ok(len) => match len.cmp(&0) {
                            Ordering::Equal => {
                                println!("len = 0");
                                break 'request;
                            }
                            Ordering::Greater => println!("{}", buf),
                            _ => {}
                        },
                        Err(_) => break 'request,
                    }
                }
            }
            Err(_) => {}
        }
    }
}

fn main() {
    println!("Hello server.");
    start_server();
}
