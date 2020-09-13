use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Represents possible events occuring inside DataDog logger
pub enum SelfLogEvent {
    /// Represents logger startup
    Start,
    /// Represents logger tear down
    Stop,
    /// Represents error during sending log to DataDog
    ClientError(String),
    /// Represents error inside logger itself
    LoggerError(String),
    /// Represents succesfull log sending to DataDog
    Succes,
}

impl Display for SelfLogEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelfLogEvent::Start => write!(f, "DataDog logger is starting"),
            SelfLogEvent::Stop => write!(f, "DataDog logger is stopping"),
            SelfLogEvent::ClientError(msg) => {
                write!(f, "Error while sending logs to DataDog : {}", msg)
            }
            SelfLogEvent::LoggerError(msg) => write!(f, "Error inside logger : {}", msg),
            SelfLogEvent::Succes => write!(f, "Messages sent to DataDog succesfully"),
        }
    }
}
