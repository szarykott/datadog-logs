use miniserde::{json, Serialize};
use std::fmt::Display;
use std::ops::Drop;
use std::sync::mpsc::{sync_channel, SyncSender, TryRecvError};
use std::thread;
use attohttpc::StatusCode;

use super::level::DataDogLogLevel;
use super::config::DataDogConfig;

#[derive(Serialize)]
struct DataDogLog {
    message: String,
    ddtags: Option<String>,
    ddsource: String,
    ddhostname: String,
    service: String,
    level: String,
}

/// Logger that logs directly to DataDog via HTTP(S)
pub struct DataDogLogger {
    config: DataDogConfig,
    sender: Option<SyncSender<DataDogLog>>,
    logger_handle: Option<thread::JoinHandle<()>>,
}

impl DataDogLogger {
    /// Creates new DataDogLogger instance
    pub fn new(config: DataDogConfig) -> Self {
        let (sender, receiver) = sync_channel::<DataDogLog>(256);
        let api_key = config.apikey.clone();
        let url = config.datadog_url.clone();

        let logger_handle = thread::spawn(move || {
            let mut messages: Vec<DataDogLog> = Vec::new();
            loop {
                match receiver.try_recv() {
                    Ok(msg) => messages.push(msg),
                    Err(TryRecvError::Disconnected) => {
                        DataDogLogger::send_messages_to_dd(
                            &messages,
                            api_key.as_str(),
                            url.as_str(),
                        );
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        DataDogLogger::send_messages_to_dd(
                            &messages,
                            api_key.as_str(),
                            url.as_str(),
                        );
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
            sender: Some(sender),
            logger_handle: Some(logger_handle),
        }
    }

    fn send_messages_to_dd(msgs: &Vec<DataDogLog>, api_key: &str, url: &str) {
        let message_formatted = json::to_string(&msgs);
        let result = attohttpc::post(url)
            .header_append("Content-Type", "application/json")
            .header_append("DD-API-KEY", api_key)
            .text(message_formatted)
            .send();

        if cfg!(feature = "self-log") {
            match result {
                Ok(res) => match res.status() {
                    StatusCode::OK => println!("Received OK response from DataDog"),
                    code => eprintln!("Received {} status code from Datadog. Body : {}", code, res.text().unwrap_or_default())
                }
                Err(e) => eprintln!("Sending to DataDog failed with error : {}", e)
            }
        } else {
            match result { _ => {/* ignoring errors */} };
        }
    }

    /// Sends logs to DataDog
    pub fn log<T: Display>(&self, message: T, level: DataDogLogLevel) {
        let log = DataDogLog {
            message: message.to_string(),
            ddtags: self.config.tags.clone(),
            service: self.config.service.clone(),
            ddhostname: self.config.hostname.clone(),
            ddsource: self.config.source.clone(),
            level: level.to_string(),
        };

        if let Some(ref sender) = self.sender {
            match sender.try_send(log) {
                Ok(()) => {}
                Err(e) => if cfg!(feature = "self-log") {
                    eprintln!("Error while sending message to logger : {}", e);
                }
            }
        }
    }
}

impl Drop for DataDogLogger {
    fn drop(&mut self) {
        // drop sender to allow logger thread to close
        std::mem::drop(self.sender.take());

        // wait for logger thread to finish to ensure all messages are flushed
        if let Some(handle) = self.logger_handle.take() {
            handle.join().unwrap_or_default();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_logger_stops() {
        let config = DataDogConfig::default();
        let logger = DataDogLogger::new(config);

        logger.log("message", DataDogLogLevel::Alert);

        // it should hang forever if logging loop does not break
        std::mem::drop(logger);
    }
}