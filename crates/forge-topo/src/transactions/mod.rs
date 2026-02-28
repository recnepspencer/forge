//! Transactions subsystem.
//!
//! DOMAIN: Epoch-versioned topology state with transactional mutation (Doctrine D6).

pub mod data;
pub mod logic;

pub mod facade;
pub use facade::*;
