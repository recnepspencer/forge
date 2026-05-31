use std::collections::BTreeSet;

use super::super::query_packetization::PacketizedQueryWork;
use super::super::query_traversal::{
    aspect_filter_matches_entity, aspect_filter_matches_relation, traversal_fragment, TraversalMode,
};
use super::super::*;
use super::field_query_matching::{entity_field_matches, relation_field_matches};
use super::fragment_builders::{entity_fragment, relation_fragment};
use crate::storage::data::{
    entity_authoritative_aspect_field_comparison_key,
    relation_authoritative_aspect_field_comparison_key,
};

pub(crate) fn execute_query_fragment(
    runtime: &RelationalRuntime,
    read_view: &RelationalReadView,
    packet: &PlannedQueryPacket,
    work: &PacketizedQueryWork,
    ordinal: u64,
    scratch: &mut QueryFragmentScratch,
) -> Option<crate::query::data::QueryWorkerFragment> {
    match work {
        PacketizedQueryWork::ExplicitTargets(targets) => read_view.execute_planned_packet_fragment(
            packet.plan_key,
            packet.ordering,
            targets,
            ordinal,
        ),
        PacketizedQueryWork::EntityKindScan {
            partition_id,
            kind_id,
        } => Some(entity_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.entity_id.partition_id == *partition_id && record.kind.kind_id == *kind_id
            },
        )),
        PacketizedQueryWork::RelationKindScan {
            partition_id,
            kind_id,
        } => Some(relation_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.relation_id.partition_id == *partition_id && record.kind.kind_id == *kind_id
            },
        )),
        PacketizedQueryWork::EntityFieldEquals {
            partition_id,
            field_locator,
            value,
        } => Some(entity_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.entity_id.partition_id == *partition_id
                    && entity_field_matches(record, field_locator, value)
            },
        )),
        PacketizedQueryWork::EntityFieldAnyOf {
            partition_id,
            field_locator,
            values,
        } => {
            let values = values.iter().cloned().collect::<BTreeSet<_>>();
            Some(entity_fragment(
                read_view,
                packet,
                ordinal,
                values.len(),
                1,
                scratch,
                |record| {
                    record.entity_id.partition_id == *partition_id
                        && entity_authoritative_aspect_field_comparison_key(record, field_locator)
                            .is_some_and(|value| values.contains(&value))
                },
            ))
        }
        PacketizedQueryWork::RelationFieldEquals {
            partition_id,
            field_locator,
            value,
        } => Some(relation_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.relation_id.partition_id == *partition_id
                    && relation_field_matches(record, field_locator, value)
            },
        )),
        PacketizedQueryWork::RelationFieldAnyOf {
            partition_id,
            field_locator,
            values,
        } => {
            let values = values.iter().cloned().collect::<BTreeSet<_>>();
            Some(relation_fragment(
                read_view,
                packet,
                ordinal,
                values.len(),
                1,
                scratch,
                |record| {
                    record.relation_id.partition_id == *partition_id
                        && relation_authoritative_aspect_field_comparison_key(record, field_locator)
                            .is_some_and(|value| values.contains(&value))
                },
            ))
        }
        PacketizedQueryWork::AspectFilteredEntities {
            partition_id,
            kind_id,
            aspect_filter,
        } => Some(entity_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.entity_id.partition_id == *partition_id
                    && kind_id.is_none_or(|kind_id| record.kind.kind_id == kind_id)
                    && aspect_filter_matches_entity(record, aspect_filter)
            },
        )),
        PacketizedQueryWork::AspectFilteredRelations {
            partition_id,
            kind_id,
            aspect_filter,
        } => Some(relation_fragment(
            read_view,
            packet,
            ordinal,
            0,
            1,
            scratch,
            |record| {
                record.relation_id.partition_id == *partition_id
                    && kind_id.is_none_or(|kind_id| record.kind.kind_id == kind_id)
                    && aspect_filter_matches_relation(record, aspect_filter)
            },
        )),
        PacketizedQueryWork::OutgoingNeighborhood {
            seeds,
            relation_kind_scope,
        } => traversal_fragment(
            runtime,
            &runtime.storage_access().current_state(),
            read_view.snapshot.version_id,
            packet,
            seeds,
            relation_kind_scope.as_deref(),
            ordinal,
            TraversalMode::OutgoingNeighborhood,
            scratch,
        ),
        PacketizedQueryWork::IncomingNeighborhood {
            seeds,
            relation_kind_scope,
        } => traversal_fragment(
            runtime,
            &runtime.storage_access().current_state(),
            read_view.snapshot.version_id,
            packet,
            seeds,
            relation_kind_scope.as_deref(),
            ordinal,
            TraversalMode::IncomingNeighborhood,
            scratch,
        ),
        PacketizedQueryWork::ConnectivityTraversal {
            seeds,
            relation_kind_scope,
            max_depth,
        } => traversal_fragment(
            runtime,
            &runtime.storage_access().current_state(),
            read_view.snapshot.version_id,
            packet,
            seeds,
            relation_kind_scope.as_deref(),
            ordinal,
            TraversalMode::ConnectivityTraversal {
                max_depth: *max_depth,
            },
            scratch,
        ),
    }
}

pub(crate) fn execute_explicit_query_fragment_from_state(
    read_context: &VisibilityReadContext<'_>,
    snapshot_state: &crate::storage::logic::state::SnapshotState,
    current_state: &(impl PartitionAccess + Sync),
    version_id: crate::identity::data::VersionId,
    packet: &PlannedQueryPacket,
    work: &PacketizedQueryWork,
    ordinal: u64,
) -> Option<crate::query::data::QueryWorkerFragment> {
    let PacketizedQueryWork::ExplicitTargets(targets) = work else {
        return None;
    };

    let mut entities = Vec::new();
    let mut relations = Vec::new();
    let mut touched_partitions = std::collections::BTreeSet::new();

    for target in targets {
        match target {
            crate::transactions::data::RecordRef::Entity(entity_id) => {
                touched_partitions.insert(entity_id.partition_id);
                let Some(pins) = snapshot_state
                    .pinned_partitions
                    .get(&entity_id.partition_id)
                else {
                    continue;
                };
                if pins
                    .entity_slots
                    .count_ones_in_range(entity_id.slot_index(), entity_id.slot_index() + 1)
                    == 0
                {
                    continue;
                }
                if let Some(record) = read_context.unmasked_entity_record_for_id_at_version(
                    current_state,
                    *entity_id,
                    version_id,
                ) {
                    entities.push(record);
                }
            }
            crate::transactions::data::RecordRef::Relation(relation_id) => {
                touched_partitions.insert(relation_id.partition_id);
                let Some(pins) = snapshot_state
                    .pinned_partitions
                    .get(&relation_id.partition_id)
                else {
                    continue;
                };
                if pins
                    .relation_slots
                    .count_ones_in_range(relation_id.slot_index(), relation_id.slot_index() + 1)
                    == 0
                {
                    continue;
                }
                if let Some(record) = read_context.unmasked_relation_record_for_id_at_version(
                    current_state,
                    *relation_id,
                    version_id,
                ) {
                    relations.push(if pins.retained_relation_slots.count_ones_in_range(
                        relation_id.slot_index(),
                        relation_id.slot_index() + 1,
                    ) == 1
                    {
                        crate::storage::data::RelationReadRecord {
                            lifecycle:
                                crate::storage::data::RecordLifecycleState::RetainedDanglingForAudit,
                            ..record
                        }
                    } else {
                        record
                    });
                }
            }
        }
    }

    let unmasked_entity_records_emitted = entities.len();
    let unmasked_relation_records_emitted = relations.len();
    Some(crate::query::data::QueryWorkerFragment {
        plan_key: packet.plan_key,
        fragment_key: crate::query::data::deterministic_query_fragment_key(
            packet.plan_key,
            ordinal,
        ),
        ordering: packet.ordering,
        counters: crate::query::data::QueryFragmentCounters {
            target_count: targets.len(),
            unmasked_entity_records_emitted,
            unmasked_relation_records_emitted,
            touched_partitions: touched_partitions.len(),
        },
        entities,
        relations,
        traversal_basis: None,
    })
}

pub(crate) fn execute_traversal_query_fragment_from_state(
    runtime: &RelationalRuntime,
    state: &(impl PartitionAccess + Sync),
    version_id: crate::identity::data::VersionId,
    packet: &PlannedQueryPacket,
    work: &PacketizedQueryWork,
    ordinal: u64,
    scratch: &mut QueryFragmentScratch,
) -> Option<crate::query::data::QueryWorkerFragment> {
    match work {
        PacketizedQueryWork::OutgoingNeighborhood {
            seeds,
            relation_kind_scope,
        } => traversal_fragment(
            runtime,
            state,
            version_id,
            packet,
            seeds,
            relation_kind_scope.as_deref(),
            ordinal,
            TraversalMode::OutgoingNeighborhood,
            scratch,
        ),
        PacketizedQueryWork::IncomingNeighborhood {
            seeds,
            relation_kind_scope,
        } => traversal_fragment(
            runtime,
            state,
            version_id,
            packet,
            seeds,
            relation_kind_scope.as_deref(),
            ordinal,
            TraversalMode::IncomingNeighborhood,
            scratch,
        ),
        PacketizedQueryWork::ConnectivityTraversal {
            seeds,
            relation_kind_scope,
            max_depth,
        } => traversal_fragment(
            runtime,
            state,
            version_id,
            packet,
            seeds,
            relation_kind_scope.as_deref(),
            ordinal,
            TraversalMode::ConnectivityTraversal {
                max_depth: *max_depth,
            },
            scratch,
        ),
        _ => None,
    }
}
