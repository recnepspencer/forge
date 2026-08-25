use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
use crate::runtime::builder::RelationalRuntimeBuilder;
use crate::storage::overlay::PartitionState;
use crate::storage::substrate::{
    EntityArena, EntityExtra, RelationArena, RelationEndpoints, RelationExtra, SlotInit,
};
use crate::transactions::data::RecordRef;

#[test]
fn initial_branch_head_bulk_pin_advancement_sets_branch_pins_for_changed_records() {
    let mut runtime = RelationalRuntimeBuilder::new().build();
    let partition_id = PartitionId(7);
    runtime.partitions.insert(
        partition_id,
        PartitionState {
            partition_id,
            adjacency_policy: AdjacencyPolicy {
                backend: AdjacencyBackend::CompressedFanoutAdjacency,
                small_degree_inline_capacity: 8,
            },
            relation_overlay_is_sparse: false,
            entity_arena: EntityArena::with_capacity(1),
            relation_arena: RelationArena::with_capacity(1),
            adjacency: Default::default(),
            reverse_adjacency: Default::default(),
        },
    );
    let partition = runtime.partitions.get_mut(&partition_id).unwrap();
    partition
        .entity_arena
        .push_slot(SlotInit::<crate::storage::substrate::EntityRecordKind> {
            partition_id,
            kind_id: KindId(1),
            version_id: VersionId(1),
            extra: EntityExtra::default(),
        });
    let entity_id = EntityId::new(partition_id, 0, 1);
    partition
        .relation_arena
        .push_slot(SlotInit::<crate::storage::substrate::RelationRecordKind> {
            partition_id,
            kind_id: KindId(2),
            version_id: VersionId(1),
            extra: RelationExtra {
                endpoints: Some(RelationEndpoints {
                    source: entity_id,
                    target: entity_id,
                }),
                authoritative_aspect_state: None,
            },
        });

    let relation_id = RelationId::new(partition_id, 0, 1);
    runtime
        .visibility_pins()
        .advance_branch_pins_for_changed_records(
            None,
            VersionId(2),
            &[
                RecordRef::Entity(entity_id),
                RecordRef::Relation(relation_id),
            ],
        );

    let partition = runtime.partitions.get(&partition_id).unwrap();
    assert_eq!(partition.entity_arena.branch_pin_count(0), Some(1));
    assert_eq!(partition.relation_arena.branch_pin_count(0), Some(1));
}
