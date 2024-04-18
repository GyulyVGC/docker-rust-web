use std::{io::prelude::*, net::TcpStream, thread::sleep, time::Duration};

fn main() {
    ctrlc::set_handler(move || {
        println!("Received termination signal; exiting...");
        std::process::exit(130);
    })
    .expect("Failed to set ctrlc handler");

    let request = "GET / HTTP/1.1";

    loop {
        let mut stream = TcpStream::connect("rust-web-server:7878").unwrap();
        stream.write_all(request.as_bytes()).unwrap();
        sleep(Duration::from_secs(5));
    }
}
