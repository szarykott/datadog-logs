mod utils;

use datadog_logs::{
    config::DataDogConfig,
    logger::{DataDogLogLevel, DataDogLogger},
};

#[test]
fn test_simple_logger_run() {
    let config = DataDogConfig::default();
    let client = utils::DataDogClientStub::new();

    let messages = client.messages.clone();

    let logger = DataDogLogger::new(client, config).unwrap();

    logger.log("message", DataDogLogLevel::Alert);

    std::mem::drop(logger);

    assert_eq!(1, messages.lock().unwrap().len());
}
