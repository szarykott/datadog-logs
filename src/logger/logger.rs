#[cfg(feature = "nonblocking")]
use super::future::LoggerFuture;
use super::{level::DataDogLogLevel, log::DataDogLog};
use crate::{
    client::{AsyncDataDogClient, DataDogClient},
    config::DataDogConfig,
    error::DataDogLoggerError,
};
use flume::{bounded, Receiver, Sender, TryRecvError};
#[cfg(feature = "nonblocking")]
use futures::Stream;
use std::{fmt::Display, ops::Drop, thread};

#[derive(Debug)]
/// Logger that logs directly to DataDog via HTTP(S)
pub struct DataDogLogger {
    config: DataDogConfig,
    sender: Option<Sender<DataDogLog>>,
    logger_handle: Option<thread::JoinHandle<()>>,
}

impl DataDogLogger {
    /// Creates new blocking DataDogLogger instance
    pub fn blocking<T>(client: T, config: DataDogConfig) -> Result<Self, DataDogLoggerError>
    where
        T: DataDogClient + Send + 'static,
    {
        let (sender, receiver) = bounded::<DataDogLog>(config.messages_channel_capacity);
        let logger_handle = thread::spawn(move || run_blocking_logger(client, receiver));

        Ok(DataDogLogger {
            config,
            sender: Some(sender),
            logger_handle: Some(logger_handle),
        })
    }

    /// Creates new non-blocking DataDogLogger instance
    #[cfg(feature = "nonblocking")]
    pub fn non_blocking<T>(
        client: T,
        config: DataDogConfig,
    ) -> (Self, impl Stream<Item = Option<String>>)
    where
        T: AsyncDataDogClient + Unpin,
    {
        let (sender, receiver) = bounded::<DataDogLog>(config.messages_channel_capacity);
        let logger_future = LoggerFuture::new(client, receiver);

        let logger = DataDogLogger {
            config,
            sender: Some(sender),
            logger_handle: None,
        };

        (logger, logger_future)
    }

    /// Sends log to DataDog
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
                        println!("{}", e);
                    }
                }
            }
        }
    }
}

fn run_blocking_logger<T: DataDogClient>(mut client: T, receiver: Receiver<DataDogLog>) {
    fn send<T: DataDogClient>(client: &mut T, messages: &[DataDogLog]) {
        match client.send(&messages) {
            Ok(_) => {}
            Err(e) => {}
        }
    }

    let mut messages: Vec<DataDogLog> = Vec::new();

    loop {
        match receiver.try_recv() {
            Ok(msg) => {
                messages.push(msg);
            }
            Err(TryRecvError::Disconnected) => {
                send(&mut client, &messages);
                break;
            }
            Err(TryRecvError::Empty) => {
                send(&mut client, &messages);
                messages.clear();
                // blocking explicitly not to spin CPU
                if let Ok(msg) = receiver.recv() {
                    messages.push(msg);
                }
            }
        };
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
