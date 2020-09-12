#[cfg(feature = "http")]
mod http;
mod tcp;

use crate::error::DataDogLoggerError;
use crate::logger::DataDogLog;
use url::Url;

/// Trait describing generic Datadog network client
pub trait DataDogClient {
    /// Returns new instance of Datadog network client or error
    fn new(api_key: &str, datadog_url: Url) -> Result<Box<Self>, DataDogLoggerError>;
    /// Sends collection of messages to DataDog
    fn send(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError>;
}

#[cfg(feature = "http")]
pub use http::HttpDataDogClient;
pub use tcp::TcpDataDogClient;
