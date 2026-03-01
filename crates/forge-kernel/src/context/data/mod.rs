//! Context data layer.
//!
//! DOMAIN: Pure data shapes used by the context subsystem.

mod modeling_context;
mod policy_decision;
mod sub_operation_metadata;

pub use modeling_context::ModelingContext;
pub use policy_decision::{ResolvedPolicyDecision, ResolvedPolicySource};
pub use sub_operation_metadata::SubOperationMetadata;
