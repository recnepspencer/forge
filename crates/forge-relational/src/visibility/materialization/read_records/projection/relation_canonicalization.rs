use crate::storage::data::RelationReadRecord;

pub(super) fn relation_records_are_canonical(records: &[RelationReadRecord]) -> bool {
    records
        .windows(2)
        .all(|window| canonical_relation_key(&window[0]) <= canonical_relation_key(&window[1]))
}

pub(super) fn canonical_relation_key(
    record: &RelationReadRecord,
) -> (u32, u64, u32, u64, u32, u64) {
    (
        record.source.partition_value(),
        record.source.local_slot_value(),
        record.target.partition_value(),
        record.target.local_slot_value(),
        record.relation_id.partition_value(),
        record.relation_id.local_slot_value(),
    )
}

#[cfg(test)]
mod tests {
    use super::canonical_relation_key;
    use crate::identity::data::{EntityId, PartitionId, RelationId};
    use crate::identity::data::{KindId, VersionId};
    use crate::schema::data::{KindResolution, SchemaId, SchemaVersionId};
    use crate::storage::data::{RecordLifecycleState, RelationReadRecord};

    #[test]
    fn canonical_relation_key_uses_named_record_identity_accessors() {
        let record = RelationReadRecord {
            relation_id: RelationId::new(PartitionId::new(9), 5, 2),
            kind: KindResolution {
                kind_id: KindId::new(6),
                kind_name: "connected_to".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
            },
            lifecycle: RecordLifecycleState::Live,
            created_at_version: VersionId::new(2),
            retired_at_version: None,
            source: EntityId::new(PartitionId::new(3), 11, 7),
            target: EntityId::new(PartitionId::new(4), 12, 8),
            authoritative_aspect_state: None,
            authoritative_field_key_comparison_keys: std::collections::BTreeMap::new(),
        };

        assert_eq!(canonical_relation_key(&record), (3, 11, 4, 12, 9, 5));
    }
}
