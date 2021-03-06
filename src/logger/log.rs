use serde::{Deserialize, Serialize};

/// Information passed to DataDog
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DataDogLog {
    /// The message
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
