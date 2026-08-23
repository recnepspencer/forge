use crate::transactions::data::{CreatedEntityRef, EntityReference};

use super::super::super::context::InvariantExecutionContext;
use super::super::common::contract_candidate_kind_matches;

pub(super) fn visible_entities_of_kinds(
    context: &InvariantExecutionContext<'_>,
    kind_ids: &[crate::identity::data::KindId],
) -> Vec<EntityReference> {
    let mut entities = Vec::new();
    collect_state_view_entities(context, kind_ids, &mut entities);
    collect_planned_entities(context, kind_ids, &mut entities);
    entities
}

fn collect_state_view_entities(
    context: &InvariantExecutionContext<'_>,
    kind_ids: &[crate::identity::data::KindId],
    entities: &mut Vec<EntityReference>,
) {
    let state_view = context.state_view();
    for partition_id in state_view.state().partition_ids() {
        let Some(partition) = state_view.state().get_partition(partition_id) else {
            continue;
        };
        if state_view.version_id() == context.current_version_id() {
            collect_current_version_entities(context, kind_ids, partition_id, partition, entities);
        } else {
            collect_historical_entities(context, kind_ids, partition_id, partition, entities);
        }
    }
}

fn collect_current_version_entities(
    context: &InvariantExecutionContext<'_>,
    kind_ids: &[crate::identity::data::KindId],
    partition_id: crate::identity::data::PartitionId,
    partition: &crate::storage::overlay::PartitionState,
    entities: &mut Vec<EntityReference>,
) {
    for slot in partition.entity_arena.live_bitset.iter_set_slots() {
        context.metrics().count_entity_slot_scans(1);
        let Some(slot_view) = partition.entity_arena.get_slot(slot) else {
            continue;
        };
        let Some(kind_id) = slot_view.kind_id() else {
            continue;
        };
        if contract_candidate_kind_matches(kind_id, kind_ids) {
            entities.push(EntityReference::Existing(
                crate::identity::data::EntityId::new(
                    partition_id,
                    slot as u64,
                    slot_view.generation(),
                ),
            ));
        }
    }
}

fn collect_historical_entities(
    context: &InvariantExecutionContext<'_>,
    kind_ids: &[crate::identity::data::KindId],
    partition_id: crate::identity::data::PartitionId,
    partition: &crate::storage::overlay::PartitionState,
    entities: &mut Vec<EntityReference>,
) {
    let state_view = context.state_view();
    for slot in partition.entity_arena.occupied_slots() {
        let Some(metadata) =
            state_view.entity_metadata_at(&partition.entity_arena, partition_id, slot)
        else {
            continue;
        };
        context.metrics().count_entity_slot_scans(1);
        if contract_candidate_kind_matches(metadata.kind_id, kind_ids) {
            entities.push(EntityReference::Existing(metadata.entity_id));
        }
    }
}

fn collect_planned_entities(
    context: &InvariantExecutionContext<'_>,
    kind_ids: &[crate::identity::data::KindId],
    entities: &mut Vec<EntityReference>,
) {
    let Some(merged_plan) = context.merged_plan() else {
        return;
    };
    for intent in &merged_plan.merged_intents {
        match intent {
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::Entity(spec),
            ) => {
                if contract_candidate_kind_matches(spec.kind_id, kind_ids) {
                    entities.push(EntityReference::Created(CreatedEntityRef {
                        partition_id: spec.partition_id,
                        kind_id: spec.kind_id,
                        client_key: spec.client_key.clone(),
                    }));
                }
            }
            crate::transactions::data::MutationIntent::Create(
                crate::transactions::data::CreateIntent::BulkEntities(spec),
            ) => {
                if contract_candidate_kind_matches(spec.kind_id, kind_ids) {
                    entities.extend(spec.client_keys.iter().cloned().map(|client_key| {
                        EntityReference::Created(CreatedEntityRef {
                            partition_id: spec.partition_id,
                            kind_id: spec.kind_id,
                            client_key,
                        })
                    }));
                }
            }
            _ => {}
        }
    }
}
