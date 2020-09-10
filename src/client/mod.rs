mod tcp;

use crate::logger::DataDogLog;
use crate::error::DataDogLoggerError;

pub trait DataDogClient {
    fn send(&mut self, messages :&Vec<DataDogLog>) -> Result<(), DataDogLoggerError>;
}