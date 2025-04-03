use thiserror::Error;

#[async_trait::async_trait]
pub trait Monitor {
    /// Type that represents wallet manager.
    type WalletManager: crate::wallet::WalletManager + Send;

    /// Start the monitor with the given scale.
    async fn start(&self, scale: &mut Self::WalletManager) -> Result<(), MonitorError>;

    /// Stop the monitor.
    fn stop(&self) -> Result<(), MonitorError>;

    /// Check if the monitor is currently running.
    fn is_running(&self) -> bool;

    /// Restart the monitor by stopping and starting it again.
    async fn restart(&self, scale: &mut Self::WalletManager) -> Result<(), MonitorError> {
        Self::stop(&self)?;
        Self::start(&self, scale).await?;
        Ok(())
    }

    /// Do a check of state.
    async fn sync(&self) -> Result<(), MonitorError>;

    /// Perform a health check on the monitor to ensure it is functioning correctly.
    fn health_check() -> Result<(), MonitorError>;
}

#[derive(Debug, Error)]
pub enum MonitorError {
    /// Error indicating that the monitor failed to start due to a scale-related issue.
    #[error("Wallet manager error: {0}")]
    WalletManagerError(#[from] crate::wallet::WalletManagerError),

    /// A custom error type for other errors, wrapped in a `Box` for dynamic dispatch.
    #[error(transparent)]
    Custom(#[from] Box<dyn std::error::Error + Send + Sync>),

    /// Error indicating that the monitor is not currently running.
    #[error("Monitor is not running")]
    NotRunning,

    /// Error indicating that the monitor's health check failed.
    #[error("Monitor health check failed")]
    HealthCheckFailed,
}
