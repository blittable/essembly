//! Essembly Core.
//!
//! This module is the core of the Essembly libraries.  It provides
//! primitives that can be used to assemble other, secure and performant
//! libraries.
//!
//!
//! Permissions (hashes, cryptographic keys, etc.) are *held*
//! by Users and *applied* to Entities.
//!
//! # Utility functions
//!
//!

#[cfg(feature = "core")]
pub mod core;
#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "inventory")]
pub mod inventory;
#[cfg(feature = "interfaces")]
pub mod interfaces;
#[cfg(feature = "pos")]
pub mod pos;
#[cfg(feature = "store")]
pub mod store;
#[cfg(feature = "logger")]
pub mod logging;
