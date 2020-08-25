use std::default::Default;
use url::Url;

/// Configuration for DataDogLogger
#[derive(Debug, Clone)]
pub struct DataDogConfig {
    /// Tags to add to each log
    pub tags: Option<String>,
    /// DataDog API key
    pub apikey: String,
    /// Service name to add to each log
    pub service: String,
    /// Hostname to add to each log
    pub hostname: String,
    /// Source to add to each log
    pub source: String,
    /// Url of DataDog service along with scheme and path
    ///
    /// Defaults to `https://http-intake.logs.datadoghq.com/v1/input`
    ///
    /// For other geographies you might want to use `https://http-intake.logs.datadoghq.eu/v1/input` for example
    pub datadog_url: Url,
}

impl Default for DataDogConfig {
    fn default() -> Self {
        DataDogConfig {
            tags: None,
            apikey: "".into(),
            service: "unknown".into(),
            hostname: "unknown".into(),
            source: "rust".into(),
            datadog_url: Url::parse("https://http-intake.logs.datadoghq.com/v1/input").unwrap(),
        }
    }
}