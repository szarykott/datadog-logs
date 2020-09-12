use super::DataDogClient;
use crate::error::DataDogLoggerError;
use crate::logger::DataDogLog;
use url::Url;

/// Datadog network client using HTTP protocol
pub struct HttpDataDogClient {
    datadog_url: Url,
    api_key: String,
}

impl DataDogClient for HttpDataDogClient {
    fn new(api_key: &str, datadog_url: Url) -> Result<Box<Self>, DataDogLoggerError> {
        Ok(Box::new(HttpDataDogClient {
            api_key: api_key.into(),
            datadog_url,
        }))
    }

    fn send(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError> {
        let formatted_message = serde_json::to_string(&messages)?;
        let result = attohttpc::post(&self.datadog_url)
            .header_append("Content-Type", "application/json")
            .header_append("DD-API-KEY", &self.api_key)
            .text(formatted_message)
            .send()?;

        if !result.is_success() {
            Err(DataDogLoggerError::OtherError(format!(
                "Datadog response does not indicate success. Status code : {}, Body : {}",
                result.status(),
                result.text().unwrap_or_default()
            )))
        } else {
            Ok(())
        }
    }
}
