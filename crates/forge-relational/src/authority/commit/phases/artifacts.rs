use crate::authority::commit::phases::publication::{
    canonical_commit_envelope, canonicalize_changed_records,
};
use crate::authority::commit::phases::schema_continuity::SchemaContinuityPlan;
use crate::authority::commit::publication::diagnostics_summary_artifact;
use crate::history::data::CommitReference;
use crate::publication::data::diff::RelationalPatchRecord;
use crate::transactions::data::{
    AspectEmissionTrace, AspectEvaluationTrace, CommitAspectSummary, CommitChangeSummary,
    CommitPublicationSummary, MergedCommitPlan, RecordRef, TransactionCommitError,
};

pub(crate) struct PublicationPreparation {
    pub(crate) change_summary: CommitChangeSummary,
    pub(crate) aspect_summary: CommitAspectSummary,
    pub(crate) aspect_evaluation_traces: Vec<AspectEvaluationTrace>,
    pub(crate) aspect_emission_traces: Vec<AspectEmissionTrace>,
    pub(crate) summary: CommitPublicationSummary,
    pub(crate) finalize: PublicationFinalizeArtifacts,
}

pub(crate) struct PublicationFinalizeArtifacts {
    pub(crate) artifacts: crate::storage::overlay::PublicationArtifacts,
    pub(crate) changed_records: Vec<RecordRef>,
    pub(crate) canonical_commit_envelope: crate::replay::data::CanonicalCommitEnvelope,
    pub(crate) adjacency_deltas: Vec<crate::authority::mutation::AdjacencyDelta>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn prepare_publication_artifacts(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    working_state: &mut crate::logic::runtime::WorkingState,
    patch: RelationalPatchRecord,
    commit_reference: &CommitReference,
    branch_id: &crate::history::data::BranchId,
    version_id: crate::identity::data::VersionId,
    merge_parent_branches: &[crate::history::data::BranchId],
    merge_base_commits: &[crate::history::data::CommitId],
    merged_plan: &MergedCommitPlan,
    schema_continuity: &SchemaContinuityPlan,
    effect: crate::authority::mutation::MutationEffect,
) -> Result<PublicationPreparation, TransactionCommitError> {
    let diagnostics_summary =
        diagnostics_summary_artifact(&runtime.config, effect.diagnostics.entries);
    let aspect_evaluation_traces = effect
        .publication
        .canonical_deltas
        .iter()
        .map(|delta| delta.evaluation_trace())
        .collect::<Vec<_>>();
    let aspect_emission_traces = derive_aspect_emission_traces(
        patch.position,
        &patch.records,
        &effect.publication.canonical_deltas,
    );
    let artifacts = runtime.publication_authority().assemble_publication_bundle(
        commit_reference.clone(),
        version_id,
        patch.clone(),
        diagnostics_summary.clone(),
    );
    let lineage_event_ids = runtime.lineage_authority().ensure_lineage_for_commit(
        working_state,
        commit_reference,
        &merged_plan.merged_intents,
        &effect.publication.changed_records,
    );
    let lineage_events = runtime
        .lineage_access()
        .events_by_ids(&lineage_event_ids);
    let lineage_event_count = lineage_event_ids.len();
    let canonical_commit_envelope = canonical_commit_envelope(
        runtime,
        commit_reference,
        branch_id,
        merge_parent_branches,
        merge_base_commits,
        merged_plan,
        patch.clone(),
        diagnostics_summary.clone(),
        lineage_event_ids,
        lineage_events,
        Vec::new(),
        Vec::new(),
        schema_continuity,
    )?;
    let mut changed_records = effect.publication.changed_records;
    let adjacency_deltas = effect.adjacency.deltas;
    canonicalize_changed_records(&mut changed_records);
    let change_summary = CommitChangeSummary {
        changed_record_count: changed_records.len(),
        adjacency_delta_count: adjacency_deltas.len(),
    };
    let aspect_summary = summarize_commit_aspects(&effect.publication.canonical_deltas);
    let summary = CommitPublicationSummary {
        patch_record_count: patch.records.len(),
        diagnostics_entry_count: artifacts.bundle.diagnostics_summary.entries.len(),
        lineage_event_count,
        patch_position: Some(patch.position),
        final_snapshot_id: Some(artifacts.bundle.snapshot.snapshot_id),
        merge_parent_count: commit_reference.parents.len().saturating_sub(1),
    };

    Ok(PublicationPreparation {
        change_summary,
        aspect_summary,
        aspect_evaluation_traces,
        aspect_emission_traces,
        summary,
        finalize: PublicationFinalizeArtifacts {
            artifacts,
            changed_records,
            canonical_commit_envelope,
            adjacency_deltas,
        },
    })
}

fn derive_aspect_emission_traces(
    patch_position: crate::publication::data::diff::PatchStreamPosition,
    patch_records: &[crate::publication::data::diff::PatchRecord],
    deltas: &[crate::authority::mutation::CanonicalRecordAspectDelta],
) -> Vec<AspectEmissionTrace> {
    patch_records
        .iter()
        .enumerate()
        .map(|(patch_record_index, record)| {
            let delta = deltas
                .iter()
                .find(|delta| delta.target == record.target)
                .unwrap_or_else(|| {
                    panic!(
                        "missing canonical aspect delta for emitted patch target {:?}",
                        record.target
                    )
                });
            AspectEmissionTrace {
                target: delta.target.clone(),
                patch_position,
                patch_record_index,
                structural_change: delta.structural_change,
                changed_aspects: delta.changed_aspects.clone(),
                contains_degraded_precision: delta.contains_degraded_precision,
            }
        })
        .collect()
}

fn summarize_commit_aspects(
    deltas: &[crate::authority::mutation::CanonicalRecordAspectDelta],
) -> CommitAspectSummary {
    let mut changed_entity_aspect_count = 0;
    let mut changed_relation_aspect_count = 0;
    let mut touched_aspects = Vec::new();
    let mut opaque_precision_delta_count = 0;
    let mut zero_aspect_structural_delta_count = 0;

    for delta in deltas {
        let aspect_count = delta.changed_aspects.iter().count();
        match delta.target {
            RecordRef::Entity(_) => changed_entity_aspect_count += aspect_count,
            RecordRef::Relation(_) => changed_relation_aspect_count += aspect_count,
        }
        touched_aspects.extend(delta.changed_aspects.iter().cloned());
        if delta.contains_degraded_precision {
            opaque_precision_delta_count += 1;
        }
        if delta.changed_aspects.is_empty() {
            zero_aspect_structural_delta_count += 1;
        }
    }

    CommitAspectSummary {
        changed_entity_aspect_count,
        changed_relation_aspect_count,
        touched_aspects: crate::publication::data::diff::CanonicalAspectSet::new(touched_aspects),
        opaque_precision_delta_count,
        zero_aspect_structural_delta_count,
    }
}
