use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{InvariantPlanContract, InvariantViolation};
use std::collections::BTreeMap;

use super::relation_integrity_scopes::prepare_relation_integrity_scopes;
use super::InvariantExecutionRequest;
use crate::validation::engine::{InvariantObservation, InvariantRequestProfile};

impl<'runtime> InvariantExecutionRequest<'runtime> {
    pub(crate) fn from_profile_with_contract(
        profile: InvariantRequestProfile,
        runtime: &'runtime crate::runtime::RelationalRuntime,
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
        let relation_scope_requirements = relation_scope_requirements_for(runtime, profile);
        let (relation_integrity_scopes, preparation_violation): (
            Option<super::PreparedRelationIntegrityScopes>,
            Option<InvariantViolation>,
        ) = match prepare_relation_integrity_scopes(
            merged_plan,
            observation.partition_access(),
            version_id,
            relation_scope_requirements,
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

fn relation_scope_requirements_for(
    runtime: &crate::runtime::RelationalRuntime,
    profile: InvariantRequestProfile,
) -> BTreeMap<crate::identity::data::KindId, super::RelationScopeRequirement> {
    runtime
        .config
        .schema
        .invariant_catalog
        .registrations_for_execution_point(profile.execution_point())
        .chain(
            runtime
                .schema_contract_runtime
                .relation_integrity_registrations
                .iter()
                .filter(|registration| registration.execution_point == profile.execution_point()),
        )
        .filter_map(|registration| super::relation_scope_requirement(&registration.rule))
        .fold(
            BTreeMap::new(),
            |mut requirements, (kind_id, requirement)| {
                let entry = requirements
                    .entry(kind_id)
                    .or_insert_with(super::RelationScopeRequirement::default);
                entry.requires_global_evaluation |= requirement.requires_global_evaluation;
                entry.requires_visible_successors |= requirement.requires_visible_successors;
                requirements
            },
        )
}
