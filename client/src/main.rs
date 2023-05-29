use std::net::Ipv6Addr;
use std::net::SocketAddr;

mod client;
mod error;

use crate::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new(SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 4433));
    client.connect().await?;

    loop {
        let data = client.step().await?;

        println!("Received '{}' from the server", std::str::from_utf8(&data)?);
    }
}
