mod admission;
mod artifacts;
mod authority;
mod counters;
mod errors;
mod saved_reuse;
mod support;

pub use admission::admit_policy_tenant_context;
pub(crate) use artifacts::{tenant_schema_identity, tenant_truth_identity};
pub use artifacts::{
    AdmittedPolicyTenantContext, PolicyAdmissionDisposition, PolicyBasis, PolicyBasisIdentity,
    PolicyExecutionModeRequest, PolicyTenantAdmissionBundle, PolicyTenantAdmissionDigest,
};
pub use authority::{
    BranchAccessGrant, BranchAccessGrantClass, PolicyCostPosture, PolicyEpoch, PolicyRuleSnapshot,
    PolicyWorkBudget,
};
pub use counters::{PolicyBasisCounters, PolicyTenantAdmissionCounters};
pub use errors::{PolicyTenantAdmissionError, PolicyTenantAdmissionFailureClass};
pub(crate) use saved_reuse::{
    build_saved_query_policy_reuse_evaluation, saved_query_policy_reuse_artifact_digest,
    saved_query_policy_reuse_disposition, saved_query_policy_reuse_surface_posture,
    SavedQueryPolicyReuseEvaluation,
};
pub use saved_reuse::{
    classify_saved_query_policy_tenant_reuse, PolicyReuseEquivalenceContract,
    SavedQueryPolicyReuseDescriptor, SavedQueryPolicyReuseDisposition,
};
pub use support::{
    runtime_backed_policy_tenant_admission_support_profile, PolicyTenantAdmissionSupportProfile,
    PolicyTenantPhaseOneSurface, PolicyTenantSupportStatus,
};

#[cfg(test)]
mod tests;
