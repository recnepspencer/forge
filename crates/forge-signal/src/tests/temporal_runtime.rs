use crate::facade::*;
use std::sync::atomic::{AtomicU32, Ordering};

#[test]
fn clock_advance_rejects_metadata_only_domains() {
    let runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let err = runtime
        .validate_clock_advance(ClockAdvanceRequest::new(
            ClockDomain::WallClock,
            ClockTick::new(10),
        ))
        .unwrap_err();

    assert!(
        format!("{err}").contains("metadata-only"),
        "wall-clock advances must be rejected as non-authoritative"
    );
}

#[test]
fn clock_advance_rejects_monotonic_regression() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(8),
        ))
        .unwrap();

    let err = runtime
        .validate_clock_advance(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(7),
        ))
        .unwrap_err();

    assert!(
        format!("{err}").contains("clock regression"),
        "authoritative monotonic time must never move backward"
    );
}

#[test]
fn clock_advance_updates_basis_and_ordinal() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let validated = runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(12),
        ))
        .unwrap();

    assert_eq!(validated.previous_tick(), ClockTick::ZERO);
    assert_eq!(validated.next_tick(), ClockTick::new(12));
    assert_eq!(validated.ordinal(), ClockAdvanceOrdinal::new(1));

    let basis = runtime.clock_basis();
    assert_eq!(basis.domain(), ClockDomain::MonotonicExecution);
    assert_eq!(basis.current_tick(), ClockTick::new(12));
    assert_eq!(basis.last_advance_ordinal(), ClockAdvanceOrdinal::new(1));
}

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
fn scheduling_temporal_wake_assigns_monotonic_identity_and_updates_summary() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let first = runtime
        .schedule_temporal_wake(TemporalCondition::after(5).unwrap(), ClockTick::new(5))
        .unwrap();
    let second = runtime
        .schedule_temporal_wake(
            TemporalCondition::stale_after(9).unwrap(),
            ClockTick::new(9),
        )
        .unwrap();

    assert_eq!(first.id(), TemporalWakeId::new(0));
    assert_eq!(second.id(), TemporalWakeId::new(1));
    assert_eq!(first.ordinal(), WakeOrdinal::new(1));
    assert_eq!(second.ordinal(), WakeOrdinal::new(2));

    let summary = runtime.temporal_wake_summary();
    assert_eq!(summary.scheduled_count(), 2);
    assert_eq!(summary.ready_count(), 0);
    assert_eq!(summary.retired_count(), 0);
    assert_eq!(summary.next_wake_id(), TemporalWakeId::new(2));
    assert_eq!(summary.next_wake_ordinal(), WakeOrdinal::new(2));
    assert_eq!(runtime.telemetry().temporal.temporal_wake_count, 2);
    assert_eq!(runtime.telemetry().temporal.scheduled_frontier_width, 2);
    assert_eq!(runtime.telemetry().temporal.wake_allocation_count, 2);
}

#[test]
fn promoting_temporal_wake_to_ready_requires_due_tick() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::throttle(4).unwrap(), ClockTick::new(4))
        .unwrap();

    let err = runtime.promote_temporal_wake_ready(wake.id()).unwrap_err();
    assert!(
        format!("{err}").contains("before due tick"),
        "promotion should deny readiness before the scheduled due tick arrives"
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(4),
        ))
        .unwrap();

    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    assert_eq!(ready.id(), wake.id());
    assert_eq!(ready.scheduled_ordinal(), wake.ordinal());
    assert_eq!(ready.ready_ordinal(), WakeOrdinal::new(2));
    assert_eq!(ready.ready_tick(), ClockTick::new(4));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 1);
    assert_eq!(
        runtime
            .temporal_frontier_snapshot()
            .scheduled_frontier_width(),
        0
    );
    assert_eq!(
        runtime.temporal_frontier_snapshot().ready_frontier_width(),
        1
    );
    assert_eq!(runtime.telemetry().temporal.ready_queue_width, 1);
}

#[test]
fn retiring_ready_temporal_wake_records_reason_and_updates_summary() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(3).unwrap(), ClockTick::new(3))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();

    let retired = runtime
        .retire_temporal_wake(ready.id(), TemporalWakeRetirementReason::Consumed)
        .unwrap();

    assert_eq!(retired.id(), ready.id());
    assert_eq!(retired.active_ordinal(), ready.ready_ordinal());
    assert_eq!(retired.retired_ordinal(), WakeOrdinal::new(3));
    assert_eq!(retired.retired_tick(), ClockTick::new(3));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Consumed);

    let summary = runtime.temporal_wake_summary();
    assert_eq!(summary.scheduled_count(), 0);
    assert_eq!(summary.ready_count(), 0);
    assert_eq!(summary.retired_count(), 1);
    assert_eq!(runtime.telemetry().temporal.retired_wake_count, 1);
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
    let snapshot = runtime.capture_snapshot();

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
fn retiring_unknown_temporal_wake_is_rejected() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let err = runtime
        .retire_temporal_wake(
            TemporalWakeId::new(99),
            TemporalWakeRetirementReason::Cancelled,
        )
        .unwrap_err();

    assert!(
        format!("{err}").contains("unknown temporal wake 99"),
        "runtime should reject retirement for wake ids that were never admitted"
    );
}

#[test]
fn ready_temporal_wake_grants_previous_value_access_and_captures_committed_state() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().build();
    let value_aspect = Aspect::new(0);

    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                        value_aspect,
                        7,
                    )]))
                    .with_output_identity("baseline-value"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(2).unwrap(), ClockTick::new(2))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    let access = runtime
        .grant_temporal_previous_value_access(ready.id())
        .unwrap();
    let reconstructed_reads_before = runtime
        .observe()
        .metrics()
        .storage
        .reconstructed_artifact_read_count;
    let reference = runtime.previous_temporal_value(&access, source).unwrap();

    assert_eq!(access.wake_id(), ready.id());
    assert_eq!(access.branch_id(), runtime.current_branch().id);
    assert_eq!(reference.revision(), PreviousValueRevision::new(1));
    assert_eq!(reference.branch_id(), runtime.current_branch().id);
    assert_eq!(reference.access_wake_id(), ready.id());
    assert_eq!(reference.node(), source);
    assert_eq!(reference.captured_at_tick(), ClockTick::new(2));
    assert_eq!(reference.aspect_version().get(value_aspect), 7);
    assert_eq!(
        reference.output_identity().map(OutputIdentity::as_str),
        Some("baseline-value")
    );
    assert_eq!(
        runtime.telemetry().temporal.previous_value_reference_count,
        1
    );
    assert_eq!(
        runtime
            .observe()
            .metrics()
            .storage
            .reconstructed_artifact_read_count,
        reconstructed_reads_before
    );
}

#[test]
fn previous_value_access_is_rejected_after_wake_retirement() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().build();
    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(1).unwrap(), ClockTick::new(1))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    let access = runtime
        .grant_temporal_previous_value_access(ready.id())
        .unwrap();
    runtime
        .retire_temporal_wake(ready.id(), TemporalWakeRetirementReason::Consumed)
        .unwrap();

    let err = runtime
        .previous_temporal_value(&access, source)
        .unwrap_err();
    assert!(
        format!("{err}").contains("inactive temporal access wake"),
        "retired wakes must not keep previous-value access alive"
    );
}

#[test]
fn previous_value_access_is_branch_scoped() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().build();

    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                        Aspect::new(0),
                        5,
                    )]))
                    .with_output_identity("main-branch"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(1).unwrap(), ClockTick::new(1))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    let access = runtime
        .grant_temporal_previous_value_access(ready.id())
        .unwrap();

    let feature = runtime.create_branch("feature-previous-value").unwrap();
    runtime.switch_branch(feature).unwrap();

    let err = runtime
        .previous_temporal_value(&access, source)
        .unwrap_err();
    assert!(
        format!("{err}").contains("belongs to branch"),
        "previous-value capabilities must not cross branch boundaries"
    );
}

#[test]
fn previous_value_access_is_rejected_after_restore_epoch_changes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().build();

    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                        Aspect::new(0),
                        5,
                    )]))
                    .with_output_identity("restore-epoch"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(1).unwrap(), ClockTick::new(1))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    let snapshot = runtime.capture_snapshot();
    let access = runtime
        .grant_temporal_previous_value_access(ready.id())
        .unwrap();

    runtime.restore_snapshot(&snapshot).unwrap();

    let err = runtime
        .previous_temporal_value(&access, source)
        .unwrap_err();
    assert!(
        format!("{err}").contains("stale restore epoch"),
        "previous-value capabilities minted before restore must not revive on the restored branch"
    );
}

#[test]
fn previous_value_reads_committed_branch_truth_after_failed_transaction() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().build();
    let value_aspect = Aspect::new(1);

    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                        value_aspect,
                        11,
                    )]))
                    .with_output_identity("committed-baseline"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let _ = runtime.transaction(&mut (), |tx| {
        tx.read(source, &|view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                    value_aspect,
                    99,
                )]))
                .with_output_identity("staged-only"),
            ))
        })?;
        Err::<(), SignalError>(SignalError::invalid_input("force rollback"))
    });

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(4).unwrap(), ClockTick::new(4))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(4),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    let access = runtime
        .grant_temporal_previous_value_access(ready.id())
        .unwrap();
    let reference = runtime.previous_temporal_value(&access, source).unwrap();

    assert_eq!(reference.aspect_version().get(value_aspect), 11);
    assert_eq!(
        reference.output_identity().map(OutputIdentity::as_str),
        Some("committed-baseline")
    );
}

#[test]
fn due_temporal_wake_batch_promotion_is_canonical_by_due_tick_then_ordinal() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let later = runtime
        .schedule_temporal_wake(TemporalCondition::after(9).unwrap(), ClockTick::new(9))
        .unwrap();
    let first_due = runtime
        .schedule_temporal_wake(TemporalCondition::after(3).unwrap(), ClockTick::new(3))
        .unwrap();
    let second_due_same_tick = runtime
        .schedule_temporal_wake(TemporalCondition::throttle(3).unwrap(), ClockTick::new(3))
        .unwrap();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(9),
        ))
        .unwrap();

    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    let promoted_ids = promoted.iter().map(|wake| wake.id()).collect::<Vec<_>>();
    assert_eq!(
        promoted_ids,
        vec![first_due.id(), second_due_same_tick.id(), later.id()]
    );
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_eligibility_lowering_count,
        3
    );
    assert_eq!(
        runtime
            .temporal_frontier_snapshot()
            .scheduled_frontier_width(),
        0
    );
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_ready_wake_id(),
        Some(first_due.id())
    );
}

#[test]
fn due_temporal_wake_batch_promotion_leaves_future_frontier_entries_scheduled() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let ready_now = runtime
        .schedule_temporal_wake(TemporalCondition::after(2).unwrap(), ClockTick::new(2))
        .unwrap();
    let future = runtime
        .schedule_temporal_wake(TemporalCondition::after(5).unwrap(), ClockTick::new(5))
        .unwrap();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();

    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].id(), ready_now.id());
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(future.id())
    );
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(5))
    );
}

#[test]
fn retiring_ready_wake_updates_frontier_indexes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let first = runtime
        .schedule_temporal_wake(TemporalCondition::after(1).unwrap(), ClockTick::new(1))
        .unwrap();
    let second = runtime
        .schedule_temporal_wake(TemporalCondition::after(1).unwrap(), ClockTick::new(1))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted[0].id(), first.id());
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_ready_wake_id(),
        Some(first.id())
    );

    runtime
        .retire_temporal_wake(first.id(), TemporalWakeRetirementReason::Consumed)
        .unwrap();

    assert_eq!(
        runtime.temporal_frontier_snapshot().next_ready_wake_id(),
        Some(second.id())
    );
}

#[test]
fn rescheduling_scheduled_wake_supersedes_old_wake_and_updates_frontier() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(5).unwrap(), ClockTick::new(5))
        .unwrap();

    let reschedule = runtime
        .reschedule_temporal_wake(wake.id(), ClockTick::new(9))
        .unwrap();

    assert_eq!(
        reschedule.retired().reason(),
        TemporalWakeRetirementReason::Superseded
    );
    assert_eq!(reschedule.retired().id(), wake.id());
    assert_eq!(reschedule.scheduled().due_tick(), ClockTick::new(9));
    assert_ne!(reschedule.scheduled().id(), wake.id());
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(9))
    );
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 1);
}

#[test]
fn interval_regeneration_collapse_to_one_skips_missed_boundaries_into_future_successor() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let interval = IntervalCondition::try_new(4)
        .unwrap()
        .with_missed_tick_policy(MissedTickPolicy::CollapseToOne);
    let _wake = runtime
        .schedule_temporal_wake(TemporalCondition::interval(interval), ClockTick::new(2))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .unwrap();
    let ready = runtime
        .promote_due_temporal_wakes_ready()
        .unwrap()
        .remove(0);

    let regeneration = runtime.regenerate_interval_wake(ready.id()).unwrap();

    assert_eq!(
        regeneration.retired().reason(),
        TemporalWakeRetirementReason::Consumed
    );
    assert_eq!(regeneration.suppressed_interval_count(), 2);
    assert_eq!(regeneration.scheduled().due_tick(), ClockTick::new(14));
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(14))
    );
}

#[test]
fn interval_regeneration_skip_to_latest_materializes_one_latest_immediate_successor() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let interval = IntervalCondition::try_new(4)
        .unwrap()
        .with_missed_tick_policy(MissedTickPolicy::SkipToLatest);
    let _wake = runtime
        .schedule_temporal_wake(TemporalCondition::interval(interval), ClockTick::new(2))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .unwrap();
    let ready = runtime
        .promote_due_temporal_wakes_ready()
        .unwrap()
        .remove(0);

    let regeneration = runtime.regenerate_interval_wake(ready.id()).unwrap();

    assert_eq!(regeneration.suppressed_interval_count(), 1);
    assert_eq!(regeneration.scheduled().due_tick(), ClockTick::new(10));
    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].due_tick(), ClockTick::new(10));
}

#[test]
fn interval_regeneration_catch_up_all_requires_explicit_repeated_catch_up_steps() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let interval = IntervalCondition::try_new(4)
        .unwrap()
        .with_missed_tick_policy(MissedTickPolicy::CatchUpAll);
    let _wake = runtime
        .schedule_temporal_wake(TemporalCondition::interval(interval), ClockTick::new(2))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .unwrap();
    let first_ready = runtime
        .promote_due_temporal_wakes_ready()
        .unwrap()
        .remove(0);

    let first_regeneration = runtime.regenerate_interval_wake(first_ready.id()).unwrap();
    assert_eq!(first_regeneration.suppressed_interval_count(), 0);
    assert_eq!(first_regeneration.scheduled().due_tick(), ClockTick::new(6));

    let second_ready = runtime
        .promote_due_temporal_wakes_ready()
        .unwrap()
        .remove(0);
    assert_eq!(second_ready.due_tick(), ClockTick::new(6));

    let second_regeneration = runtime.regenerate_interval_wake(second_ready.id()).unwrap();
    assert_eq!(
        second_regeneration.scheduled().due_tick(),
        ClockTick::new(10)
    );

    let third_ready = runtime
        .promote_due_temporal_wakes_ready()
        .unwrap()
        .remove(0);
    assert_eq!(third_ready.due_tick(), ClockTick::new(10));
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 2);
}

#[test]
fn owned_temporal_wake_lifecycle_preserves_owner_across_schedule_ready_retire_and_reschedule() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().build();

    let wake = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(node),
            TemporalCondition::after(5).unwrap(),
            ClockTick::new(5),
        )
        .unwrap();
    assert_eq!(wake.owner(), TemporalWakeOwner::Node(node));

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    assert_eq!(ready.owner(), TemporalWakeOwner::Node(node));

    let reschedule = runtime
        .reschedule_temporal_wake(ready.id(), ClockTick::new(8))
        .unwrap();
    assert_eq!(reschedule.retired().owner(), TemporalWakeOwner::Node(node));
    assert_eq!(
        reschedule.scheduled().owner(),
        TemporalWakeOwner::Node(node)
    );
}

#[test]
fn retiring_temporal_wakes_for_owner_is_selective_and_updates_frontiers() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner_a = runtime.graph_mut().node().build();
    let owner_b = runtime.graph_mut().node().build();

    let wake_a_due_2 = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner_a),
            TemporalCondition::after(2).unwrap(),
            ClockTick::new(2),
        )
        .unwrap();
    let _wake_a_due_4 = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner_a),
            TemporalCondition::after(4).unwrap(),
            ClockTick::new(4),
        )
        .unwrap();
    let wake_b_due_3 = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner_b),
            TemporalCondition::after(3).unwrap(),
            ClockTick::new(3),
        )
        .unwrap();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].id(), wake_a_due_2.id());

    let retired = runtime
        .retire_temporal_wakes_for_owner(
            TemporalWakeOwner::Node(owner_a),
            TemporalWakeRetirementReason::Disposed,
        )
        .unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner_a));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Disposed);
    assert_eq!(retired.retired().len(), 2);
    assert!(retired
        .retired()
        .iter()
        .all(|wake| wake.owner() == TemporalWakeOwner::Node(owner_a)));
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(wake_b_due_3.id())
    );
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
}

#[test]
fn default_manual_temporal_wake_owner_is_explicit() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(3).unwrap(), ClockTick::new(3))
        .unwrap();

    assert_eq!(wake.owner(), TemporalWakeOwner::Manual);
}

#[test]
fn scheduling_owned_temporal_wake_rejects_stale_or_forged_node_owner() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().build();
    runtime.graph_mut().unregister_node(node).unwrap();

    let stale_err = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(node),
            TemporalCondition::after(3).unwrap(),
            ClockTick::new(3),
        )
        .unwrap_err();
    assert!(
        format!("{stale_err}").contains("non-live node owner"),
        "stale node handles must not be allowed to mint node-owned temporal wakes"
    );

    let forged_err = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(NodeId::new(999_999, 0)),
            TemporalCondition::after(4).unwrap(),
            ClockTick::new(4),
        )
        .unwrap_err();
    assert!(
        format!("{forged_err}").contains("non-live node owner"),
        "forged node handles must not be allowed to mint node-owned temporal wakes"
    );
}

#[test]
fn due_promotion_retires_disposed_node_owned_wakes_instead_of_promoting_them() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();

    let wake = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(2).unwrap(),
            ClockTick::new(2),
        )
        .unwrap();
    runtime.graph_mut().unregister_node(owner).unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();

    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();

    assert!(promoted.is_empty());
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    let retired = runtime
        .retire_temporal_wake(wake.id(), TemporalWakeRetirementReason::Disposed)
        .unwrap_err();
    assert!(
        format!("{retired}").contains("unknown temporal wake"),
        "disposed due wakes should already be retired by the batch promotion path"
    );
}

#[test]
fn ready_wake_cannot_grant_previous_value_access_after_owner_unregistration() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();

    let wake = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(1).unwrap(),
            ClockTick::new(1),
        )
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let ready = runtime.promote_temporal_wake_ready(wake.id()).unwrap();
    runtime.graph_mut().unregister_node(owner).unwrap();

    let err = runtime
        .grant_temporal_previous_value_access(ready.id())
        .unwrap_err();

    assert!(
        format!("{err}").contains("non-ready temporal wake"),
        "runtime-owned node disposal should retire the ready wake before previous-value access can be re-granted"
    );
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
}

#[test]
fn runtime_unregister_node_structurally_retires_owned_temporal_wakes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();
    let other_owner = runtime.graph_mut().node().build();

    let owned_scheduled = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(4).unwrap(),
            ClockTick::new(4),
        )
        .unwrap();
    let owned_ready = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(1).unwrap(),
            ClockTick::new(1),
        )
        .unwrap();
    let surviving_other = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(other_owner),
            TemporalCondition::after(3).unwrap(),
            ClockTick::new(3),
        )
        .unwrap();
    let surviving_manual = runtime
        .schedule_temporal_wake(TemporalCondition::after(5).unwrap(), ClockTick::new(5))
        .unwrap();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].id(), owned_ready.id());

    let retired = runtime.unregister_node(owner).unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Disposed);
    assert_eq!(retired.retired().len(), 2);
    assert!(retired
        .retired()
        .iter()
        .all(|wake| wake.owner() == TemporalWakeOwner::Node(owner)));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 2);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 2);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(surviving_other.id())
    );
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(3))
    );
    let manual_due = runtime
        .retire_temporal_wake(
            surviving_manual.id(),
            TemporalWakeRetirementReason::Cancelled,
        )
        .unwrap();
    assert_eq!(manual_due.owner(), TemporalWakeOwner::Manual);
    let other_due = runtime
        .retire_temporal_wake(
            surviving_other.id(),
            TemporalWakeRetirementReason::Cancelled,
        )
        .unwrap();
    assert_eq!(other_due.owner(), TemporalWakeOwner::Node(other_owner));
    let owned_err = runtime
        .retire_temporal_wake(
            owned_scheduled.id(),
            TemporalWakeRetirementReason::Cancelled,
        )
        .unwrap_err();
    assert!(format!("{owned_err}").contains("unknown temporal wake"));
}

#[test]
fn graph_mut_unregister_node_uses_runtime_temporal_disposal_protocol() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();

    runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(2).unwrap(),
            ClockTick::new(2),
        )
        .unwrap();

    let retired = runtime.graph_mut().unregister_node(owner).unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Disposed);
    assert_eq!(retired.retired().len(), 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
}

#[test]
fn replacing_node_checkpoint_image_supersedes_owned_temporal_wakes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();
    let other_owner = runtime.graph_mut().node().build();

    let scheduled = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(4).unwrap(),
            ClockTick::new(4),
        )
        .unwrap();
    let ready = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(1).unwrap(),
            ClockTick::new(1),
        )
        .unwrap();
    let surviving_other = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(other_owner),
            TemporalCondition::after(3).unwrap(),
            ClockTick::new(3),
        )
        .unwrap();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .unwrap();
    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].id(), ready.id());

    let image = runtime.graph().node_checkpoint_image(owner).unwrap();
    let retired = runtime
        .replace_node_from_checkpoint_image(owner, image)
        .unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Superseded);
    assert_eq!(retired.retired().len(), 2);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 2);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(surviving_other.id())
    );

    let scheduled_err = runtime
        .retire_temporal_wake(scheduled.id(), TemporalWakeRetirementReason::Cancelled)
        .unwrap_err();
    assert!(format!("{scheduled_err}").contains("unknown temporal wake"));
}

#[test]
fn rewriting_node_evaluation_config_supersedes_owned_temporal_wakes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();
    let manual = runtime
        .schedule_temporal_wake(TemporalCondition::after(9).unwrap(), ClockTick::new(9))
        .unwrap();

    runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(5).unwrap(),
            ClockTick::new(5),
        )
        .unwrap();

    let mut eval_config = runtime.graph().node_eval_config(owner).unwrap().clone();
    eval_config.condition = EvaluationCondition::Temporal(TemporalCondition::throttle(7).unwrap());
    let retired = runtime
        .replace_node_evaluation_config(owner, eval_config.clone())
        .unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Superseded);
    assert_eq!(retired.retired().len(), 1);
    assert_eq!(
        runtime.graph().node_eval_config(owner).unwrap(),
        &eval_config
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_wake_id(),
        Some(manual.id())
    );
}

#[test]
fn graph_mut_replace_node_checkpoint_image_uses_runtime_temporal_supersession_protocol() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();

    runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(2).unwrap(),
            ClockTick::new(2),
        )
        .unwrap();

    let image = runtime.graph().node_checkpoint_image(owner).unwrap();
    let retired = runtime
        .graph_mut()
        .replace_node_from_checkpoint_image(owner, image)
        .unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Superseded);
    assert_eq!(retired.retired().len(), 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
}

#[test]
fn graph_mut_replace_node_evaluation_config_uses_runtime_temporal_supersession_protocol() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let owner = runtime.graph_mut().node().build();

    runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(owner),
            TemporalCondition::after(2).unwrap(),
            ClockTick::new(2),
        )
        .unwrap();

    let mut eval_config = runtime.graph().node_eval_config(owner).unwrap().clone();
    eval_config.condition =
        EvaluationCondition::Temporal(TemporalCondition::stale_after(6).unwrap());
    let retired = runtime
        .graph_mut()
        .replace_node_evaluation_config(owner, eval_config.clone())
        .unwrap();

    assert_eq!(retired.owner(), TemporalWakeOwner::Node(owner));
    assert_eq!(retired.reason(), TemporalWakeRetirementReason::Superseded);
    assert_eq!(retired.retired().len(), 1);
    assert_eq!(
        runtime.graph().node_eval_config(owner).unwrap(),
        &eval_config
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
}

#[test]
fn runtime_execute_prepared_plan_uses_clock_basis_for_at_or_after_without_temporal_resolver() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().at_or_after(5).build();
    let plan = runtime
        .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
        .unwrap();
    let calls = AtomicU32::new(0);
    let aspect = Aspect::new(0);

    let before = runtime
        .execute_prepared_plan(&plan, &(), &|_ctx| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(aspect, 1)]),
            ))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(before.tasks_executed, 0);
    assert_eq!(
        runtime.graph().get_state(node).unwrap(),
        NodeState::MaybeStale
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();

    let after = runtime
        .execute_prepared_plan(&plan, &(), &|_ctx| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(aspect, 1)]),
            ))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(after.tasks_executed, 1);
    assert_eq!(runtime.graph().get_state(node).unwrap(), NodeState::Clean);
    assert_eq!(
        runtime
            .graph()
            .node_aspect_version(node)
            .unwrap()
            .get(aspect),
        1
    );
}

#[test]
fn runtime_target_execution_uses_clock_basis_for_at_or_after_without_temporal_resolver() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().at_or_after(7).build();
    let calls = AtomicU32::new(0);
    let aspect = Aspect::new(1);

    let before = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 3)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(before.tasks_executed, 0);
    assert_eq!(
        runtime.graph().get_state(node).unwrap(),
        NodeState::MaybeStale
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(7),
        ))
        .unwrap();

    let after = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 3)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(after.tasks_executed, 1);
    assert_eq!(runtime.graph().get_state(node).unwrap(), NodeState::Clean);
    assert_eq!(
        runtime
            .graph()
            .node_aspect_version(node)
            .unwrap()
            .get(aspect),
        3
    );
}
