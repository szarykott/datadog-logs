use super::{AsyncDataDogClient, DataDogClient};
use crate::config::DataDogConfig;
use crate::error::DataDogLoggerError;
use crate::logger::DataDogLog;
use async_trait::async_trait;
use reqwest;
use url::Url;

/// Datadog network client using HTTP protocol
#[derive(Debug)]
pub struct HttpDataDogClient {
    datadog_url: Url,
    api_key: String,
}

impl HttpDataDogClient {
    /// Creates new DataDog HTTP(S) logger
    pub fn new(config: &DataDogConfig) -> Result<Self, DataDogLoggerError> {
        let http_config = config.http_config.clone();

        Ok(HttpDataDogClient {
            api_key: config.apikey.clone().into(),
            datadog_url: Url::parse(&http_config.url)?,
        })
    }
}

impl DataDogClient for HttpDataDogClient {
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

#[async_trait]
impl AsyncDataDogClient for HttpDataDogClient {
    async fn send_async(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError> {
        let client = reqwest::Client::new();
        let response = client
            .post(self.datadog_url.clone())
            .header("Content-Type", "application/json")
            .header("DD-API-KEY", &self.api_key)
            .json(messages)
            .send()
            .await?;

        if !response.status().is_success() {
            Err(DataDogLoggerError::OtherError(format!(
                "Datadog response does not indicate success. Status code : {}, Body : {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )))
        } else {
            Ok(())
        }
    }
}
