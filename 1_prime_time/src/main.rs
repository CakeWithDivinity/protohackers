use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Write},
    net::{Shutdown, TcpListener, TcpStream},
    thread,
};

#[derive(Deserialize, Debug)]
struct Request {
    method: String,
    number: f64,
}

#[derive(Serialize)]
struct Response {
    method: String,
    prime: bool,
}

impl Response {
    fn new(is_prime: bool) -> Response {
        Response {
            method: "isPrime".to_string(),
            prime: is_prime,
        }
    }
}

enum RequestError {
    Malformed,
}

fn main() {
    let listener = TcpListener::bind("192.168.178.85:25565").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_connection(&stream));
            }
            Err(_) => continue,
        }
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let mut lines = BufReader::new(stream).lines();

    loop {
        match lines.next() {
            Some(Ok(line)) => {
                println!("Received line: {:?}", line);

                match process_request_line(&line) {
                    Ok(result) => {
                        println!("Responding: {}", result);
                        stream.write_all(result.as_bytes()).unwrap();
                    }
                    Err(RequestError::Malformed) => {
                        println!("Input malformed");
                        stream.write_all("malformed".as_bytes()).unwrap();
                        stream.shutdown(Shutdown::Both).unwrap();
                        return;
                    }
                };
            }
            Some(Err(_)) | None => {
                stream.shutdown(Shutdown::Both).unwrap();
                return;
            }
        }
    }
}

fn process_request_line(line: &String) -> Result<String, RequestError> {
    let request: Request = match serde_json::from_str(line.as_str()) {
        Ok(value) => value,
        Err(_) => {
            return Err(RequestError::Malformed);
        }
    };

    println!("Could parse request {:?}", request);

    if request.method != "isPrime" {
        return Err(RequestError::Malformed);
    }

    let response = Response::new(is_prime(request.number));
    let mut response = serde_json::to_string(&response).unwrap();
    response += "\n";

    return Ok(response);
}

fn is_prime(number: f64) -> bool {
    if number <= 1.0 {
        return false;
    }

    let limit = (number.sqrt() + 1.0) as i64;
    let number = number as i64;

    for dividend in 2..limit{
        if number % dividend == 0 {
            return false;
        }
    }

    true
}
