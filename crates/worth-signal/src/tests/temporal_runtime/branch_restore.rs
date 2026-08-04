use crate::facade::{
    ClockAdvanceOrdinal, ClockAdvanceRequest, ClockDomain, ClockTick, SignalGraph, SignalRuntime,
    TemporalCondition, TemporalReconstructabilityArtifact, TemporalWakeId,
    TemporalWakeRetirementReason, WakeOrdinal,
};

#[test]
fn branch_switch_preserves_clock_basis_identity() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.current_branch();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    let feature = runtime.create_branch("feature-clock").unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    assert_eq!(runtime.clock_basis().current_tick(), ClockTick::new(5));

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(9),
        ))
        .unwrap();
    assert_eq!(
        runtime.clock_basis().last_advance_ordinal(),
        ClockAdvanceOrdinal::new(2)
    );

    runtime.switch_branch(main).unwrap();
    assert_eq!(runtime.clock_basis().current_tick(), ClockTick::new(5));
    assert_eq!(
        runtime.clock_basis().last_advance_ordinal(),
        ClockAdvanceOrdinal::new(1)
    );
}

#[test]
fn branch_switch_preserves_temporal_wake_state() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.current_branch();

    let main_wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(6).unwrap(), ClockTick::new(6))
        .unwrap();
    let feature = runtime.create_branch("feature-temporal").unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(
        runtime.temporal_wake_summary().next_wake_id(),
        TemporalWakeId::new(1)
    );
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(6))
    );
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .branch_local_temporal_restore_count,
        0
    );

    runtime
        .schedule_temporal_wake(
            TemporalCondition::stale_after(8).unwrap(),
            ClockTick::new(8),
        )
        .unwrap();
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 2);

    runtime.switch_branch(main).unwrap();
    let summary = runtime.temporal_wake_summary();
    assert_eq!(summary.scheduled_count(), 1);
    assert_eq!(summary.ready_count(), 0);
    assert_eq!(summary.retired_count(), 0);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(TemporalWakeId::new(0))
    );
    assert_eq!(summary.next_wake_id(), TemporalWakeId::new(1));
    assert_eq!(summary.next_wake_ordinal(), WakeOrdinal::new(1));
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .branch_local_temporal_restore_count,
        0
    );
    assert_eq!(main_wake.id(), TemporalWakeId::new(0));
}

#[test]
fn active_temporal_snapshot_restore_counts_restore_and_reinstates_frontier() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(4).unwrap(), ClockTick::new(4))
        .unwrap();
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
    let snapshot_proof = snapshot.reconstructability_proof().unwrap();
    assert_eq!(snapshot_proof.temporal.wake_summary.scheduled_count(), 1);
    assert_eq!(snapshot_proof.temporal.scheduled_wake_count, 1);
    assert_eq!(
        snapshot_proof.temporal.clock_basis.current_tick(),
        ClockTick::ZERO
    );
    assert_ne!(
        snapshot_proof.temporal.certification_digest,
        TemporalReconstructabilityArtifact::default().certification_digest
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(4),
        ))
        .unwrap();
    runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 1);

    runtime.restore_snapshot(&snapshot).unwrap();

    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(wake.id())
    );
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .branch_local_temporal_restore_count,
        1
    );
}

#[test]
fn temporal_snapshot_restore_preserves_ready_wake_frontier_without_rebuild_scan() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(2).unwrap(), ClockTick::new(2))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let ready = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(ready[0].id(), wake.id());
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
    let snapshot_proof = snapshot.reconstructability_proof().unwrap();

    assert_eq!(snapshot_proof.temporal.wake_summary.ready_count(), 1);
    assert_eq!(snapshot_proof.temporal.ready_wake_count, 1);
    assert_eq!(snapshot_proof.temporal.scheduled_wake_count, 0);
    assert_eq!(
        snapshot_proof.temporal.clock_basis.current_tick(),
        ClockTick::new(2)
    );

    runtime
        .retire_temporal_wake(wake.id(), TemporalWakeRetirementReason::Consumed)
        .unwrap();
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);

    runtime.restore_snapshot(&snapshot).unwrap();

    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 0);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_ready_wake_id(),
        Some(wake.id())
    );
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .branch_restore_temporal_rebuild_denial_count,
        1,
        "snapshot restore must consume retained temporal state instead of rebuilding from node conditions"
    );
}
