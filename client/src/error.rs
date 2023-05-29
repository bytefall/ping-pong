use thiserror::Error;
use wtransport::error::{ConnectionError, StreamError};

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("I/O error: {0}")]
    IoError(std::io::Error),
    #[error("Connection error: {0:#?}")]
    ConnectionError(ConnectionError),
    #[error("Stream error: {0:#?}")]
    StreamError(StreamError),
    #[error("Connection is not established.")]
    NoConnection,
    #[error("Unable to read data from server.")]
    NoData,
}

impl From<std::io::Error> for ClientError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ConnectionError> for ClientError {
    fn from(e: ConnectionError) -> Self {
        Self::ConnectionError(e)
    }
}

impl From<StreamError> for ClientError {
    fn from(e: StreamError) -> Self {
        Self::StreamError(e)
    }
}
