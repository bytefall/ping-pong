use std::net::Ipv6Addr;
use std::net::SocketAddr;

mod error;
mod server;

use crate::error::ServerError;
use crate::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new(SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 4433));

    loop {
        println!("Waiting for incoming connection...");
        let id = server.listen().await?;
        println!("Client {} connected.", id);

        loop {
            match server.step().await {
                Ok(data) => {
                    println!(
                        "Received '{}' from the client.",
                        std::str::from_utf8(&data)?
                    );
                }
                Err(ServerError::StreamClosed) => {
                    println!("Connection closed.");
                    break;
                }
                Err(e) => {
                    eprintln!("Server error: {e:#?}.");
                    return Err(e.into());
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
