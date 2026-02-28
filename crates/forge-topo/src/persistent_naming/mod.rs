//! Persistent Naming subsystem.
//!
//! DOMAIN: Stable entity references that survive parametric rebuild.

pub mod data;
pub mod logic;

pub mod facade;

#[cfg(test)]
mod tests;
pub use facade::*;
