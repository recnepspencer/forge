use crate::{
    evidence::StoreCounters, ComplexityStatus, WORTHStoreBuilder, Milestone13CounterContract,
    StoreCounterSnapshot, StoreErrorKind,
};

#[test]
fn milestone_13_counter_fields_default_to_zero() {
    let snapshot = StoreCounterSnapshot::default();

    assert_eq!(snapshot.placement_state_manifest_load_count, 0);
    assert_eq!(snapshot.placement_state_recovery_count, 0);
    assert_eq!(snapshot.working_set_observation_window_count, 0);
    assert_eq!(snapshot.working_set_reclassification_count, 0);
    assert_eq!(snapshot.hot_tier_resident_read_count, 0);
    assert_eq!(snapshot.warm_tier_resident_read_count, 0);
    assert_eq!(snapshot.cold_tier_recall_count, 0);
    assert_eq!(snapshot.foreground_cold_recall_count, 0);
    assert_eq!(snapshot.background_tier_move_count, 0);
    assert_eq!(snapshot.restart_recall_count, 0);
    assert_eq!(snapshot.tier_move_plan_count, 0);
    assert_eq!(snapshot.tier_move_cutover_count, 0);
    assert_eq!(snapshot.tier_move_cutover_rejection_count, 0);
    assert_eq!(snapshot.authoritative_tier_move_count, 0);
    assert_eq!(snapshot.derived_tier_move_count, 0);
    assert_eq!(snapshot.tier_move_rejection_count, 0);
    assert_eq!(snapshot.tier_miss_count, 0);
    assert_eq!(snapshot.broadened_recall_plan_count, 0);
    assert_eq!(snapshot.recall_coalesced_request_count, 0);
    assert_eq!(snapshot.recall_duplicate_suppression_count, 0);
    assert_eq!(snapshot.tier_interleaved_read_count, 0);
    assert_eq!(snapshot.tier_interleaved_continuation_count, 0);
    assert_eq!(snapshot.tier_interleaving_recall_count, 0);
    assert_eq!(snapshot.tier_interleaving_parity_failure_count, 0);
    assert_eq!(snapshot.placement_debt_count, 0);
    assert_eq!(snapshot.working_set_debt_count, 0);
    assert_eq!(snapshot.tier_truth_parity_failure_count, 0);
    assert_eq!(snapshot.tier_restore_parity_failure_count, 0);
    assert_eq!(snapshot.tier_recall_failure_count, 0);
}

#[test]
fn live_store_snapshot_exposes_zeroed_milestone_13_counters_before_wiring() {
    let store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let snapshot = store.counters();

    assert_eq!(snapshot.placement_state_manifest_load_count, 0);
    assert_eq!(snapshot.tier_move_plan_count, 0);
    assert_eq!(snapshot.cold_tier_recall_count, 0);
    assert_eq!(snapshot.tier_recall_failure_count, 0);
}

#[test]
fn milestone_13_counter_recorders_increment_exact_fields() {
    let counters = StoreCounters::default();

    counters.record_placement_state_manifest_loads(1);
    counters.record_placement_state_recovery(2);
    counters.record_working_set_observation_windows(3);
    counters.record_working_set_reclassifications(4);
    counters.record_hot_tier_resident_reads(5);
    counters.record_warm_tier_resident_reads(6);
    counters.record_cold_tier_recalls(7);
    counters.record_foreground_cold_recalls(8);
    counters.record_background_tier_moves(9);
    counters.record_restart_recalls(10);
    counters.record_tier_move_plans(11);
    counters.record_tier_move_cutovers(12);
    counters.record_tier_move_cutover_rejections(13);
    counters.record_authoritative_tier_moves(14);
    counters.record_derived_tier_moves(15);
    counters.record_tier_move_rejections(16);
    counters.record_tier_misses(17);
    counters.record_broadened_recall_plans(18);
    counters.record_recall_coalesced_requests(19);
    counters.record_recall_duplicate_suppression(20);
    counters.record_tier_interleaved_reads(21);
    counters.record_tier_interleaved_continuations(22);
    counters.record_tier_interleaving_recalls(23);
    counters.record_tier_interleaving_parity_failures(24);
    counters.record_placement_debt(25);
    counters.record_working_set_debt(26);
    counters.record_tier_truth_parity_failures(27);
    counters.record_tier_restore_parity_failures(28);
    counters.record_tier_recall_failures(29);

    let snapshot = counters.snapshot();
    assert_eq!(snapshot.placement_state_manifest_load_count, 1);
    assert_eq!(snapshot.placement_state_recovery_count, 2);
    assert_eq!(snapshot.working_set_observation_window_count, 3);
    assert_eq!(snapshot.working_set_reclassification_count, 4);
    assert_eq!(snapshot.hot_tier_resident_read_count, 5);
    assert_eq!(snapshot.warm_tier_resident_read_count, 6);
    assert_eq!(snapshot.cold_tier_recall_count, 7);
    assert_eq!(snapshot.foreground_cold_recall_count, 8);
    assert_eq!(snapshot.background_tier_move_count, 9);
    assert_eq!(snapshot.restart_recall_count, 10);
    assert_eq!(snapshot.tier_move_plan_count, 11);
    assert_eq!(snapshot.tier_move_cutover_count, 12);
    assert_eq!(snapshot.tier_move_cutover_rejection_count, 13);
    assert_eq!(snapshot.authoritative_tier_move_count, 14);
    assert_eq!(snapshot.derived_tier_move_count, 15);
    assert_eq!(snapshot.tier_move_rejection_count, 16);
    assert_eq!(snapshot.tier_miss_count, 17);
    assert_eq!(snapshot.broadened_recall_plan_count, 18);
    assert_eq!(snapshot.recall_coalesced_request_count, 19);
    assert_eq!(snapshot.recall_duplicate_suppression_count, 20);
    assert_eq!(snapshot.tier_interleaved_read_count, 21);
    assert_eq!(snapshot.tier_interleaved_continuation_count, 22);
    assert_eq!(snapshot.tier_interleaving_recall_count, 23);
    assert_eq!(snapshot.tier_interleaving_parity_failure_count, 24);
    assert_eq!(snapshot.placement_debt_count, 25);
    assert_eq!(snapshot.working_set_debt_count, 26);
    assert_eq!(snapshot.tier_truth_parity_failure_count, 27);
    assert_eq!(snapshot.tier_restore_parity_failure_count, 28);
    assert_eq!(snapshot.tier_recall_failure_count, 29);
    assert_eq!(snapshot.retention_policy_evaluation_count, 0);
    assert_eq!(snapshot.maintenance_declaration_count, 0);
}

#[test]
fn milestone_13_counter_contract_matches_snapshot() {
    let counters = StoreCounters::default();
    counters.record_tier_move_plans(3);
    counters.record_foreground_cold_recalls(5);
    counters.record_tier_recall_failures(1);

    let snapshot = counters.snapshot();
    let contract = Milestone13CounterContract::from_snapshot(&snapshot);

    assert_eq!(contract.tier_move_plan_count, snapshot.tier_move_plan_count);
    assert_eq!(
        contract.foreground_cold_recall_count,
        snapshot.foreground_cold_recall_count
    );
    assert_eq!(
        contract.tier_recall_failure_count,
        snapshot.tier_recall_failure_count
    );
}

#[test]
fn worth_store_exposes_phase_1_milestone_13_support_surfaces() {
    let store = WORTHStoreBuilder::new().in_memory().build().unwrap();

    let counters = store.milestone_13_counter_contract();
    assert_eq!(counters.tier_move_plan_count, 0);
    assert_eq!(counters.cold_tier_recall_count, 0);

    let complexity = store.milestone_13_complexity_surface();
    assert_eq!(
        complexity.placement_state_reconstruction.status,
        ComplexityStatus::Verified
    );
    assert_eq!(
        complexity.working_set_classification.status,
        ComplexityStatus::Verified
    );
    assert_eq!(
        complexity.tier_move_planning.status,
        ComplexityStatus::Verified
    );
    assert_eq!(complexity.tier_move_cutover.status, ComplexityStatus::Debt);
    assert_eq!(
        complexity.tier_move_execution.status,
        ComplexityStatus::Debt
    );
    assert_eq!(
        complexity.cold_recall_execution.status,
        ComplexityStatus::Debt
    );
    assert_eq!(complexity.recall_coalescing.status, ComplexityStatus::Debt);
    assert_eq!(
        complexity
            .tier_move_planning
            .proof_basis
            .as_deref()
            .unwrap(),
        "tier move planning lowers conservative-policy placement into typed authoritative and derived plans"
    );
}

#[test]
fn proof_bearing_constructors_normalize_lists_deterministically() {
    let window = crate::WorkingSetObservationWindow::new(
        crate::PlacementObservationScopeClass::Branch,
        "branch:main",
        vec![
            "artifact:z".to_string(),
            "artifact:a".to_string(),
            "artifact:z".to_string(),
        ],
    );
    let manifest = crate::CanonicalResidencyManifest::new(
        vec!["b".to_string(), "a".to_string(), "a".to_string()],
        vec![
            "transfer:2".to_string(),
            "transfer:1".to_string(),
            "transfer:2".to_string(),
        ],
    );

    assert_eq!(
        window.observed_artifact_keys(),
        &["artifact:a".to_string(), "artifact:z".to_string()]
    );
    assert_eq!(
        window.scope_class(),
        crate::PlacementObservationScopeClass::Branch
    );
    assert_eq!(window.scope_key(), "branch:main");
    assert_eq!(
        manifest.resident_artifact_keys(),
        &["a".to_string(), "b".to_string()]
    );
    assert_eq!(
        manifest.in_flight_transfer_keys(),
        &["transfer:1".to_string(), "transfer:2".to_string()]
    );
}

#[test]
fn aggressive_placement_policy_markers_map_to_placement_policy_error() {
    let policy = crate::PlacementPolicyClass::AdaptiveDebt(
        crate::AdaptivePlacementDebtMarker::AggressiveColdAuthorityPlacement,
    );

    let error = policy.require_conservative().unwrap_err();
    assert_eq!(error.kind(), &StoreErrorKind::PlacementPolicyUnsupported);
}
