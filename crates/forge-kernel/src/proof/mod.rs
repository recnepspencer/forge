//! Decision query and replay infrastructure.
//!
//! DOMAIN: Proof system — causal chain reconstruction, counterfactual
//! replay, region extraction, invariant checkpoint system, and
//! structural proof validators.
//!
//! DEPENDENCIES: `forge-topo` (arena traversal, lineage, replay),
//!               `forge-core` (decisions, errors), `geometry_state` (positions)

pub mod causal_chain;
pub mod checkpoint;
pub mod counterfactual;
pub mod invariants;
pub mod region_extractor;

// Invariant step implementations (moved from operations/steps/)
pub(crate) mod detect_slivers;
#[cfg(test)]
mod tests;
pub(crate) mod validate_manifold;
