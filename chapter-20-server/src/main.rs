use std::net::TcpListener; // Built-in module to listen to TCP streams

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // Returns a new TcpListener instance

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
