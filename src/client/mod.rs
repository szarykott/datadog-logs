mod tcp;
mod http;

use crate::logger::DataDogLog;
use crate::error::DataDogLoggerError;
use url::Url;

pub trait DataDogClient {
    fn new(api_key : &str, datadog_url : Url) -> Result<Box<Self>, DataDogLoggerError>;
    fn send(&mut self, messages :&[DataDogLog]) -> Result<(), DataDogLoggerError>;
}