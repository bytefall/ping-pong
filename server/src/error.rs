use thiserror::Error;
use wtransport::error::{ConnectionError, StreamError};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("I/O error: {0}")]
    IoError(std::io::Error),
    #[error("Connection error: {0:#?}")]
    ConnectionError(ConnectionError),
    #[error("Stream connection closed.")]
    StreamClosed,
    #[error("Stream stopped.")]
    StreamStopped,
    #[error("Endpoint is closed.")]
    EndpointClosed,
    #[error("Connection is not established.")]
    NoConnection,
    #[error("Unable to read data from server.")]
    NoData,
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ConnectionError> for ServerError {
    fn from(e: ConnectionError) -> Self {
        Self::ConnectionError(e)
    }
}

impl From<StreamError> for ServerError {
    fn from(e: StreamError) -> Self {
        match e {
            StreamError::ConnectionClosed => Self::StreamClosed,
            StreamError::Stopped => Self::StreamStopped,
        }
    }
}
