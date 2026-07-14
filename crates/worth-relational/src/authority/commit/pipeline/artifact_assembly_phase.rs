use super::rejection::{attach_rejection, elapsed_micros};
use crate::authority::commit::phases::artifacts::{
    prepare_publication_artifacts, PublicationPreparation,
};
use crate::authority::commit::phases::publication::enforce_patch_budget;
use crate::authority::commit::phases::schema_continuity::SchemaContinuityPlan;
use crate::authority::commit::publication::assemble_patch;
#[cfg(test)]
use crate::authority::commit::{
    preparation::diagnostics::{emit_preparation_failure, failures::PreparationFailureClass},
    publication::{current_test_diff_preparation_fault, TestDiffPreparationFault},
};
use crate::authority::mutation::MutationEffect;
use crate::commit_strategies::data::StrategyCommitArtifactBundle;
use crate::diagnostics::data::RelationalDiagnosticsEntry;
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitLog, CommitPatchBudgetSummary, CommitPhase, CommitPhaseTiming, MergedCommitPlan,
    PublishedMergeExecutionAuthority, TransactionCommitError,
};

pub(super) struct ArtifactAssemblyInput<'a> {
    pub(super) working_state: &'a mut crate::storage::overlay::WorkingState,
    pub(super) effect: MutationEffect,
    pub(super) commit_reference: &'a CommitReference,
    pub(super) branch_id: &'a BranchId,
    pub(super) version_id: VersionId,
    pub(super) merge_parent_branches: &'a [BranchId],
    pub(super) merge_base_commits: &'a [CommitId],
    pub(super) merged_plan: &'a MergedCommitPlan,
    pub(super) strategy_commit_artifacts: Option<StrategyCommitArtifactBundle>,
    pub(super) merge_execution_authority: Option<PublishedMergeExecutionAuthority>,
    pub(super) schema_continuity: &'a SchemaContinuityPlan,
    pub(super) additional_diagnostics_entries: Vec<RelationalDiagnosticsEntry>,
}

pub(super) fn assemble_authoritative_publication_phase(
    runtime: &mut RelationalRuntime,
    commit_log: &mut CommitLog,
    phase_timing: &mut CommitPhaseTiming,
    input: ArtifactAssemblyInput<'_>,
) -> Result<PublicationPreparation, TransactionCommitError> {
    commit_log.begin_phase(CommitPhase::ArtifactAssembly);
    let phase_started = std::time::Instant::now();
    let mut effect = input.effect;
    let patch_fragments = std::mem::take(&mut effect.publication.patch_fragments);
    let patch = assemble_patch(runtime, input.commit_reference.commit_id, patch_fragments);
    emit_test_diff_preparation_failure(runtime, input.commit_reference.commit_id, &patch);
    let patch_budget_summary = CommitPatchBudgetSummary {
        patch_record_count: patch.authoritative_record_patches.len(),
        max_patch_records_per_commit: runtime
            .config
            .publication
            .policy
            .max_patch_records_per_commit,
    };
    commit_log.record_patch_budget(&patch_budget_summary);
    enforce_patch_budget(runtime, &patch)
        .map_err(|error| attach_rejection(commit_log, CommitPhase::ArtifactAssembly, error))?;
    let publication = prepare_publication_artifacts(
        runtime,
        input.working_state,
        patch,
        input.commit_reference,
        input.branch_id,
        input.version_id,
        input.merge_parent_branches,
        input.merge_base_commits,
        input.merged_plan,
        input.strategy_commit_artifacts,
        input.merge_execution_authority,
        input.schema_continuity,
        effect,
        input.additional_diagnostics_entries,
    )
    .map_err(|error| attach_rejection(commit_log, CommitPhase::ArtifactAssembly, error))?;
    record_publication_phase_artifacts(runtime, commit_log, &publication);
    commit_log.complete_phase(CommitPhase::ArtifactAssembly);
    phase_timing.artifact_assembly_micros = elapsed_micros(phase_started);
    Ok(publication)
}

fn record_publication_phase_artifacts(
    runtime: &mut RelationalRuntime,
    commit_log: &mut CommitLog,
    publication: &PublicationPreparation,
) {
    if runtime.config.diagnostics.profile.detailed_traces_enabled {
        for trace in &publication.aspect_evaluation_traces {
            runtime
                .publication_authority()
                .push_diagnostic_artifact(trace.diagnostic_artifact());
        }
        for trace in &publication.aspect_emission_traces {
            runtime
                .publication_authority()
                .push_diagnostic_artifact(trace.diagnostic_artifact());
        }
    }
    commit_log.record_changed_records(&publication.change_summary);
    commit_log.record_aspect_summary(&publication.aspect_summary);
    commit_log.record_publication_artifacts(&publication.summary);
}

#[cfg(test)]
fn emit_test_diff_preparation_failure(
    runtime: &mut RelationalRuntime,
    commit_id: CommitId,
    patch: &crate::publication::patch::data::PublishedAuthoritativePatchEnvelope,
) {
    if let Some(fault) = current_test_diff_preparation_fault() {
        let failure_class = match fault {
            TestDiffPreparationFault::FragmentCanonicalizationFailure => {
                PreparationFailureClass::FragmentCanonicalizationFailure
            }
            TestDiffPreparationFault::PacketOverlapDetected => {
                PreparationFailureClass::PacketOverlapDetected
            }
        };
        emit_preparation_failure(
            runtime,
            crate::diagnostics::data::DiagnosticsScope::PatchPublication,
            failure_class,
            commit_id,
            patch.authoritative_record_patches.len(),
        )
    }
}

#[cfg(not(test))]
fn emit_test_diff_preparation_failure(
    _runtime: &mut RelationalRuntime,
    _commit_id: CommitId,
    _patch: &crate::publication::patch::data::PublishedAuthoritativePatchEnvelope,
) {
}
