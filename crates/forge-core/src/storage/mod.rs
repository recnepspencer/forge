//! Transactional key-value storage primitives.
//!
//! DOMAIN: Generic infrastructure for mapping typed keys to values
//! with transactional commit/rollback semantics. Used by geometry,
//! attribute, and material stores across the kernel.
//!
//! INVARIANTS:
//! - `PropertyPatch::commit()` is idempotent per key.
//! - `PropertyPatch::rollback()` is a no-op that drops pending mutations.

mod schema;
#[cfg(test)]
mod tests;

pub use schema::{PropertyLayer, PropertyPatch};
