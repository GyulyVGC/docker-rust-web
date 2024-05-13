use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use gethostname::gethostname;

fn main() {
    ctrlc::set_handler(move || {
        println!("Received termination signal; exiting...");
        std::process::exit(130);
    })
    .expect("Failed to set ctrlc handler");

    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        println!("Responding with hello.html");
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        println!("Responding with 404.html");
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut contents = fs::read_to_string(filename).unwrap();
    let host_string = format!("\nFrom host: {}", gethostname().to_string_lossy().to_string());
    contents.push_str(&host_string);
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
