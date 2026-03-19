use crate::facade::identity::EntityId;
use crate::facade::runtime::EntityRecordProjection;
use crate::tests::support::*;
use std::sync::OnceLock;

fn entity_name_aspects() -> &'static [AspectKey] {
    static ASPECTS: OnceLock<Vec<AspectKey>> = OnceLock::new();
    ASPECTS
        .get_or_init(|| vec![AspectKey(InternedString::Raw("name".to_string()))])
        .as_slice()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NamedEntityProjection {
    entity_id: EntityId,
    payload: RecordPayload,
}

impl EntityRecordProjection for NamedEntityProjection {
    const KIND: KindId = KindId(1);

    fn required_aspects() -> &'static [AspectKey] {
        entity_name_aspects()
    }

    fn from_record(record: &EntityReadRecord) -> Option<Self> {
        Some(Self {
            entity_id: record.entity_id,
            payload: record.payload.clone(),
        })
    }
}

// CONTRACT: entity_kind_scans
// LANES: success, adversarial, determinism, historical

#[test]
fn entity_kind_scans_can_be_partition_scoped_without_cross_partition_leakage() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let left = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
    let _right = create_entity_in_partition(&mut runtime, "right-a", PartitionId(11));
    let version_id = runtime.history_access().latest_commit().unwrap().version_id;

    let scoped = runtime
        .visibility_reads()
        .project_version(version_id)
        .entities_in::<NamedEntityProjection>(PartitionId(7));

    assert_eq!(scoped.len(), 1);
    assert_eq!(scoped[0].entity_id, left);
    assert!(scoped
        .iter()
        .all(|record| record.entity_id.partition_id == PartitionId(7)));
}

#[test]
fn entity_kind_scans_are_deterministic_across_equivalent_insert_order() {
    let mut ordered =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let ordered_a = create_entity_in_partition(&mut ordered, "a", PartitionId(3));
    let ordered_b = create_entity_in_partition(&mut ordered, "b", PartitionId(3));

    let mut reversed =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let reversed_b = create_entity_in_partition(&mut reversed, "b", PartitionId(3));
    let reversed_a = create_entity_in_partition(&mut reversed, "a", PartitionId(3));

    let ordered_records = ordered
        .visibility_reads()
        .project_version(ordered.current_version_id())
        .entities_in::<NamedEntityProjection>(PartitionId(3));
    let reversed_records = reversed
        .visibility_reads()
        .project_version(reversed.current_version_id())
        .entities_in::<NamedEntityProjection>(PartitionId(3));

    assert_eq!(
        ordered_records
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        vec![ordered_a, ordered_b]
    );
    assert_eq!(
        reversed_records
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        vec![reversed_b, reversed_a]
    );
    assert_eq!(
        ordered
            .visibility_reads()
            .project_version(ordered.current_version_id())
            .entities_in::<NamedEntityProjection>(PartitionId(3))
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        ordered_records
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>()
    );
    assert!(reversed_b.local_slot.0 < reversed_a.local_slot.0);
}

#[test]
fn entity_kind_scans_preserve_historical_partition_visibility() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let baseline =
        create_entity_outcome_on_branch(&mut runtime, "base", BranchId("main".to_string()));
    let main_entity = changed_entities(&baseline)[0];
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(17));
    let historical_version = runtime.history_access().latest_commit().unwrap().version_id;
    let _other_partition = create_entity_in_partition(&mut runtime, "other", PartitionId(23));
    let _update = update_entity(&mut runtime, main_entity, "base-updated");
    let _later_left = create_entity_in_partition(&mut runtime, "left-later", PartitionId(17));

    let historical = runtime
        .visibility_reads()
        .project_version(historical_version)
        .entities_in::<NamedEntityProjection>(PartitionId(17));

    assert_eq!(historical.len(), 1);
    assert_eq!(historical[0].entity_id, left);
    assert_eq!(
        historical[0].payload,
        RecordPayload::StructuredJson(json!({ "name": "left" }))
    );
}
