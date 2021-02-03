use std::net::UdpSocket;


pub fn run() {
    
    let socket = match UdpSocket::bind("localhost:6789") {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Couldn't bind to socket, error: {}", e);
            std::process::exit(1);
        }
    };
    let mut content = [0;1024];
    socket.recv(&mut content).expect("Failed to receive.");
    let content = std::str::from_utf8(&content).expect("Failed to parse.");
    println!("{}",content);
}