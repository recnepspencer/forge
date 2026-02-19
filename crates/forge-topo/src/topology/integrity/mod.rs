//! Integrity checking subsystem for the topology arena.
//!
//! DOMAIN: Structural and geometric invariant validation.
//!
//! - `structural`: Pure connectivity checks (twins, loops, Euler formula)
//! - `geometric`: Position-dependent checks (zero-area, zero-length, signed volume)
//! - `validate`: Public API — re-exports + `ValidationLevel`
//! - `diff`: Model diffing utilities
//! - `hashing`: Structural hashing

pub mod diff;
pub mod hashing;
pub mod validate;

mod structural;
mod geometric;
