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

    let endp =
        std::env::var("PING_PONG_ENDPOINT").expect("Unable to get env var PING_PONG_ENDPOINT");

    let uri: http::Uri = endp.parse()?;

    if uri.scheme() != Some(&http::uri::Scheme::HTTPS) {
        Err("URI scheme must be HTTPS.")?;
    }

    let auth = uri.authority().ok_or("URI must have a host")?;
    let port = auth.port_u16().unwrap_or(4433);

    let addr = tokio::net::lookup_host((auth.host(), port))
        .await?
        .next()
        .ok_or("DNS found no addresses")?;

    info!("Connecting to {addr} ({uri})...");

    let mut client = Client::new(addr);
    client.connect().await?;

    loop {
        let data = client.step().await?;

        info!("Received '{}' from the server", std::str::from_utf8(&data)?);
    }
}
