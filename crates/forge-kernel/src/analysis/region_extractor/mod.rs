//! Minimal region extractor for causal debugging.
//!
//! DOMAIN: Extract the minimal topological sub-region around a problematic
//! entity to produce a standalone, serializable test case (P3.2).
//!
//! - `schema`: `ExtractedRegion` data shape
//! - `eval`: `extract_n_ring()` BFS extraction algorithm
//!
//! DEPENDENCIES: `forge-topo` (arena, traversal), `geometry_state` (planes, positions)

mod schema;
mod eval;

pub use schema::ExtractedRegion;
pub use eval::extract_n_ring;
