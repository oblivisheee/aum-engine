#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    AddressError(#[from] crate::address::AddressError),
    #[error("{0}")]
    TransactionError(#[from] crate::transaction::TransactionError),
    #[error("{0}")]
    KeypairError(#[from] crate::keypair::KeyPairError),
    #[error("{0}")]
    MonitorError(#[from] crate::monitor::MonitorError),
    #[error("{0}")]
    WalletManagerError(#[from] crate::wallet::WalletManagerError),
    #[error("{0}")]
    WalletError(#[from] crate::wallet::WalletError),
    #[error("{0}")]
    HashError(#[from] crate::hash::HashError),
}
