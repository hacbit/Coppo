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
            println!("{}", message.bright_blue());
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
pub static LOGGER: OnceLock<Logger> = OnceLock::new();

/// Output an info message with the `bright_blue` color.
#[macro_export]
macro_rules! info {
    ($( $arg:expr ),*) => {
        $crate::LOGGER.get_or_init(|| Logger::new(false)).info(&format!($( $arg ),*));
    };
}

/// Output a warning message with the `bright_yellow` color.
#[macro_export]
macro_rules! warn {
    ($( $arg:expr ),*) => {
        $crate::LOGGER.get_or_init(|| Logger::new(false)).warn(&format!($( $arg ),*));
    };
}

/// Output an error message with the `bright_red` color.
#[macro_export]
macro_rules! error {
    ($( $arg:expr ),*) => {
        $crate::LOGGER.get_or_init(|| Logger::new(false)).error(&format!($( $arg ),*));
    };
}

/// Output a success message with the `bright_green` color.
#[macro_export]
macro_rules! success {
    ($( $arg:expr ),*) => {
        $crate::LOGGER.get_or_init(|| Logger::new(false)).success(&format!($( $arg ),*));
    };
}

pub mod prelude {
    pub use crate::{LOGGER, Logger, init_logger};
    pub use crate::{error, info, success, warn};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_logger() {
        let logger = Logger::new(false);
        logger.info("This is an info message");
        logger.warn("This is a warning message");
        logger.error("This is an error message");
        logger.success("This is a success message");
    }
}
