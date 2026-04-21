mod admission;
mod artifacts;
mod authority;
mod counters;
mod errors;
mod saved_reuse;
mod support;

pub use admission::{admit_policy_tenant_context, classify_saved_query_policy_tenant_reuse};
pub(crate) use artifacts::{tenant_schema_identity, tenant_truth_identity};
pub use artifacts::{
    AdmittedPolicyTenantContext, PolicyAdmissionDisposition, PolicyBasis, PolicyBasisIdentity,
    PolicyExecutionModeRequest, PolicyReuseEquivalenceContract, PolicyTenantAdmissionBundle,
    PolicyTenantAdmissionDigest, SavedQueryPolicyReuseDescriptor, SavedQueryPolicyReuseDisposition,
};
pub use authority::{
    BranchAccessGrant, BranchAccessGrantClass, PolicyCostPosture, PolicyEpoch, PolicyRuleSnapshot,
    PolicyWorkBudget,
};
pub use counters::{PolicyBasisCounters, PolicyTenantAdmissionCounters};
pub use errors::{PolicyTenantAdmissionError, PolicyTenantAdmissionFailureClass};
pub use support::{
    runtime_backed_policy_tenant_admission_support_profile, PolicyTenantAdmissionSupportProfile,
    PolicyTenantPhaseOneSurface, PolicyTenantSupportStatus,
};

#[cfg(test)]
mod tests;
