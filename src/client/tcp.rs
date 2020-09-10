use url::Url;
use std::net::TcpStream;
use crate::logger::DataDogLog;
use std::io::Write;
use crate::error::DataDogLoggerError;
use super::DataDogClient;
use std::io::ErrorKind;

pub struct TcpDataDogClient {
    api_key : String,
    datadog_url : Url,
    tcp_stream : TcpStream,
    buffer : Vec<u8>
}

impl TcpDataDogClient {
    pub fn new(api_key : &str, datadog_url : Url) -> Result<Self, DataDogLoggerError> {
        let tcp_stream = TcpStream::connect(datadog_url.clone().into_string())?;
        Ok(TcpDataDogClient {
            api_key : api_key.into(),
            datadog_url,
            tcp_stream,
            buffer : Vec::new()
        })
    }
}

impl DataDogClient for TcpDataDogClient {
    fn send(&mut self, messages :&Vec<DataDogLog>) -> Result<(), DataDogLoggerError> {
        // Fill buffer
        self.buffer.clear();
        self.buffer.append(&mut self.api_key.bytes().collect());
        self.buffer.append(&mut " ".bytes().collect());
        self.buffer.append(&mut serde_json::to_vec(&messages)?);

        // Send the message
        let mut num_retries = 0;
        loop {
            match self.tcp_stream.write(&self.buffer).and(self.tcp_stream.flush()) {
                Ok(_) => break Ok(()),
                Err(e) => {
                    if num_retries < 3 && should_try_reconnect(e.kind()) {
                        self.tcp_stream = TcpStream::connect(self.datadog_url.clone().into_string())?;
                        num_retries += 1;
                    } else {
                        break Err(e.into());
                    }
                }
            } 
        }
    }
}

fn should_try_reconnect(error_kind : ErrorKind) -> bool {
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
        _ => false
    }
}