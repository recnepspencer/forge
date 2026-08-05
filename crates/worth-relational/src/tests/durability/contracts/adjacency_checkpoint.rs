use super::*;

#[test]
fn checkpoint_rebuilds_current_and_historical_adjacency_kind_buckets() {
    let mut runtime = persisted_runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let created = create_relation_outcome(&mut runtime, source, target, "checkpoint-relation");
    let relation = changed_relations(&created)[0];
    let historical_version = created.version_id;
    let _deleted = delete_relation_on_branch(&mut runtime, relation, BranchId("main".to_string()));

    let (_, recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
    let relation_kind = crate::facade::identity::KindId(2);
    let historical = recovered
        .read_truth()
        .bounded_outgoing_relations_of_kind_at_version(source, relation_kind, historical_version, 2)
        .expect("recovery must rebuild retained historical kind membership");
    let current = recovered
        .read_truth()
        .bounded_outgoing_relations_of_kind_at_version(
            source,
            relation_kind,
            recovered.current_version_id(),
            1,
        )
        .expect("deleted current adjacency must be empty without scanning history");

    assert_eq!(historical.into_records()[0].relation_id, relation);
    assert!(current.into_records().is_empty());
}
