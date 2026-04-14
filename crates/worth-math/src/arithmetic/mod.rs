//! Core arithmetic types for the kernel.
//!
//! Includes `Rational` (exact arithmetic), `Double` (compensated float),
//! `Interval` (conservative bounds), `PrecisionBudget` (complexity tracking),
//! `PrecisionMode` / `PrecisionEscalation` (predicate resolution metadata),
//! and `expansion` (Shewchuk adaptive expansion arithmetic).

pub mod double;
pub mod expansion;
pub mod interval;
pub mod precision;
pub mod rational;

pub use double::Double;
pub use interval::Interval;
pub use precision::{PrecisionBudget, PrecisionEscalation, PrecisionMode};
pub use rational::Rational;
