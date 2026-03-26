use std::collections::BTreeSet;

use crate::history::data::CommitReference;
use crate::identity::data::{EntityId, KindId, LineageId, PartitionId, VersionId};
use crate::lineage::data::{
    FinalizedLineageEventBatch, LineageDecisionKind, LineageDecisionLog, LineageEventKind,
    LineageFinalizationArtifact, LineageNode,
};
use crate::lineage::logic::authority::LineageAuthority;
use crate::logic::runtime::{PartitionAccess, WorkingState};
use crate::transactions::data::{EntityMutationIntent, MutationIntent, RecordRef};

impl<'runtime> LineageAuthority<'runtime> {
    pub(crate) fn ensure_lineage_for_commit(
        &mut self,
        staged: &mut WorkingState,
        commit: &CommitReference,
        merged_plan: &[MutationIntent],
        changed_records: &[RecordRef],
    ) -> LineageFinalizationArtifact {
        let mut events = Vec::new();
        let mut decisions = Vec::new();
        for record in changed_records {
            let RecordRef::Entity(entity_id) = record else {
                continue;
            };
            let partition = staged.get_partition_mut(entity_id.partition_id);
            let slot = entity_id.local_slot.0 as usize;
            if partition.entity_arena.created_at.get(slot).copied() != Some(commit.version_id) {
                continue;
            }
            let Some(existing_lineage_id) = partition
                .entity_arena
                .extra
                .get(slot)
                .map(|extra| extra.lineage_id)
            else {
                continue;
            };
            let lineage_id = existing_lineage_id.unwrap_or_else(|| {
                let lineage_id = LineageId(self.runtime.lineage.next_lineage_id);
                self.runtime.lineage.next_lineage_id += 1;
                if let Some(extra) = partition.entity_arena.extra.get_mut(slot) {
                    extra.lineage_id = Some(lineage_id);
                }
                if let Some(metadata) = partition
                    .entity_arena
                    .metadata_history_at_mut(slot)
                    .and_then(|history| history.last_mut())
                {
                    metadata.lineage_id = Some(lineage_id);
                }
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
            let event = self.emit_authoritative_lineage_event(
                commit,
                LineageEventKind::Create,
                Vec::new(),
                vec![lineage_id],
            );
            decisions.push(self.accepted_decision_record(
                LineageDecisionKind::CreateAccepted,
                &event,
                None,
            ));
            events.push(event);
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
                        let event = self.emit_authoritative_lineage_event(
                            commit,
                            LineageEventKind::Retire,
                            vec![lineage_id],
                            Vec::new(),
                        );
                        decisions.push(self.accepted_decision_record(
                            LineageDecisionKind::RetireAccepted,
                            &event,
                            None,
                        ));
                        events.push(event);
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
                    let event = self.emit_authoritative_lineage_event(
                        commit,
                        LineageEventKind::Replace,
                        vec![source_lineage_id],
                        vec![replacement_lineage_id],
                    );
                    decisions.push(self.accepted_decision_record(
                        LineageDecisionKind::ReplaceAccepted,
                        &event,
                        None,
                    ));
                    events.push(event);
                }
                MutationIntent::Create(_)
                | MutationIntent::Relation(_)
                | MutationIntent::Entity(EntityMutationIntent::Update(_)) => {}
            }
        }
        let artifact = LineageFinalizationArtifact::new(
            commit.branch_id.clone(),
            FinalizedLineageEventBatch::new(events),
            LineageDecisionLog::new(decisions),
        );
        self.runtime
            .performance_access()
            .count_lineage_finalization(
                artifact.event_batch().events().len(),
                artifact.decision_log().decisions().len(),
            );
        artifact
    }
}

fn find_replace_target_entity(
    staged: &WorkingState,
    changed_records: &[RecordRef],
    source_entity_id: EntityId,
    replacement_partition_id: PartitionId,
    replacement_kind_id: KindId,
    version_id: VersionId,
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
