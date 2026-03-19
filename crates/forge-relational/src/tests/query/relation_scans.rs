use crate::facade::identity::EntityId;
use crate::facade::runtime::{RelationReadRecord, RelationRecordProjection};
use crate::tests::support::*;
use std::sync::OnceLock;

fn relation_label_aspects() -> &'static [AspectKey] {
    static ASPECTS: OnceLock<Vec<AspectKey>> = OnceLock::new();
    ASPECTS
        .get_or_init(|| vec![AspectKey(InternedString::Raw("label".to_string()))])
        .as_slice()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EdgeProjection {
    relation_id: RelationId,
    source: EntityId,
    target: EntityId,
}

impl RelationRecordProjection for EdgeProjection {
    const KIND: KindId = KindId(2);

    fn required_aspects() -> &'static [AspectKey] {
        relation_label_aspects()
    }

    fn from_record(record: &RelationReadRecord) -> Option<Self> {
        Some(Self {
            relation_id: record.relation_id,
            source: record.source,
            target: record.target,
        })
    }
}

// CONTRACT: relation_scans
// LANES: success, determinism, adversarial

#[test]
fn relation_kind_scans_return_only_visible_relations_of_that_kind() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let left = create_entity_outcome(&mut runtime, "left");
    let right = create_entity_outcome(&mut runtime, "right");
    let third = create_entity_outcome(&mut runtime, "third");
    let left = changed_entities(&left)[0];
    let right = changed_entities(&right)[0];
    let third = changed_entities(&third)[0];
    let r1 = create_relation(&mut runtime, left, right, "r1");
    let r2 = create_relation(&mut runtime, right, third, "r2");
    let deleted = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("delete-r1").push(MutationIntent::Relation(
                RelationMutationIntent::Delete(DeleteRelationIntent { relation_id: r1 }),
            )),
        );
        txn.commit().unwrap()
    };
    let visible = runtime
        .visibility_reads()
        .project_version(deleted.version_id)
        .relations::<EdgeProjection>();

    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].relation_id, r2);
}

#[test]
fn relation_kind_scans_are_deterministic_across_equivalent_insert_order() {
    let mut runtime_a =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let a_left = create_entity(&mut runtime_a, "left");
    let a_right = create_entity(&mut runtime_a, "right");
    let a_third = create_entity(&mut runtime_a, "third");
    let _ = create_relation(&mut runtime_a, a_left, a_right, "r1");
    let _ = create_relation(&mut runtime_a, a_right, a_third, "r2");
    let scan_a = runtime_a
        .visibility_reads()
        .project_version(runtime_a.current_version_id())
        .relations::<EdgeProjection>();

    let mut runtime_b =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let b_left = create_entity(&mut runtime_b, "left");
    let b_right = create_entity(&mut runtime_b, "right");
    let b_third = create_entity(&mut runtime_b, "third");
    let _ = create_relation(&mut runtime_b, b_right, b_third, "r2");
    let _ = create_relation(&mut runtime_b, b_left, b_right, "r1");
    let scan_b = runtime_b
        .visibility_reads()
        .project_version(runtime_b.current_version_id())
        .relations::<EdgeProjection>();

    assert_eq!(scan_a.len(), scan_b.len());
    assert_eq!(
        scan_a
            .iter()
            .map(|record| (record.source.local_slot.0, record.target.local_slot.0))
            .collect::<Vec<_>>(),
        scan_b
            .iter()
            .map(|record| (record.source.local_slot.0, record.target.local_slot.0))
            .collect::<Vec<_>>()
    );
}
