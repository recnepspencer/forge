//! Topology stitching logic.
//!
//! DOMAIN: Pair halfedges via twin pointers after face assembly.
//!
//! ALGORITHM: For each directed edge (origin→dest), find the matching
//! reverse edge (dest→origin) and set twin pointers. When multiple
//! halfedges share the same directed edge (non-manifold junction from
//! boolean intersection), radially sort them by face normal around the
//! edge axis for deterministic pairing.

mod eval;
mod fallback;

pub use eval::stitch_twins;
