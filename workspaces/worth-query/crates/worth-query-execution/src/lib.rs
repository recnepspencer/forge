//! Installed Query execution authority.
//!
//! Admission decides resource eligibility. This package consumes that proof,
//! mints attempt-local provider sessions, and carries execution evidence.

#![forbid(unsafe_code)]

mod domain_computation;
mod execution_digest;

pub mod facade;
