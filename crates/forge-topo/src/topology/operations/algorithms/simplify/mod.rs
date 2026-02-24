//! Topology simplification algorithms.
//!
//! DOMAIN: Certified/validated graph cleanup and consolidation routines that
//! operate on `MutableDraft` and topology queries/operators.

pub mod cleanup;
pub mod consolidate_collinear_vertices;

pub use cleanup::cleanup_degenerate_topology;
pub use consolidate_collinear_vertices::{
    consolidate_one_collinear_vertex,
    find_collinear_vertex_candidate,
};
