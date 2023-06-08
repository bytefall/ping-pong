use std::net::Ipv4Addr;
use std::net::SocketAddr;
use tracing::{error, info, trace_span};

mod error;
mod server;

use crate::error::ServerError;
use crate::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut server = Server::new(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 4433));

    loop {
        info!("Waiting for incoming connection...");
        let id = server.listen().await?;
        trace_span!("Client {} connected.", id);

        loop {
            match server.step().await {
                Ok(data) => {
                    info!(
                        "Received '{}' from the client.",
                        std::str::from_utf8(&data)?
                    );
                }
                Err(ServerError::StreamClosed) => {
                    info!("Connection closed.");
                    break;
                }
                Err(e) => {
                    error!("Server error: {e:#?}.");
                    return Err(e.into());
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
