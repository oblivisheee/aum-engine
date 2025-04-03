use std::{
    fmt::{Debug, Display},
    hash::Hash,
};
use thiserror::Error;

pub trait TransactionId:
    Clone + Debug + Display + Send + Sync + 'static + Eq + Sized + Hash
{
}

pub trait Transaction: Clone + Send + Sync + 'static {
    type Hash: crate::hash::Hash;
    type Address: crate::address::Address;
    type TransactionId: TransactionId;
    type TransactionParameters;

    fn new(from: Self::Address, to: Self::Address, parameters: Self::TransactionParameters)
    -> Self;
    fn transaction_id(&self) -> Result<Self::TransactionId, TransactionError>;
    fn hash(&self) -> Self::Hash;
    fn from_bytes(bytes: &[u8]) -> Result<Self, TransactionError>;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait SignedTransaction:
    Clone + Debug + Send + Sync + 'static + Eq + Ord + Hash + TransactionSignature
{
    fn signature(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, TransactionError>;
    fn to_bytes(&self) -> Vec<u8>;
}
pub trait TransactionSignature: Clone + Debug + Send + Sync + 'static + Eq + Ord + Hash {
    type Transaction: Transaction;
    fn from_transaction(transaction: &Self::Transaction) -> Self;
    fn to_transaction(&self) -> Self::Transaction;
}
#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Invalid transaction ID")]
    InvalidTransactionId,
    #[error("Invalid transaction bytes")]
    InvalidBytes,

    #[error("{0}")]
    Custom(#[from] Box<dyn std::error::Error + Send + Sync>),
}
