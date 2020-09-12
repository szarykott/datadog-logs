use super::DataDogClient;
use crate::error::DataDogLoggerError;
use crate::logger::DataDogLog;
use crate::config::{DataDogConfig, DataDogTcpConfig};
use std::io::ErrorKind;
use std::io::Write;
use std::net::TcpStream;
use url::Url;

/// Datadog network client using TCP protocol directly
///
/// This client only sends data to DataDog, not verifying response.
/// While it increases performance it also hinders debugging with `self-log` feature giving misleading results.
/// This issue might be tackled in future releases.
pub struct TcpDataDogClient {
    api_key: String,
    tcp_stream: TcpStream,
    buffer: Vec<u8>,
    tcp_config : DataDogTcpConfig
}

impl DataDogClient for TcpDataDogClient {
    fn new(config : &DataDogConfig) -> Result<Box<Self>, DataDogLoggerError> {
        let tcp_config = config.tcp_config.clone();
        let datadog_domain = Url::parse(&tcp_config.domain)?;

        let tcp_stream = TcpStream::connect(datadog_domain.clone().into_string())?;
        Ok(Box::new(TcpDataDogClient {
            api_key: config.apikey.clone().into(),
            tcp_stream,
            buffer: Vec::new(),
            tcp_config
        }))
    }

    fn send(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError> {
        // Fill buffer
        self.buffer.clear();
        self.buffer.append(&mut self.api_key.bytes().collect());
        self.buffer.append(&mut " ".bytes().collect());
        self.buffer.append(&mut serde_json::to_vec(&messages)?);

        // Send the message
        let mut num_retries: u8 = 0;
        loop {
            match self
                .tcp_stream
                .write(&self.buffer)
                .and(self.tcp_stream.flush())
            {
                Ok(_) => break Ok(()),
                Err(e) => {
                    if num_retries < 3 && should_try_reconnect(e.kind()) {
                        self.tcp_stream =
                            TcpStream::connect(self.tcp_config.domain.clone())?;
                        num_retries += 1;
                    } else {
                        break Err(e.into());
                    }
                }
            }
        }
    }
}

fn should_try_reconnect(error_kind: ErrorKind) -> bool {
    match error_kind {
        ErrorKind::NotFound => false,
        ErrorKind::PermissionDenied => false,
        ErrorKind::ConnectionRefused => false,
        ErrorKind::ConnectionReset => true,
        ErrorKind::ConnectionAborted => true,
        ErrorKind::NotConnected => true,
        ErrorKind::AddrInUse => false,
        ErrorKind::AddrNotAvailable => false,
        ErrorKind::BrokenPipe => true,
        ErrorKind::AlreadyExists => false,
        ErrorKind::WouldBlock => false,
        ErrorKind::InvalidInput => false,
        ErrorKind::InvalidData => false,
        ErrorKind::TimedOut => true,
        ErrorKind::WriteZero => false,
        ErrorKind::Interrupted => true,
        ErrorKind::Other => false,
        ErrorKind::UnexpectedEof => false,
        _ => false,
    }
}
