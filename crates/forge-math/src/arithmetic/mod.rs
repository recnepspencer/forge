//! Core arithmetic types for the kernel.
//!
//! Includes `Rational` (exact arithmetic), `Double` (compensated float),
//! `Interval` (conservative bounds), `PrecisionBudget` (complexity tracking),
//! `PrecisionMode` / `PrecisionEscalation` (predicate resolution metadata),
//! and `expansion` (Shewchuk adaptive expansion arithmetic).

pub mod double;
pub mod expansion;
pub mod interval;
pub mod rational;
pub mod precision;

pub use double::Double;
pub use interval::Interval;
pub use rational::Rational;
pub use precision::{PrecisionBudget, PrecisionEscalation, PrecisionMode};

