
use miniserde::{Serialize, json};
use std::default::Default;
use std::fmt::Display;
use std::sync::mpsc::{sync_channel, SyncSender};
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
    tags : Option<String>,
    apikey : String,
    service : String,
    hostname : String,
    source : String
}

impl Default for DataDogConfig {
    fn default() -> Self {
        DataDogConfig {
            tags : None,
            apikey : "".into(),
            service : "unknown".into(),
            hostname : "unknown".into(),
            source : "rust".into()
        }
    }
}

#[derive(Serialize)]
struct DataDogLog {
    message : String,
    ddtags : Option<String>,
    ddsource: String,
    ddhostname : String,
    service : String
}

impl DataDogLogger {
    /// Creates new DataDogLogger instance
    pub fn new(config : DataDogConfig) -> Self {
        let (sender, receiver) = sync_channel::<DataDogLog>(256);
        let api_key = config.apikey.clone();

        let logger_handle = thread::spawn(move || {
            let api_key = api_key;
            while let Ok(msg) = receiver.recv() {
                DataDogLogger::send_message_to_dd(msg, api_key.as_str());
            }
        });

        DataDogLogger { 
            config,
            sender,
            _logger_handle : logger_handle
        }
    }

    fn send_message_to_dd(msg : DataDogLog, api_key : &str) {
        let message_formatted = json::to_string(&msg);
        let result = attohttpc::post("https://http-intake.logs.datadoghq.com/v1/input")
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
    pub fn log<T : Display>(&self, message : T) {
        let log = DataDogLog {
            message : message.to_string(),
            ddtags : self.config.tags.clone(),
            service : self.config.service.clone(),
            ddhostname : self.config.hostname.clone(),
            ddsource : self.config.source.clone()
        };

        match self.sender.try_send(log) {
            Ok(()) => {},
            Err(_) => { /* ignoring errors */ }
        }
    }
}