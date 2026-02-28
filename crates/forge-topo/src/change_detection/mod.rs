//! Change Detection subsystem.
//!
//! DOMAIN: Computing structured deltas between topology snapshots.

pub mod data;
pub mod logic;

pub mod facade;
pub use facade::*;
