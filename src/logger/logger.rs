use log::*;
use std::fmt::Display;
use std::ops::Drop;
use std::sync::mpsc::{sync_channel, SyncSender, TryRecvError};
use std::thread;

use super::level::DataDogLogLevel;
use super::log::DataDogLog;
use crate::client::DataDogClient;
use crate::config::DataDogConfig;
use crate::error::DataDogLoggerError;
use crate::statics::CRATE_NAME;

/// Logger that logs directly to DataDog via HTTP(S) or TCP
pub struct DataDogLogger {
    config: DataDogConfig,
    sender: Option<SyncSender<DataDogLog>>,
    logger_handle: Option<thread::JoinHandle<()>>,
}

impl DataDogLogger {
    /// Creates new DataDogLogger instance
    pub fn new<T>(mut client: T, config: DataDogConfig) -> Result<Self, DataDogLoggerError>
    where
        T: DataDogClient + Send + 'static,
    {
        let (sender, receiver) = sync_channel::<DataDogLog>(config.messages_channel_capacity);
        let self_log_enabled = config.enable_self_log;

        let logger_handle = thread::spawn(move || {
            trace!(
                target: CRATE_NAME,
                "Datadog logger thread starting to ingest messages."
            );

            let mut messages: Vec<DataDogLog> = Vec::new();

            loop {
                match receiver.try_recv() {
                    Ok(msg) => {
                        messages.push(msg);
                        if self_log_enabled {
                            trace!(
                                target: CRATE_NAME,
                                "Logger thread pushed new message to batch."
                            );
                        }
                    }
                    Err(TryRecvError::Disconnected) => {
                        DataDogLogger::send_with_self_log(&mut client, self_log_enabled, &messages);
                        if self_log_enabled {
                            trace!(
                                target: CRATE_NAME,
                                "Logger thread sent batch of messages after channel disconnect."
                            );
                        }
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        DataDogLogger::send_with_self_log(&mut client, self_log_enabled, &messages);
                        messages.clear();
                        if self_log_enabled {
                            trace!(target: CRATE_NAME, "Logger thread sent batch of messages after emptying the channel. About to block on channel.");
                        }
                        if let Ok(msg) = receiver.recv() {
                            messages.push(msg);
                        }
                    }
                };
            }

            trace!(
                target: CRATE_NAME,
                "Datadog logger thread stopping to ingest messages."
            );
        });

        Ok(DataDogLogger {
            config,
            sender: Some(sender),
            logger_handle: Some(logger_handle),
        })
    }

    fn send_with_self_log<ClientType: DataDogClient>(
        client: &mut ClientType,
        self_log_enabled: bool,
        messages: &[DataDogLog],
    ) {
        match client.send(&messages) {
            Ok(_) => {
                if self_log_enabled {
                    trace!(
                        target: CRATE_NAME,
                        "Client succesfully sent messages to Datadog."
                    );
                }
            }
            Err(e) => {
                if self_log_enabled {
                    trace!(
                        target: CRATE_NAME,
                        "Client failed to send messages to Datadog. Error : {}",
                        e
                    );
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
                    if self.config.enable_self_log {
                        trace!(
                            target: CRATE_NAME,
                            "Failed to send message to logger thread. Error : {}",
                            e
                        );
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

        trace!(target: CRATE_NAME, "Datadog logger is stopping");
    }
}
