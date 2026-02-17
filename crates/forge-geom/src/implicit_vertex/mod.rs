//! DOMAIN: Implicit Vertex
//! INVARIANTS:
//! - Every vertex is defined by at least 3 plane references
//! - Position is derived on demand, never stored
//! - Overconstrained vertices (4+ planes) select the best-conditioned triple
//! - Inconsistent vertices return `PolicyRequired` or error (D2)
//!
//! DEPENDENCIES: `plane`, `forge-math` (predicates, error)

mod schema;
mod eval;
#[cfg(test)]
mod tests;

pub use schema::{ImplicitVertex, PlaneRef};
pub use eval::{resolve_position, select_best_triple};
