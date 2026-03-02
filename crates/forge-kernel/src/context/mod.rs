//! # Context — ModelingContext and policy engine
//!
//! DOMAIN: The context governs all policy decisions and decision tracing.
//! INVARIANTS: Every tolerance decision is logged (Doctrine D2).
//! DEPENDENCIES: `forge-core` (DecisionLog, TracedDecision), configuration (policy structs)
//!
//! ## Slices
//!
//! - `state`       — ModelingContext struct, constructors, config accessors
//! - `resolution`  — Policy cascade engine and decision types
//! - `tracing`     — Decision logging, span management, budget checking
//! - `absorption`  — Sub-operation envelope metadata ingestion

pub mod state;
mod resolution;
mod tracing;
mod absorption;

pub mod facade;

pub use state::ModelingContext;
pub use resolution::{ResolvedPolicyDecision, ResolvedPolicySource};
pub use absorption::SubOperationMetadata;
