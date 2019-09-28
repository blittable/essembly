use std::error::Error;
use std::fmt;

/// Errors returned by `Cli::spawn`.
///
/// Cli errors should represent relatively rare scenarios. Currently, the two
/// scenarios represented by `CliError` are:
///
#[derive(Debug)]
pub struct CliError {
    is_shutdown: bool,
}

impl CliError {
    /// Return a new `CliError` reflecting a shutdown executor failure.
    pub fn shutdown() -> Self {
        CliError { is_shutdown: true }
    }

    /// Return a new `CliError` reflecting an executor at capacity failure.
    pub fn at_capacity() -> Self {
        CliError { is_shutdown: false }
    }

    /// Returns `true` if the error reflects a shutdown executor failure.
    pub fn is_shutdown(&self) -> bool {
        self.is_shutdown
    }

    /// Returns `true` if the error reflects an executor at capacity failure.
    pub fn is_at_capacity(&self) -> bool {
        !self.is_shutdown
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "attempted to run a cli task while the executor is at capacity or shut down"
        )
    }
}

impl Error for CliError {}
