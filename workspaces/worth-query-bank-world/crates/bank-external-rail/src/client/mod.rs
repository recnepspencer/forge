//! Client-side helpers: process spawning and TCP exchange, used by this
//! crate's own exit-proof tests and available to any future caller-side
//! integration.

pub mod connection;
pub mod outcome;
pub mod process_handle;
