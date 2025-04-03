use serde::{Deserialize, Serialize};

/// Represents various types of requests that can be made.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Request {
    /// Request to retrieve the address of the best wallet to use.
    RetrieveAddress,

    /// Request to send a transaction to a specific address with a specified amount.
    SendTransaction { to: String, amount: u64 },

    /// Request to send a transaction from a specific address to another with a specified amount.
    SendTransactionFrom {
        from: String,
        to: String,
        amount: u64,
    },

    /// Request to retrieve the balance of specified wallet.
    RetrieveBalance { address: String },

    /// Request to retrieve balances of all wallets.
    RetrieveBalances,
    /// Request to list all available wallets.
    ListWallets,
    /// Request to synchronize the system state.
    Sync,
}

/// Represents various types of responses that can be returned.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Response {
    /// Response containing the retrieved wallet address.
    RetrieveAddress { address: String },

    /// Response containing the balance of a specified wallet.
    RetrieveBalance { address: String, balance: u64 },

    /// Response containing the balances of all wallets.
    RetrieveBalances { balances: Vec<(String, u64)> },

    /// Response containing a list of wallet identifiers.
    ListWallets { wallets: Vec<String> },

    /// Response indicating the success of a synchronization operation.
    Sync { success: bool },

    /// Response containing the transaction ID for a sent transaction.
    SendTransaction { txid: String },

    /// Response containing the transaction ID and the originating address for a sent transaction.
    SendTransactionFrom { from: String, txid: String },
}

impl Response {
    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
impl Into<String> for Response {
    fn into(self) -> String {
        self.to_string()
    }
}
