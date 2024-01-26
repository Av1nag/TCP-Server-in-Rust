pub mod tcp_server;
use chrono::prelude::*;
use std::{
    net::TcpListener,
    thread,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let connected_time = Local::now();
                println!(
                    "New connection from {} at {}",
                    stream.peer_addr().unwrap(),
                    connected_time.format("%Y-%m-%d %H:%M:%S")
                );

                thread::spawn(move || {
                    tcp_server::handle_client(stream);

                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        break;
    }
    drop(listener);
}
