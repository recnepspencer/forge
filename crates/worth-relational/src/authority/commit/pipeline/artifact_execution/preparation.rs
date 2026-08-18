use crate::authority::commit::phases::publication::{
    canonical_commit_envelope, canonicalize_changed_records,
};
use crate::authority::commit::phases::schema_continuity::SchemaContinuityPlan;
use crate::authority::commit::publication::diagnostics_summary_artifact;
use crate::diagnostics::data::RelationalDiagnosticsEntry;
use crate::diagnostics::data::{DiagnosticsArtifactKind, DiagnosticsScope};
use crate::history::data::RelationalCommitReceipt;
use crate::publication::patch::data::PublishedAuthoritativePatchEnvelope;
use crate::transactions::data::{
    AspectEmissionTrace, AspectEvaluationTrace, CommitAspectSummary, CommitChangeSummary,
    CommitPublicationSummary, MergedCommitPlan, PublishedMergeExecutionAuthority, RecordRef,
    TransactionCommitError,
};
mod traces;

use traces::derive_aspect_emission_traces;

pub(in crate::authority::commit::pipeline) struct PublicationPreparation {
    change_summary: CommitChangeSummary,
    aspect_summary: CommitAspectSummary,
    aspect_evaluation_traces: Vec<AspectEvaluationTrace>,
    aspect_emission_traces: Vec<AspectEmissionTrace>,
    summary: CommitPublicationSummary,
    finalize: PublicationFinalizeArtifacts,
}

pub(in crate::authority::commit::pipeline) struct PublicationFinalizeArtifacts {
    artifacts: crate::storage::overlay::PublicationArtifacts,
    changed_records: Vec<RecordRef>,
    canonical_commit_envelope: crate::history::data::CanonicalCommitEnvelope,
    adjacency_deltas: Vec<crate::authority::mutation::AdjacencyDelta>,
}

pub(super) struct PublicationPreparationInput<'a> {
    pub(super) working_state: &'a mut crate::runtime::WorkingState,
    pub(super) patch: PublishedAuthoritativePatchEnvelope,
    pub(super) commit_reference: &'a RelationalCommitReceipt,
    pub(super) branch_id: &'a crate::history::data::BranchId,
    pub(super) version_id: crate::identity::data::VersionId,
    pub(super) merge_parent_branches: &'a [crate::history::data::BranchId],
    pub(super) merge_base_commits: &'a [crate::history::data::CommitId],
    pub(super) merged_plan: &'a MergedCommitPlan,
    pub(super) strategy_artifacts:
        Option<crate::commit_strategies::data::StrategyCommitArtifactBundle>,
    pub(super) merge_execution_authority: Option<PublishedMergeExecutionAuthority>,
    pub(super) schema_continuity: &'a SchemaContinuityPlan,
    pub(super) effect: crate::authority::mutation::MutationEffect,
    pub(super) additional_diagnostics_entries: Vec<RelationalDiagnosticsEntry>,
}

pub(super) fn prepare_publication_artifacts(
    runtime: &mut crate::runtime::RelationalRuntime,
    input: PublicationPreparationInput<'_>,
) -> Result<PublicationPreparation, TransactionCommitError> {
    let PublicationPreparationInput {
        working_state,
        patch,
        commit_reference,
        branch_id,
        version_id,
        merge_parent_branches,
        merge_base_commits,
        merged_plan,
        strategy_artifacts,
        merge_execution_authority,
        schema_continuity,
        effect,
        additional_diagnostics_entries,
    } = input;
    let crate::authority::mutation::MutationEffect {
        publication,
        diagnostics,
        adjacency,
    } = effect;
    let traces = capture_publication_traces(runtime, &patch, &publication.canonical_deltas);
    let diagnostics_summary = diagnostics_summary_artifact(
        &runtime.config,
        additional_diagnostics_entries,
        diagnostics.entries,
    );
    let authority = prepare_authoritative_publication(
        runtime,
        PublicationAuthorityInput {
            working_state,
            patch: &patch,
            commit_reference,
            branch_id,
            version_id,
            merge_parent_branches,
            merge_base_commits,
            merged_plan,
            strategy_artifacts,
            merge_execution_authority,
            schema_continuity,
            changed_records: &publication.changed_records,
            diagnostics_summary: &diagnostics_summary,
        },
    )?;
    Ok(finish_publication_preparation(PublicationCompletionInput {
        patch,
        commit_reference,
        changed_records: publication.changed_records,
        canonical_deltas: publication.canonical_deltas,
        adjacency_deltas: adjacency.deltas,
        traces,
        authority,
    }))
}

struct PublicationTraceCapture {
    aspect_evaluation_traces: Vec<AspectEvaluationTrace>,
    aspect_emission_traces: Vec<AspectEmissionTrace>,
}

fn capture_publication_traces(
    runtime: &crate::runtime::RelationalRuntime,
    patch: &PublishedAuthoritativePatchEnvelope,
    canonical_deltas: &[crate::authority::mutation::CanonicalRecordAspectDelta],
) -> PublicationTraceCapture {
    let diagnostics_profile = &runtime.config.diagnostics.profile;
    let capture_transaction_traces = diagnostics_profile.should_capture_artifact(
        DiagnosticsScope::Transaction,
        DiagnosticsArtifactKind::DetailedTrace,
    );
    let capture_patch_publication_traces = diagnostics_profile.should_capture_artifact(
        DiagnosticsScope::PatchPublication,
        DiagnosticsArtifactKind::DetailedTrace,
    );
    let aspect_evaluation_traces = if capture_transaction_traces {
        canonical_deltas
            .iter()
            .map(|delta| delta.evaluation_trace())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let aspect_emission_traces = if capture_patch_publication_traces {
        derive_aspect_emission_traces(
            patch.position,
            &patch.authoritative_record_patches,
            canonical_deltas,
        )
    } else {
        Vec::new()
    };
    PublicationTraceCapture {
        aspect_evaluation_traces,
        aspect_emission_traces,
    }
}

struct PublicationAuthorityInput<'a> {
    working_state: &'a mut crate::runtime::WorkingState,
    patch: &'a PublishedAuthoritativePatchEnvelope,
    commit_reference: &'a RelationalCommitReceipt,
    branch_id: &'a crate::history::data::BranchId,
    version_id: crate::identity::data::VersionId,
    merge_parent_branches: &'a [crate::history::data::BranchId],
    merge_base_commits: &'a [crate::history::data::CommitId],
    merged_plan: &'a MergedCommitPlan,
    strategy_artifacts: Option<crate::commit_strategies::data::StrategyCommitArtifactBundle>,
    merge_execution_authority: Option<PublishedMergeExecutionAuthority>,
    schema_continuity: &'a SchemaContinuityPlan,
    changed_records: &'a [RecordRef],
    diagnostics_summary: &'a crate::diagnostics::data::RelationalDiagnosticArtifact,
}

struct PreparedPublicationAuthority {
    artifacts: crate::storage::overlay::PublicationArtifacts,
    canonical_commit_envelope: crate::history::data::CanonicalCommitEnvelope,
    lineage_event_count: usize,
}

fn prepare_authoritative_publication(
    runtime: &mut crate::runtime::RelationalRuntime,
    input: PublicationAuthorityInput<'_>,
) -> Result<PreparedPublicationAuthority, TransactionCommitError> {
    let artifacts = runtime.publication_authority().assemble_publication_bundle(
        input.commit_reference.clone(),
        input.version_id,
        input.patch.clone(),
        input.diagnostics_summary.clone(),
    );
    let lineage_artifact = runtime.lineage_authority().ensure_lineage_for_commit(
        input.working_state,
        input.commit_reference,
        &input.merged_plan.merged_intents,
        input.changed_records,
    );
    let lineage_event_count = lineage_artifact.event_batch().counters().event_batch_width;
    let canonical_commit_envelope = canonical_commit_envelope(
        runtime,
        input.commit_reference,
        input.branch_id,
        crate::history::data::CanonicalCommitAuthorityKind::VersionedTransaction,
        input.strategy_artifacts,
        input.merge_execution_authority,
        input.merge_parent_branches,
        input.merge_base_commits,
        input.merged_plan,
        input.patch.clone(),
        input.diagnostics_summary.clone(),
        lineage_artifact,
        crate::indexes::data::DerivedIndexArtifacts::default(),
        input.schema_continuity,
    )?;
    Ok(PreparedPublicationAuthority {
        artifacts,
        canonical_commit_envelope,
        lineage_event_count,
    })
}

struct PublicationCompletionInput<'a> {
    patch: PublishedAuthoritativePatchEnvelope,
    commit_reference: &'a RelationalCommitReceipt,
    changed_records: Vec<RecordRef>,
    canonical_deltas: Vec<crate::authority::mutation::CanonicalRecordAspectDelta>,
    adjacency_deltas: Vec<crate::authority::mutation::AdjacencyDelta>,
    traces: PublicationTraceCapture,
    authority: PreparedPublicationAuthority,
}

fn finish_publication_preparation(input: PublicationCompletionInput<'_>) -> PublicationPreparation {
    let PublicationCompletionInput {
        patch,
        commit_reference,
        changed_records,
        canonical_deltas,
        adjacency_deltas,
        traces,
        authority,
    } = input;
    let mut changed_records = changed_records;
    canonicalize_changed_records(&mut changed_records);
    let change_summary = CommitChangeSummary {
        changed_record_count: changed_records.len(),
        adjacency_delta_count: adjacency_deltas.len(),
    };
    let aspect_summary = summarize_commit_aspects(&canonical_deltas);
    let summary = CommitPublicationSummary {
        patch_record_count: patch.authoritative_record_patches.len(),
        diagnostics_entry_count: authority.artifacts.bundle.diagnostics_summary.entries.len(),
        lineage_event_count: authority.lineage_event_count,
        patch_position: Some(patch.position),
        final_snapshot_id: Some(authority.artifacts.bundle.snapshot.snapshot_id),
        merge_parent_count: commit_reference.parents.len().saturating_sub(1),
    };

    PublicationPreparation {
        change_summary,
        aspect_summary,
        aspect_evaluation_traces: traces.aspect_evaluation_traces,
        aspect_emission_traces: traces.aspect_emission_traces,
        summary,
        finalize: PublicationFinalizeArtifacts {
            artifacts: authority.artifacts,
            changed_records,
            canonical_commit_envelope: authority.canonical_commit_envelope,
            adjacency_deltas,
        },
    }
}

impl PublicationPreparation {
    pub(super) fn summaries(
        &self,
    ) -> (
        &CommitChangeSummary,
        &CommitAspectSummary,
        &CommitPublicationSummary,
    ) {
        (&self.change_summary, &self.aspect_summary, &self.summary)
    }

    pub(super) fn aspect_evaluation_traces(&self) -> &[AspectEvaluationTrace] {
        &self.aspect_evaluation_traces
    }

    pub(super) fn aspect_emission_traces(&self) -> &[AspectEmissionTrace] {
        &self.aspect_emission_traces
    }

    pub(in crate::authority::commit::pipeline) fn snapshot(
        &self,
    ) -> &crate::snapshots::data::SnapshotHandle {
        &self.finalize.artifacts.bundle.snapshot
    }

    pub(in crate::authority::commit::pipeline) fn canonical_commit_envelope(
        &self,
    ) -> &crate::history::data::CanonicalCommitEnvelope {
        &self.finalize.canonical_commit_envelope
    }

    pub(in crate::authority::commit::pipeline) fn patch_position(
        &self,
    ) -> crate::publication::patch::data::PatchStreamPosition {
        self.finalize.canonical_commit_envelope.patch.position
    }

    pub(in crate::authority::commit::pipeline) fn into_finalize(
        self,
    ) -> PublicationFinalizeArtifacts {
        self.finalize
    }
}

impl PublicationFinalizeArtifacts {
    pub(in crate::authority::commit::pipeline) fn into_parts(
        self,
    ) -> (
        crate::storage::overlay::PublicationArtifacts,
        Vec<RecordRef>,
        crate::history::data::CanonicalCommitEnvelope,
        Vec<crate::authority::mutation::AdjacencyDelta>,
    ) {
        (
            self.artifacts,
            self.changed_records,
            self.canonical_commit_envelope,
            self.adjacency_deltas,
        )
    }
}

fn summarize_commit_aspects(
    deltas: &[crate::authority::mutation::CanonicalRecordAspectDelta],
) -> CommitAspectSummary {
    let mut changed_entity_aspect_count = 0;
    let mut changed_relation_aspect_count = 0;
    let mut touched_aspects = Vec::new();
    let mut opaque_aspect_delta_count = 0;
    let mut zero_aspect_structural_delta_count = 0;

    for delta in deltas {
        let aspect_count = delta.changed_aspects.len();
        match delta.target {
            RecordRef::Entity(_) => changed_entity_aspect_count += aspect_count,
            RecordRef::Relation(_) => changed_relation_aspect_count += aspect_count,
        }
        touched_aspects.extend(delta.changed_aspects.iter().cloned());
        if delta.contains_opaque_aspect {
            opaque_aspect_delta_count += 1;
        }
        if delta.changed_aspects.is_empty() {
            zero_aspect_structural_delta_count += 1;
        }
    }

    CommitAspectSummary {
        changed_entity_aspect_count,
        changed_relation_aspect_count,
        touched_aspects: crate::publication::patch::data::ordered_aspect_keys(touched_aspects),
        opaque_aspect_delta_count,
        zero_aspect_structural_delta_count,
    }
}
