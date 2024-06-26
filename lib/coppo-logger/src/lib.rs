//! A simple logger for Coppo.
//! # Example
//! ```rust
//! use coppo_logger::prelude::*;
//!
//! success!("This is a success message");
//! ```

#![forbid(unsafe_code)]

use std::sync::OnceLock;

use colored::Colorize;

/// A simple logger for Coppo.
/// # Example
/// ```rust
/// use coppo_logger::Logger;
///
/// let logger = Logger::new(false);
/// logger.info("This is an info message");
/// logger.warn("This is a warning message");
/// logger.error("This is an error message");
/// logger.success("This is a success message");
/// ```
pub struct Logger {
    quiet: bool,
}

impl Logger {
    /// Create a new `Logger`.
    /// You can specify whether to output messages or not by passing `true` or `false` to the `quiet` parameter.
    pub fn new(quiet: bool) -> Self {
        Self { quiet }
    }

    /// Output an info message with the `bright_blue` color.
    pub fn info(&self, message: &str) {
        if !self.quiet {
            println!("{}", message.bright_cyan());
        }
    }

    /// Output a warning message with the `bright_yellow` color.
    pub fn warn(&self, message: &str) {
        if !self.quiet {
            eprintln!("{}", message.bright_yellow());
        }
    }

    /// Output an error message with the `bright_red` color.
    pub fn error(&self, message: &str) {
        if !self.quiet {
            eprintln!("{}", message.bright_red());
        }
    }

    /// Output a success message with the `bright_green` color.
    pub fn success(&self, message: &str) {
        if !self.quiet {
            println!("{}", message.bright_green());
        }
    }
}

/// Initialize the global logger for Coppo.
pub fn init_logger(quite: bool) {
    if !quite {
        LOGGER.get_or_init(|| Logger::new(false));
    } else {
        LOGGER.get_or_init(|| Logger::new(true));
    }
}

/// The global logger for Coppo.
/// You can use this logger to output messages.
///
/// Use the `info!`, `warn!`, `error!`, and `success!` macros to output messages is recommended.
/// These macros will automatically initialize the global logger if it has not been initialized.
/// They are wrappers around the `LOGGER` global variable.
pub static LOGGER: OnceLock<Logger> = OnceLock::new();

/// Output an info message with the `bright_blue` color.
/// It use the global logger for Coppo.
#[macro_export]
macro_rules! info {
    ($( $arg:expr ),*) => {
        $crate::LOGGER.get_or_init(|| Logger::new(false)).info(&format!($( $arg ),*));
    };
}

/// Output a warning message with the `bright_yellow` color.
/// It use the global logger for Coppo.
#[macro_export]
macro_rules! warn {
    ($( $arg:expr ),*) => {
        $crate::LOGGER.get_or_init(|| Logger::new(false)).warn(&format!($( $arg ),*));
    };
}

/// Output an error message with the `bright_red` color.
/// It use the global logger for Coppo.
#[macro_export]
macro_rules! error {
    ($( $arg:expr ),*) => {
        $crate::LOGGER.get_or_init(|| Logger::new(false)).error(&format!($( $arg ),*));
    };
}

/// Output a success message with the `bright_green` color.
/// It use the global logger for Coppo.
#[macro_export]
macro_rules! success {
    ($( $arg:expr ),*) => {
        $crate::LOGGER.get_or_init(|| Logger::new(false)).success(&format!($( $arg ),*));
    };
}

pub mod prelude {
    pub use crate::{error, info, success, warn};
    pub use crate::{init_logger, Logger, LOGGER};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_logger() {
        info!("This is an info message");
        warn!("This is a warning message");
        error!("This is an error message");
        success!("This is a success message");
    }
}
