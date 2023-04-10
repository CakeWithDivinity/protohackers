use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Debug)]
struct InsertMessage {
    timestamp: i32,
    price: i32,
}

#[derive(Debug)]
struct QueryMessage {
    min_time: i32,
    max_time: i32,
}

struct TransactionDatabase {
    transaction_list: HashMap<i32, i32>,
}

impl TransactionDatabase {
    fn insert(&mut self, message: InsertMessage) {
        self.transaction_list
            .insert(message.timestamp, message.price);
    }

    fn query(&self, message: QueryMessage) -> i32 {
        let (sum, count) = self
            .transaction_list
            .iter()
            .filter(|entry| *entry.0 >= message.min_time && *entry.0 <= message.max_time)
            .map(|(_, value)| *value as i64) 
            .fold((0, 0), |acc, curr| (acc.0 + curr, acc.1 + 1));

        println!("Sum: {}, Count: {}", sum, count);

        if count == 0 {
            return 0;
        }

        return (sum / count) as i32;
    }
}

impl Default for TransactionDatabase {
    fn default() -> Self {
        TransactionDatabase {
            transaction_list: HashMap::new(),
        }
    }
}

fn main() {
    let listener = TcpListener::bind("192.168.178.85:25565").unwrap();

    for stream in listener.incoming() {
        thread::spawn(move || handle_connection(stream.unwrap()));
    }
}
// Byte:  |  0  |  1     2     3     4  |  5     6     7     8  |
// Type:  |char |         int32         |         int32         |
fn handle_connection(mut stream: TcpStream) {
    let mut transaction_db = TransactionDatabase::default();

    loop {
        let mut buffer = [0; 9];

        if let Err(_) = stream.read_exact(&mut buffer) {
            continue;
        };

        match buffer[0] {
            b'I' => {
                let message = InsertMessage {
                    timestamp: i32::from_be_bytes(buffer[1..=4].try_into().unwrap()),
                    price: i32::from_be_bytes(buffer[5..].try_into().unwrap()),
                };

                transaction_db.insert(message);
            }
            b'Q' => {
                let message = QueryMessage {
                    min_time: i32::from_be_bytes(buffer[1..=4].try_into().unwrap()),
                    max_time: i32::from_be_bytes(buffer[5..].try_into().unwrap()),
                };

                stream
                    .write_all(&transaction_db.query(message).to_be_bytes())
                    .unwrap();
            }
            x => {
                println!("Incorrect message type {:?} received", x);
                continue;
            }
        };
    }
}
