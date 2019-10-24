//!
//! Essembly provides no-std APIs, typically for embedded platforms.
//!
//! Additionally, [`Error`], [`ErrorKind`], and [`Result`] are re-exported
//!

#[cfg(feature = "micro")]
pub use essembly-micro::MicroDevice;
