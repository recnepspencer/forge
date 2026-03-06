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

mod absorption;
mod resolution;
pub mod scope;
pub mod state;
mod tracing;

pub mod facade;

pub use absorption::SubOperationMetadata;
pub use resolution::{ResolvedPolicyDecision, ResolvedPolicySource};
pub use scope::OperationScope;
pub use state::ModelingContext;
