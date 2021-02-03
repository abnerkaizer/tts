extern crate clap;
use clap::{App};

mod udpsocket;

pub fn run() {
    App::new("Text Transfer")
        .version("0.1.1")
        .author("Abner K. <abnerkaizer@protonmail.com>")
        .about("Transfers a text file through UDP or TCP. UDP is the default one.")
        .get_matches();
    udpsocket::run();
}
