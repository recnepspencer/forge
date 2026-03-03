//! Pipeline vertical slice.
//!
//! DOMAIN: Feature evaluation lifecycle — executor orchestration,
//! post-execution invariant validation, pipeline fingerprinting,
//! and RAII conditioning safety.

pub mod executor;
pub mod invariants;
pub mod fingerprint;
pub mod conditioning_guard;
