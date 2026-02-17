//! # forge-test
//!
//! Test infrastructure for the Forge geometry kernel.
//!
//! ## Modules
//!
//! - [`fixtures`] — Reusable test fixtures and topology builders
//! - [`generators`] — Random polyhedra and Boolean pair generators
//! - [`harness`] — Self-consistency harness for corpus fuzzing

#![forbid(unsafe_code)]

pub mod fixtures;
pub mod generators;
pub mod harness;

#[cfg(test)]
mod feature_tests;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
