//! Public API for context orchestration.
//!
//! External modules should import context primitives from this facade.

pub use super::{
    compute_topology_delta, ArenaSnapshot, ModelingContext, ResolvedPolicyDecision,
    ResolvedPolicySource, SubOperationMetadata,
};
