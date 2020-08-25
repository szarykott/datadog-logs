//! Simple crate to send logs directly to DataDog via HTTP
#![deny(missing_docs)]
#[deny(unsafe_code)]

/// Contains the logger
pub mod ddlogger;
mod log_integration;
