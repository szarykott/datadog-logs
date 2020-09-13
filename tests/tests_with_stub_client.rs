mod utils;

use datadog_logs::{
    config::DataDogConfig, 
    logger::{DataDogLogLevel, DataDogLogger}, 
    self_log::SelfLogEvent
};

#[test]
fn test_simple_logger_run() {
    let mut config = DataDogConfig::default();
    config.enable_self_log = true;
    let (logger, self_log) = DataDogLogger::new::<utils::DataDogClientStub>(config).unwrap();

    logger.log("message", DataDogLogLevel::Alert);

    std::mem::drop(logger);

    let logs : Vec<SelfLogEvent> = self_log.iter().collect();
    assert_eq!(vec![SelfLogEvent::Start, SelfLogEvent::Succes, SelfLogEvent::Stop], logs);
}