use std::{
    net::{TcpListener, TcpStream}, 
    io::{BufRead, BufReader, Write}, fs, thread, time::Duration
}; // Built-in module to listen to TCP streams


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let full_http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let request_header = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    println!("{:#?}", request_header);

    let (status_line, filename) = if request_header.contains(" / ") {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
   
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
 

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // Returns a new TcpListener instance

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}