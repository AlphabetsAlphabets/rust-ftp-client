use std::{
    io::{BufRead, BufReader, ErrorKind, Write},
    net::TcpStream,
};

/// Dnaka:
///
/// The keyword here is framing, which basically means, you have some kind of indicator to know when one message is fully read. That can be some length prefix at the start of the message, some byte flag,... in your case the \r\n or \n delimiter. Because each time you call read() you read some raw data from the network stream, but that doesn't guarantee to be the full message. Therefore, you have to find back the markers to detect when a message is considered complete.
fn receive_response(stream: &mut BufReader<TcpStream>) -> String {
    let mut response = String::new();

    // This will read until the end of the line. Need to keep reading until I reach Ok(0)
    match stream.read_line(&mut response) {
        // Reached EOF
        Ok(0) => {}
        // Return number of bytes read if there are no issues
        Ok(_bytes_read) => {}
        Err(e) => {
            if let ErrorKind::Interrupted = e.kind() {
                {};
            } else {
                eprintln!("Error: {}", e);
                panic!("Encountered IOError.");
            }
        }
    }

    // Responses are always '123 abcd' or '123-abcd'
    // With hypen = multi line
    // Without hypen = single line
    let is_hypen = &response[3..4] == "-";
    if is_hypen {
        response.push_str(&receive_response(stream));
    }

    response
}

fn send_msg(stream: &mut BufReader<TcpStream>, msg: &str) {
    let msg = format!("{}\r\n", msg);
    stream.get_mut().write(msg.as_bytes()).unwrap();
}

fn main() -> Result<(), std::io::Error> {
    let stream = TcpStream::connect("ftp.gnu.org:21")?;
    let mut stream = BufReader::new(stream);
    let response = receive_response(&mut stream);
    println!("{}", response);

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
