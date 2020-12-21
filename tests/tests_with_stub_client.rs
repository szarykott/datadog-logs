mod utils;

use datadog_logs::{
    config::DataDogConfig,
    logger::{DataDogLog, DataDogLogLevel, DataDogLogger},
};
use flume::unbounded;
use futures::StreamExt;

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

#[tokio::test]
async fn test_simple_nonblocking_logger_run() {
    let (sender, receiver) = unbounded();
    let (logger, future) = DataDogLogger::non_blocking_cold(
        utils::DataDogClientStub::new(sender),
        DataDogConfig::default(),
    );

    println!("Before spawning future!");

    tokio::spawn(future);

    println!("After spawning future!");

    logger.log("message", DataDogLogLevel::Alert);

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    println!("After wait in test");

    std::mem::drop(logger);

    let messages: Vec<DataDogLog> = receiver.stream().collect().await;
    assert_eq!(1, messages.len());
}
