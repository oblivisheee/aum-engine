use aum_core::prelude::{Monitor, Storage, WalletManager};
pub struct Runtime<S: Storage, Wm: WalletManager, M: Monitor> {
    storage: S,
    scale: Wm,
    monitor: M,
}

impl<
    S: Storage + Send + 'static,
    Wm: WalletManager + Send + 'static,
    M: Monitor<WalletManager = Wm> + Send + 'static,
> Runtime<S, Wm, M>
{
    pub async fn new(storage: S, scale: Wm, monitor: M) -> Self {
        Self {
            storage,
            scale,
            monitor,
        }
    }
    pub fn run(mut self) {
        tokio::spawn(async move {
            if let Err(e) = self.monitor.start(&mut self.scale).await {
                panic!("Error starting monitor: {:?}", e);
            }
        });
    }
    pub fn storage(&self) -> &S {
        &self.storage
    }
    pub fn wallet_manager(&self) -> &Wm {
        &self.scale
    }
    pub fn monitor(&self) -> &M {
        &self.monitor
    }
}
