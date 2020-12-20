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

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test() {
    let (sender, receiver) = unbounded();
    let (logger, mut stream) = DataDogLogger::non_blocking(
        utils::DataDogClientStub::new(sender),
        DataDogConfig::default(),
    );

    println!("Before spawning future!");

    tokio::spawn(async move {
        println!("Inside spawned future!");
        while let Some(e) = stream.next().await {
            println!("{:?}", e);
        }
        println!("Task with polling stream terminated!")
    });

    logger.log("message", DataDogLogLevel::Alert);

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    println!("After wait in test");

    std::mem::drop(logger);

    let messages: Vec<DataDogLog> = receiver.stream().collect().await;
    assert_eq!(1, messages.len());
}
