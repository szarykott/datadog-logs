use serde::{Deserialize, Serialize};
use std::default::Default;

/// Configuration for DataDogLogger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDogConfig {
    /// Tags to add to each log.
    pub tags: Option<String>,
    /// DataDog API key.
    /// It is required to specify API key. Not doing it is considered an error.
    pub apikey: String,
    /// Service name to add to each log.
    pub service: Option<String>,
    /// Hostname to add to each log.
    pub hostname: Option<String>,
    /// Source to add to each log.
    /// Default value is `rust`.
    #[serde(default)]
    pub source: String,
    /// HTTP client specific configuration.
    /// It only needs to be specified for HTTP logging in case of non-default settings.
    /// Otherwise default is assumed.
    #[serde(default)]
    pub http_config: DataDogHttpConfig,
    /// TCP client specific configuration
    /// It only needs to be specified for TCP logging in case of non-default settings.
    /// Otherwise default is assumed.
    ///
    /// Even though the crate does not support TCP client now, config is here to be an extensibility point.
    #[serde(default)]
    pub tcp_config: DataDogTcpConfig,
    /// Capacity of channel connecting logger thread with other threads.
    /// If not set explicitly, it defaults to 10 000 messages.
    /// If explicitly set to `None`, channel will be unbounded.
    #[serde(default)]
    pub messages_channel_capacity: Option<usize>,
    /// Enables or disables self logging. Disabled by default.
    #[serde(default)]
    pub enable_self_log: bool,
}

impl Default for DataDogConfig {
    fn default() -> Self {
        DataDogConfig {
            tags: None,
            apikey: "".into(),
            service: None,
            hostname: None,
            http_config: Default::default(),
            tcp_config: Default::default(),
            source: "rust".into(),
            messages_channel_capacity: Some(10_000),
            enable_self_log: false,
        }
    }
}

/// HTTP specific Datadog connectivity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDogHttpConfig {
    /// Url of DataDog service along with scheme and path.
    /// To keeps library as flexible as possible no check is performed on passed URL.
    /// It is important that you specify correct geography and subdomain.
    /// To log via HTTPS simply specify HTTPS scheme in the URL.
    /// If you prefer unencrypted connection, specify HTTP scheme.
    ///
    /// Default value is `https://http-intake.logs.datadoghq.com/v1/input`.
    #[serde(default)]
    pub url: String,
}

impl Default for DataDogHttpConfig {
    fn default() -> Self {
        DataDogHttpConfig {
            url: "https://http-intake.logs.datadoghq.com/v1/input".into(),
        }
    }
}

/// TCP specific Datadog connectivity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDogTcpConfig {
    /// If set to true will force TLS connction to DataDog for TCP. True by default.
    #[serde(default)]
    pub use_tls: bool,
    /// Datadog service domain without scheme or path parts of URL e.g. `intake.logs.datadoghq.com`.
    ///
    /// Default value is `intake.logs.datadoghq.com`. However it might need to be changed for differerent geographies e.g. Europe.
    #[serde(default)]
    pub domain: String,
    /// Port for unencrypted connections to Datadog. By default it is `10514` as specified in Datadog documentation.
    /// It is possible to change it in case Datadog changes it in the future.
    #[serde(default)]
    pub non_tls_port: usize,
    /// Port for encrypted connections. It defaults to 443.
    #[serde(default)]
    pub tls_port: usize,
}

impl Default for DataDogTcpConfig {
    /// Default configuration is US default config.
    /// EU config needs to be input manually.
    fn default() -> Self {
        DataDogTcpConfig {
            use_tls: true,
            domain: "intake.logs.datadoghq.com".into(),
            non_tls_port: 10514,
            tls_port: 443,
        }
    }
}
