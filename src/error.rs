use thiserror::Error;

/// Errors for DataDogLogger
#[derive(Error, Debug)]
pub enum DataDogLoggerError {
    /// Error that can happen if DataDog URL is not valid
    #[error("{0}")]
    UrlParsingError(#[from] url::ParseError),
    /// Error that can happen during serialization of message
    #[error("{0}")]
    MessageSerializationError(#[from] serde_json::Error),
    /// I/O error
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    /// Logger configuration error
    #[error("{0}")]
    ConfigError(String),
    /// Generic error container
    #[error("{0}")]
    OtherError(String),
    /// Http logger error
    #[error("{0}")]
    HttpError(#[from] attohttpc::Error),
    /// Error that can happen during DataDogLogger initialization with log
    #[error("{0}")]
    LogIntegrationError(#[from] log::SetLoggerError),
    /// Http error in non blocking client
    #[cfg(feature = "nonblocking")]
    #[error("{0}")]
    AsyncHttpError(#[from] reqwest::Error),
}
