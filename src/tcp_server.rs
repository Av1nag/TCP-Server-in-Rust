use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub fn handle_client(mut stream: TcpStream) -> f64 {
    let mut data: [u8; 8] = [0 as u8; 8];
    let mut total_bytes_read = 0;
    let mut message: f64 = 0.0;

    while total_bytes_read < 8 {
        match stream.read(&mut data[total_bytes_read..]) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Connection closed by client");
                    break;
                }
                total_bytes_read += bytes_read;
            }
            Err(e) => {
                println!(
                    "An error occurred, terminating connection with {} {}",
                    stream.peer_addr().unwrap(),
                    e
                );
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }

    if total_bytes_read == 8 {
        message = f64::from_ne_bytes(data);
        println!("{}", &message);
        // echo everything!
        stream.write(&data).unwrap();
    }
    message
}
