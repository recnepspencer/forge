//! Persistent naming for the topology arena.
//!
//! DOMAIN: Stable entity references that survive parametric rebuild.
//!
//! DEPENDENCIES:
//! - `arena::TopologyArena` — read-only access to entity data + lineage
//! - `topology::attributes::EntityKey` — typed entity key discriminant
//! - `history::lineage::Lineage` — source of `ancestry_hash`
//!
//! INVARIANTS:
//! - Resolution is read-only (never mutates topology).
//! - A `PersistentName` with ordinal 0 is unambiguous (no split history).
//! - A `PersistentName` with ordinal > 0 was produced by a split.
//! - `resolve_name` may return zero results (entity deleted) or many (split).

pub mod eval;
pub mod schema;

#[cfg(test)]
mod tests;

pub use eval::{assign_name, resolve_name, resolve_selector};
pub use schema::{PersistentName, Selector};
