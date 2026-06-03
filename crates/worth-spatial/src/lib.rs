//! # worth-spatial
//!
//! Construction-time topology/geometry interaction authority for Worth.

#![forbid(unsafe_code)]

mod bindings;
mod spatial_intent;
#[cfg(test)]
mod structure_guard;

pub mod facade;
pub mod test_support;
