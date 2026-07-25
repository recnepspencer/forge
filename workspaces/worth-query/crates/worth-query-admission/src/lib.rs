//! Query admission authority.
//!
//! This package owns basis, policy, support, and resource decisions together
//! with the proof-bearing handoffs accepted by execution. It does not plan,
//! allocate, contact providers, execute work, or publish results.

#![forbid(unsafe_code)]

mod admission_digest;
mod domain_computation;

pub mod facade;

#[doc(hidden)]
pub mod integration {
    pub use crate::domain_computation::execution_resource_admission::{
        admit_execution_resource_plan, reserve_execution_resource_plan,
        reserve_workflow_resource_plan, WorthQueryCapacityReservedExecutionResourcePlan,
        WorthQueryCapacityReservedWorkflowResourcePlan,
    };
}
