use super::artifact_execution::AssembledCommitExecution;

mod phase;

use phase::{
    append_durable_commit_phase, finalize_publication_phase, DurableAppendPhaseInput,
    FinalizePublicationPhaseInput,
};

pub(crate) struct CommitDurableAppendAdmission {
    runtime_instance_id: u64,
    commit_id: crate::history::data::CommitId,
    branch_id: crate::history::data::BranchId,
}

impl CommitDurableAppendAdmission {
    fn new(
        runtime: &crate::logic::runtime::RelationalRuntime,
        commit_id: crate::history::data::CommitId,
        branch_id: &crate::history::data::BranchId,
    ) -> Self {
        Self {
            runtime_instance_id: runtime.runtime_instance_id(),
            commit_id,
            branch_id: branch_id.clone(),
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        u64,
        crate::history::data::CommitId,
        crate::history::data::BranchId,
    ) {
        (self.runtime_instance_id, self.commit_id, self.branch_id)
    }
}

pub(super) struct DurableCommitExecution {
    assembled: AssembledCommitExecution,
}

pub(super) struct PublishedCommitExecution {
    admitted: super::execution_admission::AdmittedCommitExecution,
    public_structural_summary: crate::transactions::data::CommitStructuralSummary,
    invariant_executions: Vec<crate::validation::engine::InvariantExecutionResult>,
    version_id: crate::identity::data::VersionId,
    created_entities: crate::transactions::data::CommitCreatedEntityBindings,
    history: crate::authority::commit::phases::history::ResolvedCommitHistory,
    canonical_commit_envelope: std::sync::Arc<crate::replay::data::CanonicalCommitEnvelope>,
    changed_records: Vec<crate::transactions::data::RecordRef>,
    publication_snapshot: crate::snapshots::data::SnapshotHandle,
    aspect_evaluation_traces: Vec<crate::transactions::data::AspectEvaluationTrace>,
    aspect_emission_traces: Vec<crate::transactions::data::AspectEmissionTrace>,
}

impl PublishedCommitExecution {
    #[allow(clippy::type_complexity)]
    pub(super) fn into_parts(
        self,
    ) -> (
        super::execution_admission::AdmittedCommitExecution,
        crate::transactions::data::CommitStructuralSummary,
        Vec<crate::validation::engine::InvariantExecutionResult>,
        crate::identity::data::VersionId,
        crate::transactions::data::CommitCreatedEntityBindings,
        crate::authority::commit::phases::history::ResolvedCommitHistory,
        std::sync::Arc<crate::replay::data::CanonicalCommitEnvelope>,
        Vec<crate::transactions::data::RecordRef>,
        crate::snapshots::data::SnapshotHandle,
        Vec<crate::transactions::data::AspectEvaluationTrace>,
        Vec<crate::transactions::data::AspectEmissionTrace>,
    ) {
        (
            self.admitted,
            self.public_structural_summary,
            self.invariant_executions,
            self.version_id,
            self.created_entities,
            self.history,
            self.canonical_commit_envelope,
            self.changed_records,
            self.publication_snapshot,
            self.aspect_evaluation_traces,
            self.aspect_emission_traces,
        )
    }
}

pub(super) fn append_commit_durably(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    mut assembled: AssembledCommitExecution,
) -> Result<DurableCommitExecution, crate::transactions::data::TransactionCommitError> {
    let (admitted, publication, commit_id, branch_id) = assembled.append_parts();
    let append_authority = crate::durability::authority::DurableAppendAuthority::from_commit(
        CommitDurableAppendAdmission::new(runtime, commit_id, branch_id),
    );
    let (_, _, _, _, commit_log, phase_timing) = admitted.phase_view().into_parts();
    append_durable_commit_phase(
        runtime,
        DurableAppendPhaseInput {
            commit_log,
            phase_timing,
            publication,
            append_authority,
            commit_id,
            branch_id,
        },
    )?;
    Ok(DurableCommitExecution { assembled })
}

pub(super) fn publish_commit_execution(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    durable: DurableCommitExecution,
) -> PublishedCommitExecution {
    let (
        mut admitted,
        public_structural_summary,
        working_state,
        invariant_executions,
        version_id,
        created_entities,
        history,
        merge_parent_branches,
        publication,
        publication_snapshot,
        aspect_evaluation_traces,
        aspect_emission_traces,
    ) = durable.assembled.into_parts();
    let (_, _, _, _, commit_log, phase_timing) = admitted.phase_view().into_parts();
    let finalized = finalize_publication_phase(
        runtime,
        FinalizePublicationPhaseInput {
            commit_log,
            phase_timing,
            working_state,
            publication,
            version_id,
            previous_branch_head_version: history.previous_branch_head_version,
            commit_id: history.commit_id,
            commit_reference: &history.commit_reference,
            branch_id: &history.branch_id,
            merge_base_commits: &history.merge_base_commits,
            merge_parent_branches: &merge_parent_branches,
        },
    );
    let (canonical_commit_envelope, changed_records) = finalized.into_parts();
    if let Some(accounting) = admitted.take_merge_accounting() {
        runtime.performance_access().count_merge_execution_request(
            accounting.admitted_records,
            accounting.emitted_mutation_intents,
        );
    }
    PublishedCommitExecution {
        admitted,
        public_structural_summary,
        invariant_executions,
        version_id,
        created_entities,
        history,
        canonical_commit_envelope,
        changed_records,
        publication_snapshot,
        aspect_evaluation_traces,
        aspect_emission_traces,
    }
}
