//! # forge-math
//!
//! Exact arithmetic, certified predicates, and filtered evaluation
//! for the Forge geometry kernel.
//!
//! This crate provides the mathematical foundation that all other Forge
//! crates build upon. Key concepts:
//!
//! - **Tri-valued predicates**: [`TriSign`](sign::TriSign) (Neg/Zero/Pos) — never just true/false
//! - **Certified results**: [`CertifiedTriSign`](sign::CertifiedTriSign) — only constructable by verified predicates
//! - **Filtered evaluation**: fast f64 path with exact fallback when needed
//! - **Structured errors**: [`MathError`](error::MathError) — every failure is diagnosable and replayable
//!
//! See `docs/exactness-contract.md` for the precision guarantee model.

#![forbid(unsafe_code)]

pub use error::MathError;

pub mod sign;
pub mod error;
pub mod arithmetic;
pub mod coincidence;
pub mod env;
pub mod deterministic_rng;
pub mod linalg;
pub mod predicates;

#[cfg(feature = "strict_env")]
pub use env::init_fpu;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
