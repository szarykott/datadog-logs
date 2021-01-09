use std::convert::From;
use std::fmt::Display;

/// Errors for DataDogLogger
#[derive(Debug)]
pub enum DataDogLoggerError {
    /// Error that can happen if DataDog URL is not valid
    UrlParsingError(url::ParseError),
    /// Error that can happen during serialization of message
    MessageSerializationError(serde_json::Error),
    /// I/O error
    IoError(std::io::Error),
    /// Logger configuration error
    ConfigError(String),
    /// Generic error container
    OtherError(String),
    /// Http logger error
    HttpError(attohttpc::Error),
    /// Error that can happen during DataDogLogger initialization with log
    #[cfg(feature = "log-integration")]
    LogIntegrationError(log::SetLoggerError),
    /// Http error in non blocking client
    #[cfg(feature = "nonblocking")]
    AsyncHttpError(reqwest::Error),
}

impl Display for DataDogLoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataDogLoggerError::UrlParsingError(e) => write!(f, "{}", e),
            DataDogLoggerError::MessageSerializationError(e) => write!(f, "{}", e),
            DataDogLoggerError::IoError(e) => write!(f, "{}", e),
            DataDogLoggerError::ConfigError(e) => write!(f, "{}", e),
            DataDogLoggerError::OtherError(e) => write!(f, "{}", e),
            DataDogLoggerError::HttpError(e) => write!(f, "{}", e),
            #[cfg(feature = "log-integration")]
            DataDogLoggerError::LogIntegrationError(e) => write!(f, "{}", e),
            #[cfg(feature = "nonblocking")]
            DataDogLoggerError::AsyncHttpError(e) => write!(f, "{}", e),
        }
    }
}

impl From<url::ParseError> for DataDogLoggerError {
    fn from(e: url::ParseError) -> Self {
        DataDogLoggerError::UrlParsingError(e)
    }
}

impl From<serde_json::Error> for DataDogLoggerError {
    fn from(e: serde_json::Error) -> Self {
        DataDogLoggerError::MessageSerializationError(e)
    }
}

impl From<std::io::Error> for DataDogLoggerError {
    fn from(e: std::io::Error) -> Self {
        DataDogLoggerError::IoError(e)
    }
}

impl From<attohttpc::Error> for DataDogLoggerError {
    fn from(e: attohttpc::Error) -> Self {
        DataDogLoggerError::HttpError(e)
    }
}

#[cfg(feature = "log-integration")]
impl From<log::SetLoggerError> for DataDogLoggerError {
    fn from(e: log::SetLoggerError) -> Self {
        DataDogLoggerError::LogIntegrationError(e)
    }
}

#[cfg(feature = "nonblocking")]
impl From<reqwest::Error> for DataDogLoggerError {
    fn from(e: reqwest::Error) -> Self {
        DataDogLoggerError::AsyncHttpError(e)
    }
}
