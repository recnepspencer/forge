use crate::logic::runtime::PartitionAccess;
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::InvariantRegistration;
use crate::validation::data::{InvariantExecutionPoint, InvariantGroupSet};

use super::policy::InvariantExecutionPolicy;
use super::profile::InvariantRequestProfile;

pub struct InvariantExecutionRequest<'runtime> {
    state: &'runtime dyn PartitionAccess,
    version_id: crate::identity::data::VersionId,
    execution_point: InvariantExecutionPoint,
    groups: InvariantGroupSet,
    policy: InvariantExecutionPolicy,
    merged_plan: Option<&'runtime MergedCommitPlan>,
}

impl<'runtime> InvariantExecutionRequest<'runtime> {
    pub fn from_profile(
        profile: InvariantRequestProfile,
        state: &'runtime dyn PartitionAccess,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> Self {
        debug_assert!(
            !profile.requires_plan() || merged_plan.is_some(),
            "invariant request profile requires a merged plan"
        );
        Self {
            state,
            version_id,
            execution_point: profile.execution_point(),
            groups: profile.groups(),
            policy: profile.policy(),
            merged_plan,
        }
    }

    pub fn state(&self) -> &'runtime dyn PartitionAccess {
        self.state
    }

    pub fn version_id(&self) -> crate::identity::data::VersionId {
        self.version_id
    }

    pub fn execution_point(&self) -> InvariantExecutionPoint {
        self.execution_point
    }

    pub fn merged_plan(&self) -> Option<&'runtime MergedCommitPlan> {
        self.merged_plan
    }

    pub fn includes_registration(&self, registration: &InvariantRegistration) -> bool {
        registration.matches_groups(self.groups) && self.policy.allows(registration.cost)
    }

    #[cfg(test)]
    pub(crate) fn with_groups(mut self, groups: InvariantGroupSet) -> Self {
        self.groups = groups;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_policy(mut self, policy: InvariantExecutionPolicy) -> Self {
        self.policy = policy;
        self
    }
}
