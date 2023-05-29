use std::net::SocketAddr;
use wtransport::tls::Certificate;
use wtransport::Connection;
use wtransport::Endpoint;
use wtransport::ServerConfig;

use crate::error::ServerError;

const PONG_STR: &[u8] = b"pong";

pub struct Server {
    addr: SocketAddr,
    conn: Option<Connection>,
}

impl Server {
    /// Creates a new server instance.
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr, conn: None }
    }

    /// Listens for incoming connection.
    pub async fn listen(&mut self) -> Result<u64, ServerError> {
        self.conn = None;

        let config = ServerConfig::builder()
            .with_bind_address(self.addr)
            .with_certificate(Certificate::load("cert.pem", "key.pem")?);

        let endpoint = Endpoint::server(config)?;
        let conn = endpoint
            .accept()
            .await
            .ok_or(ServerError::EndpointClosed)?
            .await?;
        let id = conn.session_id();

        self.conn = Some(conn);

        Ok(id)
    }

    /// Reads a data and sends "pong".
    pub async fn step(&mut self) -> Result<Vec<u8>, ServerError> {
        let (mut sx, mut rx) = self
            .conn
            .as_ref()
            .ok_or(ServerError::NoConnection)?
            .accept_bi()
            .await?;

        let mut buf = [0; 256];
        let len = rx.read(&mut buf).await?.ok_or(ServerError::NoData)?;

        sx.write_all(PONG_STR).await?;
        sx.finish().await?;

        Ok(buf[..len].to_owned())
    }
}

#[cfg(test)]
mod test {
    use std::net::Ipv6Addr;
    use std::net::SocketAddr;

    use super::Server;
    use crate::error::ServerError;

    #[tokio::test]
    async fn fail_when_connection_is_not_established() {
        let mut server = Server::new(SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 4433));

        assert!(matches!(
            server.step().await,
            Err(ServerError::NoConnection)
        ));
    }
}
