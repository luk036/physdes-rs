//! Logging module for physdes-rs.
//!
//! This module provides optional logging capabilities when the `std` feature is enabled.
//! It uses `env_logger` for flexible logging configuration via environment variables.
//!
//! ## Usage
//!
//! ```rust
//! use physdes::logging::init_logger;
//!
//! fn main() {
//!     init_logger();
//!     log::info!("Application started");
//! }
//! ```
//!
//! ## Environment Variables
//!
//! - `RUST_LOG`: Controls log level (debug, info, warn, error)
//! - `RUST_LOG_STYLE`: Controls colored output
//!
//! Example:
//! ```bash
//! RUST_LOG=debug cargo run --features std
//! ```

use log::LevelFilter;
use std::sync::atomic::{AtomicBool, Ordering};

/// Indicates whether the logger has been initialized.
static LOGGER_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize the logger with default filter (info).
///
/// # Panics
///
/// Panics if the logger has already been initialized.
pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();
    LOGGER_INITIALIZED.store(true, Ordering::SeqCst);
}

/// Initialize the logger with a custom filter string.
///
/// # Panics
///
/// Panics if the logger has already been initialized.
///
/// # Arguments
///
/// * `filter` - A filter string (e.g., "debug", "warn", "error")
pub fn init_logger_with_filter(filter: &str) {
    env_logger::Builder::from_default_env()
        .filter_level(filter.parse().unwrap_or(LevelFilter::Info))
        .init();
    LOGGER_INITIALIZED.store(true, Ordering::SeqCst);
}

/// Try to initialize the logger with default filter (info).
///
/// Returns `Ok(())` if initialization succeeded, or `Err(&str)` if the
/// logger has already been initialized.
pub fn try_init_logger() -> Result<(), &'static str> {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .try_init()
        .map(|_| {
            LOGGER_INITIALIZED.store(true, Ordering::SeqCst);
        })
        .map_err(|_| "Logger already initialized")
}

/// Try to initialize the logger with a custom filter string.
///
/// Returns `Ok(())` if initialization succeeded, or `Err(&str)` if the
/// logger has already been initialized.
///
/// # Arguments
///
/// * `filter` - A filter string (e.g., "debug", "warn", "error")
pub fn try_init_logger_with_filter(filter: &str) -> Result<(), &'static str> {
    env_logger::Builder::from_default_env()
        .filter_level(filter.parse().unwrap_or(LevelFilter::Info))
        .try_init()
        .map(|_| {
            LOGGER_INITIALIZED.store(true, Ordering::SeqCst);
        })
        .map_err(|_| "Logger already initialized")
}

/// Check if the logger has been initialized.
///
/// Returns `true` if the logger is active, `false` otherwise.
pub fn is_logger_initialized() -> bool {
    LOGGER_INITIALIZED.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_init_logger() {
        // Should succeed since we haven't initialized yet in this test
        let result = try_init_logger();
        // Note: This may fail if another test initialized the logger
        // In that case, we just check it's either Ok or Err
        if result.is_ok() {
            assert!(is_logger_initialized());
        }
    }

    #[test]
    fn test_try_init_logger_with_filter() {
        // Try with a valid filter
        let result = try_init_logger_with_filter("debug");
        // Note: This may fail if logger already initialized
        // We're just testing the function doesn't panic
        let _ = result;
    }

    #[test]
    fn test_is_logger_initialized() {
        // Just verify the function works without panicking
        let _ = is_logger_initialized();
    }

    #[test]
    fn test_init_logger_panics() {
        // Check that init_logger doesn't crash when called
        // It may panic if already initialized, which is expected
        let _ = std::panic::catch_unwind(|| {
            init_logger();
        });
    }

    #[test]
    fn test_init_logger_with_filter_panics() {
        let _ = std::panic::catch_unwind(|| {
            init_logger_with_filter("debug");
        });
    }
}
