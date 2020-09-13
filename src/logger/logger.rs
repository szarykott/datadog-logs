use std::fmt::Display;
use std::ops::Drop;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError};
use std::thread;

use super::level::DataDogLogLevel;
use super::log::DataDogLog;
use crate::client::DataDogClient;
use crate::config::DataDogConfig;
use crate::error::DataDogLoggerError;
use crate::self_log::SelfLogEvent;

/// Logger that logs directly to DataDog via HTTP(S) or TCP
pub struct DataDogLogger {
    config: DataDogConfig,
    sender: Option<SyncSender<DataDogLog>>,
    logger_handle: Option<thread::JoinHandle<()>>,
    self_log_sender: SyncSender<SelfLogEvent>,
}

impl DataDogLogger {
    /// Creates new DataDogLogger instance
    pub fn new<ClientType>(
        config: DataDogConfig,
    ) -> Result<(Self, Receiver<SelfLogEvent>), DataDogLoggerError>
    where
        ClientType: DataDogClient + Send + 'static,
    {
        let (sender, receiver) = sync_channel::<DataDogLog>(config.messages_channel_capacity);
        let mut client = *ClientType::new(&config)?;
        let self_log_enabled = config.enable_self_log;
        let (self_log_sender, self_log_receiver) = sync_channel::<SelfLogEvent>(64);
        let self_log_sender_clone = self_log_sender.clone();

        let logger_handle = thread::spawn(move || {
            let mut messages: Vec<DataDogLog> = Vec::new();

            loop {
                match receiver.try_recv() {
                    Ok(msg) => messages.push(msg),
                    Err(TryRecvError::Disconnected) => {
                        DataDogLogger::send_with_self_log(
                            &mut client,
                            self_log_enabled,
                            &self_log_sender_clone,
                            &messages,
                        );
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        DataDogLogger::send_with_self_log(
                            &mut client,
                            self_log_enabled,
                            &self_log_sender_clone,
                            &messages,
                        );
                        messages.clear();
                        if let Ok(msg) = receiver.recv() {
                            messages.push(msg);
                        }
                    }
                };
            }
        });

        self_log_sender.try_send(SelfLogEvent::Start).unwrap_or_default();

        Ok((
            DataDogLogger {
                config,
                sender: Some(sender),
                logger_handle: Some(logger_handle),
                self_log_sender,
            },
            self_log_receiver,
        ))
    }

    fn send_with_self_log<ClientType: DataDogClient>(
        client: &mut ClientType,
        self_log_enabled: bool,
        sender: &SyncSender<SelfLogEvent>,
        messages: &[DataDogLog],
    ) {
        match client.send(&messages) {
            Ok(_) => {
                if self_log_enabled {
                    sender.try_send(SelfLogEvent::Succes).unwrap_or_default();
                }
            }
            Err(e) => {
                if self_log_enabled {
                    sender
                        .try_send(SelfLogEvent::ClientError(e.to_string()))
                        .unwrap_or_default();
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
                        self.self_log_sender
                            .try_send(SelfLogEvent::LoggerError(format!(
                                "Error while sending message to logger : {}",
                                e
                            )))
                            .unwrap_or_default();
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

        self.self_log_sender.try_send(SelfLogEvent::Stop).unwrap_or_default();
    }
}
