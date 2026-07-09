use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{
    InvariantFailureEffect, InvariantGroup, InvariantGroupSet, InvariantPlanContract,
    InvariantVerdict,
};
use crate::validation::engine::{
    InvariantEngine, InvariantExecutionDisposition, InvariantExecutionRequest,
    InvariantExecutionResult, InvariantObservation, InvariantRequestProfile,
};
use crate::validation::logic::invariant_access::InvariantAccess;

impl<'runtime> InvariantAccess<'runtime> {
    pub(super) fn execute_for_runtime(
        &self,
        profile: InvariantRequestProfile,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            profile,
            InvariantObservation::committed(self.runtime.storage_access().current_state()),
            self.runtime.current_version_id(),
            None,
        )
    }

    pub(super) fn execute_for_runtime_plan(
        &self,
        profile: InvariantRequestProfile,
        merged_plan: &'runtime MergedCommitPlan,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            profile,
            InvariantObservation::committed(self.runtime.storage_access().current_state()),
            self.runtime.current_version_id(),
            Some(merged_plan),
        )
    }

    pub(super) fn execute_for_state(
        &self,
        profile: InvariantRequestProfile,
        observation: InvariantObservation<'runtime>,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        let plan_contract = merged_plan.map(InvariantPlanContract::from_merged_plan);
        let consumed_groups = profile.consumed_groups();
        let observation_kind = observation.kind();
        if plan_contract
            .is_some_and(|contract| !contract.intersects_consumed_groups(consumed_groups))
        {
            return InvariantExecutionResult::skipped(self.execution_metadata(
                profile,
                observation_kind,
                version_id,
                merged_plan,
                plan_contract,
                InvariantGroupSet::empty(),
                crate::validation::data::InvariantCostClass::Global,
                InvariantExecutionDisposition::SkippedByPlanContract,
            ));
        }

        let request = InvariantExecutionRequest::from_profile_with_contract(
            profile,
            self.runtime,
            observation,
            version_id,
            merged_plan,
            plan_contract,
        );
        if let Some(preparation_violation) = request.preparation_violation().cloned() {
            return self.preparation_violation_result(
                profile,
                observation_kind,
                version_id,
                merged_plan,
                plan_contract,
                &request,
                preparation_violation,
            );
        }
        if !request.should_execute_anything() {
            return InvariantExecutionResult::skipped(self.execution_metadata(
                profile,
                observation_kind,
                version_id,
                merged_plan,
                plan_contract,
                request.applicable_groups(),
                request.max_cost(),
                InvariantExecutionDisposition::SkippedByMayBreakMask,
            ));
        }
        InvariantEngine::new(self.runtime).execute(request)
    }

    fn preparation_violation_result(
        &self,
        profile: InvariantRequestProfile,
        observation_kind: crate::validation::engine::InvariantObservationKind,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
        plan_contract: Option<InvariantPlanContract>,
        request: &InvariantExecutionRequest<'runtime>,
        preparation_violation: crate::validation::data::InvariantViolation,
    ) -> InvariantExecutionResult {
        InvariantExecutionResult::executed(
            self.execution_metadata(
                profile,
                observation_kind,
                version_id,
                merged_plan,
                plan_contract,
                request.applicable_groups(),
                request.max_cost(),
                InvariantExecutionDisposition::Executed,
            ),
            vec![crate::validation::data::InvariantCheckResult {
                execution_point: profile.execution_point(),
                failure_effect: InvariantFailureEffect::BlockCommit,
                rule: crate::validation::data::InvariantReportedRule::Native(
                    crate::validation::data::InvariantRule::RelationIntegrityScopeBudget(
                        self.runtime
                            .config
                            .execution
                            .relation_integrity_scope_budget
                            .max_planned_edges,
                    ),
                ),
                groups: InvariantGroupSet::of(InvariantGroup::RelationIntegrity)
                    .union(InvariantGroupSet::of(InvariantGroup::PublicationCoherence)),
                witness: preparation_violation.witness_key(),
                cost: crate::validation::data::InvariantCostClass::Touched,
                custom_provenance: None,
                verdict: InvariantVerdict::Violation(preparation_violation),
            }],
        )
    }
}
