//Reexport this for logger initialization

#[cfg(feature = "simple")]
pub use essembly_logging::simple;

#[cfg(feature = "trace")]
pub use essembly_logging::trace;

#[cfg(feature = "logging")]
pub use essembly_logging::*;
