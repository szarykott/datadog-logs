use url::Url;
use crate::logger::DataDogLog;
use crate::error::DataDogLoggerError;
use super::DataDogClient;
use attohttpc::StatusCode;

struct HttpDataDogLogger {
    datadog_url : Url,
    api_key : String
}

impl DataDogClient for HttpDataDogLogger {
    fn new(api_key : &str, datadog_url : Url) -> Result<Box<Self>, DataDogLoggerError> {
        Ok(Box::new(HttpDataDogLogger {
            api_key : api_key.into(),
            datadog_url,
        }))
    }

    fn send(&mut self, messages :&[DataDogLog]) -> Result<(), DataDogLoggerError> {
        if let Ok(message_formatted) = serde_json::to_string(&messages) {
            let result = attohttpc::post(&self.datadog_url)
                .header_append("Content-Type", "application/json")
                .header_append("DD-API-KEY", &self.api_key)
                .text(message_formatted)
                .send();

            if cfg!(feature = "self-log") {
                match result {
                    Ok(res) => match res.status() {
                        StatusCode::OK => println!("Received OK response from DataDog"),
                        code => eprintln!(
                            "Received {} status code from Datadog. Body : {}",
                            code,
                            res.text().unwrap_or_default()
                        ),
                    },
                    Err(e) => eprintln!("Sending to DataDog failed with error : {}", e),
                }
            } else {
                match result {
                    _ => { /* ignoring errors */ }
                };
            }
        } else if cfg!(feature = "self-log") {
            eprintln!("Error serializing message to string");
        }
        todo!()
    }
}
