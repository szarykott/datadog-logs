use std::convert::From;
use std::fmt::Display;

/// Errors for DataDogLogger
#[derive(Debug)]
pub enum DataDogLoggerError {
    /// Error that can happen if DataDog URL is not valid
    UrlParsingError(url::ParseError),
    /// Error that can happen during DataDogLogger initialization with log
    #[cfg(feature = "log-integration")]
    LogIntegrationError(log::SetLoggerError)
}

impl Display for DataDogLoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataDogLoggerError::UrlParsingError(e) => write!(f, "{}", e),
            #[cfg(feature = "log-integration")]
            DataDogLoggerError::LogIntegrationError(e) => write!(f, "{}", e)
        }
    }
}

impl From<url::ParseError> for DataDogLoggerError {
    fn from(e: url::ParseError) -> Self {
        DataDogLoggerError::UrlParsingError(e)
    }
}

#[cfg(feature = "log-integration")]
impl From<log::SetLoggerError> for DataDogLoggerError {
    fn from(e: log::SetLoggerError) -> Self {
        DataDogLoggerError::LogIntegrationError(e)
    }
}

