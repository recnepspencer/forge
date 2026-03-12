use std::collections::BTreeSet;

use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::{CommitId, CommitReference};
use crate::identity::data::{EntityId, LineageId};
use crate::lineage::data::{
    CorrespondenceCandidate, CorrespondenceResolution, LineageEventKind, LineageEventRecord,
    LineageNode, LineageResolutionStatus,
};
use crate::logic::runtime::{PartitionAccess, RelationalRuntime, WorkingState};
use crate::transactions::data::{EntityMutationIntent, MutationIntent, RecordRef};

pub struct LineageAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub fn lineage_authority(&mut self) -> LineageAuthority<'_> {
        LineageAuthority::new(self)
    }
}

impl<'runtime> LineageAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn record_correspondence_candidate(
        &mut self,
        branch_id: crate::history::data::BranchId,
        sources: Vec<LineageId>,
        targets: Vec<LineageId>,
        note: impl Into<String>,
    ) -> CorrespondenceCandidate {
        let candidate = CorrespondenceCandidate {
            candidate_id: self.runtime.lineage.next_event_id,
            branch_id: branch_id.clone(),
            sources,
            targets,
            note: note.into(),
        };
        self.runtime.lineage.next_event_id += 1;
        self.runtime
            .lineage
            .correspondence_candidates
            .push(candidate.clone());
        self.runtime.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::Lineage,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "correspondence candidate recorded".to_string(),
                fields: json!({
                    "candidate_id": candidate.candidate_id,
                    "branch_id": branch_id.0,
                }),
            }],
        );
        candidate
    }

    pub fn promote_correspondence(
        &mut self,
        candidate_id: u64,
        commit: CommitReference,
    ) -> Option<CorrespondenceResolution> {
        let candidate = self
            .runtime
            .lineage
            .correspondence_candidates
            .iter()
            .find(|candidate| candidate.candidate_id == candidate_id)?
            .clone();
        if candidate
            .sources
            .iter()
            .chain(candidate.targets.iter())
            .any(|lineage_id| !self.runtime.lineage.nodes.contains_key(lineage_id))
        {
            self.runtime.publication_authority().push_bounded_diagnostic(
                DiagnosticsScope::Lineage,
                DiagnosticsArtifactKind::Failure,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::InvariantViolation,
                    message: "correspondence promotion referenced missing lineage".to_string(),
                    fields: json!({ "candidate_id": candidate_id }),
                }],
            );
            return None;
        }
        let event_id = self.runtime.lineage.next_event_id;
        self.runtime.lineage.next_event_id += 1;
        let event = LineageEventRecord {
            event_id,
            commit: commit.clone(),
            branch_id: candidate.branch_id.clone(),
            kind: LineageEventKind::Correspond,
            sources: candidate.sources.clone(),
            targets: candidate.targets.clone(),
        };
        self.runtime.lineage.events.push(event);
        self.attach_events_to_commit(commit.commit_id, &[event_id]);
        let resolution = CorrespondenceResolution {
            candidate_id,
            status: LineageResolutionStatus::Promoted,
            promoted_event_id: Some(event_id),
        };
        self.runtime.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::Lineage,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "correspondence promoted into lineage".to_string(),
                fields: json!({
                    "candidate_id": candidate_id,
                    "event_id": event_id,
                    "commit_id": commit.commit_id.0,
                }),
            }],
        );
        Some(resolution)
    }

    pub(crate) fn ensure_lineage_for_commit(
        &mut self,
        staged: &mut WorkingState,
        commit: &CommitReference,
        merged_plan: &[MutationIntent],
        changed_records: &[RecordRef],
    ) -> Vec<u64> {
        let mut event_ids = Vec::new();
        for record in changed_records {
            let RecordRef::Entity(entity_id) = record else {
                continue;
            };
            let partition = staged.get_partition_mut(entity_id.partition_id);
            let slot = entity_id.local_slot.0 as usize;
            if partition.entity_arena.created_at.get(slot).copied() != Some(commit.version_id) {
                continue;
            }
            let lineage_id = partition.entity_arena.extra[slot].lineage_id.unwrap_or_else(|| {
                let lineage_id = LineageId(self.runtime.lineage.next_lineage_id);
                self.runtime.lineage.next_lineage_id += 1;
                partition.entity_arena.extra[slot].lineage_id = Some(lineage_id);
                lineage_id
            });
            self.runtime
                .lineage
                .nodes
                .entry(lineage_id)
                .or_insert(LineageNode {
                    lineage_id,
                    entity_id: *entity_id,
                });
            let event_id = self.runtime.lineage.next_event_id;
            self.runtime.lineage.next_event_id += 1;
            self.runtime.lineage.events.push(LineageEventRecord {
                event_id,
                commit: commit.clone(),
                branch_id: commit.branch_id.clone(),
                kind: LineageEventKind::Create,
                sources: Vec::new(),
                targets: vec![lineage_id],
            });
            event_ids.push(event_id);
        }
        let mut consumed_replace_targets = BTreeSet::new();
        for intent in merged_plan {
            match intent {
                MutationIntent::Entity(EntityMutationIntent::Delete(spec)) => {
                    let entity_id = spec.entity_id;
                    if let Some(lineage_id) = staged
                        .get_partition(entity_id.partition_id)
                        .and_then(|partition| partition.entity_arena.get(&entity_id))
                        .and_then(|slot_view| slot_view.extra().lineage_id)
                    {
                        let event_id = self.runtime.lineage.next_event_id;
                        self.runtime.lineage.next_event_id += 1;
                        self.runtime.lineage.events.push(LineageEventRecord {
                            event_id,
                            commit: commit.clone(),
                            branch_id: commit.branch_id.clone(),
                            kind: LineageEventKind::Retire,
                            sources: vec![lineage_id],
                            targets: Vec::new(),
                        });
                        event_ids.push(event_id);
                    }
                }
                MutationIntent::Entity(EntityMutationIntent::Replace(spec)) => {
                    let entity_id = spec.entity_id;
                    let replacement = &spec.replacement;
                    let source_lineage_id = staged
                        .get_partition(entity_id.partition_id)
                        .and_then(|partition| partition.entity_arena.get(&entity_id))
                        .and_then(|slot_view| slot_view.extra().lineage_id);
                    let replacement_entity_id = find_replace_target_entity(
                        staged,
                        changed_records,
                        entity_id,
                        replacement.partition_id,
                        replacement.kind_id,
                        commit.version_id,
                        &mut consumed_replace_targets,
                    );
                    let Some(source_lineage_id) = source_lineage_id else {
                        continue;
                    };
                    let Some(replacement_entity_id) = replacement_entity_id else {
                        continue;
                    };
                    let replacement_lineage_id = staged
                        .get_partition(replacement_entity_id.partition_id)
                        .and_then(|partition| partition.entity_arena.get(&replacement_entity_id))
                        .and_then(|slot_view| slot_view.extra().lineage_id);
                    let Some(replacement_lineage_id) = replacement_lineage_id else {
                        continue;
                    };
                    let event_id = self.runtime.lineage.next_event_id;
                    self.runtime.lineage.next_event_id += 1;
                    self.runtime.lineage.events.push(LineageEventRecord {
                        event_id,
                        commit: commit.clone(),
                        branch_id: commit.branch_id.clone(),
                        kind: LineageEventKind::Replace,
                        sources: vec![source_lineage_id],
                        targets: vec![replacement_lineage_id],
                    });
                    event_ids.push(event_id);
                }
                MutationIntent::Create(_)
                | MutationIntent::Relation(_)
                | MutationIntent::Entity(EntityMutationIntent::Update(_)) => {}
            }
        }
        event_ids
    }

    fn attach_events_to_commit(&mut self, commit_id: CommitId, event_ids: &[u64]) {
        if let Some(envelope) = self.runtime.history.commit_envelopes.get_mut(&commit_id) {
            envelope.lineage_event_ids.extend(event_ids.iter().copied());
        }
        if let Some(log_entry) = self
            .runtime
            .durability
            .log
            .iter_mut()
            .find(|entry| entry.commit.commit_id == commit_id)
        {
            log_entry.lineage_event_ids.extend(event_ids.iter().copied());
        }
        self.runtime.publication_authority().push_bounded_diagnostic(
            DiagnosticsScope::Lineage,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "lineage events attached to commit".to_string(),
                fields: json!({
                    "commit_id": commit_id.0,
                    "event_count": event_ids.len(),
                }),
            }],
        );
    }
}

fn find_replace_target_entity(
    staged: &WorkingState,
    changed_records: &[RecordRef],
    source_entity_id: EntityId,
    replacement_partition_id: crate::identity::data::PartitionId,
    replacement_kind_id: crate::identity::data::KindId,
    version_id: crate::identity::data::VersionId,
    consumed_replace_targets: &mut BTreeSet<EntityId>,
) -> Option<EntityId> {
    let source_index = changed_records.iter().position(
        |record| matches!(record, RecordRef::Entity(candidate) if *candidate == source_entity_id),
    )?;
    for record in changed_records.iter().skip(source_index + 1) {
        let RecordRef::Entity(candidate) = record else {
            continue;
        };
        if consumed_replace_targets.contains(candidate) {
            continue;
        }
        let partition = staged.get_partition(candidate.partition_id)?;
        let slot = candidate.local_slot.0 as usize;
        let created_now = partition.entity_arena.created_at.get(slot) == Some(&version_id);
        let matching_partition = candidate.partition_id == replacement_partition_id;
        let matching_kind = partition
            .entity_arena
            .kind_ids
            .get(slot)
            .copied()
            .flatten()
            .is_some_and(|kind_id| kind_id == replacement_kind_id);
        if created_now && matching_partition && matching_kind {
            consumed_replace_targets.insert(*candidate);
            return Some(*candidate);
        }
    }
    None
}
