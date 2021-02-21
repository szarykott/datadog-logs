//! # About
//!
//! `datadog-logs` is a DataDog logs API client with `log` integration.
//!
//! Provides support for HTTP DataDog logs ingestion API.
//! Supports blocking and nonblocking HTTP(S) clients activated by feature flags.
//!
//! Logger is easily configurable with extensive `DataDogConfig` that can be deserialized directly from file thanks to `serde`.
//!
//! It offloads the job of sending logs to DataDog to a separate thread (blocking logger) or task (nonblocking logger).
//!
//! # Using with `log` crate
//!
//!```rust
//!use datadog_logs::{config::DataDogConfig, logger::DataDogLogger, client::HttpDataDogClient};
//!use log::*;
//!
//!# async fn func() {
//!let config = DataDogConfig::default();
//!let client = HttpDataDogClient::new(&config).unwrap();
//! // there is also a blocking logger available that does not require runtime
//!let future = DataDogLogger::set_nonblocking_logger(client, config, LevelFilter::Error).unwrap();
//!
//! // there is a convinence function available to spawn future to tokio
//! // however, this design makes it compatible with every runtime without effort
//!tokio::spawn(future);
//!
//! // now you can log
//!error!("An error occured");
//!warn!("A warning");
//!# }
//!```
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]
/// Datadog network clients
pub mod client;
/// Logger configuration
pub mod config;
/// Errors
pub mod error;
/// DataDog logger implementations
pub mod logger;
