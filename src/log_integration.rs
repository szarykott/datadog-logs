use crate::{
    client::TcpDataDogClient,
    config::DataDogConfig,
    error::DataDogLoggerError,
    logger::{DataDogLogLevel, DataDogLogger},
    self_log::SelfLogEvent,
};
use log::{LevelFilter, Log, Metadata, Record};
use std::sync::mpsc::Receiver;

/// Requires `log` feature enabled
impl Log for DataDogLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        let level = match record.level() {
            log::Level::Error => DataDogLogLevel::Error,
            log::Level::Warn => DataDogLogLevel::Warning,
            log::Level::Info => DataDogLogLevel::Informational,
            log::Level::Debug | log::Level::Trace => DataDogLogLevel::Debug,
        };

        &self.log(format!("{}", record.args()), level);
    }
    fn flush(&self) {}
}

impl DataDogLogger {
    /// Initializes DataDogLogger with `log` crate
    ///
    /// Requires `log` feature enabled
    pub fn init_with_log(
        config: DataDogConfig,
        level: LevelFilter,
    ) -> Result<(), DataDogLoggerError> {
        let (logger, _) = DataDogLogger::new::<TcpDataDogClient>(config)?;
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(level);
        Ok(())
    }

    /// Initializes DataDogLogger with `log` crate and self log receiver
    ///
    /// Requires `log` feature enabled
    pub fn init_with_log_and_self_log(
        config: DataDogConfig,
        level: LevelFilter,
    ) -> Result<Receiver<SelfLogEvent>, DataDogLoggerError> {
        let (logger, receiver) = DataDogLogger::new::<TcpDataDogClient>(config)?;
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(level);
        Ok(receiver)
    }
}
