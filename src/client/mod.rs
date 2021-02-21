mod http;

pub use http::HttpDataDogClient;

use crate::error::DataDogLoggerError;
use crate::logger::DataDogLog;
#[cfg(feature = "nonblocking")]
use async_trait::async_trait;

/// Describes blocking Datadog network client
pub trait DataDogClient {
    /// Sends collection of messages to DataDog
    fn send(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError>;
}

/// Describes asynchronous (non-blocking) DataDog client
#[cfg(feature = "nonblocking")]
#[async_trait]
pub trait AsyncDataDogClient {
    /// Sends logs to DataDog in a non-blocking fashion
    async fn send_async(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError>;
}
