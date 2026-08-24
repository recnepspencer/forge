use crate::branch::SelectedRelationalBranchState;
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{
    InvariantFailureEffect, InvariantGroup, InvariantGroupSet, InvariantPlanContract,
    InvariantVerdict,
};
use crate::validation::engine::{
    InvariantEngine, InvariantExecutionDisposition, InvariantExecutionRequest,
    InvariantExecutionResult, InvariantObservation, InvariantRequestProfile,
};
use crate::validation::invariant_access::InvariantAccess;

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

    pub(super) fn execute_for_selected_branch_committed_plan<'state>(
        &self,
        profile: InvariantRequestProfile,
        selected_state: &'state SelectedRelationalBranchState,
        merged_plan: &'state MergedCommitPlan,
    ) -> InvariantExecutionResult
    where
        'runtime: 'state,
    {
        let version_id = selected_state.version_id();
        self.execute_for_state_with_current_version(
            profile,
            InvariantObservation::committed_branch(selected_state.state()),
            version_id,
            version_id,
            Some(merged_plan),
        )
    }

    pub(super) fn execute_for_selected_branch_plan<'state>(
        &self,
        profile: InvariantRequestProfile,
        selected_state: &'state SelectedRelationalBranchState,
        proposed_working_state: &'state crate::storage::overlay::WorkingState,
        proposed_version_id: crate::identity::data::VersionId,
        merged_plan: &'runtime MergedCommitPlan,
        proposal_identity: Option<&crate::mvcc::RelationalMutationProposalIdentity>,
    ) -> InvariantExecutionResult
    where
        'runtime: 'state,
    {
        let version_id = selected_state.version_id();
        self.execute_for_state_with_current_version(
            profile,
            InvariantObservation::committed_branch_with_proposed(
                selected_state.state(),
                proposed_working_state,
                proposed_version_id,
                proposal_identity.cloned(),
            ),
            version_id,
            version_id,
            Some(merged_plan),
        )
    }

    pub(super) fn execute_for_state<'state>(
        &self,
        profile: InvariantRequestProfile,
        observation: InvariantObservation<'state>,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'state MergedCommitPlan>,
    ) -> InvariantExecutionResult
    where
        'runtime: 'state,
    {
        self.execute_for_state_with_current_version(
            profile,
            observation,
            version_id,
            self.runtime.current_version_id(),
            merged_plan,
        )
    }

    fn execute_for_state_with_current_version<'state>(
        &self,
        profile: InvariantRequestProfile,
        observation: InvariantObservation<'state>,
        version_id: crate::identity::data::VersionId,
        current_version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'state MergedCommitPlan>,
    ) -> InvariantExecutionResult
    where
        'runtime: 'state,
    {
        let plan_contract = merged_plan.map(InvariantPlanContract::from_merged_plan);
        let consumed_groups = profile.consumed_groups();
        let observation_kind = observation.kind();
        let proposal_identity = observation.proposal_identity().cloned();
        if plan_contract
            .is_some_and(|contract| !contract.intersects_consumed_groups(consumed_groups))
        {
            return InvariantExecutionResult::skipped(self.execution_metadata(
                profile,
                observation_kind,
                version_id,
                current_version_id,
                merged_plan,
                plan_contract,
                InvariantGroupSet::empty(),
                crate::validation::data::InvariantCostClass::Global,
                InvariantExecutionDisposition::SkippedByPlanContract,
                proposal_identity.as_ref(),
            ));
        }

        let request = InvariantExecutionRequest::from_profile_with_contract_at_current_version(
            profile,
            self.runtime,
            observation,
            version_id,
            current_version_id,
            merged_plan,
            plan_contract,
        );
        if let Some(preparation_violation) = request.preparation_violation().cloned() {
            return self.preparation_violation_result(
                profile,
                observation_kind,
                version_id,
                current_version_id,
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
                current_version_id,
                merged_plan,
                plan_contract,
                request.applicable_groups(),
                request.max_cost(),
                InvariantExecutionDisposition::SkippedByMayBreakMask,
                request.proposal_identity(),
            ));
        }
        InvariantEngine::new(self.runtime).execute(request)
    }

    fn preparation_violation_result<'state>(
        &self,
        profile: InvariantRequestProfile,
        observation_kind: crate::validation::engine::InvariantObservationKind,
        version_id: crate::identity::data::VersionId,
        current_version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'state MergedCommitPlan>,
        plan_contract: Option<InvariantPlanContract>,
        request: &InvariantExecutionRequest<'state>,
        preparation_violation: crate::validation::data::InvariantViolation,
    ) -> InvariantExecutionResult
    where
        'runtime: 'state,
    {
        InvariantExecutionResult::executed(
            self.execution_metadata(
                profile,
                observation_kind,
                version_id,
                current_version_id,
                merged_plan,
                plan_contract,
                request.applicable_groups(),
                request.max_cost(),
                InvariantExecutionDisposition::Executed,
                request.proposal_identity(),
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
