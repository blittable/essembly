//Reexport this for logger initialization
pub use logging::trace::EssemblySubscriber;

#[cfg(feature = "simple")]
pub use logging::simple::*;

#[cfg(feature = "trace")]
pub use logging::trace::*;

#[cfg(feature = "logging")]
pub use logging::*;
