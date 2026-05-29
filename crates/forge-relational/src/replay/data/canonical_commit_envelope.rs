use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::indexes::data::{DerivedIndexArtifacts, DerivedIndexGeneration};
use crate::lineage::data::{
    LineageArtifactCounters, LineageDecisionLogDigestBasis, LineageDecisionRecord,
    LineageDigestBasis, LineageEventBatchDigestBasis, LineageEventRecord, PublishedLineageArtifact,
};
use crate::publication::patch::data::{PatchRecord, RelationalPatchRecord};
use crate::schema::data::{
    DescriptorSemanticsVersion, SchemaAuthoritySnapshot, SchemaContinuationDescriptor,
    SchemaReconciliationDescriptor, SchemaTransitionArtifact, SchemaVersionId,
};
use crate::transactions::data::{
    ExistingRecordTarget, MergedCommitPlan, MutationIntent, PublishedMergeExecutionAuthority,
    RecordRef,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalCommitEnvelope {
    pub commit: CommitReference,
    pub branch_context: BranchId,
    pub authority_kind: CanonicalCommitAuthorityKind,
    pub strategy_artifacts: Option<crate::commit_strategies::data::StrategyCommitArtifactBundle>,
    pub merge_execution_authority: Option<PublishedMergeExecutionAuthority>,
    pub merge_parent_branches: Vec<BranchId>,
    pub merge_base_commits: Vec<CommitId>,
    pub schema_version: SchemaVersionId,
    pub schema_authority: SchemaAuthoritySnapshot,
    pub merged_plan: MergedCommitPlan,
    pub patch: RelationalPatchRecord,
    pub diagnostics_summary: RelationalDiagnosticArtifact,
    lineage: PublishedLineageArtifact,
    pub derived_index_artifacts: DerivedIndexArtifacts,
    pub schema_transition: Option<SchemaTransitionArtifact>,
    pub schema_continuation_descriptor: Option<SchemaContinuationDescriptor>,
    pub schema_reconciliation_descriptor: Option<SchemaReconciliationDescriptor>,
    pub descriptor_semantics_version: DescriptorSemanticsVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalCommitAuthorityKind {
    VersionedTransaction,
    MetadataOnlyLineage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CommittedRecordChange<'a> {
    pub commit: &'a CommitReference,
    pub record: &'a PatchRecord,
}

impl CanonicalCommitEnvelope {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        commit: CommitReference,
        branch_context: BranchId,
        authority_kind: CanonicalCommitAuthorityKind,
        strategy_artifacts: Option<crate::commit_strategies::data::StrategyCommitArtifactBundle>,
        merge_execution_authority: Option<PublishedMergeExecutionAuthority>,
        merge_parent_branches: Vec<BranchId>,
        merge_base_commits: Vec<CommitId>,
        schema_version: SchemaVersionId,
        schema_authority: SchemaAuthoritySnapshot,
        merged_plan: MergedCommitPlan,
        patch: RelationalPatchRecord,
        diagnostics_summary: RelationalDiagnosticArtifact,
        lineage: PublishedLineageArtifact,
        derived_index_artifacts: DerivedIndexArtifacts,
        schema_transition: Option<SchemaTransitionArtifact>,
        schema_continuation_descriptor: Option<SchemaContinuationDescriptor>,
        schema_reconciliation_descriptor: Option<SchemaReconciliationDescriptor>,
        descriptor_semantics_version: DescriptorSemanticsVersion,
    ) -> Self {
        Self {
            commit,
            branch_context,
            authority_kind,
            strategy_artifacts,
            merge_execution_authority,
            merge_parent_branches,
            merge_base_commits,
            schema_version,
            schema_authority,
            merged_plan,
            patch,
            diagnostics_summary,
            lineage,
            derived_index_artifacts,
            schema_transition,
            schema_continuation_descriptor,
            schema_reconciliation_descriptor,
            descriptor_semantics_version,
        }
    }

    pub fn lineage_event_ids(&self) -> &[u64] {
        self.lineage.lineage_event_ids()
    }

    pub fn lineage_events(&self) -> &[LineageEventRecord] {
        self.lineage.lineage_events()
    }

    pub fn lineage_decision_log(&self) -> &[LineageDecisionRecord] {
        self.lineage.lineage_decision_log()
    }

    pub fn lineage_decisions_for_candidate(
        &self,
        candidate_id: crate::lineage::data::CorrespondenceCandidateId,
    ) -> Vec<&LineageDecisionRecord> {
        self.lineage.decisions_for_candidate(candidate_id).collect()
    }

    pub fn lineage_decisions_for_event_id(&self, event_id: u64) -> Vec<&LineageDecisionRecord> {
        self.lineage.decisions_for_event_id(event_id).collect()
    }

    pub fn lineage_decisions_for_rejection_class(
        &self,
        rejection_class: crate::lineage::data::CorrespondencePromotionRejectionClass,
    ) -> Vec<&LineageDecisionRecord> {
        self.lineage
            .decisions_for_rejection_class(rejection_class)
            .collect()
    }

    pub fn lineage_digest_basis(&self) -> &LineageDigestBasis {
        self.lineage.digest_basis()
    }

    pub fn event_batch_digest_basis(&self) -> &LineageEventBatchDigestBasis {
        self.lineage.event_batch_digest_basis()
    }

    pub fn decision_log_digest_basis(&self) -> &LineageDecisionLogDigestBasis {
        self.lineage.decision_log_digest_basis()
    }

    pub fn lineage_artifact_counters(&self) -> LineageArtifactCounters {
        self.lineage.counters()
    }

    pub fn has_lineage_authority(&self) -> bool {
        self.lineage.has_authority_content()
    }

    pub fn authority_kind(&self) -> CanonicalCommitAuthorityKind {
        self.authority_kind
    }

    pub(crate) fn published_lineage(&self) -> &PublishedLineageArtifact {
        &self.lineage
    }

    pub fn derived_index_artifacts(&self) -> &DerivedIndexArtifacts {
        &self.derived_index_artifacts
    }

    pub(crate) fn touched_record_refs(&self) -> BTreeSet<RecordRef> {
        let mut touched = self
            .patch
            .records
            .iter()
            .map(|record| record.target.clone())
            .collect::<BTreeSet<_>>();
        for intent in &self.merged_plan.merged_intents {
            if let Some(target) = intent_record_ref(intent) {
                touched.insert(target);
            }
        }
        touched
    }

    pub(crate) fn committed_record_changes(
        &self,
    ) -> impl Iterator<Item = CommittedRecordChange<'_>> {
        self.patch
            .records
            .iter()
            .map(|record| CommittedRecordChange {
                commit: &self.commit,
                record,
            })
    }

    pub(crate) fn committed_record_changes_for_target<'a>(
        &'a self,
        target: &'a RecordRef,
    ) -> impl Iterator<Item = CommittedRecordChange<'a>> + 'a {
        self.committed_record_changes()
            .filter(move |change| &change.record.target == target)
    }

    #[cfg(test)]
    pub(crate) fn published_lineage_mut_for_test(&mut self) -> &mut PublishedLineageArtifact {
        &mut self.lineage
    }

    pub fn append_index_generations_canonical(&mut self, generations: &[DerivedIndexGeneration]) {
        self.derived_index_artifacts.extend_canonical(generations);
    }
}

fn intent_record_ref(intent: &MutationIntent) -> Option<RecordRef> {
    intent.existing_record_target().map(|target| match target {
        ExistingRecordTarget::Entity(entity_id) => RecordRef::Entity(entity_id),
        ExistingRecordTarget::Relation(relation_id) => RecordRef::Relation(relation_id),
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySchemaVersion(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalReplayRecord {
    pub schema_version: ReplaySchemaVersion,
    pub commit_id: crate::history::data::CommitId,
    pub version_id: crate::identity::data::VersionId,
    pub snapshot_id: crate::snapshots::data::SnapshotId,
    pub patch: crate::publication::patch::data::RelationalPatchRecord,
    pub schema_authority: SchemaAuthoritySnapshot,
}

#[cfg(test)]
mod tests {
    use super::{CanonicalCommitAuthorityKind, CanonicalCommitEnvelope};
    use crate::diagnostics::data::{
        DeterminismExpectation, DiagnosticsArtifactKind, DiagnosticsScope,
        RelationalDiagnosticArtifact,
    };
    use crate::history::data::{BranchId, CommitId, CommitReference};
    use crate::identity::data::{EntityId, PartitionId, VersionId};
    use crate::indexes::data::DerivedIndexArtifacts;
    use crate::lineage::data::{
        FinalizedLineageEventBatch, LineageDecisionLog, LineageFinalizationArtifact,
    };
    use crate::publication::patch::data::{
        PatchDetail, PatchOrdering, PatchPublicationMode, PatchRecord, PatchStreamPosition,
        RecordStructuralChange, RelationalPatchRecord,
    };
    use crate::schema::data::{
        DescriptorSemanticsVersion, RelationalSchemaRegistry, SchemaVersionId,
    };
    use crate::transactions::data::{
        AspectFieldPatch, EntityMutationIntent, MergedCommitPlan, MutationIntent, RecordRef,
        TransactionId, UpdateEntityFieldsIntent,
    };

    fn envelope_with_patch_and_update(
        patch_target: RecordRef,
        update_target: EntityId,
    ) -> CanonicalCommitEnvelope {
        CanonicalCommitEnvelope::new(
            CommitReference {
                commit_id: CommitId(1),
                version_id: VersionId(1),
                branch_id: BranchId("main".to_string()),
                parents: vec![],
            },
            BranchId("main".to_string()),
            CanonicalCommitAuthorityKind::VersionedTransaction,
            None,
            None,
            vec![],
            vec![],
            SchemaVersionId(1),
            RelationalSchemaRegistry::new().authority_snapshot(),
            MergedCommitPlan {
                transaction_id: TransactionId(1),
                merged_intents: vec![MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                    UpdateEntityFieldsIntent {
                        entity_id: update_target,
                        fields: AspectFieldPatch::default(),
                    },
                ))]
                .into(),
            },
            RelationalPatchRecord {
                ordering: PatchOrdering::CanonicalCommitOrder,
                publication_mode: PatchPublicationMode::CommitNative,
                position: PatchStreamPosition(1),
                records: vec![PatchRecord {
                    target: patch_target,
                    structural_change: RecordStructuralChange::Updated,
                    authoritative_patch:
                        crate::publication::patch::data::PublishedAuthoritativePatch::empty(),
                    contains_opaque_aspect: false,
                    detail: PatchDetail::DenseBitset(vec![1]),
                }]
                .into(),
            },
            RelationalDiagnosticArtifact::new(
                DiagnosticsScope::Replay,
                DiagnosticsArtifactKind::MinimalSummary,
                DeterminismExpectation::Required,
                vec![],
            ),
            LineageFinalizationArtifact::new(
                BranchId("main".to_string()),
                FinalizedLineageEventBatch::new(vec![]),
                LineageDecisionLog::new(vec![]),
            )
            .publish(),
            DerivedIndexArtifacts::default(),
            None,
            None,
            None,
            DescriptorSemanticsVersion(1),
        )
    }

    #[test]
    fn envelope_touched_record_refs_include_patch_and_existing_record_intents() {
        let patch_entity = RecordRef::Entity(EntityId::new(PartitionId::main(), 1, 1));
        let updated_entity = EntityId::new(PartitionId::main(), 2, 1);
        let envelope = envelope_with_patch_and_update(patch_entity.clone(), updated_entity);

        let touched = envelope.touched_record_refs();
        assert!(touched.contains(&patch_entity));
        assert!(touched.contains(&RecordRef::Entity(updated_entity)));
    }

    #[test]
    fn committed_record_changes_for_target_filters_to_matching_record() {
        let entity = EntityId::new(PartitionId::main(), 1, 1);
        let envelope = envelope_with_patch_and_update(RecordRef::Entity(entity), entity);
        let target = RecordRef::Entity(entity);

        let matched = envelope
            .committed_record_changes_for_target(&target)
            .collect::<Vec<_>>();

        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0].commit.commit_id, CommitId(1));
        assert_eq!(matched[0].record.target, target);
    }
}
