mod errors;
mod executor;
mod interface;
mod runtime;
pub use errors::Error;

pub struct Engine;

impl Engine {
    pub async fn start<S, Wm, M>(
        bind: &str,
        runtime: runtime::Runtime<S, Wm, M>,
    ) -> Result<Self, Error>
    where
        S: aum_core::prelude::Storage + Send + 'static + Sync,
        Wm: aum_core::prelude::WalletManager + Send + 'static + Sync,
        M: aum_core::prelude::Monitor<WalletManager = Wm> + Send + 'static + Sync,
    {
        let executor = executor::Executor::new(runtime);
        interface::Server::new(bind, executor).await?;
        Ok(Self {})
    }
}

pub async fn create_runtime<S, Wm, M>(
    storage: S,
    wallet_manager: Wm,
    monitor: M,
) -> runtime::Runtime<S, Wm, M>
where
    S: aum_core::prelude::Storage + Send + 'static + Sync,
    Wm: aum_core::prelude::WalletManager + Send + 'static + Sync,
    M: aum_core::prelude::Monitor<WalletManager = Wm> + Send + 'static + Sync,
{
    runtime::Runtime::new(storage, wallet_manager, monitor).await
}
