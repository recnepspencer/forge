//! Topology simplification algorithms.
//!
//! DOMAIN: Certified/validated graph cleanup and consolidation routines that
//! operate on `MutableDraft` and topology queries/operators.

pub mod cleanup;

pub use cleanup::cleanup_degenerate_topology;
