use std::{
    io::{BufRead, BufReader, ErrorKind, Write},
    net::TcpStream,
};

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

    let start_with_space = &response[0..1] == " ";
    let index = response.len();
    let end_with_crlf = &response[index - 2..index] == "\r\n";
    
    // NOTE: This is true for the "HELP" command as far as I'm aware.
    // It's the only command that I know of that has a multi line response.
    // Not sure if the other responses will have the same format, I assume
    // it does.
    //
    // Will be true if output is a multi-line command
    if start_with_space && end_with_crlf {
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

    send_msg(&mut stream, "PASV");
    let response = receive_response(&mut stream);
    println!("{}", response);

    send_msg(&mut stream, "LIST");
    let response = receive_response(&mut stream);
    println!("{}", response);

    send_msg(&mut stream, "QUIT");

    Ok(())
}
