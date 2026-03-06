//! Pipeline vertical slice.
//!
//! DOMAIN: Feature evaluation lifecycle — executor orchestration,
//! post-execution invariant validation, pipeline fingerprinting,
//! and RAII conditioning safety.

pub mod conditioning_guard;
pub mod executor;
pub mod fingerprint;
pub mod invariants;
