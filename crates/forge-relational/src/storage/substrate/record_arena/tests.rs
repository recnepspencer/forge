#[cfg(test)]
mod tests {
    use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
    use crate::payloads::data::RecordPayload;
    use crate::storage::data::RecordLifecycleState;
    use crate::symbols::data::Symbol;

    use super::super::{EntityArena, EntityExtra, RelationArena, RelationEndpoints};

    #[test]
    fn reusing_entity_slot_clears_entity_sidecars_and_increments_generation() {
        let mut arena = EntityArena::with_capacity(1);
        let partition_id = PartitionId(7);
        let version_one = VersionId(1);
        let version_two = VersionId(2);
        let payload = RecordPayload::OpaqueBytes(vec![1, 2, 3]);

        let (slot, generation, _) = arena.push_slot(super::super::SlotInit {
            partition_id,
            kind_id: KindId(11),
            payload: Some(payload.clone()),
            version_id: version_one,
            extra: EntityExtra::default(),
        });
        assert_eq!(generation, 1);
        arena.extra[slot] = EntityExtra {
            structural_fingerprint: Some(crate::identity::data::StructuralFingerprint {
                family: Symbol(9),
                value: 42,
            }),
            lineage_id: Some(crate::identity::data::LineageId(12)),
        };
        arena.retire(slot, version_two);
        arena.lifecycle[slot] = RecordLifecycleState::Reusable;
        arena.reset_slot(slot);

        let (_, reused_generation, reused) = arena.push_slot(super::super::SlotInit {
            partition_id,
            kind_id: KindId(12),
            payload: Some(payload),
            version_id: VersionId(3),
            extra: EntityExtra::default(),
        });
        assert!(reused);
        assert_eq!(reused_generation, 2);
        assert!(arena.extra[slot].structural_fingerprint.is_none());
        assert!(arena.extra[slot].lineage_id.is_none());
    }

    #[test]
    fn reusing_relation_slot_replaces_endpoints_and_increments_generation() {
        let mut arena = RelationArena::with_capacity(1);
        let partition_id = PartitionId(3);
        let first = RelationEndpoints {
            source: EntityId::new(partition_id, 1, 1),
            target: EntityId::new(partition_id, 2, 1),
        };
        let second = RelationEndpoints {
            source: EntityId::new(partition_id, 3, 1),
            target: EntityId::new(partition_id, 4, 1),
        };

        let (slot, generation, _) = arena.push_slot(super::super::SlotInit {
            partition_id,
            kind_id: KindId(21),
            payload: Some(RecordPayload::OpaqueBytes(vec![9])),
            version_id: VersionId(1),
            extra: Some(first),
        });
        assert_eq!(generation, 1);
        arena.retire(slot, VersionId(2));
        arena.lifecycle[slot] = RecordLifecycleState::Reusable;
        arena.reset_slot(slot);

        let (_, reused_generation, reused) = arena.push_slot(super::super::SlotInit {
            partition_id,
            kind_id: KindId(22),
            payload: None,
            version_id: VersionId(3),
            extra: Some(second.clone()),
        });
        assert!(reused);
        assert_eq!(reused_generation, 2);
        assert_eq!(arena.extra[slot], Some(second));
    }

    #[test]
    fn get_rejects_id_from_different_partition_even_with_same_slot_and_generation() {
        let mut arena = RelationArena::with_capacity(1);
        let partition_id = PartitionId(3);
        let other_partition_id = PartitionId(4);
        let (slot, generation, _) = arena.push_slot(super::super::SlotInit {
            partition_id,
            kind_id: KindId(21),
            payload: Some(RecordPayload::OpaqueBytes(vec![9])),
            version_id: VersionId(1),
            extra: Some(RelationEndpoints {
                source: EntityId::new(partition_id, 1, 1),
                target: EntityId::new(partition_id, 2, 1),
            }),
        });

        let wrong_partition_id = RelationId::new(other_partition_id, slot as u64, generation);
        assert!(arena.get(&wrong_partition_id).is_none());
    }
}
