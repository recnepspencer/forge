use crate::policy_execution_seam::PolicyAwareExecutionMode;
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;

use super::{PolicyAwarePlanCore, PolicyAwarePlanCostPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareCurrentPlan {
    core: PolicyAwarePlanCore,
}

impl PolicyAwareCurrentPlan {
    pub fn core(&self) -> &PolicyAwarePlanCore {
        &self.core
    }
}

pub fn lower_policy_aware_current_plan(
    artifact: &NarrowedPolicyQueryArtifact,
) -> PolicyAwareCurrentPlan {
    PolicyAwareCurrentPlan {
        core: PolicyAwarePlanCore::from_narrowed(
            artifact,
            PolicyAwareExecutionMode::CurrentRead,
            PolicyAwarePlanCostPosture::RuntimeCurrentBounded,
            artifact.authorized_projection().visible_field_paths().len(),
            0,
        ),
    }
}
