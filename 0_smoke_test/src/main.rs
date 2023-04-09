use smoke_test::ThreadPool;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("192.168.178.85:25565").unwrap();
    let thread_pool = ThreadPool::build(5).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    while let Ok(byte_count) = stream.read(&mut buffer) {
        if byte_count == 0 {
            break;
        }

        println!("Buffer: {:?}", buffer);

        stream.write(&buffer[..byte_count]).unwrap();
    }
}
