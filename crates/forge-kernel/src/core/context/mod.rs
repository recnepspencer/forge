//! # Context — ModelingContext and policy engine
//!
//! DOMAIN: The context governs all policy decisions and decision tracing.
//! INVARIANTS: Every tolerance decision is logged (Doctrine D2).
//! DEPENDENCIES: `forge-core` (DecisionLog, TracedDecision), `tolerance` (policy structs)
//!
//! ## Modules
//!
//! - `schema`             — `ModelingContext` struct definition + Default + Drop
//! - `accessors`          — config/policy getters and setters
//! - `decision_logging`   — `log_decision`, `log_escalation`, `scope`, reset
//! - `sub_operations`     — sub-operation absorption and metadata draining
//! - `counterfactual`     — classification overrides for replay
//! - `policy_resolution`  — `resolve_policy_query`, registry snapshot, cascade
//! - `topology_delta`     — `ArenaSnapshot`, `compute_topology_delta`

mod schema;
mod accessors;
mod decision_logging;
mod sub_operations;
mod counterfactual;
mod policy_resolution;
mod topology_delta;

#[cfg(test)]
mod tests;

pub use schema::{ModelingContext, SubOperationMetadata};
pub use policy_resolution::{
    ResolvedPolicySource, ResolvedPolicyDecision,
};
pub use topology_delta::{ArenaSnapshot, compute_topology_delta};
