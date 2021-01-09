mod utils;

use datadog_logs::{
    config::DataDogConfig,
    logger::{DataDogLog, DataDogLogLevel, DataDogLogger},
};
use flume::{unbounded, Receiver, RecvTimeoutError};

#[test]
fn test_no_messages_sent() {
    let (logger, receiver) = create_default_logger();

    std::mem::drop(logger);

    let messages: Vec<DataDogLog> = receiver.iter().collect();
    assert_eq!(0, messages.len());
}

#[test]
fn test_one_message_sent() {
    let (logger, receiver) = create_default_logger();

    logger.log("message", DataDogLogLevel::Alert);

    std::mem::drop(logger);

    let messages: Vec<DataDogLog> = receiver.iter().collect();
    assert_eq!(1, messages.len());
}

#[test]
fn test_many_messages_sent() {
    let (logger, receiver) = create_default_logger();

    let iterations = 1000;

    for i in 0..iterations {
        logger.log(format!("message{}", i), DataDogLogLevel::Informational);
    }

    std::mem::drop(logger);

    let messages: Vec<DataDogLog> = receiver.iter().collect();
    assert_eq!(iterations, messages.len());
}

#[test]
fn test_messages_are_not_duplicated() {
    let (logger, receiver) = create_default_logger();

    let iterations = 1000;

    for i in 0..iterations {
        logger.log(format!("message{}", i), DataDogLogLevel::Informational);
    }

    std::mem::drop(logger);

    let mut messages: Vec<DataDogLog> = receiver.iter().collect();
    messages.dedup();
    assert_eq!(iterations, messages.len());
}

// multithreaded executor is needed here due to blocking in a loop
#[test]
fn test_messages_are_streamed() {
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
                    count += 1;
                    if count % 10 == 0 {
                        break;
                    }
                }
                Err(RecvTimeoutError::Disconnected) => {
                    while let Ok(msg) = selflog.recv() {
                        println!("{}", msg);
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
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
    let logger = DataDogLogger::blocking(
        utils::DataDogClientStub::new(sender),
        DataDogConfig {
            enable_self_log: true,
            ..Default::default()
        },
    );

    (logger, receiver)
}
