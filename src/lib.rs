//! Simple crate to send logs directly to DataDog via HTTP or TCP.
//!
//! It offloads the job of sending logs to DataDog to a separate thread.
//! Therefore it is easy to integrate it with some crates providing synchronous logging API like `log`.
//!
//! # Feature flags
//! ### full
//! Enables all features except for `self-log` that needs to be enabled separately.
//!
//! ### http
//! Enables optional HTTP logger. 
//! It is disabled by default not to bring unnecessary dependencies that increase compilation time.
//!
//! ### log-integration
//! Enables optional integration with `log` crate.
//! To set DataDogLogger as the `log` logger it is enough to call function `init_with_log`.
//!
//! ### self-log
//! Enables console logging of events inside DataDogLogger itself for debugging purposes.
#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Datadog network clients
pub mod client;
/// Logger configuration
pub mod config;
/// Library's errors
pub mod error;
#[cfg(feature = "log-integration")]
mod log_integration;
/// Contains DataDog logger implementation
pub mod logger;
