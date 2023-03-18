use std::io::{BufRead, BufReader, BufWriter, Write};
// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    drop(listener)
}

fn handle_connection(stream: TcpStream) {
    println!("Connection opened");
    loop {
        let mut message = String::new();
        let mut reader = BufReader::new(&stream);
        if let Ok(size) = reader.read_line(&mut message) {
            if size == 0 {
                println!("Connection closed");
                break;
            }
            println!("Server received: {} | Message size: {}", message, size);

            let mut writer = BufWriter::new(&stream);
            writer.write_all("+PONG\r\n".as_bytes()).expect("unable to write");
            writer.flush().expect("could not flush");
        }
    }
}
