mod address;
mod error;
mod hash;
mod keypair;
mod monitor;
mod network;
mod reqres;
mod storage;
mod transaction;
mod wallet;

pub mod errors {
    pub use crate::address::AddressError;
    pub use crate::error::Error;
    pub use crate::hash::HashError;
    pub use crate::keypair::KeyPairError;
    pub use crate::monitor::MonitorError;
    pub use crate::transaction::TransactionError;
    pub use crate::wallet::WalletError;
    pub use crate::wallet::WalletManagerError;
}

pub mod prelude {
    pub use crate::address::{Address, Format};
    pub use crate::error::Error;
    pub use crate::hash::Hash;
    pub use crate::keypair::{PublicKey, SecretKey};
    pub use crate::monitor::Monitor;
    pub use crate::network::Network;
    pub use crate::reqres::{Request, Response};
    pub use crate::storage::Storage;
    pub use crate::transaction::{SignedTransaction, Transaction, TransactionId};
    pub use crate::wallet::{Wallet, WalletManager};
}
