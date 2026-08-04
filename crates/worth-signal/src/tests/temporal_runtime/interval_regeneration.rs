use crate::facade::{
    ClockAdvanceRequest, ClockDomain, ClockTick, IntervalCondition, MissedTickPolicy, SignalGraph,
    SignalRuntime, TemporalCondition, TemporalWakeRetirementReason,
};

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
