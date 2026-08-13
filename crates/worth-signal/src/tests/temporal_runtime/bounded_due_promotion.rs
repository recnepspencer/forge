use std::num::NonZeroUsize;

use crate::facade::{
    ClockAdvanceRequest, ClockDomain, ClockTick, SignalGraph, SignalRuntime, TemporalCondition,
};

#[test]
fn bounded_promotion_advances_only_the_indexed_due_batch() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    for _ in 0..3 {
        runtime
            .schedule_temporal_wake(
                TemporalCondition::at_or_after(ClockTick::new(5)),
                ClockTick::new(5),
            )
            .expect("fresh due wake schedules");
    }
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("clock reaches the due frontier");

    let first = runtime
        .promote_due_temporal_wakes_ready_bounded(NonZeroUsize::new(2).unwrap())
        .expect("bounded promotion succeeds");
    assert_eq!(first.promotion().promoted_wake_count(), 2);
    assert!(first.due_work_remaining());
    assert_eq!(
        first
            .promotion()
            .frontier_after()
            .scheduled_frontier_width(),
        1
    );

    let second = runtime
        .promote_due_temporal_wakes_ready_bounded(NonZeroUsize::new(2).unwrap())
        .expect("remaining due work promotes independently");
    assert_eq!(second.promotion().promoted_wake_count(), 1);
    assert!(!second.due_work_remaining());
    assert_eq!(
        second
            .promotion()
            .frontier_after()
            .scheduled_frontier_width(),
        0
    );
}
