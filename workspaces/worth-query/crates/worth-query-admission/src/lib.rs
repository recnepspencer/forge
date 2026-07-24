//! Query admission authority.
//!
//! This package owns basis, policy, support, and resource decisions together
//! with the proof-bearing handoffs accepted by execution. It does not plan,
//! allocate, contact providers, execute work, or publish results.

#![forbid(unsafe_code)]

mod admission_digest;
mod domain_computation;

pub mod facade;
