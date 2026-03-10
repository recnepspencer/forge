use serde_json::json;

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry, RelationalDiagnosticsFacade,
};
use crate::logic::runtime::{RelationalReplayRecord, RelationalRuntime, ReplaySchemaVersion};
use crate::publication::data::{PublicationBundle, PublicationStatus};
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use std::collections::BTreeMap;

use crate::identity::data::{PartitionId, RelationId};
use crate::storage::logic::state::{
    DenseSlotBitSet, PartitionAccess, PublicationArtifacts, SnapshotPartitionPins, SnapshotState,
};

impl RelationalRuntime {
    pub fn diagnostics(&self) -> RelationalDiagnosticsFacade {
        RelationalDiagnosticsFacade {
            artifacts: self.publication.diagnostics.clone(),
        }
    }

    pub fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>> {
        self.publication.latest_bundle.as_ref()
    }

    pub fn latest_patch(&self) -> Option<&crate::publication::data::diff::RelationalPatchRecord> {
        self.publication
            .latest_bundle
            .as_ref()
            .map(|bundle| &bundle.patch)
    }

    pub fn latest_replay(&self) -> Option<&RelationalReplayRecord> {
        self.publication
            .latest_bundle
            .as_ref()
            .map(|bundle| &bundle.replay)
    }

    pub(crate) fn push_diagnostic_artifact(&mut self, artifact: RelationalDiagnosticArtifact) {
        self.publication.diagnostics.push(artifact);
    }

    pub(crate) fn push_bounded_diagnostic(
        &mut self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
        entries: Vec<RelationalDiagnosticsEntry>,
    ) -> RelationalDiagnosticArtifact {
        let max_entries = self.config.diagnostics.max_entries_per_artifact;
        let artifact = RelationalDiagnosticArtifact {
            scope,
            kind,
            determinism: DeterminismExpectation::Required,
            entries: entries.into_iter().take(max_entries).collect(),
        };
        self.push_diagnostic_artifact(artifact.clone());
        artifact
    }

    pub(crate) fn assemble_publication_bundle(
        &mut self,
        staged: &impl PartitionAccess,
        commit_reference: crate::history::data::CommitReference,
        version_id: crate::identity::data::VersionId,
        patch: crate::publication::data::diff::RelationalPatchRecord,
        diagnostics_summary: RelationalDiagnosticArtifact,
    ) -> PublicationArtifacts {
        let snapshot_id = SnapshotId(self.snapshots.next_snapshot_id);
        self.snapshots.next_snapshot_id += 1;
        let snapshot = SnapshotHandle {
            snapshot_id,
            version_id,
            read_policy: SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        };
        let replay = RelationalReplayRecord {
            schema_version: ReplaySchemaVersion(1),
            commit_id: commit_reference.commit_id,
            version_id,
            snapshot_id,
            patch: patch.clone(),
            schema_registry: self.config.schema_registry.clone(),
        };
        let bundle = PublicationBundle {
            commit: commit_reference,
            snapshot: snapshot.clone(),
            diagnostics_summary: diagnostics_summary.clone(),
            patch: patch.clone(),
            replay: replay.clone(),
            status: PublicationStatus::Published,
        };
        let entities = self.visible_entities_from_state(staged, version_id);
        let relations = self.visible_relations_from_state(staged, version_id);
        let mut pinned_partitions: BTreeMap<PartitionId, SnapshotPartitionPins> = BTreeMap::new();
        for entity_id in entities.iter().map(|record| record.entity_id) {
            let partition_pins = pinned_partitions
                .entry(entity_id.partition_id)
                .or_insert_with(|| SnapshotPartitionPins {
                    entity_slots: DenseSlotBitSet::with_capacity(
                        entity_id.local_slot.0 as usize + 1,
                    ),
                    relation_slots: DenseSlotBitSet::with_capacity(0),
                });
            partition_pins
                .entity_slots
                .set(entity_id.local_slot.0 as usize, true);
        }
        for relation_id in relations.iter().map(|record| record.relation_id) {
            insert_snapshot_relation_pin(&mut pinned_partitions, relation_id);
        }
        let snapshot_state = SnapshotState {
            handle: snapshot.clone(),
            pinned_entity_count: entities.len(),
            pinned_relation_count: relations.len(),
            pinned_partitions,
        };
        PublicationArtifacts {
            snapshot,
            snapshot_state,
            diagnostics_summary,
            bundle,
        }
    }
}

fn insert_snapshot_relation_pin(
    pinned_partitions: &mut BTreeMap<PartitionId, SnapshotPartitionPins>,
    relation_id: RelationId,
) {
    let partition_pins = pinned_partitions
        .entry(relation_id.partition_id)
        .or_insert_with(|| SnapshotPartitionPins {
            entity_slots: DenseSlotBitSet::with_capacity(0),
            relation_slots: DenseSlotBitSet::with_capacity(relation_id.local_slot.0 as usize + 1),
        });
    partition_pins
        .relation_slots
        .set(relation_id.local_slot.0 as usize, true);
}

pub(crate) fn publication_failure_diagnostic(detail: String) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code: crate::diagnostics::data::DiagnosticCode::InvariantViolation,
        message: detail,
        fields: json!({ "execution_point": "snapshot_publication" }),
    }
}
