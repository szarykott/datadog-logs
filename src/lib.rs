//! Simple crate to send logs directly to DataDog via HTTP
//! 
//! It offloads the job of sending logs to DataDog to a separate thread.
//! Therefore it is easy to integrate it with some crates providing synchronous logging API like `log`.
//!
//! ## Feature flags
//! `log-integration` - enables optional integration with `log` crate
//!
//! `self-log` - enables console logging of events inside DataDogLogger itself
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(clippy::all)]

/// Contains DataDog logger implementation
pub mod logger;
#[cfg(feature = "log-integration")]
mod log_integration;
