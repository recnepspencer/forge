// CONTRACT: complexity budgets and executable work proofs
// LANES: declarations, incremental-pin-maintenance, visibility-scan-accounting, live-history-trimming, bidirectional-adjacency

use super::*;

#[test]
fn complexity_contract_registry_covers_runtime_hot_paths() {
    let runtime = runtime_with_test_schema();
    let contracts = runtime.complexity_contracts();

    assert!(contracts.len() >= 6);
    assert!(contracts
        .iter()
        .all(|contract| !contract.proof_tests.is_empty()));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.current_state.clone"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.snapshot_pin_maintenance"));
}

#[test]
fn complexity_contract_current_state_clone_is_declared_and_measured() {
    let mut runtime = runtime_with_test_schema();
    for index in 0..8 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }

    runtime.reset_complexity_counters();
    let entity = create_entity(&mut runtime, "target");
    runtime.reset_complexity_counters();
    let _ = update_entity(&mut runtime, entity, "target-updated");
    let counters = runtime.complexity_counters();

    assert_eq!(counters.full_state_clones, 1);
    assert!(counters.entity_slots_cloned >= 9);
    assert!(counters.relation_slots_cloned == 0);
}

#[test]
fn complexity_budget_snapshot_pin_maintenance_is_incremental() {
    let mut runtime = runtime_with_test_schema();
    for index in 0..6 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }
    let snapshot = runtime.snapshot();
    let target = create_entity(&mut runtime, "target");

    runtime.reset_complexity_counters();
    let _ = update_entity(&mut runtime, target, "updated");
    let after_commit = runtime.complexity_counters();
    assert_eq!(after_commit.snapshot_pin_full_rebuilds, 0);

    runtime.reset_complexity_counters();
    assert!(runtime.release_snapshot(&snapshot));
    let after_release = runtime.complexity_counters();
    assert_eq!(after_release.snapshot_pin_full_rebuilds, 0);
    assert!(after_release.snapshot_pin_adjustments > 0);
}

#[test]
fn complexity_contract_visibility_scans_are_explicitly_measured() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let _relation = create_relation(&mut runtime, source, target, "r0");
    let snapshot = runtime.snapshot();

    runtime.reset_complexity_counters();
    let _ = runtime.read_snapshot(&snapshot).unwrap();
    let counters = runtime.complexity_counters();

    assert!(counters.visibility_entity_slot_scans >= 2);
    assert!(counters.visibility_relation_slot_scans >= 1);
    assert!(counters.visible_entity_records_materialized >= 2);
    assert!(counters.visible_relation_records_materialized >= 1);
}

#[test]
fn complexity_contract_invariant_materialization_is_declared_and_measured() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        commit_boundary: vec![InvariantRule::UniqueEntityPayloadField("name".to_string())],
        ..InvariantCatalog::default()
    });
    let entity = create_entity(&mut runtime, "a");

    runtime.reset_complexity_counters();
    let _ = update_entity(&mut runtime, entity, "a-2");
    let counters = runtime.complexity_counters();

    assert!(counters.invariant_entity_slot_scans >= 1);
    assert_eq!(counters.invariant_entity_records_materialized, 0);
}

#[test]
fn complexity_budget_live_history_trimming_is_touched_record_bounded() {
    let mut runtime = runtime_with_test_schema();
    let create_a = create_entity_outcome(&mut runtime, "a");
    let entity_a = changed_entities(&create_a)[0];
    let create_b = create_entity_outcome(&mut runtime, "b");
    let entity_b = changed_entities(&create_b)[0];
    assert!(runtime.release_snapshot(&create_a.snapshot));
    assert!(runtime.release_snapshot(&create_b.snapshot));

    runtime.reset_complexity_counters();
    let update_a1 = update_entity(&mut runtime, entity_a, "a-1");
    assert!(runtime.release_snapshot(&update_a1.snapshot));
    let _ = update_entity(&mut runtime, entity_a, "a-2");
    let counters = runtime.complexity_counters();

    assert_eq!(runtime.entity_history_len_for_test(entity_a), 1);
    assert_eq!(runtime.entity_history_len_for_test(entity_b), 1);
    assert!(counters.live_entity_history_entries_trimmed >= 1);
}

#[test]
fn complexity_budget_bidirectional_adjacency_avoids_relation_scans() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation = create_relation(&mut runtime, source, target, "r0");
    let version_id = runtime.latest_commit().unwrap().version_id;

    runtime.reset_complexity_counters();
    let outgoing = runtime.outgoing_relations_for_entity(source, version_id);
    let incoming = runtime.incoming_relations_for_entity(target, version_id);
    let counters = runtime.complexity_counters();

    assert_eq!(outgoing, vec![relation]);
    assert_eq!(incoming, vec![relation]);
    assert_eq!(counters.visibility_relation_slot_scans, 0);
}
