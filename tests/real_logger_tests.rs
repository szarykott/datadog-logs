use datadog_logs::client::HttpDataDogClient;
use datadog_logs::{config::DataDogConfig, logger::DataDogLogLevel, logger::DataDogLogger};

#[test]
fn test_logger_stops_http() {
    let config = DataDogConfig::default();
    let client = HttpDataDogClient::new(&config).unwrap();
    let logger = DataDogLogger::new::<HttpDataDogClient>(client, config).unwrap();

    logger.log("message", DataDogLogLevel::Alert);

    // it should hang forever if logging loop does not break
    std::mem::drop(logger);
}
