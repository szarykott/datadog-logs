use std::fmt::Display;
use std::ops::Drop;
use std::sync::mpsc::{sync_channel, SyncSender, TryRecvError};
use std::thread;

use super::level::DataDogLogLevel;
use super::log::DataDogLog;
use crate::client::DataDogClient;
use crate::config::DataDogConfig;
use crate::error::DataDogLoggerError;

/// Logger that logs directly to DataDog via HTTP(S) or TCP
pub struct DataDogLogger {
    config: DataDogConfig,
    sender: Option<SyncSender<DataDogLog>>,
    logger_handle: Option<thread::JoinHandle<()>>,
}

impl DataDogLogger {
    /// Creates new DataDogLogger instance
    pub fn new<ClientType>(config: DataDogConfig) -> Result<Self, DataDogLoggerError>
    where
        ClientType: DataDogClient + Send + 'static,
    {
        let (sender, receiver) =
            sync_channel::<DataDogLog>(config.messages_channel_capacity);
        let mut client = *ClientType::new(&config)?;

        let logger_handle = thread::spawn(move || {
            let mut messages: Vec<DataDogLog> = Vec::new();

            loop {
                match receiver.try_recv() {
                    Ok(msg) => messages.push(msg),
                    Err(TryRecvError::Disconnected) => {
                        DataDogLogger::send_with_self_log(&mut client, &messages);
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        DataDogLogger::send_with_self_log(&mut client, &messages);
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

    fn send_with_self_log<ClientType: DataDogClient>(
        client: &mut ClientType,
        messages: &[DataDogLog],
    ) {
        match client.send(&messages) {
            Ok(_) => {
                if cfg!(feature = "self-log") {
                    println!("DatadogLogger : messages sent succesfully");
                }
            }
            Err(e) => {
                if cfg!(feature = "self-log") {
                    eprintln!("DatadogLogger : error while sending messages : {}", e);
                }
            }
        }
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
