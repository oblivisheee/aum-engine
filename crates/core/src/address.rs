use crate::keypair::{PublicKey, SecretKey};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

/// A trait representing an address that can be derived from a secret or public key.
pub trait Address:
    'static + Clone + Debug + Display + FromStr + Hash + PartialEq + Eq + Send + Sized + Sync
{
    type SecretKey: SecretKey;
    type PublicKey: PublicKey;
    type Format: Format;

    /// Creates an address from a secret key and a specific format.
    fn from_secret_key(
        secret_key: &Self::SecretKey,
        format: &Self::Format,
    ) -> Result<Self, AddressError>;

    /// Creates an address from a public key and a specific format.
    fn from_public_key(
        public_key: &Self::PublicKey,
        format: &Self::Format,
    ) -> Result<Self, AddressError>;

    /// Validates if a given string is a valid address.
    fn is_valid(address: &str) -> bool {
        Self::from_str(address).is_ok()
    }
}

/// A trait representing the format of an address.
pub trait Format:
    'static + Clone + Debug + Display + Send + Sync + Eq + Ord + Sized + Hash
{
}

/// An enumeration of possible errors that can occur when working with addresses.
#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("Invalid address format")]
    InvalidFormat,

    #[error("Failed to parse address")]
    ParseError,

    #[error("Unsupported address format")]
    UnsupportedFormat,

    #[error("Invalid public key")]
    InvalidPublicKey,

    #[error("Invalid secret key")]
    InvalidSecretKey,
}
