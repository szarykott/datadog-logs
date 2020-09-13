#[cfg(feature = "http")]
use datadog_logs::client::HttpDataDogClient;
use datadog_logs::{
    client::TcpDataDogClient, config::DataDogConfig, logger::DataDogLogLevel, logger::DataDogLogger,
};

mod utils;

#[test]
#[cfg(feature = "http")]
fn test_logger_stops_http() {
    let config = DataDogConfig::default();
    let (logger, _) = DataDogLogger::new::<HttpDataDogClient>(config).unwrap();

    logger.log("message", DataDogLogLevel::Alert);

    // it should hang forever if logging loop does not break
    std::mem::drop(logger);
}

#[test]
fn test_logger_stops_tcp_without_tls() {
    let mut config = DataDogConfig::default();
    config.tcp_config.use_tls = false;
    let (logger, _) = DataDogLogger::new::<TcpDataDogClient>(config).unwrap();

    logger.log("message", DataDogLogLevel::Alert);

    // it should hang forever if logging loop does not break
    std::mem::drop(logger);
}

#[test]
fn test_logger_stops_tcp_with_tls() {
    let config = DataDogConfig::default();
    let (logger, _) = DataDogLogger::new::<TcpDataDogClient>(config).unwrap();

    logger.log("message", DataDogLogLevel::Alert);

    // it should hang forever if logging loop does not break
    std::mem::drop(logger);
}
