use crate::Error;
use aum_core::prelude::{
    Address, Error as CoreError, Monitor, Request, Response, Storage, Wallet, WalletManager,
};
use std::{str::FromStr, sync::Arc};

pub struct Executor<S, Wm, M>
where
    S: Storage + Send + 'static,
    Wm: WalletManager + Send + 'static,
    M: Monitor<WalletManager = Wm> + Send + 'static,
{
    runtime: crate::runtime::Runtime<S, Wm, M>,
}

impl<S, Wm, M> Executor<S, Wm, M>
where
    S: Storage + Send + 'static,
    Wm: WalletManager + Send + 'static,
    M: Monitor<WalletManager = Wm> + Send + 'static,
{
    pub fn new(runtime: crate::runtime::Runtime<S, Wm, M>) -> Arc<Self> {
        Arc::new(Self { runtime })
    }
    pub async fn execute(&self, req: Request) -> Result<Response, Error> {
        match req {
            Request::RetrieveAddress => {
                let address = self.process_retrieve_address()?;
                Ok(Response::RetrieveAddress {
                    address: address.to_string(),
                })
            }

            Request::SendTransaction { to, amount } => {
                let address = self.parse_address(&to)?;
                let txid = self.process_send_transaction(&address, amount)?;
                Ok(Response::SendTransaction { txid })
            }
            Request::SendTransactionFrom { from, to, amount } => {
                let from_address = self.parse_address(&from)?;
                let to_address = self.parse_address(&to)?;
                let txid =
                    self.process_send_transaction_from(&from_address, &to_address, amount)?;
                Ok(Response::SendTransactionFrom { from, txid })
            }
            Request::Sync => {
                let success = self.process_sync().await?;
                Ok(Response::Sync { success })
            }
            Request::RetrieveBalance { address } => {
                let address = self.parse_address(&address)?;
                let balance = self.retrieve_balance(&address)?;
                Ok(Response::RetrieveBalance {
                    address: address.to_string(),
                    balance,
                })
            }
            Request::ListWallets => {
                let wallets = self.process_list_wallets()?;
                let wallets = wallets
                    .iter()
                    .map(|w| w.address().to_string())
                    .collect::<Vec<_>>();
                Ok(Response::ListWallets { wallets })
            }
            Request::RetrieveBalances => {
                let balances = self.retrieve_balances()?;
                let balances = balances
                    .iter()
                    .map(|(addr, balance)| (addr.to_string(), *balance))
                    .collect::<Vec<_>>();
                Ok(Response::RetrieveBalances { balances })
            }
        }
    }
    fn parse_address(
        &self,
        address_str: &str,
    ) -> Result<<Wm as WalletManager>::Address, CoreError> {
        match <Wm as WalletManager>::Address::from_str(address_str) {
            Ok(addr) => Ok(addr),
            Err(_) => Err(CoreError::AddressError(
                aum_core::errors::AddressError::ParseError,
            )),
        }
    }
    fn process_retrieve_address(&self) -> Result<impl Address, CoreError> {
        let address = self.runtime.wallet_manager().retrieve_address()?;
        Ok(address)
    }
    fn process_send_transaction(
        &self,
        to: &<Wm as WalletManager>::Address,
        amount: u64,
    ) -> Result<String, CoreError> {
        let txid = self
            .runtime
            .wallet_manager()
            .send_transaction(&to, amount)?;
        Ok(txid.to_string())
    }
    fn process_send_transaction_from(
        &self,
        from: &<Wm as WalletManager>::Address,
        to: &<Wm as WalletManager>::Address,
        amount: u64,
    ) -> Result<String, CoreError> {
        let txid = self
            .runtime
            .wallet_manager()
            .send_transaction_from(&from, &to, amount)?;
        Ok(txid.to_string())
    }
    async fn process_sync(&self) -> Result<bool, CoreError> {
        self.runtime.monitor().sync().await?;
        Ok(true)
    }
    fn retrieve_balance(&self, address: &<Wm as WalletManager>::Address) -> Result<u64, CoreError> {
        let balance = self.runtime.wallet_manager().retrieve_balance(&address)?;
        Ok(balance)
    }
    fn retrieve_balances(&self) -> Result<Vec<(impl Address, u64)>, CoreError> {
        let balances = self.runtime.wallet_manager().retrieve_balances()?;
        Ok(balances)
    }
    fn process_list_wallets(&self) -> Result<Vec<&<Wm as WalletManager>::Wallet>, CoreError> {
        let wallets = self.runtime.wallet_manager().list_wallets()?;
        Ok(wallets)
    }
}
