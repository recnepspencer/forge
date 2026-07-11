mod phase22_fixture;

use forge_store_layout_indexes::layout_strategy_admission::{
    phase22_bounded_wal_tail_rule, phase22_replay_index_rule,
};
use forge_store_recovery_physics::{
    RecoveryLayoutAccess, RecoveryLayoutAccessDenialKind, WalValidPrefix,
};

#[test]
fn phase22_replay_index_and_bounded_tail_families_consume_phase21_authority() {
    let fixture = phase22_fixture::fixture();
    let source = phase22_fixture::admitted_source_with_residue();
    let access = RecoveryLayoutAccess::s8();

    let replay_family = access
        .replay_index_layout(&phase22_replay_index_rule().unwrap())
        .expect("replay family");
    let replay = replay_family
        .admit_checkpoint_replay_index(&fixture.checkpoint_report, &fixture.replay_cursor)
        .expect("replay index");
    assert_eq!(
        replay.checkpoint_id(),
        Some(fixture.checkpoint_report.checkpoint_id())
    );
    assert_eq!(replay.replay_frontier(), phase22_fixture::wal_range(30, 45));
    assert_eq!(replay.counters().checkpoint_cutover_inputs(), 1);

    let row = &source.trace().decision_rows()[0];
    let denial = replay_family.reject_row_projection(row).unwrap_err();
    assert_eq!(
        denial.kind(),
        RecoveryLayoutAccessDenialKind::ReplayProjectionCannotStandInForWalAuthority
    );

    let bounded = access
        .bounded_wal_tail_layout(&phase22_bounded_wal_tail_rule().unwrap())
        .expect("bounded tail family")
        .lookup_tail_range(&replay, phase22_fixture::wal_range(32, 40))
        .expect("bounded range");
    assert_eq!(
        bounded.requested_range(),
        phase22_fixture::wal_range(32, 40)
    );
    assert_eq!(
        bounded.replay_frontier(),
        phase22_fixture::wal_range(30, 45)
    );

    let denial = access
        .bounded_wal_tail_layout(&phase22_bounded_wal_tail_rule().unwrap())
        .expect("bounded tail family")
        .lookup_tail_range(&replay, phase22_fixture::wal_range(29, 46))
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        RecoveryLayoutAccessDenialKind::BoundedWalTailLookupOutOfRange
    );
}

#[test]
fn phase22_bounded_source_and_valid_prefix_production_paths_consume_family_authority() {
    let admission = phase22_fixture::bounded_source_admission();
    let report = admission.layout_report();
    assert_eq!(report.candidate_count(), 3);
    assert_eq!(
        report.selected_wal_range(),
        Some(phase22_fixture::wal_range(30, 45))
    );

    let denial = WalValidPrefix::from_observation_scan(
        admission.source(),
        forge_store_recovery_physics::WalSegmentGeneration::new(1).unwrap(),
        phase22_fixture::wal_range(30, 45),
        forge_store_recovery_physics::WalPrefixObservationScan::from_observations(Vec::new()),
    )
    .unwrap_err();
    assert!(matches!(
        denial.kind(),
        forge_store_recovery_physics::RedoPlanningDenialKind::MissingAcknowledgedWalRange(_)
    ));
}
