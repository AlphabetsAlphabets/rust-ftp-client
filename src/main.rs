#![feature(iter_collect_into)]

use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

fn msg_server(stream: &mut TcpStream, msg: &[u8]) -> Result<(), std::io::Error> {
    stream.write_all(msg)
}

// TODO: Handle errors
fn receive_response(stream: &mut TcpStream) -> Vec<u8> {
    let mut buf = vec![0; 1024 * 2];
    let mut response = vec![];

    // Will only return Err if no bytes are read.
    let bytes_read = stream.read(&mut buf).unwrap();
    response.extend_from_slice(&buf[..bytes_read]);

    // Response code is 3 digit long
    let mut is_multiline = false;
    for char in &response[..3] {
        let value = char::from(char.clone());
        is_multiline = value.is_ascii_digit();
    }

    is_multiline = response[4].to_string() == "-";
    if is_multiline {
        // continue reading
    }

    response
}

fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("ftp.gnu.org:21")?;

    loop {
        let response = receive_response(&mut stream);
        let s = String::from_utf8_lossy(&response);

        println!(">>>> {}", s);
        let mut command = String::new();
        io::stdin().read_line(&mut command)?;

        if command.trim() == "q" {
            break;
        }

        command = format!("{}\r\n", command.trim());
        msg_server(&mut stream, command.as_bytes())?;
    }

    Ok(())
}
