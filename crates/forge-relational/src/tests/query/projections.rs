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
    name: String,
}

impl EntityRecordProjection for NamedEntityProjection {
    const KIND: KindId = KindId(1);

    fn required_aspects() -> &'static [AspectKey] {
        entity_name_aspects()
    }

    fn from_record(record: &EntityReadRecord) -> Option<Self> {
        Some(Self {
            entity_id: record.entity_id,
            name: record.payload.as_json()?.get("name")?.as_str()?.to_string(),
        })
    }
}

#[test]
fn entity_projections_collapse_kind_and_partition_threading() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let left = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
    let _other_left = create_entity_in_partition(&mut runtime, "left-b", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right-a", PartitionId(11));
    let view = runtime
        .visibility_reads()
        .project_version(runtime.current_version_id());

    let projected = view.entities::<NamedEntityProjection>();
    let scoped = view.entities_in::<NamedEntityProjection>(PartitionId(7));
    let single = view.entity::<NamedEntityProjection>(right).unwrap();

    assert_eq!(projected.len(), 3);
    assert_eq!(
        projected
            .iter()
            .map(|entry| entry.entity_id)
            .collect::<Vec<_>>(),
        vec![left, scoped[1].entity_id, right]
    );
    assert_eq!(scoped.len(), 2);
    assert!(scoped
        .iter()
        .all(|entry| entry.entity_id.partition_id == PartitionId(7)));
    assert_eq!(single.name, "right-a");
}

#[test]
fn snapshot_projection_resolves_version_without_manual_kind_scan_parameters() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "visible");

    let projected = runtime
        .visibility_reads()
        .project_snapshot(&created.snapshot)
        .unwrap()
        .entities::<NamedEntityProjection>();

    assert_eq!(projected.len(), 1);
    assert_eq!(projected[0].name, "visible");
    assert_eq!(
        <NamedEntityProjection as EntityRecordProjection>::required_aspects(),
        entity_name_aspects()
    );
}

#[test]
fn projection_raw_record_escape_hatches_preserve_full_visible_record_sets() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));
    let relation = create_relation(&mut runtime, left, right, "r0");
    let version_id = runtime.current_version_id();
    let view = runtime.visibility_reads().project_version(version_id);
    let read = runtime.visibility_reads().read_version(version_id);

    assert_eq!(
        view.all_entity_records()
            .into_iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>(),
        read.entities()
            .iter()
            .map(|record| record.entity_id)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        view.all_relation_records()
            .into_iter()
            .map(|record| record.relation_id)
            .collect::<Vec<_>>(),
        vec![relation]
    );
}

#[test]
#[should_panic(expected = "requires undeclared aspect")]
fn projection_rejects_undeclared_required_aspects() {
    let mut runtime = runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "visible");
    let _ = runtime
        .visibility_reads()
        .project_version(runtime.current_version_id())
        .entities::<NamedEntityProjection>();
}
