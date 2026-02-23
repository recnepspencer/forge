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

pub mod schema;
pub mod eval;

#[cfg(test)]
mod tests;

pub use schema::{PersistentName, Selector};
pub use eval::{resolve_name, resolve_selector, assign_name};
