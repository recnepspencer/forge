use crate::authority::commit::phases::publication::{
    canonical_commit_envelope, canonicalize_changed_records,
};
use crate::authority::commit::phases::schema_continuity::SchemaContinuityPlan;
use crate::authority::commit::publication::diagnostics_summary_artifact;
use crate::diagnostics::data::RelationalDiagnosticsEntry;
use crate::diagnostics::data::{DiagnosticsArtifactKind, DiagnosticsScope};
use crate::history::data::CommitReference;
use crate::publication::patch::data::PublishedAuthoritativePatchEnvelope;
use crate::transactions::data::{
    AspectEmissionTrace, AspectEvaluationTrace, CommitAspectSummary, CommitChangeSummary,
    CommitPublicationSummary, MergedCommitPlan, PublishedMergeExecutionAuthority, RecordRef,
    TransactionCommitError,
};
use std::collections::BTreeMap;

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
    patch: PublishedAuthoritativePatchEnvelope,
    commit_reference: &CommitReference,
    branch_id: &crate::history::data::BranchId,
    version_id: crate::identity::data::VersionId,
    merge_parent_branches: &[crate::history::data::BranchId],
    merge_base_commits: &[crate::history::data::CommitId],
    merged_plan: &MergedCommitPlan,
    strategy_artifacts: Option<crate::commit_strategies::data::StrategyCommitArtifactBundle>,
    merge_execution_authority: Option<PublishedMergeExecutionAuthority>,
    schema_continuity: &SchemaContinuityPlan,
    effect: crate::authority::mutation::MutationEffect,
    additional_diagnostics_entries: Vec<RelationalDiagnosticsEntry>,
) -> Result<PublicationPreparation, TransactionCommitError> {
    let diagnostics_summary = diagnostics_summary_artifact(
        &runtime.config,
        additional_diagnostics_entries,
        effect.diagnostics.entries,
    );
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
        effect
            .publication
            .canonical_deltas
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
            &effect.publication.canonical_deltas,
        )
    } else {
        Vec::new()
    };
    let artifacts = runtime.publication_authority().assemble_publication_bundle(
        commit_reference.clone(),
        version_id,
        patch.clone(),
        diagnostics_summary.clone(),
    );
    let lineage_artifact = runtime.lineage_authority().ensure_lineage_for_commit(
        working_state,
        commit_reference,
        &merged_plan.merged_intents,
        &effect.publication.changed_records,
    );
    let lineage_event_count = lineage_artifact.event_batch().counters().event_batch_width;
    let canonical_commit_envelope = canonical_commit_envelope(
        runtime,
        commit_reference,
        branch_id,
        crate::replay::data::CanonicalCommitAuthorityKind::VersionedTransaction,
        strategy_artifacts,
        merge_execution_authority,
        merge_parent_branches,
        merge_base_commits,
        merged_plan,
        patch.clone(),
        diagnostics_summary.clone(),
        lineage_artifact,
        crate::indexes::data::DerivedIndexArtifacts::default(),
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
        patch_record_count: patch.authoritative_record_patches.len(),
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
    patch_position: crate::publication::patch::data::PatchStreamPosition,
    patch_records: &[crate::publication::patch::data::PublishedAuthoritativeRecordPatch],
    deltas: &[crate::authority::mutation::CanonicalRecordAspectDelta],
) -> Vec<AspectEmissionTrace> {
    let delta_index = deltas
        .iter()
        .map(|delta| (delta.target.clone(), delta))
        .collect::<BTreeMap<_, _>>();
    patch_records
        .iter()
        .enumerate()
        .map(|(patch_record_index, record)| {
            let delta = delta_index.get(&record.target).copied().unwrap_or_else(|| {
                panic!(
                    "missing canonical aspect delta for emitted patch target {:?}",
                    record.target
                )
            });
            AspectEmissionTrace {
                target: delta.target.clone(),
                patch_position,
                patch_record_index: patch_record_index as u64,
                structural_change: delta.structural_change,
                changed_aspects: delta.changed_aspects.clone(),
                contains_opaque_aspect: delta.contains_opaque_aspect,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::derive_aspect_emission_traces;
    use crate::authority::mutation::CanonicalRecordAspectDelta;
    use crate::identity::data::{EntityId, KindId, PartitionId};
    use crate::publication::patch::data::{
        ordered_aspect_keys, PatchDetail, PatchStreamPosition, PublishedAuthoritativeRecordPatch,
        RecordStructuralChange,
    };
    use crate::schema::data::AspectContractPlanRevision;
    use crate::transactions::data::RecordRef;
    use forge_foundational::facade::AspectKey;

    #[test]
    fn aspect_emission_traces_use_indexed_target_lookup() {
        let target_a = RecordRef::Entity(EntityId::new(PartitionId(3), 1, 1));
        let target_b = RecordRef::Entity(EntityId::new(PartitionId(3), 2, 1));
        let aspect_a = AspectKey::new("a").unwrap();
        let aspect_b = AspectKey::new("b").unwrap();
        let deltas = vec![
            CanonicalRecordAspectDelta {
                target: target_a.clone(),
                kind_id: KindId(7),
                plan_revision: AspectContractPlanRevision(1),
                structural_change: RecordStructuralChange::Updated,
                changed_aspects: ordered_aspect_keys([aspect_a.clone()]),
                evaluated_bindings: Default::default(),
                contains_opaque_aspect: false,
            },
            CanonicalRecordAspectDelta {
                target: target_b.clone(),
                kind_id: KindId(7),
                plan_revision: AspectContractPlanRevision(1),
                structural_change: RecordStructuralChange::Created,
                changed_aspects: ordered_aspect_keys([aspect_b.clone()]),
                evaluated_bindings: Default::default(),
                contains_opaque_aspect: true,
            },
        ];
        let patch_records = vec![
            PublishedAuthoritativeRecordPatch {
                target: target_b.clone(),
                structural_change: RecordStructuralChange::Created,
                authoritative_patch:
                    crate::publication::patch::data::PublishedAuthoritativePatch::empty(),
                contains_opaque_aspect: true,
                detail: PatchDetail::DenseBitset(Vec::new()),
            },
            PublishedAuthoritativeRecordPatch {
                target: target_a.clone(),
                structural_change: RecordStructuralChange::Updated,
                authoritative_patch:
                    crate::publication::patch::data::PublishedAuthoritativePatch::empty(),
                contains_opaque_aspect: false,
                detail: PatchDetail::DenseBitset(Vec::new()),
            },
        ];

        let traces = derive_aspect_emission_traces(PatchStreamPosition(9), &patch_records, &deltas);
        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0].target, target_b);
        assert_eq!(traces[0].changed_aspects, ordered_aspect_keys([aspect_b]));
        assert!(traces[0].contains_opaque_aspect);
        assert_eq!(traces[1].target, target_a);
        assert_eq!(traces[1].changed_aspects, ordered_aspect_keys([aspect_a]));
        assert!(!traces[1].contains_opaque_aspect);
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
        let aspect_count = delta.changed_aspects.iter().count();
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
