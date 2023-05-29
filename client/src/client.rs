use std::net::Ipv6Addr;
use std::net::SocketAddr;
use wtransport::ClientConfig;
use wtransport::Connection;
use wtransport::Endpoint;

use crate::error::ClientError;

const PING_STR: &[u8] = b"ping";

pub struct Client {
    addr: SocketAddr,
    conn: Option<Connection>,
}

impl Client {
    /// Creates a new client instance.
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr, conn: None }
    }

    /// Connects to the server.
    pub async fn connect(&mut self) -> Result<(), ClientError> {
        let config = ClientConfig::builder()
            .with_bind_address(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 0));

        self.conn = Some(
            Endpoint::client(config)?
                .connect(self.addr, "localhost")?
                .await?,
        );

        Ok(())
    }

    /// Sends "ping" and reads for a reply.
    pub async fn step(&mut self) -> Result<Vec<u8>, ClientError> {
        let (mut sx, mut rx) = self
            .conn
            .as_ref()
            .ok_or(ClientError::NoConnection)?
            .open_bi()
            .await?;

        sx.write_all(PING_STR).await?;
        sx.finish().await?;

        let mut buf = [0; 256];
        let len = rx.read(&mut buf).await?.ok_or(ClientError::NoData)?;

        Ok(buf[..len].to_owned())
    }
}

#[cfg(test)]
mod test {
    use std::net::Ipv6Addr;
    use std::net::SocketAddr;

    use super::Client;
    use crate::error::ClientError;

    #[tokio::test]
    async fn fail_when_connection_is_not_established() {
        let mut client = Client::new(SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 4433));

        assert!(matches!(
            client.step().await,
            Err(ClientError::NoConnection)
        ));
    }
}
