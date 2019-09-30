use std::fmt;
use std::io;
use std::result;

/// An enum that represents all types of errors that can occur when using PickleDB
#[derive(Debug)]
pub(crate) enum ErrorType {
    /// I/O error when reading or writing to file, for example: file not found, etc.
    Io,
    /// An error when trying to serialize or deserialize data
    Serialization,
    /// Locking contention monsters 
    Lockness,
    /// DB File access errors 
    DBFileAccess,
    /// Error in request or response
    MessageFormat,
}

/// SusuDB struct Error enum wrapper 
pub struct Error {
    err_code: ErrorCode,
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub fn new(err_code: ErrorCode) -> Error {
        Error { err_code }
    }

    /// Get the error type
    fn get_type(&self) -> ErrorType {
        match self.err_code {
            ErrorCode::Io(_) => ErrorType::Io,
            ErrorCode::Serialization(_) => ErrorType::Serialization,
            ErrorCode::Lockness(_) => ErrorType::Lockness,
            ErrorCode::DBFileAccess(_) => ErrorType::DBFileAccess,
            ErrorCode::MessageFormat(ref err_str) => ErrorType::MessageFormat,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err_code {
            ErrorCode::Io(ref err) => fmt::Display::fmt(err, f),
            ErrorCode::Serialization(ref err_str) => f.write_str(err_str),
            ErrorCode::Lockness(ref err_str) => f.write_str(err_str),
            ErrorCode::DBFileAccess(ref err_str) => f.write_str(err_str),
            ErrorCode::MessageFormat(ref err_str) => f.write_str(err_str),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!(
            "Error {{ msg: {} }}",
            match self.err_code {
                ErrorCode::Io(ref err) => err.to_string(),
                ErrorCode::Serialization(ref err_str) => err_str.to_string(),
                ErrorCode::Lockness(ref err_str) => err_str.to_string(),
                ErrorCode::DBFileAccess(ref err_str) => err_str.to_string(),
                ErrorCode::MessageFormat(ref err_str) => err_str.to_string(),
            }
        ))
    }
}

pub enum ErrorCode {
    Io(io::Error),
    Serialization(String),
    Lockness(String),
    DBFileAccess(String),
    MessageFormat(String)
}
