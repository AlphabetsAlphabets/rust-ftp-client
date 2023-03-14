#![feature(iter_collect_into)]

use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

fn send_msg(stream: &mut TcpStream, msg: &str) {
    let msg = format!("{}\r\n", msg);
    stream.write_all(msg.as_bytes()).unwrap();
}

// TODO: Handle errors
fn receive_response(stream: &mut TcpStream) -> String {
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

    response.iter().collect()
}

fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("ftp.gnu.org:21")?;

    println!("Logging in as anonymous.");
    send_msg(&mut stream, "USER anonymous");
    let response = receive_response(&mut stream);
    println!("{}", response);

    send_msg(&mut stream, "HELP");
    let response = receive_response(&mut stream);
    println!("{}", response);

    Ok(())
}
