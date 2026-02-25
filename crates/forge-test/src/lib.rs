//! # forge-test
//!
//! Test infrastructure for the Forge geometry kernel.
//!
//! ## Modules
//!
//! - [`fixtures`] — Reusable test fixtures and topology builders
//! - [`generators`] — Random polyhedra and Boolean pair generators
//! - [`harness`] — Self-consistency harness for corpus fuzzing
//! - [`logging`] — Universal test logging helpers

#![forbid(unsafe_code)]

pub mod fixtures;
pub mod region_merge_fixtures;
pub mod generators;
pub mod harness;
pub mod logging;

#[cfg(test)]
mod feature_tests;

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
