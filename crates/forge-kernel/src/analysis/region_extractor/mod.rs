//! Minimal region extractor for causal debugging.
//!
//! DOMAIN: Extract the minimal topological sub-region around a problematic
//! entity to produce a standalone, serializable test case (P3.2).
//!
//! - `schema`: `ExtractedRegion` data shape
//! - `eval`: `extract_n_ring()` BFS extraction algorithm
//!
//! DEPENDENCIES: `forge-topo` (arena, traversal), `geometry_state` (planes, positions)

mod eval;
mod schema;

pub use eval::extract_n_ring;
pub use schema::ExtractedRegion;
