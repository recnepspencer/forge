use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::InvariantCostClass;
use crate::validation::data::InvariantGroupSet;
use crate::validation::engine::{
    HarnessAuditMode, InvariantEngine, InvariantExecutionDisposition, InvariantExecutionMetadata,
    InvariantExecutionRequest, InvariantExecutionResult, InvariantObservation,
    InvariantObservationKind, InvariantRequestProfile,
};

impl RelationalRuntime {
    pub fn invariant_access(&self) -> InvariantAccess<'_> {
        InvariantAccess::new(self)
    }
}

pub struct InvariantAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> InvariantAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn harness_audit(&self, mode: HarnessAuditMode) -> InvariantExecutionResult {
        mode.request_profile().map_or_else(
            || {
                InvariantExecutionResult::skipped(self.execution_metadata(
                    InvariantRequestProfile::HarnessAudit,
                    InvariantObservationKind::Committed,
                    self.runtime.current_version_id(),
                    None,
                    None,
                    InvariantGroupSet::empty(),
                    InvariantCostClass::Global,
                    InvariantExecutionDisposition::SkippedByMayBreakMask,
                ))
            },
            |profile| self.execute_for_runtime(profile),
        )
    }

    pub fn mutation_sensitive_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::MutationSensitive)
    }

    pub fn snapshot_publication_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::SnapshotPublication)
    }

    pub(crate) fn mutation_sensitive_for_state(
        &self,
        state: crate::storage::overlay::OverlayStateView<
            'runtime,
            crate::logic::runtime::WorkingState,
        >,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::MutationSensitive,
            InvariantObservation::speculative(state),
            version_id,
            merged_plan,
        )
    }

    pub(crate) fn commit_boundary(
        &self,
        merged_plan: &MergedCommitPlan,
    ) -> InvariantExecutionResult {
        self.execute_for_runtime_plan(InvariantRequestProfile::CommitBoundary, merged_plan)
    }

    pub(crate) fn snapshot_publication_for_state(
        &self,
        state: crate::storage::overlay::OverlayStateView<
            'runtime,
            crate::logic::runtime::WorkingState,
        >,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::SnapshotPublication,
            InvariantObservation::speculative(state),
            version_id,
            merged_plan,
        )
    }

    fn execute_for_runtime(&self, profile: InvariantRequestProfile) -> InvariantExecutionResult {
        self.execute_for_state(
            profile,
            InvariantObservation::committed(self.runtime.storage_access().current_state()),
            self.runtime.current_version_id(),
            None,
        )
    }

    fn execute_for_runtime_plan(
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

    fn execute_for_state(
        &self,
        profile: InvariantRequestProfile,
        observation: InvariantObservation<'runtime>,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        let plan_contract =
            merged_plan.map(crate::validation::data::InvariantPlanContract::from_merged_plan);
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
                InvariantCostClass::Global,
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

    fn execution_metadata(
        &self,
        profile: InvariantRequestProfile,
        observation_kind: InvariantObservationKind,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
        plan_contract: Option<crate::validation::data::InvariantPlanContract>,
        applicable_groups: InvariantGroupSet,
        max_cost: InvariantCostClass,
        disposition: InvariantExecutionDisposition,
    ) -> InvariantExecutionMetadata {
        InvariantExecutionMetadata::new(
            profile.execution_point(),
            observation_kind,
            version_id,
            self.runtime.current_version_id(),
            profile.consumed_groups(),
            applicable_groups,
            max_cost,
            disposition,
            plan_contract,
            merged_plan.is_some(),
            self.runtime.config.execution.execution_model,
            None,
            Vec::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::InvariantAccess;
    use crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection;
    use crate::facade::identity::PartitionId;
    use crate::facade::runtime::{
        InvariantCatalog, InvariantRegistration, InvariantRule, RelationalExecutionModel,
    };
    use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
    use crate::facade::schema::RelationalSchemaRegistry;
    use crate::identity::data::KindId;
    use crate::payloads::data::RecordPayload;
    use crate::symbols::data::InternedString;
    use crate::transactions::data::{
        CreateIntent, DeleteRelationIntent, EntitySpec, MergedCommitPlan, MutationIntent,
        RelationMutationIntent, TransactionId,
    };
    use crate::validation::data::{InvariantFailureEffect, InvariantVerdict};
    use serde_json::json;

    fn runtime_with_invariants(
        invariant_catalog: InvariantCatalog,
        execution_model: RelationalExecutionModel,
    ) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .invariant_catalog(invariant_catalog)
            .execution_model(execution_model)
            .build()
    }

    #[test]
    fn commit_boundary_short_circuits_when_plan_contract_cannot_touch_profile_groups() {
        let runtime = runtime_with_invariants(
            InvariantCatalog {
                registrations: vec![InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::UniqueEntityPayloadField("name".to_string()),
                )],
                ..InvariantCatalog::default()
            },
            RelationalExecutionModel::SerialAuthority,
        );
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(1),
            merged_intents: vec![MutationIntent::Relation(RelationMutationIntent::Delete(
                DeleteRelationIntent {
                    relation_id: crate::identity::data::RelationId::new(PartitionId::main(), 0, 1),
                },
            ))],
        };

        let results = InvariantAccess::new(&runtime).commit_boundary(&plan);

        assert!(results.results().is_empty());
    }

    #[test]
    fn staged_parallel_commit_boundary_matches_serial_reference_results() {
        let invariant_catalog = InvariantCatalog {
            registrations: vec![
                InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::UniqueEntityPayloadField("name".to_string()),
                ),
                InvariantRegistration::commit_boundary_blocking(InvariantRule::MaxMergedIntents(0)),
            ],
            ..InvariantCatalog::default()
        };
        let serial_runtime = runtime_with_invariants(
            invariant_catalog.clone(),
            RelationalExecutionModel::SerialAuthority,
        );
        let staged_runtime = runtime_with_invariants(
            invariant_catalog,
            RelationalExecutionModel::StagedParallelPreparation,
        );
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(2),
            merged_intents: vec![MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("dup".to_string()),
                payload: RecordPayload::StructuredJson(json!({"name":"dup"})),
            }))],
        };

        let serial = InvariantAccess::new(&serial_runtime).commit_boundary(&plan);
        let staged = InvariantAccess::new(&staged_runtime).commit_boundary(&plan);

        assert_eq!(serial.results(), staged.results());
        assert_eq!(
            serial.summary().result_count(),
            staged.summary().result_count()
        );
        assert_eq!(
            staged
                .metadata()
                .preparation_strategy()
                .map(|strategy| strategy.selected_mode),
            Some(PreparationStrategySelection::StagedParallel)
        );
        assert!(staged.results().iter().any(|result| {
            result.failure_effect == InvariantFailureEffect::BlockCommit
                && matches!(result.verdict, InvariantVerdict::Violation(_))
        }));
    }
}
