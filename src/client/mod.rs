#[cfg(feature = "http")]
mod http;
mod tcp;

use crate::config::DataDogConfig;
use crate::error::DataDogLoggerError;
use crate::logger::DataDogLog;

/// Trait describing generic Datadog network client
pub trait DataDogClient {
    /// Returns new instance of Datadog network client or error
    fn new(config : &DataDogConfig) -> Result<Box<Self>, DataDogLoggerError>;
    /// Sends collection of messages to DataDog
    fn send(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError>;
}

#[cfg(feature = "http")]
pub use http::HttpDataDogClient;
pub use tcp::TcpDataDogClient;
