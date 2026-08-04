use crate::facade::{
    ClockAdvanceRequest, ClockDomain, ClockTick, DiagnosticsLevel, SignalGraph, SignalRuntime,
    TemporalCondition, TemporalPerformanceFailureMode,
};

#[test]
fn ready_promotion_summary_reports_frontier_width_and_broad_scan_denial() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let first = runtime
        .schedule_temporal_wake(TemporalCondition::after(3).unwrap(), ClockTick::new(3))
        .unwrap();
    let second = runtime
        .schedule_temporal_wake(TemporalCondition::after(6).unwrap(), ClockTick::new(6))
        .unwrap();

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(6),
        ))
        .unwrap();

    let summary = runtime
        .promote_due_temporal_wakes_ready_with_summary()
        .unwrap();

    assert_eq!(summary.promoted_wake_count(), 2);
    assert_eq!(summary.ready_wakes().len(), 2);
    assert_eq!(summary.ready_wakes()[0].id(), first.id());
    assert_eq!(summary.ready_wakes()[1].id(), second.id());
    assert_eq!(summary.frontier_before().scheduled_frontier_width(), 2);
    assert_eq!(summary.frontier_before().ready_frontier_width(), 0);
    assert_eq!(summary.frontier_after().scheduled_frontier_width(), 0);
    assert_eq!(summary.frontier_after().ready_frontier_width(), 2);
    assert_eq!(summary.broad_scan_denial_count_delta(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_broad_scan_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_eligibility_lowering_count,
        2
    );
}

#[test]
fn temporal_diagnostics_summary_exposes_artifact_without_tier_deciding_truth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(4).unwrap(), ClockTick::new(4))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(4),
        ))
        .unwrap();
    runtime
        .promote_due_temporal_wakes_ready_with_summary()
        .unwrap();

    let operational = runtime
        .observe()
        .temporal_diagnostics_summary(DiagnosticsLevel::Operational);
    let forensic = runtime
        .observe()
        .temporal_diagnostics_summary(DiagnosticsLevel::Forensic);

    assert_eq!(operational.profile, DiagnosticsLevel::Operational);
    assert_eq!(forensic.profile, DiagnosticsLevel::Forensic);
    assert_eq!(
        operational.with_profile(DiagnosticsLevel::Forensic),
        forensic
    );
    assert_eq!(operational.clock_basis.current_tick(), ClockTick::new(4));
    assert_eq!(operational.wake_summary.ready_count(), 1);
    assert_eq!(operational.frontier.next_ready_wake_id(), Some(wake.id()));
    assert_eq!(operational.artifact.ready_wake_count, 1);
    assert_eq!(operational.artifact.scheduled_wake_count, 0);
    assert_eq!(
        operational.artifact.certification_digest, forensic.artifact.certification_digest,
        "diagnostics richness may change presentation, not temporal truth"
    );
    assert_eq!(operational.telemetry.temporal_broad_scan_denial_count, 1);
    assert!(operational
        .cost_contracts
        .diagnostics_expansion
        .contains("do not re-decide readiness"));
    assert!(operational
        .cost_contracts
        .clock_advance
        .contains("separate frontier operation"));
    assert_eq!(
        operational.cost_contracts.prohibited_failure_modes,
        vec![
            TemporalPerformanceFailureMode::TemporalBroadScan,
            TemporalPerformanceFailureMode::IntervalCatchUpExplosion,
            TemporalPerformanceFailureMode::WakeAllocationChurn,
            TemporalPerformanceFailureMode::BranchRestoreTemporalRebuild,
            TemporalPerformanceFailureMode::RescheduleBreadthLeak,
        ],
        "Milestone A's named temporal performance failure modes must remain machine-visible"
    );
}
