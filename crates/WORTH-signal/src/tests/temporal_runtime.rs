use crate::data::telemetry::TemporalTelemetry;
use crate::facade::*;
use std::sync::atomic::{AtomicU32, Ordering};

type TestRuntime = SignalRuntime<(), (), (), (), ()>;

#[derive(Debug, Clone)]
struct TemporalPhase9MixedBranchState {
    branch_id: SignalBranchId,
    restored_snapshot_id: SignalSnapshotId,
    head_snapshot_before_restore: Option<SignalSnapshotId>,
    head_snapshot_after_restore: Option<SignalSnapshotId>,
    replay_before_restore: ReplaySlice,
    replay_after_snapshot_drift: ReplaySlice,
    replay_after_restore: ReplaySlice,
    temporal_telemetry_after_restore: TemporalTelemetry,
    reconstructability_before_restore: TemporalReconstructabilityArtifact,
    reconstructability_after_snapshot_drift: TemporalReconstructabilityArtifact,
    reconstructability_after_restore: TemporalReconstructabilityArtifact,
    restore_parity: TemporalReplayParityReport,
}

#[derive(Debug, Clone)]
struct TemporalPhase9MixedWorkloadOutcome {
    bundle: TemporalCertificationBundle,
    eligibility_artifact: TemporalReconstructabilityArtifact,
    eligibility_parity: TemporalReplayParityReport,
    boundedness_artifact: TemporalReconstructabilityArtifact,
    previous_value_artifact: TemporalReconstructabilityArtifact,
    feature: TemporalPhase9MixedBranchState,
    sibling: TemporalPhase9MixedBranchState,
    diagnostics_operational: TemporalDiagnosticsSummary,
    diagnostics_forensic: TemporalDiagnosticsSummary,
    temporal_telemetry: TemporalTelemetry,
}

fn exercise_temporal_phase9_hostile_suffix_on_active_branch(
    runtime: &mut TestRuntime,
) -> TemporalPhase9MixedBranchState {
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    let ready = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert!(
        ready.len() >= 3,
        "hostile branch suffix should admit after, throttle, and interval wakes at tick 5"
    );

    let snapshot = runtime.capture_snapshot();
    let restored_snapshot_id = snapshot.meta().snapshot_id;
    let reconstructability_before_restore = snapshot
        .reconstructability
        .clone()
        .expect("temporal hostile branch snapshot should carry reconstructability");
    let head_snapshot_before_restore = runtime
        .observe()
        .branch_head_snapshot_id(runtime.observe().current_branch().id);
    let replay_before_restore = runtime
        .observe()
        .replay_for_branch(runtime.observe().current_branch().id);

    for wake in ready.iter().take(2) {
        runtime
            .retire_temporal_wake(wake.id(), TemporalWakeRetirementReason::Consumed)
            .unwrap();
    }
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(9),
        ))
        .unwrap();
    let reconstructability_after_snapshot_drift = runtime
        .capture_snapshot()
        .reconstructability
        .expect("post-drift temporal snapshot should carry reconstructability")
        .temporal;
    let replay_after_snapshot_drift = runtime
        .observe()
        .replay_for_branch(runtime.observe().current_branch().id);

    runtime.restore_snapshot(&snapshot).unwrap();
    let head_snapshot_after_restore = runtime
        .observe()
        .branch_head_snapshot_id(runtime.observe().current_branch().id);
    let replay_after_restore = runtime
        .observe()
        .replay_for_branch(runtime.observe().current_branch().id);
    let reconstructability_after_restore = runtime
        .capture_snapshot()
        .reconstructability
        .expect("restored temporal snapshot should carry reconstructability")
        .temporal;
    let restore_parity = temporal_replay_parity_report(
        &reconstructability_before_restore.temporal,
        &reconstructability_after_restore,
    );

    TemporalPhase9MixedBranchState {
        branch_id: runtime.observe().current_branch().id,
        restored_snapshot_id,
        head_snapshot_before_restore,
        head_snapshot_after_restore,
        replay_before_restore,
        replay_after_snapshot_drift,
        replay_after_restore,
        temporal_telemetry_after_restore: runtime.telemetry().temporal,
        reconstructability_before_restore: reconstructability_before_restore.temporal,
        reconstructability_after_snapshot_drift,
        reconstructability_after_restore,
        restore_parity,
    }
}

fn temporal_phase9_mixed_workload() -> TemporalPhase9MixedWorkloadOutcome {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let output_aspect = Aspect::new(3);
    let source = runtime.graph_mut().node().build();
    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                        output_aspect,
                        41,
                    )]))
                    .with_output_identity("phase9-temporal-previous"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let after = runtime.graph_mut().node().after(3).unwrap().build();
    let debounce = runtime.graph_mut().node().debounce(5).unwrap().build();
    let throttle = runtime.graph_mut().node().throttle(5).unwrap().build();
    let stale_after = runtime.graph_mut().node().stale_after(7).unwrap().build();
    let interval = runtime.graph_mut().node().interval(5).unwrap().build();
    let temporal_nodes = [after, debounce, throttle, stale_after, interval];

    let initial = runtime
        .transaction(&mut (), |tx| {
            for node in temporal_nodes {
                tx.evaluate_with_plan(
                    node,
                    &|_ctx| {
                        Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                            AspectVersion::from_updates([(output_aspect, 1)]),
                        ))
                    },
                    EvaluationRequestMode::Default,
                )?;
            }
            Ok(())
        })
        .unwrap();
    assert_eq!(initial.temporal_summary.deferred_count(), 5);
    assert_eq!(initial.temporal_summary.resolver_fallback_count(), 0);
    assert_eq!(initial.temporal_evidence.scheduled_wakes.len(), 5);
    assert_eq!(initial.temporal_evidence.eligibility_facts.len(), 5);
    let eligibility_artifact = initial.reconstructability.temporal.clone();
    let eligibility_parity = runtime
        .temporal_replay_parity_report(&initial.reconstructability, &initial.reconstructability);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let burst = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                debounce,
                &|_ctx| {
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(output_aspect, 2)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            tx.evaluate_with_plan(
                throttle,
                &|_ctx| {
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(output_aspect, 3)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();
    assert_eq!(burst.temporal_evidence.rescheduled_wakes.len(), 1);
    assert_eq!(burst.temporal_evidence.reused_wakes.len(), 1);
    assert_eq!(
        runtime.temporal_wake_summary().scheduled_count(),
        5,
        "debounce supersession and throttle reuse must not widen the live frontier"
    );

    let main = runtime.current_branch();
    let feature = runtime.create_branch("phase9-temporal-feature").unwrap();
    let sibling = runtime.create_branch("phase9-temporal-sibling").unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature = exercise_temporal_phase9_hostile_suffix_on_active_branch(&mut runtime);

    runtime.switch_branch(sibling.clone()).unwrap();
    let sibling = exercise_temporal_phase9_hostile_suffix_on_active_branch(&mut runtime);

    runtime.switch_branch(main).unwrap();
    let collapse = IntervalCondition::try_new(5)
        .unwrap()
        .with_missed_tick_policy(MissedTickPolicy::CollapseToOne);
    let skip = IntervalCondition::try_new(5)
        .unwrap()
        .with_missed_tick_policy(MissedTickPolicy::SkipToLatest);
    let catch_up = IntervalCondition::try_new(5)
        .unwrap()
        .with_missed_tick_policy(MissedTickPolicy::CatchUpAll);
    let bounded_wakes = [
        runtime
            .schedule_temporal_wake(TemporalCondition::interval(collapse), ClockTick::new(10))
            .unwrap(),
        runtime
            .schedule_temporal_wake(TemporalCondition::interval(skip), ClockTick::new(10))
            .unwrap(),
        runtime
            .schedule_temporal_wake(TemporalCondition::interval(catch_up), ClockTick::new(10))
            .unwrap(),
    ];
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1_010),
        ))
        .unwrap();
    runtime.promote_due_temporal_wakes_ready().unwrap();
    let mut wake_boundedness_evidence = TemporalTransactionEvidence::default();
    wake_boundedness_evidence.clock_basis = runtime.clock_basis();
    for wake in bounded_wakes {
        let regeneration = runtime.regenerate_interval_wake(wake.id()).unwrap();
        wake_boundedness_evidence
            .interval_regenerations
            .push(regeneration);
    }
    let boundedness_artifact = TemporalReconstructabilityArtifact::from_evidence(
        runtime.temporal_wake_summary(),
        &wake_boundedness_evidence,
    );

    let previous_wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(1).unwrap(), ClockTick::new(1_011))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1_011),
        ))
        .unwrap();
    runtime.promote_due_temporal_wakes_ready().unwrap();
    let previous_access = runtime
        .grant_temporal_previous_value_access(previous_wake.id())
        .unwrap();
    let previous_reference = runtime
        .previous_temporal_value(&previous_access, source)
        .unwrap();
    let mut previous_evidence = TemporalTransactionEvidence::default();
    previous_evidence.clock_basis = runtime.clock_basis();
    previous_evidence
        .previous_value_references
        .push(previous_reference);
    let previous_value_artifact = TemporalReconstructabilityArtifact::from_evidence(
        runtime.temporal_wake_summary(),
        &previous_evidence,
    );

    let bundle = runtime
        .temporal_certification_builder()
        .with_temporal_eligibility_replay_parity(
            eligibility_artifact.clone(),
            eligibility_parity.clone(),
        )
        .unwrap()
        .with_temporal_branch_restore_equivalence(
            feature.reconstructability_after_restore.clone(),
            feature.restore_parity.clone(),
        )
        .unwrap()
        .with_temporal_wake_boundedness(boundedness_artifact.clone())
        .unwrap()
        .with_previous_value_time_gated_equivalence(previous_value_artifact.clone())
        .unwrap()
        .build()
        .unwrap();

    let diagnostics_operational = runtime
        .observe()
        .temporal_diagnostics_summary(DiagnosticsLevel::Operational);
    let diagnostics_forensic = runtime
        .observe()
        .temporal_diagnostics_summary(DiagnosticsLevel::Forensic);

    TemporalPhase9MixedWorkloadOutcome {
        bundle,
        eligibility_artifact,
        eligibility_parity,
        boundedness_artifact,
        previous_value_artifact,
        feature,
        sibling,
        diagnostics_operational,
        diagnostics_forensic,
        temporal_telemetry: runtime.telemetry().temporal,
    }
}

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
fn clock_advance_summary_is_cost_honest_and_does_not_promote_wakes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(5).unwrap(), ClockTick::new(5))
        .unwrap();

    let summary = runtime
        .advance_clock_with_summary(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();

    assert_eq!(summary.validated_advance().previous_tick(), ClockTick::ZERO);
    assert_eq!(summary.validated_advance().next_tick(), ClockTick::new(5));
    assert_eq!(summary.promoted_wake_count(), 0);
    assert!(
        summary.ready_selection_deferred(),
        "clock advance must not hide ready promotion behind the clock input surface"
    );
    assert_eq!(
        summary.frontier_before().next_due_wake_id(),
        Some(wake.id())
    );
    assert_eq!(summary.frontier_after().next_due_wake_id(), Some(wake.id()));
    assert_eq!(summary.frontier_after().scheduled_frontier_width(), 1);
    assert_eq!(summary.frontier_after().ready_frontier_width(), 0);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_broad_scan_denial_count,
        0,
        "clock advance should not claim or perform ready-frontier selection work"
    );
}

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
    let snapshot = runtime.capture_snapshot();
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
fn scheduling_owned_temporal_wake_rejects_stale_or_WORTHd_node_owner() {
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

    let WORTHd_err = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(NodeId::new(999_999, 0)),
            TemporalCondition::after(4).unwrap(),
            ClockTick::new(4),
        )
        .unwrap_err();
    assert!(
        format!("{WORTHd_err}").contains("non-live node owner"),
        "WORTHd node handles must not be allowed to mint node-owned temporal wakes"
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

#[test]
fn node_owned_after_declaration_schedules_defers_admits_and_consumes_one_wake() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().after(5).unwrap().build();
    let aspect = Aspect::new(2);
    let calls = AtomicU32::new(0);

    let before = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 1)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(before.tasks_deferred_by_condition, 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    let deferred = before.stages[0].task_records[0]
        .temporal_eligibility
        .as_ref()
        .expect("deferred node-owned temporal condition should carry proof");
    assert_eq!(
        deferred.authority(),
        TemporalEligibilityAuthority::RuntimeScheduledWake
    );
    assert_eq!(before.temporal_summary.deferred_count(), 1);
    assert_eq!(before.temporal_summary.runtime_scheduled_wake_count(), 1);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    let after = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 1)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(after.tasks_executed, 1);
    let ready = after.stages[0].task_records[0]
        .temporal_eligibility
        .as_ref()
        .expect("ready node-owned temporal condition should carry proof");
    assert_eq!(
        ready.authority(),
        TemporalEligibilityAuthority::RuntimeScheduledWake
    );
    assert!(ready.ready_by_time());
    assert!(matches!(
        ready,
        LoweredTemporalEligibility::Ready(ready) if ready.wake_id().is_some()
    ));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
}

#[test]
fn sealed_temporal_policy_family_uses_node_owned_runtime_wakes_without_resolver() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let debounce = runtime.graph_mut().node().debounce(3).unwrap().build();
    let throttle = runtime.graph_mut().node().throttle(3).unwrap().build();
    let stale_after = runtime.graph_mut().node().stale_after(3).unwrap().build();
    let interval = runtime.graph_mut().node().interval(3).unwrap().build();
    let nodes = [debounce, throttle, stale_after, interval];
    let calls = AtomicU32::new(0);

    let before = runtime
        .targets(nodes)
        .run(&(), &|_ctx| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(Aspect::new(3), 1)]),
            ))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(before.tasks_deferred_by_condition, 4);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 4);
    assert_eq!(before.temporal_summary.deferred_count(), 4);
    assert_eq!(before.temporal_summary.resolver_fallback_count(), 0);
    assert_eq!(before.temporal_summary.runtime_scheduled_wake_count(), 4);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .unwrap();
    let after = runtime
        .targets(nodes)
        .run(&(), &|_ctx| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(Aspect::new(3), 2)]),
            ))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 4);
    assert_eq!(after.tasks_executed, 4);
    assert_eq!(after.temporal_summary.ready_count(), 4);
    assert_eq!(after.temporal_summary.resolver_fallback_count(), 0);
    assert_eq!(after.temporal_summary.runtime_scheduled_wake_count(), 4);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 4);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(6))
    );
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .interval_wake_regeneration_count,
        1
    );
}

#[test]
fn debounce_burst_supersedes_owned_wake_and_waits_for_new_quiet_period() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().debounce(5).unwrap().build();
    let aspect = Aspect::new(7);
    let calls = AtomicU32::new(0);

    runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 1)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(5))
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 2)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(7))
    );
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 1);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    assert!(runtime
        .promote_due_temporal_wakes_ready()
        .unwrap()
        .is_empty());
    assert_eq!(calls.load(Ordering::Relaxed), 0);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(7),
        ))
        .unwrap();
    let admitted = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(aspect, 4)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();
    assert_eq!(admitted.tasks_executed, 1);
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[test]
fn debounce_admission_summary_records_each_burst_supersession_without_extra_live_wakes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().debounce(5).unwrap().build();

    let first = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(first.scheduled_count(), 1);
    assert_eq!(first.rescheduled_count(), 0);
    assert_eq!(first.reused_count(), 0);
    assert_eq!(first.scheduled()[0].due_tick(), ClockTick::new(5));

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let second = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(second.scheduled_count(), 0);
    assert_eq!(second.rescheduled_count(), 1);
    assert_eq!(second.reused_count(), 0);
    assert_eq!(
        second.rescheduled()[0].retired().reason(),
        TemporalWakeRetirementReason::Superseded
    );
    assert_eq!(
        second.rescheduled()[0].scheduled().due_tick(),
        ClockTick::new(7)
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(4),
        ))
        .unwrap();
    let third = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(third.scheduled_count(), 0);
    assert_eq!(third.rescheduled_count(), 1);
    assert_eq!(third.reused_count(), 0);
    assert_eq!(
        third.rescheduled()[0].scheduled().due_tick(),
        ClockTick::new(9)
    );
    assert_eq!(third.total_decision_count(), 1);

    let wake_summary = runtime.temporal_wake_summary();
    assert_eq!(
        wake_summary.scheduled_count(),
        1,
        "debounce burst coalescing should keep one live scheduled wake"
    );
    assert_eq!(wake_summary.retired_count(), 2);
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 2);
    assert_eq!(
        runtime.telemetry().temporal.scheduled_frontier_width,
        1,
        "rescheduling one owner must not widen the active frontier"
    );
}

#[test]
fn legacy_temporal_wake_admission_return_does_not_treat_debounce_reschedule_as_new_wake() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().debounce(5).unwrap().build();

    let first = runtime.admit_node_temporal_wake(node).unwrap();
    assert!(first.is_some());

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let rescheduled = runtime.admit_node_temporal_wake(node).unwrap();

    assert!(
        rescheduled.is_none(),
        "single-wake admission convenience reports fresh schedules; summaries carry reschedule evidence"
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(7))
    );
}

#[test]
fn throttle_burst_reuses_original_window_without_reschedule() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().throttle(5).unwrap().build();
    let calls = AtomicU32::new(0);

    runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(Aspect::new(0), 1)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(Aspect::new(0), 2)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(5))
    );
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 0);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    let admitted = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(Aspect::new(0), 3)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(admitted.tasks_executed, 1);
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[test]
fn throttle_admission_summary_records_reuse_without_window_drift() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().throttle(5).unwrap().build();

    let first = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(first.scheduled_count(), 1);
    assert_eq!(first.scheduled()[0].due_tick(), ClockTick::new(5));

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let second = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(second.scheduled_count(), 0);
    assert_eq!(second.rescheduled_count(), 0);
    assert_eq!(second.reused_count(), 1);
    assert_eq!(second.reused()[0].original_due_tick(), ClockTick::new(5));
    assert_eq!(second.reused()[0].attempted_due_tick(), ClockTick::new(7));
    assert_eq!(second.reused()[0].decision_tick(), ClockTick::new(2));

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(4),
        ))
        .unwrap();
    let third = runtime.admit_node_temporal_wake_with_summary(node).unwrap();
    assert_eq!(third.scheduled_count(), 0);
    assert_eq!(third.rescheduled_count(), 0);
    assert_eq!(third.reused_count(), 1);
    assert_eq!(third.reused()[0].original_due_tick(), ClockTick::new(5));
    assert_eq!(third.reused()[0].attempted_due_tick(), ClockTick::new(9));
    assert_eq!(third.reused()[0].decision_tick(), ClockTick::new(4));

    let frontier = runtime.temporal_frontier_snapshot();
    assert_eq!(frontier.next_due_tick(), Some(ClockTick::new(5)));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 0);
    assert_eq!(runtime.telemetry().temporal.wake_reuse_count, 2);
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 0);
}

#[test]
fn stale_ready_owned_wake_is_superseded_before_temporal_lowering() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().throttle(5).unwrap().build();
    let stale = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(node),
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
    runtime.promote_temporal_wake_ready(stale.id()).unwrap();
    let calls = AtomicU32::new(0);

    let report = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(Aspect::new(1), 1)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(report.tasks_executed, 0);
    assert_eq!(report.tasks_deferred_by_condition, 1);
    assert_eq!(report.temporal_summary.resolver_fallback_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_due_tick(),
        Some(ClockTick::new(6))
    );
    assert_eq!(runtime.telemetry().temporal.rescheduled_wake_count, 1);
}

#[test]
fn transaction_stale_ready_policy_drift_records_supersession_evidence() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().debounce(5).unwrap().build();
    let stale = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(node),
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
    runtime.promote_temporal_wake_ready(stale.id()).unwrap();
    let calls = AtomicU32::new(0);

    let outcome = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                node,
                &|_ctx| {
                    calls.fetch_add(1, Ordering::Relaxed);
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(Aspect::new(6), 1)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(outcome.temporal_evidence.rescheduled_wakes.len(), 1);
    assert_eq!(outcome.temporal_evidence.retired_wakes.len(), 1);
    assert_eq!(outcome.temporal_evidence.scheduled_wakes.len(), 1);
    let supersession = &outcome.temporal_evidence.rescheduled_wakes[0];
    assert_eq!(supersession.retired().id(), stale.id());
    assert_eq!(
        supersession.retired().reason(),
        TemporalWakeRetirementReason::Superseded
    );
    assert!(matches!(
        supersession.scheduled().condition(),
        TemporalCondition::Debounce(_)
    ));
    assert_eq!(supersession.scheduled().due_tick(), ClockTick::new(6));
    assert_eq!(
        outcome.reconstructability.temporal.rescheduled_wake_count,
        1
    );
    assert_ne!(
        outcome.reconstructability.temporal.rescheduled_wake_digest,
        TemporalReconstructabilityArtifact::default().rescheduled_wake_digest
    );
}

#[test]
fn graph_only_sealed_temporal_execution_cannot_use_host_resolver_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().after(5).unwrap().build();
    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
        .unwrap();

    let err = graph
        .execute_prepared_plan(&plan, &(), &|_ctx| {
            Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(Aspect::new(4), 1)]),
            ))
        })
        .unwrap_err();

    assert!(
        format!("{err}").contains("runtime-owned temporal lowering"),
        "sealed temporal policies must not be admitted by graph-only host resolver truth"
    );
}

#[test]
fn transaction_temporal_evidence_freezes_wake_and_reconstructability_artifacts() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().after(5).unwrap().build();
    let aspect = Aspect::new(5);
    let calls = AtomicU32::new(0);

    let deferred = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                node,
                &|_ctx| {
                    calls.fetch_add(1, Ordering::Relaxed);
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(aspect, 1)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert!(deferred.temporal_evidence.has_temporal_facts());
    assert_eq!(deferred.temporal_evidence.scheduled_wakes.len(), 1);
    assert_eq!(deferred.temporal_evidence.eligibility_facts.len(), 1);
    assert_eq!(deferred.reconstructability.temporal.scheduled_wake_count, 1);
    assert_eq!(
        deferred.reconstructability.temporal.eligibility_fact_count,
        1
    );
    assert_eq!(
        deferred
            .reconstructability
            .temporal
            .wake_summary
            .scheduled_count(),
        1
    );
    assert!(
        deferred
            .reconstructability
            .proof()
            .required_rebuild
            .iter()
            .any(|requirement| matches!(requirement, RequiredDerivedRebuildSet::TemporalState(_))),
        "temporal state must be an explicit reconstructability rebuild surface"
    );
    assert_ne!(
        deferred.reconstructability.temporal.certification_digest,
        TemporalReconstructabilityArtifact::default().certification_digest
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .unwrap();
    let admitted = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                node,
                &|_ctx| {
                    calls.fetch_add(1, Ordering::Relaxed);
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(aspect, 2)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(admitted.temporal_evidence.ready_wakes.len(), 1);
    assert_eq!(admitted.temporal_evidence.retired_wakes.len(), 1);
    assert_eq!(admitted.temporal_evidence.eligibility_facts.len(), 1);
    assert_eq!(admitted.reconstructability.temporal.ready_wake_count, 1);
    assert_eq!(admitted.reconstructability.temporal.retired_wake_count, 1);
    assert_eq!(
        admitted
            .temporal_evidence
            .eligibility_facts
            .first()
            .unwrap()
            .eligibility
            .authority(),
        TemporalEligibilityAuthority::RuntimeScheduledWake
    );
}

#[test]
fn transaction_debounce_burst_records_supersession_evidence_and_digest() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().debounce(5).unwrap().build();

    let first = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                node,
                &|_ctx| {
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(Aspect::new(1), 1)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();
    assert_eq!(first.temporal_evidence.scheduled_wakes.len(), 1);
    assert_eq!(first.temporal_evidence.rescheduled_wakes.len(), 0);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let rescheduled = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                node,
                &|_ctx| {
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(Aspect::new(1), 2)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();

    assert_eq!(rescheduled.temporal_evidence.rescheduled_wakes.len(), 1);
    let supersession = &rescheduled.temporal_evidence.rescheduled_wakes[0];
    assert_eq!(
        supersession.retired().reason(),
        TemporalWakeRetirementReason::Superseded
    );
    assert_eq!(supersession.scheduled().due_tick(), ClockTick::new(7));
    assert_eq!(
        rescheduled
            .reconstructability
            .temporal
            .rescheduled_wake_count,
        1
    );
    assert_ne!(
        rescheduled
            .reconstructability
            .temporal
            .rescheduled_wake_digest,
        TemporalReconstructabilityArtifact::default().rescheduled_wake_digest
    );

    let mut drifted = rescheduled.reconstructability.clone();
    drifted.temporal.rescheduled_wake_digest = "reschedule-drift".to_owned();
    drifted.temporal.certification_digest = "certification-drift".to_owned();
    let parity = runtime.temporal_replay_parity_report(&rescheduled.reconstructability, &drifted);
    assert!(!parity.parity);
    assert!(parity
        .mismatch_classes
        .contains(&TemporalReplayMismatchClass::RescheduledWakeDigestMismatch));
}

#[test]
fn transaction_throttle_burst_records_reuse_evidence_and_digest() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().throttle(5).unwrap().build();

    let first = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                node,
                &|_ctx| {
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(Aspect::new(2), 1)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();
    assert_eq!(first.temporal_evidence.scheduled_wakes.len(), 1);
    assert_eq!(first.temporal_evidence.reused_wakes.len(), 0);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .unwrap();
    let reused = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                node,
                &|_ctx| {
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(Aspect::new(2), 2)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();

    assert_eq!(reused.temporal_evidence.reused_wakes.len(), 1);
    let reuse = &reused.temporal_evidence.reused_wakes[0];
    assert_eq!(reuse.original_due_tick(), ClockTick::new(5));
    assert_eq!(reuse.attempted_due_tick(), ClockTick::new(7));
    assert_eq!(reuse.decision_tick(), ClockTick::new(2));
    assert_eq!(runtime.telemetry().temporal.wake_reuse_count, 1);
    assert_eq!(reused.reconstructability.temporal.reused_wake_count, 1);
    assert_ne!(
        reused.reconstructability.temporal.reused_wake_digest,
        TemporalReconstructabilityArtifact::default().reused_wake_digest
    );

    let mut drifted = reused.reconstructability.clone();
    drifted.temporal.reused_wake_digest = "reuse-drift".to_owned();
    drifted.temporal.certification_digest = "certification-drift".to_owned();
    let parity = runtime.temporal_replay_parity_report(&reused.reconstructability, &drifted);
    assert!(!parity.parity);
    assert!(parity
        .mismatch_classes
        .contains(&TemporalReplayMismatchClass::ReusedWakeDigestMismatch));
}

#[test]
fn temporal_replay_parity_report_compares_canonical_temporal_digests() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().after(4).unwrap().build();
    let aspect = Aspect::new(6);

    let expected = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                node,
                &|_ctx| {
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(aspect, 1)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap()
        .reconstructability;
    let replayed = expected.clone();

    let parity = runtime.temporal_replay_parity_report(&expected, &replayed);
    assert!(parity.parity);
    assert!(parity.mismatch_classes.is_empty());
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_replay_parity_check_count,
        1
    );

    let mut drifted = expected.clone();
    drifted.temporal.ready_wake_digest = "drifted-ready-wake-digest".to_owned();
    drifted.temporal.certification_digest = "drifted-certification-digest".to_owned();
    let mismatch = runtime.temporal_replay_parity_report(&expected, &drifted);

    assert!(!mismatch.parity);
    assert!(mismatch
        .mismatch_classes
        .contains(&TemporalReplayMismatchClass::ReadyWakeDigestMismatch));
    assert!(mismatch
        .mismatch_classes
        .contains(&TemporalReplayMismatchClass::CertificationDigestMismatch));
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_replay_parity_check_count,
        2
    );
}

#[test]
fn temporal_replay_parity_survives_snapshot_restore_of_ready_frontier() {
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
    runtime.promote_due_temporal_wakes_ready().unwrap();
    let snapshot = runtime.capture_snapshot();
    let expected = snapshot
        .reconstructability
        .clone()
        .expect("snapshot should carry temporal reconstructability");

    runtime
        .retire_temporal_wake(wake.id(), TemporalWakeRetirementReason::Consumed)
        .unwrap();
    runtime.restore_snapshot(&snapshot).unwrap();
    let restored = runtime.capture_snapshot();
    let replayed = restored
        .reconstructability
        .clone()
        .expect("restored snapshot should carry temporal reconstructability");

    let parity = runtime.temporal_replay_parity_report(&expected, &replayed);
    assert!(parity.parity, "{:?}", parity.mismatch_classes);
    let record = temporal_certification_record(
        TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
        replayed.temporal.clone(),
        Some(parity),
    );
    assert!(record.passed);
    assert_eq!(
        record.family,
        TemporalCertificationFamily::TemporalBranchRestoreEquivalence
    );
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 1);
    assert_eq!(
        runtime.temporal_frontier_snapshot().next_ready_wake_id(),
        Some(wake.id())
    );
}

fn complete_temporal_certification_bundle_for_artifact(
    artifact: TemporalReconstructabilityArtifact,
    parity: TemporalReplayParityReport,
) -> TemporalCertificationBundle {
    temporal_certification_bundle([
        temporal_certification_record(
            TemporalCertificationFamily::TemporalEligibilityReplayParity,
            artifact.clone(),
            Some(parity.clone()),
        ),
        temporal_certification_record(
            TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
            artifact.clone(),
            Some(parity),
        ),
        temporal_certification_record(
            TemporalCertificationFamily::TemporalWakeBoundedness,
            artifact.clone(),
            None,
        ),
        temporal_certification_record(
            TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
            artifact,
            None,
        ),
    ])
}

#[test]
fn temporal_branch_restore_equivalence_certifies_full_bundle_parity() {
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
    runtime.promote_due_temporal_wakes_ready().unwrap();
    let snapshot = runtime.capture_snapshot();
    let expected = snapshot
        .reconstructability
        .clone()
        .expect("snapshot should carry temporal reconstructability");

    runtime
        .retire_temporal_wake(wake.id(), TemporalWakeRetirementReason::Consumed)
        .unwrap();
    runtime.restore_snapshot(&snapshot).unwrap();
    let replayed = runtime
        .capture_snapshot()
        .reconstructability
        .expect("restored snapshot should carry temporal reconstructability");

    let artifact_parity = runtime.temporal_replay_parity_report(&expected, &replayed);
    assert!(
        artifact_parity.parity,
        "{:?}",
        artifact_parity.mismatch_classes
    );

    let expected_bundle = complete_temporal_certification_bundle_for_artifact(
        expected.temporal.clone(),
        artifact_parity.clone(),
    );
    let replayed_bundle =
        complete_temporal_certification_bundle_for_artifact(replayed.temporal, artifact_parity);
    expected_bundle.ensure_passed().unwrap();
    replayed_bundle.ensure_passed().unwrap();

    let bundle_parity =
        runtime.temporal_certification_bundle_parity_report(&expected_bundle, &replayed_bundle);
    assert!(bundle_parity.parity, "{:?}", bundle_parity.mismatch_classes);
    assert_eq!(
        bundle_parity.proof_schema_version,
        TEMPORAL_CERTIFICATION_BUNDLE_PARITY_SCHEMA_VERSION
    );
    assert_eq!(
        bundle_parity.expected.bundle_digest,
        bundle_parity.replayed.bundle_digest
    );
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_replay_parity_check_count,
        2
    );
}

#[test]
fn temporal_certification_bundle_parity_detects_bundle_record_drift() {
    let artifact = TemporalReconstructabilityArtifact::default();
    let parity = temporal_replay_parity_report(&artifact, &artifact);
    let expected =
        complete_temporal_certification_bundle_for_artifact(artifact.clone(), parity.clone());
    let mut drifted = complete_temporal_certification_bundle_for_artifact(artifact, parity);
    drifted.bundle_digest = "drifted-temporal-certification-bundle".to_owned();
    drifted.records[0].artifact.certification_digest = "drifted-record".to_owned();

    let report = temporal_certification_bundle_parity_report(&expected, &drifted);
    assert!(!report.parity);
    assert!(report
        .mismatch_classes
        .contains(&TemporalCertificationBundleMismatchClass::BundleDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&TemporalCertificationBundleMismatchClass::RecordSetMismatch));
}

#[test]
fn temporal_eligibility_replay_parity_certification_family_records_exact_digest_match() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().after(3).unwrap().build();
    let outcome = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                node,
                &|_ctx| {
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(Aspect::new(3), 1)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();

    let parity = runtime
        .temporal_replay_parity_report(&outcome.reconstructability, &outcome.reconstructability);
    let record = temporal_certification_record(
        TemporalCertificationFamily::TemporalEligibilityReplayParity,
        outcome.reconstructability.temporal.clone(),
        Some(parity),
    );

    assert!(record.passed);
    assert_eq!(
        record.family,
        TemporalCertificationFamily::TemporalEligibilityReplayParity
    );
    assert_eq!(record.artifact.eligibility_fact_count, 1);
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_replay_parity_check_count,
        1
    );
}

#[test]
fn temporal_wake_boundedness_certification_family_covers_large_interval_jumps() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let collapse = IntervalCondition::try_new(5)
        .unwrap()
        .with_missed_tick_policy(MissedTickPolicy::CollapseToOne);
    let skip = IntervalCondition::try_new(5)
        .unwrap()
        .with_missed_tick_policy(MissedTickPolicy::SkipToLatest);
    let catch_up = IntervalCondition::try_new(5)
        .unwrap()
        .with_missed_tick_policy(MissedTickPolicy::CatchUpAll);
    runtime
        .schedule_temporal_wake(TemporalCondition::interval(collapse), ClockTick::new(5))
        .unwrap();
    runtime
        .schedule_temporal_wake(TemporalCondition::interval(skip), ClockTick::new(5))
        .unwrap();
    runtime
        .schedule_temporal_wake(TemporalCondition::interval(catch_up), ClockTick::new(5))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1_005),
        ))
        .unwrap();

    let promoted = runtime.promote_due_temporal_wakes_ready().unwrap();
    assert_eq!(promoted.len(), 3);
    let mut regenerations = Vec::new();
    for ready in promoted {
        regenerations.push(runtime.regenerate_interval_wake(ready.id()).unwrap());
    }
    let due_ticks = regenerations
        .iter()
        .map(|regeneration| regeneration.scheduled().due_tick())
        .collect::<Vec<_>>();
    assert!(due_ticks.contains(&ClockTick::new(1_010)));
    assert!(due_ticks.contains(&ClockTick::new(1_005)));
    assert!(due_ticks.contains(&ClockTick::new(10)));
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .interval_wake_regeneration_count,
        3
    );
    assert_eq!(
        runtime.telemetry().temporal.missed_interval_count,
        399,
        "large interval jumps should be charged to missed-tick policy outcomes, not hidden loops"
    );
    assert_eq!(
        runtime
            .telemetry()
            .temporal
            .temporal_broad_scan_denial_count,
        1
    );

    let snapshot = runtime.capture_snapshot();
    let record = temporal_certification_record(
        TemporalCertificationFamily::TemporalWakeBoundedness,
        snapshot.reconstructability.unwrap().temporal,
        None,
    );
    assert!(record.passed);
    assert_eq!(
        record.family,
        TemporalCertificationFamily::TemporalWakeBoundedness
    );
}

#[test]
fn stale_after_expires_without_upstream_writes_under_runtime_owned_time() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().stale_after(4).unwrap().build();
    let calls = AtomicU32::new(0);

    let deferred = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(Aspect::new(4), 1)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();
    assert_eq!(deferred.tasks_deferred_by_condition, 1);
    assert_eq!(calls.load(Ordering::Relaxed), 0);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(4),
        ))
        .unwrap();
    let admitted = runtime
        .evaluate_with_plan(
            node,
            &(),
            &|_ctx| {
                calls.fetch_add(1, Ordering::Relaxed);
                Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(Aspect::new(4), 2)]),
                ))
            },
            EvaluationRequestMode::Default,
        )
        .unwrap();
    assert_eq!(admitted.tasks_executed, 1);
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(admitted.temporal_summary.resolver_fallback_count(), 0);
}

#[test]
fn previous_value_time_gated_equivalence_certification_family_captures_committed_lineage() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().build();
    let value_aspect = Aspect::new(5);
    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                        value_aspect,
                        7,
                    )]))
                    .with_output_identity("previous-value-equivalence"),
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
    runtime.promote_due_temporal_wakes_ready().unwrap();
    let access = runtime
        .grant_temporal_previous_value_access(wake.id())
        .unwrap();
    let reference = runtime.previous_temporal_value(&access, source).unwrap();

    assert_eq!(reference.revision(), PreviousValueRevision::new(1));
    assert_eq!(
        reference.output_identity().map(OutputIdentity::as_str),
        Some("previous-value-equivalence")
    );
    let snapshot = runtime.capture_snapshot();
    let record = temporal_certification_record(
        TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
        snapshot.reconstructability.unwrap().temporal,
        None,
    );
    assert!(record.passed);
    assert_eq!(
        runtime.telemetry().temporal.previous_value_reference_count,
        1
    );
}

#[test]
fn temporal_certification_bundle_accepts_complete_required_family_set() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().after(2).unwrap().build();
    let outcome = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                node,
                &|_ctx| {
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(Aspect::new(6), 1)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();
    let parity = runtime
        .temporal_replay_parity_report(&outcome.reconstructability, &outcome.reconstructability);
    let artifact = outcome.reconstructability.temporal.clone();

    let bundle = runtime.temporal_certification_bundle([
        temporal_certification_record(
            TemporalCertificationFamily::TemporalEligibilityReplayParity,
            artifact.clone(),
            Some(parity),
        ),
        temporal_certification_record(
            TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
            artifact.clone(),
            None,
        ),
        temporal_certification_record(
            TemporalCertificationFamily::TemporalWakeBoundedness,
            artifact.clone(),
            None,
        ),
        temporal_certification_record(
            TemporalCertificationFamily::PreviousValueTimeGatedEquivalence,
            artifact,
            None,
        ),
    ]);

    assert!(bundle.passed, "{:?}", bundle.failures);
    bundle.ensure_passed().unwrap();
    assert_eq!(
        bundle.schema_version,
        TEMPORAL_CERTIFICATION_BUNDLE_SCHEMA_VERSION
    );
    assert_eq!(bundle.records.len(), 4);
    assert_eq!(bundle.summary.required_family_count, 4);
    assert_eq!(bundle.summary.provided_record_count, 4);
    assert_eq!(bundle.summary.failed_family_count, 0);
    assert_eq!(bundle.summary.missing_family_count, 0);
    assert_eq!(bundle.summary.duplicate_family_count, 0);
    assert!(!bundle.bundle_digest.is_empty());
}

#[test]
fn temporal_certification_builder_requires_distinct_family_evidence_lanes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let eligibility_node = runtime.graph_mut().node().after(2).unwrap().build();
    let eligibility_outcome = runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                eligibility_node,
                &|_ctx| {
                    Ok::<NodeEvaluationResult, SignalError>(NodeEvaluationResult::from_version(
                        AspectVersion::from_updates([(Aspect::new(6), 1)]),
                    ))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();
    let eligibility_parity = runtime.temporal_replay_parity_report(
        &eligibility_outcome.reconstructability,
        &eligibility_outcome.reconstructability,
    );
    let eligibility_artifact = eligibility_outcome.reconstructability.temporal.clone();

    let branch_wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(3).unwrap(), ClockTick::new(3))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .unwrap();
    runtime.promote_due_temporal_wakes_ready().unwrap();
    let snapshot = runtime.capture_snapshot();
    let expected_restore = snapshot
        .reconstructability
        .clone()
        .expect("snapshot should carry temporal reconstructability");
    runtime
        .retire_temporal_wake(branch_wake.id(), TemporalWakeRetirementReason::Consumed)
        .unwrap();
    runtime.restore_snapshot(&snapshot).unwrap();
    let replayed_restore = runtime
        .capture_snapshot()
        .reconstructability
        .expect("restored snapshot should carry temporal reconstructability");
    let restore_parity =
        runtime.temporal_replay_parity_report(&expected_restore, &replayed_restore);
    let restore_artifact = replayed_restore.temporal.clone();

    let interval = IntervalCondition::try_new(5)
        .unwrap()
        .with_missed_tick_policy(MissedTickPolicy::SkipToLatest);
    let interval_wake = runtime
        .schedule_temporal_wake(TemporalCondition::interval(interval), ClockTick::new(8))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(48),
        ))
        .unwrap();
    runtime.promote_due_temporal_wakes_ready().unwrap();
    let regeneration = runtime
        .regenerate_interval_wake(interval_wake.id())
        .unwrap();
    let mut wake_boundedness_evidence = TemporalTransactionEvidence::default();
    wake_boundedness_evidence.clock_basis = runtime.clock_basis();
    wake_boundedness_evidence
        .interval_regenerations
        .push(regeneration);
    let wake_boundedness_artifact = TemporalReconstructabilityArtifact::from_evidence(
        runtime.temporal_wake_summary(),
        &wake_boundedness_evidence,
    );

    let source = runtime.graph_mut().node().build();
    let value_aspect = Aspect::new(7);
    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(value_aspect, 9)]),
                )))
            })?;
            Ok(())
        })
        .unwrap();
    let previous_wake = runtime
        .schedule_temporal_wake(TemporalCondition::after(1).unwrap(), ClockTick::new(49))
        .unwrap();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(49),
        ))
        .unwrap();
    runtime.promote_due_temporal_wakes_ready().unwrap();
    let previous_access = runtime
        .grant_temporal_previous_value_access(previous_wake.id())
        .unwrap();
    let previous_reference = runtime
        .previous_temporal_value(&previous_access, source)
        .unwrap();
    let mut previous_evidence = TemporalTransactionEvidence::default();
    previous_evidence.clock_basis = runtime.clock_basis();
    previous_evidence
        .previous_value_references
        .push(previous_reference);
    let previous_artifact = TemporalReconstructabilityArtifact::from_evidence(
        runtime.temporal_wake_summary(),
        &previous_evidence,
    );

    let bundle = runtime
        .temporal_certification_builder()
        .with_temporal_eligibility_replay_parity(eligibility_artifact, eligibility_parity)
        .unwrap()
        .with_temporal_branch_restore_equivalence(restore_artifact, restore_parity)
        .unwrap()
        .with_temporal_wake_boundedness(wake_boundedness_artifact)
        .unwrap()
        .with_previous_value_time_gated_equivalence(previous_artifact)
        .unwrap()
        .build()
        .unwrap();

    assert!(bundle.passed, "{:?}", bundle.failures);
    assert_eq!(bundle.records.len(), 4);
    assert_eq!(bundle.summary.passed_family_count, 4);
}

#[test]
fn temporal_phase9_mixed_workload_preserves_parity_and_boundedness_across_branch_restore() {
    let outcome = temporal_phase9_mixed_workload();

    assert!(outcome.bundle.passed, "{:?}", outcome.bundle.failures);
    assert_eq!(outcome.bundle.summary.passed_family_count, 4);

    assert!(
        outcome.eligibility_parity.parity,
        "{:?}",
        outcome.eligibility_parity.mismatch_classes
    );
    assert_eq!(
        outcome.feature.reconstructability_before_restore.clock_checkpoint_digest,
        outcome.sibling.reconstructability_before_restore.clock_checkpoint_digest,
        "equivalent sibling branches must share the same checkpoint-honest clock basis before restore"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_before_restore
            .scheduled_wake_digest,
        outcome
            .sibling
            .reconstructability_before_restore
            .scheduled_wake_digest,
        "equivalent sibling branches must share the same scheduled wake frontier before restore"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_before_restore
            .ready_wake_digest,
        outcome
            .sibling
            .reconstructability_before_restore
            .ready_wake_digest,
        "equivalent sibling branches must share the same ready frontier before restore"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_before_restore
            .temporal_eligibility_digest,
        outcome
            .sibling
            .reconstructability_before_restore
            .temporal_eligibility_digest,
        "equivalent sibling branches must share the same temporal eligibility truth before restore"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_after_restore
            .clock_checkpoint_digest,
        outcome
            .sibling
            .reconstructability_after_restore
            .clock_checkpoint_digest,
        "equivalent restored sibling branches must converge to the same clock checkpoint digest"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_after_restore
            .scheduled_wake_digest,
        outcome
            .sibling
            .reconstructability_after_restore
            .scheduled_wake_digest,
        "equivalent restored sibling branches must converge to the same scheduled wake digest"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_after_restore
            .ready_wake_digest,
        outcome
            .sibling
            .reconstructability_after_restore
            .ready_wake_digest,
        "equivalent restored sibling branches must converge to the same ready queue digest"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_after_restore
            .temporal_eligibility_digest,
        outcome
            .sibling
            .reconstructability_after_restore
            .temporal_eligibility_digest,
        "equivalent restored sibling branches must converge to the same eligibility digest"
    );
    assert_eq!(
        outcome
            .feature
            .reconstructability_after_restore
            .previous_value_reference_digest,
        outcome
            .sibling
            .reconstructability_after_restore
            .previous_value_reference_digest,
        "equivalent restored sibling branches must converge to the same previous-value basis"
    );
    assert_ne!(
        outcome
            .feature
            .reconstructability_before_restore
            .ready_wake_digest,
        outcome
            .feature
            .reconstructability_after_snapshot_drift
            .ready_wake_digest,
        "hostile branch-local churn should perturb ready-frontier truth before restore"
    );
    assert!(
        outcome.feature.replay_after_snapshot_drift.frames.len()
            >= outcome.feature.replay_before_restore.frames.len(),
        "hostile branch-local churn may append replay evidence before restore, but it must not erase prior feature replay history"
    );
    assert_eq!(
        outcome.feature.head_snapshot_before_restore,
        Some(outcome.feature.restored_snapshot_id),
        "capturing the hostile branch checkpoint must advance the branch head to that checkpoint"
    );
    assert_eq!(
        outcome.feature.head_snapshot_after_restore,
        Some(outcome.feature.restored_snapshot_id),
        "restoring a hostile branch checkpoint must reinstate the captured branch head"
    );
    assert_eq!(
        outcome.sibling.head_snapshot_before_restore,
        Some(outcome.sibling.restored_snapshot_id),
        "capturing the sibling hostile branch checkpoint must advance the branch head to that checkpoint"
    );
    assert_eq!(
        outcome.sibling.head_snapshot_after_restore,
        Some(outcome.sibling.restored_snapshot_id),
        "restoring the sibling hostile branch checkpoint must reinstate the captured branch head"
    );
    assert!(
        outcome.feature.restore_parity.parity,
        "{:?}",
        outcome.feature.restore_parity.mismatch_classes
    );
    assert!(
        outcome.sibling.restore_parity.parity,
        "{:?}",
        outcome.sibling.restore_parity.mismatch_classes
    );
    assert!(
        outcome.feature.replay_after_restore.frames.len()
            >= outcome.feature.replay_before_restore.frames.len(),
        "restore may append replay evidence but must not erase prior feature branch replay history"
    );
    assert!(
        outcome.sibling.replay_after_restore.frames.len()
            >= outcome.sibling.replay_before_restore.frames.len(),
        "restore may append replay evidence but must not erase prior sibling branch replay history"
    );
    assert!(
        outcome
            .feature
            .replay_after_restore
            .frames
            .iter()
            .all(|frame| frame.branch_id == outcome.feature.branch_id),
        "feature replay history must stay branch-local after restore"
    );
    assert!(
        outcome
            .sibling
            .replay_after_restore
            .frames
            .iter()
            .all(|frame| frame.branch_id == outcome.sibling.branch_id),
        "sibling replay history must stay branch-local after restore"
    );
    assert!(
        outcome.temporal_telemetry.temporal_broad_scan_denial_count >= 4,
        "mixed temporal torture should exercise ready-frontier promotion enough to charge broad temporal scan denial counters"
    );
    assert!(
        outcome
            .feature
            .temporal_telemetry_after_restore
            .branch_local_temporal_restore_count
            >= 1,
        "feature branch restore must charge branch-local temporal restore work"
    );
    assert!(
        outcome
            .sibling
            .temporal_telemetry_after_restore
            .branch_local_temporal_restore_count
            >= 1,
        "sibling branch restore must charge branch-local temporal restore work"
    );
    assert!(
        outcome
            .feature
            .temporal_telemetry_after_restore
            .branch_restore_temporal_rebuild_denial_count
            >= 1,
        "feature branch restore must consume retained frontier truth instead of rebuilding from node conditions"
    );
    assert!(
        outcome
            .sibling
            .temporal_telemetry_after_restore
            .branch_restore_temporal_rebuild_denial_count
            >= 1,
        "sibling branch restore must consume retained frontier truth instead of rebuilding from node conditions"
    );
    assert!(
        outcome.temporal_telemetry.missed_interval_count >= 399,
        "large interval jumps must charge missed policy outcomes rather than hiding elapsed timer work"
    );
    assert_eq!(outcome.boundedness_artifact.interval_regeneration_count, 3);
    assert_ne!(
        outcome.eligibility_artifact.temporal_eligibility_digest,
        TemporalReconstructabilityArtifact::default().temporal_eligibility_digest
    );
    assert_ne!(
        outcome
            .previous_value_artifact
            .previous_value_reference_digest,
        TemporalReconstructabilityArtifact::default().previous_value_reference_digest
    );
}

#[test]
fn milestone_a_closeout_bundle_covers_hostile_temporal_certification_paths() {
    let outcome = temporal_phase9_mixed_workload();

    assert!(outcome.bundle.passed, "{:?}", outcome.bundle.failures);
    assert_eq!(outcome.bundle.summary.passed_family_count, 4);
    assert_eq!(outcome.bundle.summary.failed_family_count, 0);
    assert_eq!(outcome.bundle.summary.missing_family_count, 0);

    assert_eq!(
        outcome
            .diagnostics_operational
            .with_profile(DiagnosticsLevel::Forensic),
        outcome.diagnostics_forensic
    );
    assert!(outcome
        .diagnostics_operational
        .cost_contracts
        .prohibited_failure_modes
        .contains(&TemporalPerformanceFailureMode::TemporalBroadScan));
    assert_eq!(
        outcome.temporal_telemetry.temporal_replay_parity_check_count,
        1,
        "returning to main must restore main-branch telemetry instead of smearing branch-local parity counters across branches"
    );
}

#[test]
fn temporal_certification_builder_rejects_missing_duplicate_and_synthetic_evidence() {
    let artifact = TemporalReconstructabilityArtifact::default();
    let parity = temporal_replay_parity_report(&artifact, &artifact);

    let missing_err = temporal_certification_builder().build().unwrap_err();
    assert!(format!("{missing_err}").contains("required certification family"));

    let synthetic_err = temporal_certification_builder()
        .with_temporal_eligibility_replay_parity(artifact.clone(), parity.clone())
        .unwrap_err();
    assert!(format!("{synthetic_err}").contains("default temporal artifact"));

    let mut eligibility_artifact = artifact.clone();
    eligibility_artifact.eligibility_fact_count = 1;
    eligibility_artifact.certification_digest = "non-default-eligibility".to_owned();
    let mut replayed = eligibility_artifact.clone();
    replayed.scheduled_wake_digest = "different-replayed-artifact".to_owned();
    let mismatched_parity = temporal_replay_parity_report(&eligibility_artifact, &replayed);
    let drift_err = temporal_certification_builder()
        .with_temporal_eligibility_replay_parity(replayed, mismatched_parity)
        .unwrap_err();
    assert!(format!("{drift_err}").contains("passing temporal replay parity"));

    let valid_parity = temporal_replay_parity_report(&eligibility_artifact, &eligibility_artifact);
    let duplicate_err = temporal_certification_builder()
        .with_temporal_eligibility_replay_parity(eligibility_artifact.clone(), valid_parity.clone())
        .unwrap()
        .with_temporal_eligibility_replay_parity(eligibility_artifact, valid_parity)
        .unwrap_err();
    assert!(format!("{duplicate_err}").contains("duplicate certification family"));
}

#[test]
fn temporal_certification_bundle_rejects_missing_duplicate_failed_empty_and_parity_drift() {
    let artifact = TemporalReconstructabilityArtifact::default();
    let mut drifted_artifact = artifact.clone();
    drifted_artifact.ready_wake_digest.push_str("-drift");
    let drift = temporal_replay_parity_report(&artifact, &drifted_artifact);
    let mut empty_digest_artifact = artifact.clone();
    empty_digest_artifact.certification_digest.clear();

    let bundle = temporal_certification_bundle([
        temporal_certification_record(
            TemporalCertificationFamily::TemporalEligibilityReplayParity,
            artifact.clone(),
            Some(drift),
        ),
        temporal_certification_record(
            TemporalCertificationFamily::TemporalEligibilityReplayParity,
            artifact.clone(),
            None,
        ),
        temporal_certification_record(
            TemporalCertificationFamily::TemporalBranchRestoreEquivalence,
            empty_digest_artifact,
            None,
        ),
        TemporalCertificationRecord {
            family: TemporalCertificationFamily::TemporalWakeBoundedness,
            artifact,
            parity: None,
            passed: false,
        },
    ]);

    assert!(!bundle.passed);
    assert!(format!("{}", bundle.ensure_passed().unwrap_err())
        .contains("temporal certification bundle failed"));
    assert!(bundle.failures.iter().any(|failure| matches!(
        failure,
        TemporalCertificationFailure::MissingRequiredFamily {
            family: TemporalCertificationFamily::PreviousValueTimeGatedEquivalence
        }
    )));
    assert!(bundle.failures.iter().any(|failure| matches!(
        failure,
        TemporalCertificationFailure::DuplicateFamily {
            family: TemporalCertificationFamily::TemporalEligibilityReplayParity,
            count: 2
        }
    )));
    assert!(bundle.failures.iter().any(|failure| matches!(
        failure,
        TemporalCertificationFailure::ParityMismatch {
            family: TemporalCertificationFamily::TemporalEligibilityReplayParity,
            ..
        }
    )));
    assert!(bundle.failures.iter().any(|failure| matches!(
        failure,
        TemporalCertificationFailure::FailedFamily {
            family: TemporalCertificationFamily::TemporalWakeBoundedness
        }
    )));
    assert!(bundle.failures.iter().any(|failure| matches!(
        failure,
        TemporalCertificationFailure::EmptyCertificationDigest {
            family: TemporalCertificationFamily::TemporalBranchRestoreEquivalence
        }
    )));
    assert_eq!(bundle.summary.provided_record_count, 4);
    assert_eq!(bundle.summary.missing_family_count, 1);
    assert_eq!(bundle.summary.duplicate_family_count, 1);
    assert_eq!(bundle.summary.failed_family_count, 4);
    assert!(!bundle.bundle_digest.is_empty());
}
