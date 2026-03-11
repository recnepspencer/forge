use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitReference};
use crate::identity::data::{EntityId, LineageId};
use crate::lineage::data::{
    LineageDivergenceSummary, LineageEventKind, LineageEventRecord, LineageGraphSnapshot,
    LineageNode,
};
use crate::logic::runtime::{PartitionAccess, RelationalDraft, RelationalRuntime};
use crate::transactions::data::RecordRef;
use crate::transactions::data::{EntityMutationIntent, MutationIntent};
use serde_json::json;
use std::collections::BTreeSet;
use crate::storage::substrate::RecordId;

impl RelationalRuntime {
    pub fn lineage_graph(&self, branch_id: &BranchId) -> LineageGraphSnapshot {
        LineageGraphSnapshot {
            branch_id: branch_id.clone(),
            nodes: self.lineage.nodes.values().cloned().collect(),
            events: self
                .lineage
                .events
                .iter()
                .filter(|event| &event.branch_id == branch_id)
                .cloned()
                .collect(),
            correspondence_candidates: self
                .lineage
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
        let lineage_id = self
            .partitions
            .get(&entity_id.partition_id)?
            .entity_arena
            .get(&entity_id)
            .and_then(|slot_view| slot_view.extra().lineage_id)?;
        self.lineage.nodes.get(&lineage_id)
    }

    pub(crate) fn ensure_lineage_for_commit(
        &mut self,
        staged: &mut RelationalDraft,
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
            let slot = entity_id.local_slot();
            if partition.entity_arena.created_at.get(slot).copied() != Some(commit.version_id) {
                continue;
            }
            let lineage_id = partition.entity_arena.extra[slot].lineage_id.unwrap_or_else(|| {
                let lineage_id = LineageId(self.lineage.next_lineage_id);
                self.lineage.next_lineage_id += 1;
                partition.entity_arena.extra[slot].lineage_id = Some(lineage_id);
                lineage_id
            });
            self.lineage.nodes.entry(lineage_id).or_insert(LineageNode {
                lineage_id,
                entity_id: *entity_id,
            });
            let event_id = self.lineage.next_event_id;
            self.lineage.next_event_id += 1;
            self.lineage.events.push(LineageEventRecord {
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
                        let event_id = self.lineage.next_event_id;
                        self.lineage.next_event_id += 1;
                        self.lineage.events.push(LineageEventRecord {
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
                    let event_id = self.lineage.next_event_id;
                    self.lineage.next_event_id += 1;
                    self.lineage.events.push(LineageEventRecord {
                        event_id,
                        commit: commit.clone(),
                        branch_id: commit.branch_id.clone(),
                        kind: LineageEventKind::Replace,
                        sources: vec![source_lineage_id],
                        targets: vec![replacement_lineage_id],
                    });
                    event_ids.push(event_id);
                }
                MutationIntent::Create(_) | MutationIntent::Relation(_) | MutationIntent::Entity(EntityMutationIntent::Update(_)) => {}
            }
        }
        event_ids
    }

    pub(crate) fn attach_lineage_events_to_commit(
        &mut self,
        commit_id: crate::history::data::CommitId,
        event_ids: &[u64],
    ) {
        if let Some(envelope) = self.history.commit_envelopes.get_mut(&commit_id) {
            envelope.lineage_event_ids.extend(event_ids.iter().copied());
        }
        if let Some(log_entry) = self
            .durability
            .log
            .iter_mut()
            .find(|entry| entry.commit.commit_id == commit_id)
        {
            log_entry.lineage_event_ids.extend(event_ids.iter().copied());
        }
        self.push_bounded_diagnostic(
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
    staged: &RelationalDraft,
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
        let slot = candidate.local_slot();
        let created_now = partition.entity_arena.created_at.get(slot) == Some(&version_id);
        let matching_partition = candidate.partition_id == replacement_partition_id;
        let matching_kind = partition
            .entity_arena
            .get(candidate)
            .and_then(|slot_view| slot_view.kind_id())
            == Some(replacement_kind_id);
        if created_now && matching_partition && matching_kind {
            consumed_replace_targets.insert(*candidate);
            return Some(*candidate);
        }
    }
    None
}
