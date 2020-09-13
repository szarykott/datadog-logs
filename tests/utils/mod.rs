use datadog_logs::{
    client::DataDogClient,
    error::DataDogLoggerError
};

pub struct DataDogClientStub {
    pub should_error : bool
}

impl DataDogClient for DataDogClientStub {
    fn new(_config: &datadog_logs::config::DataDogConfig) -> Result<Box<Self>, datadog_logs::error::DataDogLoggerError> {
        Ok(Box::new(DataDogClientStub { should_error : false }))
    }

    fn send(&mut self, _messages: &[datadog_logs::logger::DataDogLog]) -> Result<(), datadog_logs::error::DataDogLoggerError> {
        if self.should_error {
            Err(DataDogLoggerError::OtherError("Succesfull error inside DataDogClientStub".into()))
        } else {
            Ok(())
        }
    }
}