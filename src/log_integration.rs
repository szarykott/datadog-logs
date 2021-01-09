use crate::{
    client::{AsyncDataDogClient, DataDogClient},
    config::DataDogConfig,
    error::DataDogLoggerError,
    logger::{DataDogLogLevel, DataDogLogger},
};
use futures::Future;
use log::{LevelFilter, Log, Metadata, Record};

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

// TODO: move it to main `impl` block
impl DataDogLogger {
    /// Initializes blocking DataDogLogger with `log` crate.
    ///
    /// Requires relevant feature flag activated.
    pub fn set_blocking_logger<T>(
        client: T,
        config: DataDogConfig,
        level: LevelFilter,
    ) -> Result<(), DataDogLoggerError>
    where
        T: DataDogClient + Send + 'static,
    {
        let logger = DataDogLogger::blocking(client, config);
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(level);
        Ok(())
    }

    /// Initializes nonblocking DataDogLogger with `log` crate.
    ///
    /// To make logger work, returned future has to be spawned to executor.
    ///
    /// Requires relevant feature flag activated.
    pub fn set_nonblocking_logger<T>(
        client: T,
        config: DataDogConfig,
        level: LevelFilter,
    ) -> Result<Box<dyn Future<Output = ()>>, DataDogLoggerError>
    where
        T: AsyncDataDogClient + Send + 'static,
    {
        let (logger, future) = DataDogLogger::non_blocking_cold(client, config);
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(level);
        Ok(Box::new(future))
    }
}
