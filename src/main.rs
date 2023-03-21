mod response;

mod client;
use client::Client;

fn main() -> Result<(), std::io::Error> {
    let mut client = Client::new("ftp.gnu.org", "21");

    // This line is needed to "burn" through the "server is ready" message.
    _ = client.talk_to_server("");

    let response = client.talk_to_server("USER anonymous");
    println!("<<< {}", response);

    client.enter_passive_mode();

    _ = client.talk_to_server("QUIT");

    Ok(())
}
