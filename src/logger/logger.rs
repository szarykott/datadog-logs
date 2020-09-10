use std::fmt::Display;
use std::ops::Drop;
use std::sync::mpsc::{sync_channel, SyncSender, TryRecvError};
use std::thread;
use url::Url;

use super::config::DataDogConfig;
use super::level::DataDogLogLevel;
use crate::error::DataDogLoggerError;
use super::log::DataDogLog;
use crate::client::DataDogClient;

/// Logger that logs directly to DataDog via HTTP(S)
pub struct DataDogLogger {
    config: DataDogConfig,
    sender: Option<SyncSender<DataDogLog>>,
    logger_handle: Option<thread::JoinHandle<()>>,
}

impl DataDogLogger {
    /// Creates new DataDogLogger instance
    pub fn new<ClientType>(config: DataDogConfig) -> Result<Self, DataDogLoggerError> 
    where
        ClientType : DataDogClient + Send +'static
    {
        let (sender, receiver) = sync_channel::<DataDogLog>(256);
        let mut client = *ClientType::new(config.apikey.as_str(), Url::parse(&config.datadog_url)?)?;

        let logger_handle = thread::spawn(move || {
            let mut messages: Vec<DataDogLog> = Vec::new();

            loop {
                match receiver.try_recv() {
                    Ok(msg) => messages.push(msg),
                    Err(TryRecvError::Disconnected) => {
                        client.send(&messages);
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        client.send(&messages);
                        messages.clear();
                        if let Ok(msg) = receiver.recv() {
                            messages.push(msg);
                        }
                    }
                };
            }
        });

        Ok(DataDogLogger {
            config,
            sender: Some(sender),
            logger_handle: Some(logger_handle),
        })
    }

    /// Sends logs to DataDog
    pub fn log<T: Display>(&self, message: T, level: DataDogLogLevel) {
        let log = DataDogLog {
            message: message.to_string(),
            ddtags: self.config.tags.clone(),
            service: self.config.service.clone().unwrap_or_default(),
            host: self.config.hostname.clone().unwrap_or_default(),
            ddsource: self.config.source.clone(),
            level: level.to_string(),
        };

        if let Some(ref sender) = self.sender {
            match sender.try_send(log) {
                Ok(()) => {}
                Err(e) => {
                    if cfg!(feature = "self-log") {
                        eprintln!("Error while sending message to logger : {}", e);
                    }
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
    use crate::client::{HttpDataDogLogger, TcpDataDogClient};

    #[test]
    fn test_logger_stops_http() {
        let config = DataDogConfig::default();
        let logger = DataDogLogger::new::<HttpDataDogLogger>(config).unwrap();

        logger.log("message", DataDogLogLevel::Alert);

        // it should hang forever if logging loop does not break
        std::mem::drop(logger);
    }

    #[test]
    fn test_logger_stops_tcp() {
        let config = DataDogConfig::default();
        let logger = DataDogLogger::new::<TcpDataDogClient>(config).unwrap();

        logger.log("message", DataDogLogLevel::Alert);

        // it should hang forever if logging loop does not break
        std::mem::drop(logger);
    }
}
