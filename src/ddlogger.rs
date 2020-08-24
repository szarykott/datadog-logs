
use miniserde::{Serialize, json};
use std::default::Default;
use std::fmt::Display;
use std::sync::mpsc::{sync_channel, SyncSender, TryRecvError};
use std::thread;

/// Logger that logs directly to DataDog via HTTP
pub struct DataDogLogger {
    config : DataDogConfig,
    sender : SyncSender<DataDogLog>,
    _logger_handle : thread::JoinHandle<()>
}

/// Configuration for DataDogLogger
#[derive(Debug, Clone)]
pub struct DataDogConfig {
    /// Tags to add to each log
    pub tags : Option<String>,
    /// DataDog API key
    pub apikey : String,
    /// Service name to add to each log
    pub service : String,
    /// Hostname to add to each log
    pub hostname : String,
    /// Source to add to each log
    pub source : String,
    /// Url of DataDog service along with scheme and path
    /// Defaults to https://http-intake.logs.datadoghq.com/v1/input
    /// For other geographies you might want to use https://http-intake.logs.datadoghq.eu/v1/input for example
    pub datadog_url : String
}

impl Default for DataDogConfig {
    fn default() -> Self {
        DataDogConfig {
            tags : None,
            apikey : "".into(),
            service : "unknown".into(),
            hostname : "unknown".into(),
            source : "rust".into(),
            datadog_url : "https://http-intake.logs.datadoghq.com/v1/input".into()
        }
    }
}

#[derive(Serialize)]
struct DataDogLog {
    message : String,
    ddtags : Option<String>,
    ddsource: String,
    ddhostname : String,
    service : String,
    level : String
}

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
    Debug
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
            DataDogLogLevel::Debug => write!(f, "debug")
        }
    }
}

impl DataDogLogger {
    /// Creates new DataDogLogger instance
    pub fn new(config : DataDogConfig) -> Self {
        let (sender, receiver) = sync_channel::<DataDogLog>(256);
        let api_key = config.apikey.clone();
        let url = config.datadog_url.clone();

        let logger_handle = thread::spawn(move || {
            let mut messages : Vec<DataDogLog> = Vec::new();
            loop {
                match receiver.try_recv() {
                    Ok(msg) => messages.push(msg),
                    Err(TryRecvError::Disconnected) => {
                        DataDogLogger::send_messages_to_dd(&messages, api_key.as_str(), url.as_str());
                        break;
                    },
                    Err(TryRecvError::Empty) => {
                        DataDogLogger::send_messages_to_dd(&messages, api_key.as_str(), url.as_str());
                        messages.clear();
                        if let Ok(msg) = receiver.recv() {
                            messages.push(msg);
                        }
                    }
                };
            }
        });

        DataDogLogger { 
            config,
            sender,
            _logger_handle : logger_handle
        }
    }

    fn send_messages_to_dd(msgs : &Vec<DataDogLog>, api_key : &str, url : &str) {
        let message_formatted = json::to_string(&msgs);
        let result = attohttpc::post(url)
            .header_append("Content-Type", "application/json")
            .header_append("DD-API-KEY", api_key)
            .text(message_formatted)
            .send();

        match result {
            Ok(_) => {},
            Err(_) => { /* ignoring errors */ }
        }
    }

    /// Sends logs to DataDog
    pub fn log<T : Display>(&self, message : T, level : DataDogLogLevel) {
        let log = DataDogLog {
            message : message.to_string(),
            ddtags : self.config.tags.clone(),
            service : self.config.service.clone(),
            ddhostname : self.config.hostname.clone(),
            ddsource : self.config.source.clone(),
            level : level.to_string()
        };

        match self.sender.try_send(log) {
            Ok(()) => {},
            Err(_) => { /* ignoring errors */ }
        }
    }
}