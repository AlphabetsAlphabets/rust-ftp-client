use super::response::Response;

use std::{
    io::{BufRead, BufReader, ErrorKind, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs, Ipv4Addr, IpAddr}, str::FromStr,
};

pub struct Client {
    /// The host name the client is connected to.
    hostname: String,
    /// The socket the client is connected to on the server.
    socket: SocketAddr,
    /// The client's connection with the server.
    conn: BufReader<TcpStream>,
}

// Associated functions
impl Client {
    /// Creates a new client.
    /// - `hostname`: The *hostname* of the server.
    /// - `port`: The port to connect to.
    pub fn new(hostname: &str, port: &str) -> Self {
        let addr = format!("{}:{}", hostname, port);
        let conn = match TcpStream::connect(addr) {
            Ok(stream) => BufReader::new(stream),
            Err(e) => {
                panic!("Something went wrong.\n{}", e);
            }
        };

        let mut socket = match hostname.to_socket_addrs() {
            Ok(ip) => ip,
            Err(e) => panic!("{}", e),
        };

        let socket = socket.next().unwrap();

        Self {
            hostname: hostname.to_string(),
            socket,
            conn,
        }
    }
}

// Private functions
impl Client {
    fn receive_response(&mut self) -> String {
        let mut response = String::new();

        // This will read until the end of the line. Need to keep reading until I reach Ok(0)
        match self.conn.read_line(&mut response) {
            // Reached EOF
            Ok(0) => {}
            // Return number of bytes read if there are no issues
            Ok(_bytes_read) => {}
            Err(e) => {
                if ErrorKind::Interrupted != e.kind() {
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
            response.push_str(&self.receive_response());
        }

        let start_with_space = &response[0..1] == " ";
        let index = response.len();
        let end_with_crlf = &response[index - 2..index] == "\r\n";

        // Will be true if output is a multi-line command
        if start_with_space && end_with_crlf {
            response.push_str(&self.receive_response());
        }

        response
    }

    fn send_msg(&mut self, msg: &str) {
        let msg = format!("{}\r\n", msg);
        self.conn.get_mut().write(msg.as_bytes()).unwrap();
    }
}

impl Client {
    pub fn talk_to_server(&mut self, msg: &str) -> Response {
        self.send_msg(msg);
        let response = self.receive_response();
        Response::new(response.as_bytes().to_vec())
    }

    // Connects the client to the specified IP and port as stated by the server
    pub fn enter_passive_mode(&mut self) {
        // This returns data in the form of: 227 Entering Passive Mode (0,0,0,0,109,22).
        // The important information is inside the brackets.
        let response = self.talk_to_server("PASV");
        let mut iter = response.bytes.iter();

        let start = iter.position(|b| *b == b'(').unwrap();
        let end = iter.position(|b| *b == b')').unwrap();

        // This gets the stuff inside the brackets.
        // The reason the weird syntax is there is because `position`
        // keeps track of state. So the next call to `position` starts
        // where the previous call left off. So, the value of `start`
        // can be larger than `end.
        let addr = &response.bytes[start + 1..][..end];
        let addr = String::from_utf8(addr.to_vec()).unwrap();

        // The IP is the first 4 values.
        let ip: Vec<&str> = addr.split(",").take(4).collect();

        // If the IP is 0, 0, 0, 0 then the server is listening on all interfaces.
        // So, reuse the previous IP.
        let value: Vec<&&str> = ip.iter().filter(|&&i| i == "0").collect();
        let listen_on_all_interfaces = value.len() == 4;
        
        // If true, it's not 0, 0, 0, 0
        if !listen_on_all_interfaces {
            // So, I'll need to get the new IP.
            let ip = ip.join(".");
            let ip = match Ipv4Addr::from_str(ip.as_str()) {
                Ok(ip) => ip,
                Err(e) => { 
                    eprintln!("There are two things that can cause this."); 
                    eprintln!("1. The most likely is that parsing the response to extract the IP is incorrect");
                    eprintln!("2. Something went wrong on the server side.");
                    panic!("{}", e);
                }
            };

            self.socket.set_ip(IpAddr::V4(ip));
        }

        // To get the port, take the final two values and do some math on it.
        let port_parts: Vec<&str> = addr.split(",").skip(4).collect();
        let low: u16 = port_parts[0].parse().unwrap();
        let high: u16 = port_parts[1].parse().unwrap();
        let port = low * 256 + high;

        self.socket.set_port(port);
    }
}
