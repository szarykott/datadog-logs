use async_trait::async_trait;
use datadog_logs::{
    client::{AsyncDataDogClient, DataDogClient},
    error::DataDogLoggerError,
};
use flume::Sender;

pub struct DataDogClientStub {
    pub should_error: bool,
    sender: Sender<datadog_logs::logger::DataDogLog>,
}

impl DataDogClientStub {
    pub fn new(sender: Sender<datadog_logs::logger::DataDogLog>) -> Self {
        DataDogClientStub {
            should_error: false,
            sender,
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
            for message in messages {
                self.sender.send(message.clone()).unwrap_or_default();
            }
            Ok(())
        }
    }
}

#[async_trait]
impl AsyncDataDogClient for DataDogClientStub {
    async fn send_async(
        &mut self,
        messages: &[datadog_logs::logger::DataDogLog],
    ) -> Result<(), DataDogLoggerError> {
        // pretend it took some time
        println!("Before wait in client");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("After wait in client");
        if self.should_error {
            Err(DataDogLoggerError::OtherError(
                "Succesfull error inside DataDogClientStub".into(),
            ))
        } else {
            for message in messages {
                self.sender
                    .send_async(message.clone())
                    .await
                    .unwrap_or_default();
            }
            Ok(())
        }
    }
}
