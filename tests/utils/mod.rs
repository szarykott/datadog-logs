use datadog_logs::{client::DataDogClient, error::DataDogLoggerError};
use std::sync::{Mutex, Arc};

pub struct DataDogClientStub {
    pub should_error: bool,
    pub messages : Arc<Mutex<Vec<datadog_logs::logger::DataDogLog>>>
}

impl DataDogClientStub {
    pub fn new() -> Self {
        DataDogClientStub {
            should_error : false,
            messages : Arc::new(Mutex::new(Vec::new()))
        }
    }
}

impl DataDogClient for DataDogClientStub {
    fn send(
        &mut self,
        messages: &[datadog_logs::logger::DataDogLog],
    ) -> Result<(), datadog_logs::error::DataDogLoggerError> {
        if self.should_error {
            Err(DataDogLoggerError::OtherError(
                "Succesfull error inside DataDogClientStub".into(),
            ))
        } else {
            match self.messages.lock() {
                Ok(mut messageslock) => {
                    messageslock.extend_from_slice(messages);
                }
                Err(e) => {
                    panic!("Mutex error : {}", e);
                }
            }
            Ok(())
        }
    }
}
