use log::{Log, Record, Metadata};
use crate::ddlogger::DataDogLogger;

impl Log for DataDogLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        &self.log(format!("{}:{} -- {}", record.level(), record.target(), record.args()));
    }
    fn flush(&self) { }
}