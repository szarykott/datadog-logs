mod utils;

use datadog_logs::{
    config::DataDogConfig,
    logger::{DataDogLog, DataDogLogLevel, DataDogLogger},
};
use flume::{unbounded, Receiver, RecvTimeoutError};
use futures::StreamExt;
use std::default::Default;

#[tokio::test]
async fn test_no_messages_sent() {
    let (logger, receiver) = create_default_logger();

    std::mem::drop(logger);

    let messages: Vec<DataDogLog> = receiver.stream().collect().await;
    assert_eq!(0, messages.len());
}

#[tokio::test]
async fn test_one_message_sent() {
    let (logger, receiver) = create_default_logger();

    logger.log("message", DataDogLogLevel::Alert);

    std::mem::drop(logger);

    let messages: Vec<DataDogLog> = receiver.stream().collect().await;
    assert_eq!(1, messages.len());
}

#[tokio::test]
async fn test_many_messages_sent() {
    let (logger, receiver) = create_default_logger();

    let iterations = 1000;

    for i in 0..iterations {
        logger.log(format!("message{}", i), DataDogLogLevel::Informational);
    }

    std::mem::drop(logger);

    let messages: Vec<DataDogLog> = receiver.stream().collect().await;
    assert_eq!(iterations, messages.len());
}

#[tokio::test]
async fn test_messages_are_not_duplicated() {
    let (logger, receiver) = create_default_logger();

    let iterations = 1000;

    for i in 0..iterations {
        logger.log(format!("message{}", i), DataDogLogLevel::Informational);
    }

    std::mem::drop(logger);

    let mut messages: Vec<DataDogLog> = receiver.stream().collect().await;
    messages.dedup();
    assert_eq!(iterations, messages.len());
}

// multithreaded executor is needed here due to blocking in a loop
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_messages_are_streamed() {
    let (logger, receiver) = create_default_logger();

    let selflog = logger.selflog().as_ref().unwrap();

    let mut count = 0;
    for j in 0..10 {
        for i in 0..10 {
            logger.log(format!("message{}", i), DataDogLogLevel::Informational);
        }

        loop {
            match receiver.recv_timeout(std::time::Duration::from_secs(1)) {
                Ok(_) => {
                    println!("ok");
                    count += 1;
                    if count % 10 == 0 {
                        break;
                    }
                }
                Err(RecvTimeoutError::Disconnected) => {
                    println!("disconnected");
                    while let Ok(msg) = selflog.recv() {
                        println!("{}", msg);
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    println!("timeout");
                    while let Ok(msg) = selflog.recv() {
                        println!("{}", msg);
                    }
                }
            }
        }

        assert_eq!(10 * (j + 1), count);
    }
}

fn create_default_logger() -> (DataDogLogger, Receiver<DataDogLog>) {
    let (sender, receiver) = unbounded();
    let logger = DataDogLogger::non_blocking_with_tokio(
        utils::DataDogClientStub::new(sender),
        DataDogConfig {
            enable_self_log: true,
            ..Default::default()
        },
    );
    (logger, receiver)
}
