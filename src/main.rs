use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
};

/// Dnaka:
///
/// The keyword here is framing, which basically means, you have some kind of
/// indicator to know when one message is fully read. That can be some length
/// prefix at the start of the message, some byte flag,... in your case the \r\n or \n delimiter.
/// Because each time you call read() you read some raw data from the
/// network stream, but that doesn't guarantee to be the full message.
/// Therefore, you have to find back the markers to detect when a message is considered complete.
fn receive_response(stream: &mut TcpStream) -> Vec<u8> {
    let mut buf = vec![0; 1024];
    loop {
        let bytes = match stream.read(&mut buf) {
            Ok(0) => b"SYSTEM: No more bytes left.",
            Ok(bytes_read) => &buf[..bytes_read],
            Err(e) => {
                if let ErrorKind::Interrupted = e.kind() {
                    b"SYSTEM: There is nothing else to do. Killing reader."
                } else {
                    eprintln!("Error: {}", e);
                    panic!("Encountered IOError.");
                }
            }
        };

        let mut iter = bytes.iter();
        match iter.position(|&b| char::from(b) == '2') {
            Some(index) => {
                let next = iter.nth(index + 1);
                let succeeding = iter.nth(index + 2);

                if next.is_none() || succeeding.is_none() {
                    return bytes.to_vec();
                }

                let next = char::from(next.unwrap().to_owned());
                let succeeding = char::from(succeeding.unwrap().to_owned());

                // Looks for 226 and 250. Both are responses to signify the transmission is
                // complete
                let end_of_conn_code = (next == '2' || next == '5') && (succeeding == '6' || succeeding == '0');
                if end_of_conn_code {
                    return bytes.to_vec();
                }

                continue;
            }
            None => bytes.to_vec(),
        };
    }
}

fn send_msg(stream: &mut TcpStream, msg: &str) {
    let msg = format!("{}\r\n", msg);
    stream.write_all(msg.as_bytes()).unwrap();
}

fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("ftp.gnu.org:21")?;
    _ = receive_response(&mut stream);

    send_msg(&mut stream, "USER anonymous");
    let response = receive_response(&mut stream);
    let response = String::from_utf8(response).unwrap();
    println!("{}", response);

    send_msg(&mut stream, "QUIT");

    Ok(())
}
