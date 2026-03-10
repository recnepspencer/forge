use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitReference};
use crate::identity::data::{EntityId, LineageId};
use crate::lineage::data::{
    CorrespondenceCandidate, CorrespondenceResolution, LineageDivergenceSummary, LineageEventKind,
    LineageEventRecord, LineageGraphSnapshot, LineageNode, LineageResolutionStatus,
};
use crate::logic::runtime::{PartitionAccess, RelationalRuntime, WorkingState};
use crate::transactions::data::RecordRef;
use serde_json::json;
use std::collections::BTreeSet;

impl RelationalRuntime {
    pub fn lineage_graph(&self, branch_id: &BranchId) -> LineageGraphSnapshot {
        LineageGraphSnapshot {
            branch_id: branch_id.clone(),
            nodes: self.lineage_nodes.values().cloned().collect(),
            events: self
                .lineage_events
                .iter()
                .filter(|event| &event.branch_id == branch_id)
                .cloned()
                .collect(),
            correspondence_candidates: self
                .correspondence_candidates
                .iter()
                .filter(|candidate| &candidate.branch_id == branch_id)
                .cloned()
                .collect(),
        }
    }

    pub fn lineage_divergence_between_branches(
        &self,
        left_branch: &BranchId,
        right_branch: &BranchId,
    ) -> LineageDivergenceSummary {
        let left_graph = self.lineage_graph(left_branch);
        let right_graph = self.lineage_graph(right_branch);
        let left_event_ids = left_graph
            .events
            .iter()
            .map(|event| event.event_id)
            .collect::<std::collections::BTreeSet<_>>();
        let right_event_ids = right_graph
            .events
            .iter()
            .map(|event| event.event_id)
            .collect::<std::collections::BTreeSet<_>>();
        let shared_lineage_ids = left_graph
            .nodes
            .iter()
            .map(|node| node.lineage_id)
            .collect::<std::collections::BTreeSet<_>>()
            .intersection(
                &right_graph
                    .nodes
                    .iter()
                    .map(|node| node.lineage_id)
                    .collect::<std::collections::BTreeSet<_>>(),
            )
            .copied()
            .collect::<Vec<_>>();
        LineageDivergenceSummary {
            left_branch: left_branch.clone(),
            right_branch: right_branch.clone(),
            left_only_event_ids: left_event_ids
                .difference(&right_event_ids)
                .copied()
                .collect(),
            right_only_event_ids: right_event_ids
                .difference(&left_event_ids)
                .copied()
                .collect(),
            shared_lineage_ids,
        }
    }

    pub fn lineage_for_record(&self, entity_id: EntityId) -> Option<&LineageNode> {
        let slot = entity_id.local_slot.0 as usize;
        let lineage_id = self
            .partitions
            .get(&entity_id.partition_id)?
            .entity_arena
            .lineage_ids
            .get(slot)
            .copied()
            .flatten()?;
        self.lineage_nodes.get(&lineage_id)
    }

    pub fn record_correspondence_candidate(
        &mut self,
        branch_id: BranchId,
        sources: Vec<LineageId>,
        targets: Vec<LineageId>,
        note: impl Into<String>,
    ) -> CorrespondenceCandidate {
        let candidate = CorrespondenceCandidate {
            candidate_id: self.next_lineage_event_id,
            branch_id: branch_id.clone(),
            sources,
            targets,
            note: note.into(),
        };
        self.next_lineage_event_id += 1;
        self.correspondence_candidates.push(candidate.clone());
        self.push_bounded_diagnostic(
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
            .correspondence_candidates
            .iter()
            .find(|candidate| candidate.candidate_id == candidate_id)?
            .clone();
        if candidate
            .sources
            .iter()
            .chain(candidate.targets.iter())
            .any(|lineage_id| !self.lineage_nodes.contains_key(lineage_id))
        {
            self.push_bounded_diagnostic(
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
        let event_id = self.next_lineage_event_id;
        self.next_lineage_event_id += 1;
        let event = LineageEventRecord {
            event_id,
            commit: commit.clone(),
            branch_id: candidate.branch_id.clone(),
            kind: LineageEventKind::Correspond,
            sources: candidate.sources.clone(),
            targets: candidate.targets.clone(),
        };
        self.lineage_events.push(event);
        self.attach_lineage_events_to_commit(commit.commit_id, &[event_id]);
        let resolution = CorrespondenceResolution {
            candidate_id,
            status: LineageResolutionStatus::Promoted,
            promoted_event_id: Some(event_id),
        };
        self.push_bounded_diagnostic(
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
        merged_plan: &[crate::transactions::data::TransactionIntent],
        changed_records: &[RecordRef],
    ) -> Vec<u64> {
        let mut event_ids = Vec::new();
        for record in changed_records {
            let RecordRef::Entity(entity_id) = record else {
                continue;
            };
            let slot = entity_id.local_slot.0 as usize;
            let partition = staged.get_partition_mut(entity_id.partition_id);
            if partition.entity_arena.created_at[slot] != commit.version_id {
                continue;
            }
            let lineage_id = partition.entity_arena.lineage_ids[slot].unwrap_or_else(|| {
                let lineage_id = LineageId(self.next_lineage_id);
                self.next_lineage_id += 1;
                partition.entity_arena.lineage_ids[slot] = Some(lineage_id);
                lineage_id
            });
            self.lineage_nodes.entry(lineage_id).or_insert(LineageNode {
                lineage_id,
                entity_id: *entity_id,
            });
            let event_id = self.next_lineage_event_id;
            self.next_lineage_event_id += 1;
            self.lineage_events.push(LineageEventRecord {
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
                crate::transactions::data::TransactionIntent::DeleteEntity { entity_id } => {
                    if let Some(lineage_id) =
                        staged
                            .get_partition(entity_id.partition_id)
                            .and_then(|partition| {
                                partition
                                    .entity_arena
                                    .lineage_ids
                                    .get(entity_id.local_slot.0 as usize)
                                    .copied()
                                    .flatten()
                            })
                    {
                        let event_id = self.next_lineage_event_id;
                        self.next_lineage_event_id += 1;
                        self.lineage_events.push(LineageEventRecord {
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
                crate::transactions::data::TransactionIntent::ReplaceEntity {
                    entity_id,
                    replacement,
                } => {
                    let source_lineage_id =
                        staged
                            .get_partition(entity_id.partition_id)
                            .and_then(|partition| {
                                partition
                                    .entity_arena
                                    .lineage_ids
                                    .get(entity_id.local_slot.0 as usize)
                                    .copied()
                                    .flatten()
                            });
                    let replacement_entity_id = find_replace_target_entity(
                        staged,
                        changed_records,
                        *entity_id,
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
                        .and_then(|partition| {
                            partition
                                .entity_arena
                                .lineage_ids
                                .get(replacement_entity_id.local_slot.0 as usize)
                                .copied()
                                .flatten()
                        });
                    let Some(replacement_lineage_id) = replacement_lineage_id else {
                        continue;
                    };
                    let event_id = self.next_lineage_event_id;
                    self.next_lineage_event_id += 1;
                    self.lineage_events.push(LineageEventRecord {
                        event_id,
                        commit: commit.clone(),
                        branch_id: commit.branch_id.clone(),
                        kind: LineageEventKind::Replace,
                        sources: vec![source_lineage_id],
                        targets: vec![replacement_lineage_id],
                    });
                    event_ids.push(event_id);
                }
                _ => {}
            }
        }
        event_ids
    }

    fn attach_lineage_events_to_commit(
        &mut self,
        commit_id: crate::history::data::CommitId,
        event_ids: &[u64],
    ) {
        if let Some(envelope) = self.commit_envelopes.get_mut(&commit_id) {
            envelope.lineage_event_ids.extend(event_ids.iter().copied());
        }
        if let Some(log_entry) = self
            .durable_log
            .iter_mut()
            .find(|entry| entry.envelope.commit.commit_id == commit_id)
        {
            log_entry
                .envelope
                .lineage_event_ids
                .extend(event_ids.iter().copied());
        }
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
        let matching_kind =
            partition.entity_arena.kind_ids.get(slot) == Some(&Some(replacement_kind_id));
        if created_now && matching_partition && matching_kind {
            consumed_replace_targets.insert(*candidate);
            return Some(*candidate);
        }
    }
    None
}
