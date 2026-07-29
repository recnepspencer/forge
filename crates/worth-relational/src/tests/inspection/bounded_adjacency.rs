use super::*;

#[test]
fn bounded_adjacency_stops_before_allocating_the_unbounded_fanout() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    for ordinal in 0..64 {
        let target = create_entity(&mut runtime, &format!("target-{ordinal}"));
        let _ = create_relation(&mut runtime, source, target, &format!("relation-{ordinal}"));
    }
    let relation_kind = crate::facade::identity::KindId(2);
    let version = runtime.current_version_id();

    let limit = runtime
        .read_truth()
        .bounded_outgoing_relations_of_kind_at_version(source, relation_kind, version, 5)
        .expect_err("five units cannot materialize a sixty-four-edge adjacency");

    assert_eq!(limit.consumed_work_units(), 5);
    assert_eq!(limit.relation_records_examined(), 3);
    assert_eq!(limit.endpoint_records_reserved(), 2);

    let complete = runtime
        .read_truth()
        .bounded_outgoing_relations_of_kind_at_version(source, relation_kind, version, 128)
        .expect("two work units per matching edge admit the exact fanout");
    assert_eq!(complete.relation_records_examined(), 64);
    assert_eq!(complete.endpoint_records_reserved(), 64);
    assert_eq!(complete.work_units(), 128);
    assert_eq!(complete.into_records().len(), 64);
}
