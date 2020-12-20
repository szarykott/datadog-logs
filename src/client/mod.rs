mod http;

pub use http::HttpDataDogClient;

use crate::error::DataDogLoggerError;
use crate::logger::DataDogLog;
use async_trait::async_trait;

/// Trait describing generic Datadog network client
pub trait DataDogClient {
    /// Sends collection of messages to DataDog
    fn send(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError>;
}

/// Describes asynchronous (non-blocking) DataDog client
#[async_trait]
pub trait AsyncDataDogClient {
    /// Sends logs to DataDog in a non-blocking fashion
    async fn send_async(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError>;
}
