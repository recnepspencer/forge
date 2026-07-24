mod admission;
mod artifacts;
mod authority;
mod branch_access;
mod counters;
mod errors;
mod projection;
mod support;

pub use admission::{admit_policy_tenant_context, admit_policy_tenant_context_for_query_identity};
pub use artifacts::{
    AdmittedPolicyTenantContext, PolicyAdmissionDisposition, PolicyBasis, PolicyBasisIdentity,
    PolicyExecutionModeRequest, PolicyTenantAdmissionBundle, PolicyTenantAdmissionDigest,
};
pub use authority::{
    BranchAccessGrantClass, PolicyCostPosture, PolicyEpoch, PolicyRuleSnapshot, PolicyWorkBudget,
};
pub use branch_access::BranchAccessGrant;
pub use counters::{PolicyBasisCounters, PolicyTenantAdmissionCounters};
pub use errors::{PolicyTenantAdmissionError, PolicyTenantAdmissionFailureClass};
pub use projection::{PolicyAspectMask, ProjectionVisibility};
pub use support::{
    runtime_backed_policy_tenant_admission_support_profile, PolicyTenantAdmissionSupportProfile,
    PolicyTenantPhaseOneSurface, PolicyTenantSupportStatus,
};

#[cfg(test)]
mod tests;
