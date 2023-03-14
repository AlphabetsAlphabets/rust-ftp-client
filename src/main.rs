use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
};

fn receive_response(stream: &mut TcpStream) -> String {
    let mut buf = vec![0; 1024];
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

    let mut msg = vec![];
    let mut cr = false;
    let mut lf = false;
    for byte in bytes {
        let c = char::from(*byte);

        if c == '\n' && cr {
            lf = true;
        } else if c == '\r' {
            cr = true;
        }

        msg.push(c);

        // This cuts off prematurely. A new line is always \r\n. I need to find the actual \r\n,
        // so check what the next thing item after \r\n is. Which means iterators.
        if cr && lf {
            break;
        }
    }

    let msg: String = msg.iter().collect();
    msg
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
    println!("{}", response);

    send_msg(&mut stream, "HELP");
    let response = receive_response(&mut stream);
    println!("{}", response);

    send_msg(&mut stream, "CWD");
    let response = receive_response(&mut stream);
    println!("{}", response);

    Ok(())
}
