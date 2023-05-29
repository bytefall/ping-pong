use std::net::Ipv6Addr;
use std::net::SocketAddr;
use tracing::info;

mod client;
mod error;

use crate::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut client = Client::new(SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 4433));
    client.connect().await?;

    loop {
        let data = client.step().await?;

        info!("Received '{}' from the server", std::str::from_utf8(&data)?);
    }
}
