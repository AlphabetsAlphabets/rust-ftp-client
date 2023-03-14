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
    let mut buffer = vec![0u8; 1024 * 8]; // this should be more than enough for a message

    let data = match stream.read(&mut buffer) {
        Ok(0) => panic!("connection closed"),
        Ok(n) => &buffer[..n],
        Err(e) => panic!("{e}"),
    };

    std::str::from_utf8(data).unwrap().to_string()
}

fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("ftp.gnu.org:21")?;

    println!("Logging in as anonymous.");
    send_msg(&mut stream, "USER anonymous");
    let response = receive_response(&mut stream);
    println!("\nServer:\n{}", response);

    send_msg(&mut stream, "HELP");
    let response = receive_response(&mut stream);
    println!("\nServer:\n{}", response);

    Ok(())
}
