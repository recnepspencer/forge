//! Integrity checking subsystem for the topology arena.
//!
//! DOMAIN: Structural and geometric invariant validation.
//!
//! - `structural`: Pure connectivity checks (twins, loops, Euler formula)
//! - `geometric`: Position-dependent checks (zero-area, zero-length, signed volume)
//! - `shell`: Shared shell discovery and volume computation utilities
//! - `healing`: Orientation healing (flip winding on inverted shells)
//! - `validate`: Public API — re-exports + `ValidationLevel`
//! - `diff`: Model diffing utilities
//! - `hashing`: Structural hashing

pub mod diff;
pub mod hashing;
pub mod healing;
pub mod validate;

pub(crate) mod shell;

mod structural;
