#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CoreError(#[from] aum_core::prelude::Error),
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("WebSocket Error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Wrong request.")]
    WrongRequest,
}
