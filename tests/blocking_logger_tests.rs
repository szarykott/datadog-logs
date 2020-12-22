mod utils;

use datadog_logs::{
    config::DataDogConfig,
    logger::{DataDogLog, DataDogLogLevel, DataDogLogger},
};
use flume::unbounded;

#[test]
fn test_simple_blocking_logger_run() {
    let (sender, receiver) = unbounded();
    let logger = DataDogLogger::blocking(
        utils::DataDogClientStub::new(sender),
        DataDogConfig::default(),
    )
    .unwrap();

    logger.log("message", DataDogLogLevel::Alert);

    std::mem::drop(logger);

    let messages: Vec<DataDogLog> = receiver.iter().collect();
    assert_eq!(1, messages.len());
}
