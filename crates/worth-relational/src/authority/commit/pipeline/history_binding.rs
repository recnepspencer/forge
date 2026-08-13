use super::history_resolution_phase::resolve_authoritative_history_phase;
use super::mutation_execution::MutatedCommitExecution;
use crate::authority::commit::phases::schema_continuity::{
    emit_schema_continuity_diagnostic, resolve_schema_continuity,
};
use crate::transactions::data::{CommitPhase, PublishedMergeExecutionAuthority};

pub(super) struct HistoryBoundCommitExecution {
    mutated: MutatedCommitExecution,
    history: crate::authority::commit::phases::history::ResolvedCommitHistory,
    merge_parent_branches: Vec<crate::history::data::BranchId>,
    additional_diagnostics_entries: Vec<crate::diagnostics::data::RelationalDiagnosticsEntry>,
    merge_execution_authority: Option<PublishedMergeExecutionAuthority>,
    schema_continuity: crate::authority::commit::phases::schema_continuity::SchemaContinuityPlan,
}

impl HistoryBoundCommitExecution {
    pub(super) fn mutated_mut(&mut self) -> &mut MutatedCommitExecution {
        &mut self.mutated
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        MutatedCommitExecution,
        crate::authority::commit::phases::history::ResolvedCommitHistory,
        Vec<crate::history::data::BranchId>,
        Vec<crate::diagnostics::data::RelationalDiagnosticsEntry>,
        Option<PublishedMergeExecutionAuthority>,
        crate::authority::commit::phases::schema_continuity::SchemaContinuityPlan,
    ) {
        (
            self.mutated,
            self.history,
            self.merge_parent_branches,
            self.additional_diagnostics_entries,
            self.merge_execution_authority,
            self.schema_continuity,
        )
    }
}

pub(super) fn bind_commit_history(
    runtime: &mut crate::runtime::RelationalRuntime,
    mut mutated: MutatedCommitExecution,
) -> Result<HistoryBoundCommitExecution, crate::transactions::data::TransactionCommitError> {
    let version_id = mutated.version_id();
    let admitted = mutated.validated_mut().prepared_mut().admitted_mut();
    let merge_diagnostics_plan = admitted.merge_execution_diagnostics_plan().cloned();
    let (transaction_id, options, _, merge_plan, commit_log, phase_timing) =
        admitted.phase_view().into_parts();
    let history = resolve_authoritative_history_phase(
        runtime,
        commit_log,
        phase_timing,
        transaction_id,
        options,
        version_id,
        merge_plan,
    )?;
    let additional_diagnostics_entries = merge_plan
        .map(|plan| {
            vec![crate::merge::merge_execution_summary_entry(
                &plan.merge_execution_summary,
                &plan.structural_summary,
                history.commit_id,
            )]
        })
        .unwrap_or_default();
    if let (Some(plan), Some(diagnostics_plan)) = (merge_plan, merge_diagnostics_plan.as_ref()) {
        let artifact = crate::merge::merge_execution_success_artifact(
            &plan.merge_execution_summary,
            diagnostics_plan,
            history.commit_id,
            runtime.config.diagnostics.profile.max_entries_per_artifact,
        );
        runtime
            .publication_authority()
            .push_diagnostic_artifact(artifact);
    }
    let merge_execution_authority =
        merge_plan.map(PublishedMergeExecutionAuthority::from_merge_plan);
    let schema_continuity = resolve_schema_continuity(runtime, &history.branch_id, options)
        .map_err(|error| {
            super::rejection::attach_rejection(commit_log, CommitPhase::ArtifactAssembly, error)
        })?;
    emit_schema_continuity_diagnostic(runtime, &history.branch_id, &schema_continuity);
    let merge_parent_branches = options.merge_parent_branches.clone();
    Ok(HistoryBoundCommitExecution {
        mutated,
        history,
        merge_parent_branches,
        additional_diagnostics_entries,
        merge_execution_authority,
        schema_continuity,
    })
}
