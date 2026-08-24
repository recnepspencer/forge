use super::artifact_execution::AssembledCommitExecution;

mod phase;
mod preparation;

use phase::finalize_publication_phase;
use preparation::{
    prepare_publication_phase, PreparePublicationPhaseInput, PreparedPublicationPhase,
};

pub(crate) struct CommitDurableAppendAdmission {
    runtime_instance_id: u64,
    commit_id: crate::history::data::CommitId,
    branch_id: crate::history::data::BranchId,
}

impl CommitDurableAppendAdmission {
    pub(crate) fn new(
        runtime: &crate::runtime::RelationalRuntime,
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

pub(super) struct PublishedCommitExecution {
    admitted: super::execution_admission::AdmittedCommitExecution,
    public_structural_summary: crate::transactions::data::CommitStructuralSummary,
    invariant_executions: Vec<crate::validation::engine::InvariantExecutionResult>,
    version_id: crate::identity::data::VersionId,
    created_entities: crate::transactions::data::CommitCreatedEntityBindings,
    created_relations: crate::transactions::data::CommitCreatedRelationBindings,
    history: crate::authority::commit::phases::history::ResolvedCommitHistory,
    canonical_commit_envelope: std::sync::Arc<crate::history::data::CanonicalCommitEnvelope>,
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
        crate::transactions::data::CommitCreatedRelationBindings,
        crate::authority::commit::phases::history::ResolvedCommitHistory,
        std::sync::Arc<crate::history::data::CanonicalCommitEnvelope>,
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
            self.created_relations,
            self.history,
            self.canonical_commit_envelope,
            self.changed_records,
            self.publication_snapshot,
            self.aspect_evaluation_traces,
            self.aspect_emission_traces,
        )
    }
}

pub(super) fn prepare_commit_publication_execution(
    runtime: &mut crate::runtime::RelationalRuntime,
    assembled: AssembledCommitExecution,
) -> Result<PreparedCommitPublicationExecution, crate::transactions::data::TransactionCommitError> {
    let (
        mut admitted,
        selected_branch_state,
        public_structural_summary,
        working_state,
        invariant_executions,
        version_id,
        created_entities,
        created_relations,
        record_allocations,
        history,
        merge_parent_branches,
        publication,
        publication_snapshot,
        aspect_evaluation_traces,
        aspect_emission_traces,
    ) = assembled.into_parts();
    let publication = {
        let (_, _, _, _, commit_log, _) = admitted.phase_view().into_parts();
        prepare_publication_phase(
            runtime,
            PreparePublicationPhaseInput {
                commit_log,
                working_state,
                record_allocations,
                selected_branch_state,
                publication,
                version_id,
                previous_branch_head_version: history.previous_branch_head_version,
                commit_id: history.commit_id,
                commit_reference: history.commit_reference.clone(),
                branch_basis: history.branch_basis.clone(),
                branch_id: history.branch_id.clone(),
                merge_base_commits: history.merge_base_commits.clone(),
                merge_parent_branches,
            },
        )?
    };
    Ok(PreparedCommitPublicationExecution {
        admitted,
        public_structural_summary,
        invariant_executions,
        version_id,
        created_entities,
        created_relations,
        history,
        publication,
        publication_snapshot,
        aspect_evaluation_traces,
        aspect_emission_traces,
    })
}

pub(super) fn publish_commit_execution(
    runtime: &mut crate::runtime::RelationalRuntime,
    prepared: PreparedCommitPublicationExecution,
) -> Result<PublishedCommitExecution, crate::transactions::data::TransactionCommitError> {
    let PreparedCommitPublicationExecution {
        mut admitted,
        public_structural_summary,
        invariant_executions,
        version_id,
        created_entities,
        created_relations,
        history,
        publication,
        publication_snapshot,
        aspect_evaluation_traces,
        aspect_emission_traces,
    } = prepared;
    let finalized = {
        let (_, _, _, _, commit_log, phase_timing) = admitted.phase_view().into_parts();
        finalize_publication_phase(runtime, commit_log, phase_timing, publication)?
    };
    let (canonical_commit_envelope, changed_records) = finalized.into_parts();
    runtime.publish_invariant_preparation_diagnostics(&invariant_executions);
    publish_prepared_trace_diagnostics(runtime, &aspect_evaluation_traces, &aspect_emission_traces);
    if let Some(accounting) = admitted.take_merge_accounting() {
        runtime.performance_access().count_merge_execution_request(
            accounting.admitted_records,
            accounting.emitted_mutation_intents,
        );
    }
    Ok(PublishedCommitExecution {
        admitted,
        public_structural_summary,
        invariant_executions,
        version_id,
        created_entities,
        created_relations,
        history,
        canonical_commit_envelope,
        changed_records,
        publication_snapshot,
        aspect_evaluation_traces,
        aspect_emission_traces,
    })
}

fn publish_prepared_trace_diagnostics(
    runtime: &mut crate::runtime::RelationalRuntime,
    evaluation_traces: &[crate::transactions::data::AspectEvaluationTrace],
    emission_traces: &[crate::transactions::data::AspectEmissionTrace],
) {
    if !runtime.config.diagnostics.profile.detailed_traces_enabled {
        return;
    }
    for trace in evaluation_traces {
        runtime
            .publication_authority()
            .push_diagnostic_artifact(trace.diagnostic_artifact());
    }
    for trace in emission_traces {
        runtime
            .publication_authority()
            .push_diagnostic_artifact(trace.diagnostic_artifact());
    }
}

pub(crate) struct PreparedCommitPublicationExecution {
    admitted: super::execution_admission::AdmittedCommitExecution,
    public_structural_summary: crate::transactions::data::CommitStructuralSummary,
    invariant_executions: Vec<crate::validation::engine::InvariantExecutionResult>,
    version_id: crate::identity::data::VersionId,
    created_entities: crate::transactions::data::CommitCreatedEntityBindings,
    created_relations: crate::transactions::data::CommitCreatedRelationBindings,
    history: crate::authority::commit::phases::history::ResolvedCommitHistory,
    publication: PreparedPublicationPhase,
    publication_snapshot: crate::snapshots::data::SnapshotHandle,
    aspect_evaluation_traces: Vec<crate::transactions::data::AspectEvaluationTrace>,
    aspect_emission_traces: Vec<crate::transactions::data::AspectEmissionTrace>,
}

impl PreparedCommitPublicationExecution {
    pub(crate) fn reservation_count(&self) -> usize {
        self.publication.reservation_count()
    }

    #[cfg(test)]
    pub(crate) fn materialization_counts(&self) -> (u64, u64) {
        self.publication.materialization_counts()
    }
}
