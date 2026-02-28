//! Provenance subsystem.
//!
//! DOMAIN: Lineage tracking, replay logs, and re-identification infrastructure.

pub mod data;
pub mod logic;

pub mod facade;
pub use facade::*;
