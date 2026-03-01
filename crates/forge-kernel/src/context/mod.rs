//! # Context — ModelingContext and policy engine
//!
//! DOMAIN: The context governs all policy decisions and decision tracing.
//! INVARIANTS: Every tolerance decision is logged (Doctrine D2).
//! DEPENDENCIES: `forge-core` (DecisionLog, TracedDecision), `tolerance` (policy structs)
//!
//! ## Modules
//!
//! - `data`               — struct definitions and constructors
//! - `logic`              — behavior and orchestration
//! - `facade`             — stable exports for external callers

mod data;
pub mod facade;
mod logic;

pub use data::{ModelingContext, ResolvedPolicyDecision, ResolvedPolicySource, SubOperationMetadata};
