//! Semantic Attributes subsystem.
//!
//! DOMAIN: Side-car metadata map for manufacturing data.
//! Tags (material, tolerance, surface finish) without polluting
//! topological connectivity.

pub mod data;
pub mod logic;

pub mod facade;
pub use facade::*;
