use std::{
    net::{TcpListener, TcpStream}, 
    io::{BufRead, BufReader}
}; // Built-in module to listen to TCP streams


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
}
 

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // Returns a new TcpListener instance

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!");
        handle_connection(stream);
    }
}