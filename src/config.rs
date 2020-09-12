use serde::{Deserialize, Serialize};
use std::default::Default;

/// Configuration for DataDogLogger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDogConfig {
    /// Tags to add to each log.
    pub tags: Option<String>,
    /// DataDog API key.
    pub apikey: String,
    /// Service name to add to each log.
    pub service: Option<String>,
    /// Hostname to add to each log.
    pub hostname: Option<String>,
    /// Source to add to each log.
    pub source: String,
    /// Url of DataDog service along with scheme and path.
    ///
    /// Defaults to `https://http-intake.logs.datadoghq.com/v1/input`.
    ///
    /// For other geographies you might want to use `https://http-intake.logs.datadoghq.eu/v1/input` for example.
    pub datadog_url: String,
    /// Capacity of channel connecting logger thread with other threads.
    /// If not set explicitly it defaults to 256.
    pub messages_channel_capacity: Option<usize>,
}

impl Default for DataDogConfig {
    fn default() -> Self {
        DataDogConfig {
            tags: None,
            apikey: "".into(),
            service: None,
            hostname: None,
            source: "rust".into(),
            datadog_url: "https://http-intake.logs.datadoghq.com/v1/input".into(),
            messages_channel_capacity: Some(256),
        }
    }
}
