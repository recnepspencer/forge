//! Query-owned runtime support entry points for Worth topology.
//!
//! `milestone_one_invariant_registrations()` is the public registration input
//! pack for Query-owned runtime surfaces. The lower relational runtime-builder
//! helpers in `validation::reference_integrity` are crate-private support code.

pub use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters, TopologyRuntimeFailure,
    TopologyRuntimeSchemaAdapter, TopologyRuntimeSupport, TopologyRuntimeWriteAuthority,
};
pub use crate::validation::reference_integrity::milestone_one_invariant_registrations;
