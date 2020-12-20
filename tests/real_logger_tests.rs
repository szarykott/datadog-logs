use datadog_logs::{
    client::HttpDataDogClient,
    config::DataDogConfig,
    logger::{DataDogLogLevel, DataDogLogger},
};
use futures::StreamExt;

#[test]
fn test_logger_stops_http() {
    let config = DataDogConfig::default();
    let client = HttpDataDogClient::new(&config).unwrap();
    let logger = DataDogLogger::blocking::<HttpDataDogClient>(client, config).unwrap();

    logger.log("message", DataDogLogLevel::Alert);

    // it should hang forever if logging loop does not break
    std::mem::drop(logger);
}

#[tokio::test]
async fn test_async_logger_stops_http() {
    let config = DataDogConfig::default();
    let client = HttpDataDogClient::new(&config).unwrap();
    let (logger, mut stream) = DataDogLogger::non_blocking::<HttpDataDogClient>(client, config);

    tokio::spawn(async move {
        while let Some(e) = stream.next().await {
            println!("{:?}", e);
        }
    });

    logger.log("message", DataDogLogLevel::Alert);

    // it should hang forever if logging loop does not break
    std::mem::drop(logger);
}
