use crate::logic::runtime::PartitionAccess;
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{
    InvariantExecutionPoint, InvariantPlanContract, InvariantRegistration,
};

use super::policy::{cost_allowed, RelationalInvariantRuntime};
use super::profile::InvariantRequestProfile;

pub(crate) struct InvariantExecutionRequest<'runtime> {
    state: &'runtime dyn PartitionAccess,
    version_id: crate::identity::data::VersionId,
    checkpoint: InvariantExecutionPoint,
    runtime_policy: RelationalInvariantRuntime,
    may_break_mask: u32,
    plan_contract: Option<InvariantPlanContract>,
    merged_plan: Option<&'runtime MergedCommitPlan>,
}

impl<'runtime> InvariantExecutionRequest<'runtime> {
    pub(crate) fn from_profile(
        profile: InvariantRequestProfile,
        runtime: &'runtime crate::logic::runtime::RelationalRuntime,
        state: &'runtime dyn PartitionAccess,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> Self {
        let runtime_policy =
            RelationalInvariantRuntime::resolve(profile, super::policy::derive_invariant_context(runtime));
        let base_group_mask = profile.base_groups().mask();
        let plan_contract = merged_plan.map(InvariantPlanContract::from_merged_plan);
        let may_break_mask = plan_contract
            .map(|contract| contract.may_break_groups() & base_group_mask)
            .unwrap_or(base_group_mask);
        Self {
            state,
            version_id,
            checkpoint: profile.execution_point(),
            runtime_policy,
            may_break_mask,
            plan_contract,
            merged_plan,
        }
    }

    pub(crate) fn state(&self) -> &'runtime dyn PartitionAccess {
        self.state
    }

    pub(crate) fn version_id(&self) -> crate::identity::data::VersionId {
        self.version_id
    }

    pub(crate) fn execution_point(&self) -> InvariantExecutionPoint {
        self.checkpoint
    }

    pub(crate) fn merged_plan(&self) -> Option<&'runtime MergedCommitPlan> {
        self.merged_plan
    }

    pub(crate) fn plan_contract(&self) -> Option<InvariantPlanContract> {
        self.plan_contract
    }

    pub(crate) fn should_execute_anything(&self) -> bool {
        self.merged_plan.is_none() || self.may_break_mask != 0
    }

    pub(crate) fn includes_registration(&self, registration: &InvariantRegistration) -> bool {
        let rule_groups = registration.rule.groups().mask();
        self.runtime_policy.should_run(rule_groups, self.checkpoint)
            && (self.may_break_mask == 0 || (self.may_break_mask & rule_groups) != 0)
            && cost_allowed(
                self.runtime_policy.max_cost_at(self.checkpoint),
                registration.cost(),
            )
    }

    #[cfg(test)]
    pub(crate) fn with_may_break_mask(mut self, may_break_mask: u32) -> Self {
        self.may_break_mask = may_break_mask;
        self
    }
}
