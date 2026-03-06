//! Transactions subsystem.
//!
//! DOMAIN: Epoch-versioned topology state with transactional mutation (Doctrine D6).

pub mod data;
pub mod facade;
pub mod logic;

#[cfg(test)]
mod tests;
pub use facade::*;
