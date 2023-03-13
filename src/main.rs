#![feature(iter_collect_into)]

use std::{
    io::{self, Read, Write},
    net::TcpStream,
    time::Duration,
};

fn msg_server(stream: &mut TcpStream, msg: &[u8]) -> Result<(), std::io::Error> {
    stream.write_all(msg)
}

// TODO: Handle errors
fn receive_response(stream: &mut TcpStream) -> Vec<char> {
    let mut response = vec![];
    let mut iter = stream.bytes();
    while let Some(byte) = iter.next() {
        if let Err(e) = byte {
            // So far have not encountered this situation yet.
            eprintln!("{}", e);
            todo!("Find out more about the error.");
        }

        let char = char::from(byte.unwrap());

        // Check for CR
        if char == '\r' {
            let next_byte = iter.next().unwrap();
            if let Err(e) = next_byte {
                // So far have not encountered this situation yet.
                eprintln!("{}", e);
                todo!("Find out more about the error.");
            }

            let next_char = char::from(next_byte.unwrap());

            // Check for LF
            if next_char == '\n' {
                // End of response
                response.push(next_char);
                break;
            }

            // If just CR no LF. Continue.
            response.push(next_char);
        } else {
            response.push(char);
        }
    }

    response
}

fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("ftp.gnu.org:21")?;

    let response = receive_response(&mut stream);
    let response: String = response.iter().collect();
    println!("Server:\n{}", response);

    println!("Logging in as anonymous.");
    msg_server(&mut stream, b"USER anonymous")?;
    let response = receive_response(&mut stream);
    let response: String = response.iter().collect();
    println!("Server:\n{}", response);

    Ok(())
}
