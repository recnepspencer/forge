use crate::policy_execution_seam::{
    PolicyAwareExecutionMode, PolicyAwareExecutionSeamError, PolicyAwareExecutionSeamFailureClass,
    PolicyAwareSeamCounters,
};
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;

use super::{PolicyAwarePlanCore, PolicyAwarePlanCostPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareReadBasis {
    branch_access_digest: String,
    basis_digest: String,
}

impl PolicyAwareReadBasis {
    pub fn admitted_branch(
        branch_access_digest: impl Into<String>,
        basis_digest: impl Into<String>,
    ) -> Self {
        Self {
            branch_access_digest: branch_access_digest.into(),
            basis_digest: basis_digest.into(),
        }
    }

    pub fn branch_access_digest(&self) -> &str {
        &self.branch_access_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareBranchPlan {
    core: PolicyAwarePlanCore,
    basis: PolicyAwareReadBasis,
}

impl PolicyAwareBranchPlan {
    pub fn core(&self) -> &PolicyAwarePlanCore {
        &self.core
    }

    pub fn basis(&self) -> &PolicyAwareReadBasis {
        &self.basis
    }
}

pub fn lower_policy_aware_branch_plan(
    artifact: &NarrowedPolicyQueryArtifact,
    basis: PolicyAwareReadBasis,
) -> Result<PolicyAwareBranchPlan, PolicyAwareExecutionSeamError> {
    if basis.branch_access_digest() != artifact.branch_access_digest() {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::UnsupportedPolicyExecutionMode,
            "branch policy-aware plan requires the narrowed artifact branch access digest",
            PolicyAwareSeamCounters::denied_raw_plan_bypass(),
        ));
    }
    Ok(PolicyAwareBranchPlan {
        core: PolicyAwarePlanCore::from_narrowed(
            artifact,
            PolicyAwareExecutionMode::BranchRead,
            PolicyAwarePlanCostPosture::RuntimeBranchBounded,
            artifact.authorized_projection().visible_fields().len(),
            0,
        ),
        basis,
    })
}
