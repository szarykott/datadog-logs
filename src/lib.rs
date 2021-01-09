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
//! # Examples
//!
//! ## Using with `log` crate
//!
//! TODO: Add example
//!
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]
/// Datadog network clients
pub mod client;
/// Logger configuration
pub mod config;
/// Errors
pub mod error;
#[cfg(feature = "log-integration")]
mod log_integration;
/// DataDog logger implementations
pub mod logger;
