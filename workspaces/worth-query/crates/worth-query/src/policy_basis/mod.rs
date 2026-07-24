mod saved_reuse;

pub use worth_query_admission::facade::policy::*;

pub(crate) use saved_reuse::{
    build_saved_query_policy_reuse_evaluation, saved_query_policy_reuse_artifact_digest,
    saved_query_policy_reuse_disposition, saved_query_policy_reuse_surface_posture,
    SavedQueryPolicyReuseEvaluation,
};
pub use saved_reuse::{
    classify_saved_query_policy_tenant_reuse, PolicyReuseEquivalenceContract,
    SavedQueryPolicyReuseDescriptor, SavedQueryPolicyReuseDisposition,
};

#[cfg(test)]
#[path = "tests/saved_reuse.rs"]
mod saved_reuse_tests;
