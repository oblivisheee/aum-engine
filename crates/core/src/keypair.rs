pub trait SecretKey: Send + Sync {
    type PublicKey: PublicKey;

    /// Generates a new secret key.
    fn new() -> Self;

    /// Converts the secret key to a byte vector.
    fn to_bytes(&self) -> Vec<u8>;

    /// Creates a secret key from a byte slice.
    fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, KeyPairError>
    where
        Self: Sized;

    /// Converts the secret key to a hexadecimal string.
    fn to_hex(&self) -> String;

    /// Creates a secret key from a hexadecimal string.
    fn from_hex(hex: &str) -> Result<Self, KeyPairError>
    where
        Self: Sized;

    /// Returns the corresponding public key.
    fn pubkey(&self) -> Self::PublicKey;
}

pub trait PublicKey: Clone + Send + Sync {
    type SecretKey: SecretKey;

    /// Retrieves the public key from the secret key.
    fn from_secret_key(secret_key: &Self::SecretKey) -> Self;

    /// Converts the public key to a byte vector.
    fn to_bytes(&self) -> Vec<u8>;

    /// Creates a public key from a byte slice.
    fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, KeyPairError>
    where
        Self: Sized;

    /// Converts the public key to a hexadecimal string.
    fn to_hex(&self) -> String;

    /// Creates a public key from a hexadecimal string.
    fn from_hex(hex: &str) -> Result<Self, KeyPairError>
    where
        Self: Sized;
}

#[derive(Debug, thiserror::Error)]
pub enum KeyPairError {
    #[error("Failed to generate key pair")]
    FailedToGenerateKeyPair,

    #[error("Invalid byte representation")]
    InvalidBytes,

    #[error("Invalid hexadecimal representation")]
    InvalidHex,

    #[error("Invalid public key")]
    InvalidPublicKey,

    #[error("Invalid secret key")]
    InvalidSecretKey,

    #[error(transparent)]
    Custom(#[from] Box<dyn std::error::Error + Send + Sync>),
}
