//! Core arithmetic types for the kernel.
//!
//! Includes `Rational` (exact arithmetic), `Double` (compensated float),
//! `PrecisionBudget` (complexity tracking), and `FilteredEval` (evaluation strategy).

pub mod double;
pub mod rational;
pub mod precision;
pub mod filter;

pub use double::Double;
pub use rational::Rational;
pub use precision::PrecisionBudget;
pub use filter::FilteredEval;
