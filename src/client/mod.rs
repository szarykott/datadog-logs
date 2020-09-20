mod http;

use crate::error::DataDogLoggerError;
use crate::logger::DataDogLog;

/// Trait describing generic Datadog network client
pub trait DataDogClient {
    /// Sends collection of messages to DataDog
    fn send(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError>;
}

pub use http::HttpDataDogClient;
