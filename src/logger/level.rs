use std::fmt::Display;

/// Logging levels according to SysLog
pub enum DataDogLogLevel {
    /// Emergency level
    Emergency,
    /// Alert level
    Alert,
    /// Critical level
    Critical,
    /// Error level
    Error,
    /// Warning level
    Warning,
    /// Notice level
    Notice,
    /// Informational level
    Informational,
    /// Debug level
    Debug,
}

impl Display for DataDogLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataDogLogLevel::Emergency => write!(f, "emerg"),
            DataDogLogLevel::Alert => write!(f, "alert"),
            DataDogLogLevel::Critical => write!(f, "crit"),
            DataDogLogLevel::Error => write!(f, "err"),
            DataDogLogLevel::Warning => write!(f, "warning"),
            DataDogLogLevel::Notice => write!(f, "notice"),
            DataDogLogLevel::Informational => write!(f, "info"),
            DataDogLogLevel::Debug => write!(f, "debug"),
        }
    }
}