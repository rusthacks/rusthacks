use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

/* Run the executable and visit `http://localhost:8080` in your browser. */

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 8192];
    // Read only the first 8 kB of the buffer. Don't care about the rest.
    match stream.read(&mut buf) {
        Ok(_) => {
            // UTF-8 check the string, replace errors, convert into "clone on write" (CoW) object.
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
        },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    // Write a simple HTTP response, so that we can fool the browser.
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {     // infinite loop for incoming streams
        match stream {
            Ok(stream) => {
                // (Blindly, inefficiently) spawn a thread for each connection.
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => println!("Unable to connect: {}", e),
        }
    }
}
