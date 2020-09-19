use super::DataDogClient;
use crate::config::{DataDogConfig, DataDogTcpConfig};
use crate::error::DataDogLoggerError;
use crate::logger::DataDogLog;
use std::io::ErrorKind;
use std::io::{Write, Read};
use std::net::TcpStream;
use native_tls::{TlsConnector, TlsStream};

/// Datadog network client using TCP protocol directly
///
/// This client only sends data to DataDog, not verifying response.
/// While it increases performance it also hinders debugging with `self-log` feature giving misleading results.
/// This issue might be tackled in future releases.
pub struct TcpDataDogClient {
    api_key: String,
    tcp_stream: Connection,
    buffer: Vec<u8>,
    tcp_config: DataDogTcpConfig,
}

enum Connection {
    Encrypted(TlsStream<TcpStream>),
    Unencrypted(TcpStream)
}

impl Read for Connection {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Connection::Encrypted(stream) => stream.read(&mut buf),
            Connection::Unencrypted(stream) => stream.read(&mut buf)
        }
    }
}

impl Write for Connection {
    fn write(&mut self, mut buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Connection::Encrypted(stream) => stream.write(&mut buf),
            Connection::Unencrypted(stream) => stream.write(&mut buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Connection::Encrypted(stream) => stream.flush(),
            Connection::Unencrypted(stream) => stream.flush()
        }
    }
}

impl DataDogClient for TcpDataDogClient {
    fn new(config: &DataDogConfig) -> Result<Box<Self>, DataDogLoggerError> {
        let tcp_config = config.tcp_config.clone();
        let connection = connect(&tcp_config)?;

        Ok(Box::new(TcpDataDogClient {
            api_key: config.apikey.clone().into(),
            tcp_stream : connection,
            buffer: Vec::new(),
            tcp_config,
        }))
    }

    fn send(&mut self, messages: &[DataDogLog]) -> Result<(), DataDogLoggerError> {
        // Fill buffer
        self.buffer.clear();
        self.buffer.extend_from_slice(self.api_key.as_bytes());
        self.buffer.extend_from_slice(b" ");
        self.buffer.extend_from_slice(&serde_json::to_vec(&messages)?);

        println!("{}", std::str::from_utf8(&self.buffer).unwrap());

        // Send the message
        let mut num_retries: u8 = 0;
        loop {
            match self
                .tcp_stream
                .write_all(&self.buffer)
                .and(self.tcp_stream.flush())
            {
                Ok(_) =>  {
                    let mut rec = String::new();
                    self.tcp_stream.read_to_string(&mut rec).unwrap();
                    println!("{}", rec);
                    println!("After print");
                    break Ok(())
                },
                Err(e) => {
                    if num_retries < 3 && should_try_reconnect(e.kind()) {
                        self.tcp_stream = connect(&self.tcp_config)?;
                        num_retries += 1;
                    } else {
                        break Err(e.into());
                    }
                }
            }
        }
    }
}

fn connect(tcp_config : &DataDogTcpConfig) -> Result<Connection, DataDogLoggerError> {
    let address = if tcp_config.use_tls {
        format!("{}:{}", tcp_config.domain, tcp_config.tls_port)
    } else {
        format!("{}:{}", tcp_config.domain, tcp_config.non_tls_port)
    };

    println!("{}", address);

    let tcp_stream = TcpStream::connect(address)?;

    let connection = if tcp_config.use_tls {
        let connector = TlsConnector::new().unwrap();
        let tls_tcp_stream = connector.connect(tcp_config.domain.as_str(), tcp_stream).unwrap();
        Connection::Encrypted(tls_tcp_stream)
    } else {
        Connection::Unencrypted(tcp_stream)
    };

    Ok(connection)
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
