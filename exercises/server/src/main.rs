use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // TCP server binding to localhost on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server: http://127.0.0.1:8080");

    // Infinite loop to accept incoming connections
    for stream in listener.incoming() {
        // Spawn a new thread for each connection
        std::thread::spawn(|| {
            // Handle incoming connection
            let mut stream = stream.unwrap();

            // Copy of the buffer, [0; 512] is an array of 512 bytes, it's the size of the buffer
            let mut buf = [0; 512];

            // Block until data arrives.
            stream.read(&mut buf).unwrap();

            // Response HTTP simple message
            let res = "HTTP/1.1 200 OK\r\n\r\nMessage from server";

            // Envía la respuesta al cliente
            stream.write_all(res.as_bytes()).unwrap();
        });
    }
} // SERVER. Basic example of a TCP server in Rust that handles incoming connections

// Benefitcts of using threads:
// 1. Concurrency: Threads allow the server to handle multiple connections simultaneously.
// 2. Resource Utilization: Threads can better utilize CPU resources by performing tasks in parallel.
// 3. Isolation: Each thread has its own stack, which helps in isolating tasks and preventing interference between them.
