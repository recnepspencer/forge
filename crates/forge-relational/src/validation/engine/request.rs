use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{
    InvariantCostClass, InvariantExecutionPoint, InvariantGroupSet, InvariantPlanContract,
    InvariantRegistration,
};

use super::observation::InvariantObservation;
use super::policy::{cost_allowed, RelationalInvariantRuntime};
use super::profile::InvariantRequestProfile;

pub(crate) struct InvariantExecutionRequest<'runtime> {
    observation: InvariantObservation<'runtime>,
    version_id: crate::identity::data::VersionId,
    current_version_id: crate::identity::data::VersionId,
    checkpoint: InvariantExecutionPoint,
    runtime_policy: RelationalInvariantRuntime,
    consumed_groups: InvariantGroupSet,
    applicable_groups: InvariantGroupSet,
    plan_contract: Option<InvariantPlanContract>,
    merged_plan: Option<&'runtime MergedCommitPlan>,
}

impl<'runtime> InvariantExecutionRequest<'runtime> {
    pub(crate) fn from_profile_with_contract(
        profile: InvariantRequestProfile,
        runtime: &'runtime crate::logic::runtime::RelationalRuntime,
        observation: InvariantObservation<'runtime>,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
        plan_contract: Option<InvariantPlanContract>,
    ) -> Self {
        debug_assert!(
            profile.supports_observation(observation.kind()),
            "invariant profile {:?} does not support {:?} observation",
            profile,
            observation.kind(),
        );
        let runtime_policy = RelationalInvariantRuntime::resolve(
            profile,
            super::policy::derive_invariant_context(runtime),
        );
        let consumed_groups = profile.consumed_groups();
        let applicable_groups = plan_contract
            .map(|contract| {
                contract
                    .may_invalidate_groups()
                    .intersection(consumed_groups)
            })
            .unwrap_or(consumed_groups);
        Self {
            observation,
            version_id,
            current_version_id: runtime.current_version_id(),
            checkpoint: profile.execution_point(),
            runtime_policy,
            consumed_groups,
            applicable_groups,
            plan_contract,
            merged_plan,
        }
    }

    pub(crate) fn observation(&self) -> &InvariantObservation<'runtime> {
        &self.observation
    }

    pub(crate) fn version_id(&self) -> crate::identity::data::VersionId {
        self.version_id
    }

    pub(crate) fn execution_point(&self) -> InvariantExecutionPoint {
        self.checkpoint
    }

    pub(crate) fn current_version_id(&self) -> crate::identity::data::VersionId {
        self.current_version_id
    }

    pub(crate) fn merged_plan(&self) -> Option<&'runtime MergedCommitPlan> {
        self.merged_plan
    }

    pub(crate) fn consumed_groups(&self) -> InvariantGroupSet {
        self.consumed_groups
    }

    pub(crate) fn applicable_groups(&self) -> InvariantGroupSet {
        self.applicable_groups
    }

    pub(crate) fn plan_contract(&self) -> Option<InvariantPlanContract> {
        self.plan_contract
    }

    pub(crate) fn max_cost(&self) -> InvariantCostClass {
        self.runtime_policy.max_cost_at(self.checkpoint)
    }

    pub(crate) fn should_execute_anything(&self) -> bool {
        self.merged_plan.is_none() || !self.applicable_groups.is_empty()
    }

    pub(crate) fn includes_registration(&self, registration: &InvariantRegistration) -> bool {
        let rule_groups = registration.rule.groups();
        self.runtime_policy.should_run(rule_groups, self.checkpoint)
            && (self.applicable_groups.is_empty() || self.applicable_groups.intersects(rule_groups))
            && self
                .plan_contract
                .is_none_or(|contract| contract.applies_to_rule(&registration.rule))
            && cost_allowed(
                self.runtime_policy.max_cost_at(self.checkpoint),
                registration.cost(),
            )
    }

    #[cfg(test)]
    pub(crate) fn with_applicable_groups(mut self, applicable_groups: InvariantGroupSet) -> Self {
        self.applicable_groups = applicable_groups;
        self
    }
}
