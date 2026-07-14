use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{InvariantPlanContract, InvariantViolation};

use super::relation_integrity_scopes::prepare_relation_integrity_scopes;
use super::InvariantExecutionRequest;
use crate::validation::engine::{InvariantObservation, InvariantRequestProfile};

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
        let runtime_policy = crate::validation::engine::policy::RelationalInvariantRuntime::resolve(
            profile,
            crate::validation::engine::policy::derive_invariant_context(runtime),
        );
        let consumed_groups = profile.consumed_groups();
        let applicable_groups = plan_contract
            .map(|contract| {
                contract
                    .may_invalidate_groups()
                    .intersection(consumed_groups)
            })
            .unwrap_or(consumed_groups);
        let (relation_integrity_scopes, preparation_violation): (
            Option<super::PreparedRelationIntegrityScopes>,
            Option<InvariantViolation>,
        ) = match prepare_relation_integrity_scopes(
            merged_plan,
            observation.partition_access(),
            &runtime.performance_access(),
            &runtime.config.execution.relation_integrity_scope_budget,
        ) {
            Ok(scopes) => (scopes, None),
            Err(exceeded) => (
                None,
                Some(exceeded.into_violation(profile.execution_point())),
            ),
        };
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
            relation_integrity_scopes,
            preparation_violation,
        }
    }
}
