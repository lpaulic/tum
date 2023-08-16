//! Help text for T.U.M.
//!
//! Can contain information about what the binary does, command-line options,
//! configuration, etc.

mod command;
mod configuration;
mod monitor;
mod mqtt_client;
mod resource;
// ... other modules

// This is the only export from the crate. It is marked hidden and
// is not part of the public API.
#[doc(hidden)]
pub use command::Tum;
