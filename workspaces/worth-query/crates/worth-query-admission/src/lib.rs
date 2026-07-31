//! Query admission authority.
//!
//! This package owns basis, policy, support, resource, and descriptive
//! graph-read planning decisions together with proof-bearing handoffs accepted
//! by execution. It does not mint an executable application-query plan,
//! allocate, contact providers, execute work, or publish results.

#![forbid(unsafe_code)]

mod admission_digest;
mod application_query;
mod authenticated_principal;
mod canonical_identity_derivation;
mod domain_computation;
mod graph_read_access;

pub mod facade;

#[doc(hidden)]
pub mod integration {
    pub use crate::domain_computation::execution_resource_admission::{
        admit_execution_resource_plan, reserve_execution_resource_plan,
        reserve_workflow_resource_plan, WorthQueryCapacityReservedExecutionResourcePlan,
        WorthQueryCapacityReservedWorkflowResourcePlan, WorthQueryExecutionCapacityReleaseReceipt,
        WorthQueryExecutionCapacityReservationScope,
    };
}
