use serde::{Deserialize, Serialize};

/// Holds information needed to pass to DataDog
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataDogLog {
    /// Message of the log
    pub message: String,
    /// Message tags
    pub ddtags: Option<String>,
    /// Message source
    pub ddsource: String,
    /// Host that sent the message
    pub host: String,
    /// Service that sent the message
    pub service: String,
    /// Datadog understandable string indicating level
    pub level: String,
}
