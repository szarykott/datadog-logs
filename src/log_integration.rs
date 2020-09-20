use crate::{
    client::DataDogClient,
    config::DataDogConfig,
    error::DataDogLoggerError,
    logger::{DataDogLogLevel, DataDogLogger},
};
use log::{LevelFilter, Log, Metadata, Record};

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
    pub fn init_with_log<T>(
        client: T,
        config: DataDogConfig,
        level: LevelFilter,
    ) -> Result<(), DataDogLoggerError>
    where
        T: DataDogClient + Send + 'static,
    {
        let logger = DataDogLogger::new(client, config)?;
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(level);
        Ok(())
    }
}
