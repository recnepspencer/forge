use crate::tests::support::*;

// CONTRACT: entity_kind_scans
// LANES: success, adversarial, determinism, historical

#[test]
fn entity_kind_scans_can_be_partition_scoped_without_cross_partition_leakage() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
    let _right = create_entity_in_partition(&mut runtime, "right-a", PartitionId(11));
    let version_id = runtime.latest_commit().unwrap().version_id;

    let scoped = runtime.visible_entities_of_kind_in_partition(PartitionId(7), KindId(1), version_id);

    assert_eq!(scoped.len(), 1);
    assert_eq!(scoped[0].entity_id, left);
    assert!(scoped
        .iter()
        .all(|record| record.entity_id.partition_id == PartitionId(7)));
}

#[test]
fn entity_kind_scans_are_deterministic_across_equivalent_insert_order() {
    let mut ordered = runtime_with_test_schema();
    let ordered_a = create_entity_in_partition(&mut ordered, "a", PartitionId(3));
    let ordered_b = create_entity_in_partition(&mut ordered, "b", PartitionId(3));

    let mut reversed = runtime_with_test_schema();
    let reversed_b = create_entity_in_partition(&mut reversed, "b", PartitionId(3));
    let reversed_a = create_entity_in_partition(&mut reversed, "a", PartitionId(3));

    let ordered_records =
        ordered.visible_entities_of_kind_in_partition(PartitionId(3), KindId(1), ordered.current_version_id());
    let reversed_records =
        reversed.visible_entities_of_kind_in_partition(PartitionId(3), KindId(1), reversed.current_version_id());

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
            .visible_entities_of_kind_in_partition(PartitionId(3), KindId(1), ordered.current_version_id())
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
    let mut runtime = runtime_with_test_schema();
    let baseline = create_entity_outcome_on_branch(&mut runtime, "base", BranchId("main".to_string()));
    let main_entity = changed_entities(&baseline)[0];
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(17));
    let historical_version = runtime.latest_commit().unwrap().version_id;
    let _other_partition = create_entity_in_partition(&mut runtime, "other", PartitionId(23));
    let _update = update_entity(&mut runtime, main_entity, "base-updated");
    let _later_left = create_entity_in_partition(&mut runtime, "left-later", PartitionId(17));

    let historical =
        runtime.visible_entities_of_kind_in_partition(PartitionId(17), KindId(1), historical_version);

    assert_eq!(historical.len(), 1);
    assert_eq!(historical[0].entity_id, left);
    assert_eq!(
        historical[0].payload,
        RecordPayload::StructuredJson(json!({ "name": "left" }))
    );
}
