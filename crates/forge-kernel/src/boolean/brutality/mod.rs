//! 🔥 PLANAR BRUTALITY SUITE — Phase 0–2 Stress Tests
//!
//! DOMAIN: Comprehensive stress testing for predicates, coincidence handling,
//! boolean splitting, topology integrity, determinism, fuzzing, and stability.
//!
//! Refactored from monolith to modular suite.

mod predicates;
mod coincidence;
mod splitting;
mod integrity;
mod determinism;
mod fuzzing;
mod sliver;
mod features;
mod serialization;

pub use predicates::*;
pub use coincidence::*;
pub use splitting::*;
pub use integrity::*;
pub use determinism::*;
pub use fuzzing::*;
pub use sliver::*;
pub use features::*;
pub use serialization::*;
