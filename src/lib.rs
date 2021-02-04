extern crate clap;
use clap::App;

use std::net::UdpSocket;

pub fn run() {
    App::new("Text Transfer")
        .version("0.1.2")
        .author("Abner K. <abnerkaizer@protonmail.com>")
        .about("Receive a text from a UDP socket and display it in a console.")
        .get_matches();

    let socket = match UdpSocket::bind("localhost:6789") {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Couldn't bind to socket, error: {}", e);
            std::process::exit(1);
        }
    };
    //socket.connect("localhost:6790").expect("connect function failed");
    let mut content = [0; 1024];
    match socket.recv(&mut content) {
        Ok(received) => {
            &content[..received];
        }
        Err(e) => {
            eprintln!("Couldn't receive content: {}", e);
            std::process::exit(1);
        }
    };

    let content = std::str::from_utf8(&content).expect("Failed to parse.");
    println!("{}", content);
}
