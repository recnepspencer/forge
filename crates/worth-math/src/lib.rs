//! # worth-math
//!
//! Exact arithmetic, certified predicates, and filtered evaluation
//! for the Worth geometry kernel.
//!
//! This crate provides the mathematical foundation that all other Worth
//! crates build upon. Key concepts:
//!
//! - **Tri-valued predicates**: [`TriSign`](sign::TriSign) (Neg/Zero/Pos) — never just true/false
//! - **Certified results**: [`CertifiedTriSign`](sign::CertifiedTriSign) — only constructable by verified predicates
//! - **Filtered evaluation**: fast f64 path with exact fallback when needed
//! - **Structured errors**: [`MathError`](error::MathError) — every failure is diagnosable and replayable
//!
//! See `docs/exactness-contract.md` for the precision guarantee model.

#![forbid(unsafe_code)]

pub mod data_access;
pub mod env;
pub mod error;

pub mod prelude;
pub mod traits;

pub mod arithmetic;
pub mod coincidence;
pub mod linalg;
pub mod numeric;
pub mod predicates;

pub use data_access::{GeometrySource, PlaneCoefficients};
pub use error::MathError;
// Re-exports
pub use numeric::deterministic_rng;
pub use numeric::sign; // Commonly used // Commonly used

#[cfg(feature = "strict_env")]
pub use env::init_fpu;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
