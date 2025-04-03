use thiserror::Error;

/// A trait to manage wallets.
pub trait WalletManager {
    /// Associated type representing a wallet.
    type Wallet: Wallet;
    type Address: crate::address::Address;
    type TransactionId: crate::transaction::TransactionId;

    /// Creates a new wallet and returns a reference to it.
    fn create_wallet(&mut self) -> &Self::Wallet;

    /// Deletes the current wallet and transfers its balance to a specified target wallet.
    /// Returns a reference to the target wallet on success.
    fn delete_and_transfer(
        &mut self,
        target_wallet: &Self::Wallet,
    ) -> Result<&Self::Wallet, WalletManagerError>;

    /// Deletes the current wallet and distributes its balance across specified target wallets.
    /// Returns references to the target wallets on success.
    fn delete_and_distribute(
        &mut self,
        target_wallets: &[Self::Wallet],
    ) -> Result<Vec<&Self::Wallet>, WalletManagerError>;

    /// Scales the wallet system to a specified number of wallets.
    /// Returns references to the newly created wallets on success.
    fn scale_to(&mut self, count: u64) -> Result<Vec<&Self::Wallet>, WalletManagerError>;

    /// Retrieve address of the best wallet.
    fn retrieve_address(&self) -> Result<Self::Address, WalletManagerError>;

    /// Send transaction to a specified address with a given amount.
    fn send_transaction(
        &self,
        to: &Self::Address,
        amount: u64,
    ) -> Result<Self::TransactionId, WalletManagerError>;

    /// Send transaction from a specified address to another with a given amount.
    fn send_transaction_from(
        &self,
        from: &Self::Address,
        to: &Self::Address,
        amount: u64,
    ) -> Result<Self::TransactionId, WalletManagerError>;

    /// List all available wallets.
    fn list_wallets(&self) -> Result<Vec<&Self::Wallet>, WalletManagerError>;

    /// Retrieve the balance of a specified wallet.
    fn retrieve_balance(&self, address: &Self::Address) -> Result<u64, WalletManagerError>;
    /// Retrieve balances of all wallets.
    fn retrieve_balances(&self) -> Result<Vec<(Self::Address, u64)>, WalletManagerError>;
}

/// An enumeration of possible errors that can occur during scaling operations.
#[derive(Debug, Error)]
pub enum WalletManagerError {
    #[error("Wallet error while scale: {0}")]
    WalletError(#[from] crate::wallet::WalletError),
}

pub trait Wallet {
    // Associated types for transactions, signed transactions, and addresses.
    type Transaction: crate::transaction::Transaction;
    type SignedTransaction: crate::transaction::SignedTransaction;
    type Address: crate::address::Address;
    type PublicKey: crate::keypair::PublicKey;
    type SecretKey: crate::keypair::SecretKey;

    // Returns the wallet's address.
    fn address(&self) -> &Self::Address;

    // Returns the wallet's secret key.
    fn secret_key(&self) -> &Self::SecretKey;

    // Returns the wallet's public key.
    fn pubkey(&self) -> &Self::PublicKey;

    // Returns the wallet's balance.
    fn balance(&self) -> u64;

    // Signs a transaction and returns a signed transaction or an error.
    fn sign_transaction(
        &self,
        transaction: &Self::Transaction,
    ) -> Result<Self::SignedTransaction, WalletError>;

    // Verifies the signature of a signed transaction.
    fn verify_transaction_signature(
        &self,
        signed_transaction: &Self::SignedTransaction,
    ) -> Result<bool, WalletError>;

    // Transfers funds to another address, returning a transaction or an error.
    fn transfer_funds(
        &self,
        to: &Self::Address,
        amount: u64,
    ) -> Result<Self::Transaction, WalletError>;

    // Retrieves the transaction history for the wallet.
    fn transaction_history(&self) -> Vec<Self::Transaction>;

    // Checks if the wallet has sufficient balance for a given amount.
    fn has_sufficient_balance(&self, amount: u64) -> bool {
        self.balance() >= amount
    }
}

// Define an error type for wallet-related operations.
#[derive(Debug, Error)]
pub enum WalletError {
    // Error for insufficient balance in the wallet.
    #[error("Insufficient balance")]
    InsufficientBalance,

    // Error for an invalid address.
    #[error("Invalid address")]
    InvalidAddress,

    // Error for transaction-related issues, wrapping a TransactionError.
    #[error("Transaction error: {0}")]
    TransactionError(#[from] crate::transaction::TransactionError),
}
