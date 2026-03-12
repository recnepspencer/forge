use crate::logic::runtime::{PartitionAccess, RelationalRuntime};
use crate::transactions::data::MergedCommitPlan;
use crate::validation::engine::{
    HarnessAuditMode, InvariantEngine, InvariantExecutionRequest, InvariantExecutionResult,
    InvariantRequestProfile,
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
        mode.request_profile()
            .map_or_else(|| InvariantExecutionResult::new(Vec::new()), |profile| {
                self.execute_for_runtime(profile)
            })
    }

    pub fn mutation_sensitive_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::MutationSensitive)
    }

    pub fn snapshot_publication_state(&self) -> InvariantExecutionResult {
        self.execute_for_runtime(InvariantRequestProfile::SnapshotPublication)
    }

    pub(crate) fn mutation_sensitive_for_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::MutationSensitive,
            state,
            version_id,
            merged_plan,
        )
    }

    pub(crate) fn commit_boundary(&self, merged_plan: &MergedCommitPlan) -> InvariantExecutionResult {
        self.execute_for_runtime_plan(
            InvariantRequestProfile::CommitBoundary,
            merged_plan,
        )
    }

    pub(crate) fn snapshot_publication_for_state(
        &self,
        state: &impl PartitionAccess,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            InvariantRequestProfile::SnapshotPublication,
            state,
            version_id,
            merged_plan,
        )
    }

    fn execute_for_runtime(
        &self,
        profile: InvariantRequestProfile,
    ) -> InvariantExecutionResult {
        self.execute_for_state(
            profile,
            &self.runtime.current_state(),
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
            &self.runtime.current_state(),
            self.runtime.current_version_id(),
            Some(merged_plan),
        )
    }

    fn execute_for_state(
        &self,
        profile: InvariantRequestProfile,
        state: &'runtime dyn PartitionAccess,
        version_id: crate::identity::data::VersionId,
        merged_plan: Option<&'runtime MergedCommitPlan>,
    ) -> InvariantExecutionResult {
        if merged_plan.is_some_and(|plan| {
            let contract = crate::validation::data::InvariantPlanContract::from_merged_plan(plan);
            !contract.intersects_groups(profile.base_groups().mask())
        }) {
            return InvariantExecutionResult::new(Vec::new());
        }

        let request = InvariantExecutionRequest::from_profile(
            profile,
            self.runtime,
            state,
            version_id,
            merged_plan,
        );
        if !request.should_execute_anything() {
            return InvariantExecutionResult::new(Vec::new());
        }
        InvariantEngine::new(self.runtime).execute(request)
    }
}

#[cfg(test)]
mod tests {
    use super::InvariantAccess;
    use crate::facade::{
        InvariantCatalog, InvariantRegistration, InvariantRule, PartitionId, RelationId,
        RelationalRuntimeApi, RelationalSchemaRegistry,
    };
    use crate::transactions::data::{
        DeleteRelationIntent, MergedCommitPlan, MutationIntent, RelationMutationIntent,
        TransactionId,
    };

    fn runtime_with_invariants(invariant_catalog: InvariantCatalog) -> crate::facade::RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(RelationalSchemaRegistry::new())
            .invariant_catalog(invariant_catalog)
            .build()
    }

    #[test]
    fn commit_boundary_short_circuits_when_plan_contract_cannot_touch_profile_groups() {
        let runtime = runtime_with_invariants(InvariantCatalog {
            registrations: vec![InvariantRegistration::commit_boundary_blocking(
                InvariantRule::UniqueEntityPayloadField("name".to_string()),
            )],
            ..InvariantCatalog::default()
        });
        let plan = MergedCommitPlan {
            transaction_id: TransactionId(1),
            merged_intents: vec![MutationIntent::Relation(RelationMutationIntent::Delete(
                DeleteRelationIntent {
                    relation_id: RelationId::new(PartitionId::main(), 0, 1),
                },
            ))],
        };

        let results = InvariantAccess::new(&runtime).commit_boundary(&plan);

        assert!(results.results().is_empty());
    }
}
