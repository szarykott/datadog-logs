#[cfg(feature = "http")]
use datadog_logs::client::HttpDataDogClient;
use datadog_logs::{
    client::TcpDataDogClient, config::DataDogConfig, logger::DataDogLogLevel, logger::DataDogLogger,
};

#[test]
#[cfg(feature = "http")]
fn test_logger_stops_http() {
    let config = DataDogConfig::default();
    let logger = DataDogLogger::new::<HttpDataDogClient>(config).unwrap();

    logger.log("message", DataDogLogLevel::Alert);

    // it should hang forever if logging loop does not break
    std::mem::drop(logger);
}

#[test]
fn test_logger_stops_tcp() {
    let config = DataDogConfig::default();
    let logger = DataDogLogger::new::<TcpDataDogClient>(config).unwrap();

    logger.log("message", DataDogLogLevel::Alert);

    // it should hang forever if logging loop does not break
    std::mem::drop(logger);
}
