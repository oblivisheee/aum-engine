use thiserror::Error;
pub trait Hash {
    fn from_bytes(bytes: &[u8]) -> Result<Self, HashError>
    where
        Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_hex(hex: &str) -> Result<Self, HashError>
    where
        Self: Sized;
    fn to_hex(&self) -> String;
}

#[derive(Debug, Error)]
pub enum HashError {
    #[error("Invalid byte representation")]
    InvalidBytes,
    #[error("Invalid hexadecimal representation")]
    InvalidHex,
    #[error("Hashing error")]
    HashingError,
}
