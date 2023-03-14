#![feature(iter_collect_into)]

use std::{
    io::{Read, Write, ErrorKind},
    net::TcpStream,
};

fn send_msg(stream: &mut TcpStream, msg: &str) {
    let msg = format!("{}\r\n", msg);
    stream.write_all(msg.as_bytes()).unwrap();
}

fn receive_response(stream: &mut TcpStream) -> String {
    let mut buf = vec![0; 1024];
    let bytes = match stream.read(&mut buf) {
        Ok(0) => b"SYSTEM: No more bytes left.",
        Ok(bytes_read) =>  {
            if bytes_read > buf.len() {
                let over_by = bytes_read - buf.len();
                let surplus = vec![0; over_by];
                buf.extend(surplus)
            }

            &buf[..bytes_read]
        },
        Err(e) => {
            if let ErrorKind::Interrupted = e.kind() {
                b"SYSTEM: There is nothing else to do. Killing reader."
            } else {
                eprintln!("Error: {}", e);
                panic!("Encountered IOError.");
            }
        }
    };

    match std::str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => "Invalid UTF-8 character.".to_string(),
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("ftp.gnu.org:21")?;

    send_msg(&mut stream, "USER anonymous");
    let response = receive_response(&mut stream);
    println!("{}", response);

    send_msg(&mut stream, "HELP");
    let response = receive_response(&mut stream);
    println!("{}", response);

    send_msg(&mut stream, "CWD");
    let response = receive_response(&mut stream);
    println!("{}", response);

    send_msg(&mut stream, "QUIT");

    Ok(())
}
