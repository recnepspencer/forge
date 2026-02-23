//! Tests for the tracing infrastructure.

use super::schema::*;
use super::decision_log::*;
use crate::policy::PolicyKind;

fn make_decision(id: u64, tier: DecisionTier, kind: DecisionKind) -> TracedDecision {
    TracedDecision::new(
        DecisionId(id),
        kind,
        tier,
        0.5,
        DecisionContext::Tolerance { measured: 1e-8, threshold: 1e-6 },
    )
}

#[test]
fn traced_decision_creation() {
    let decision = TracedDecision::new(
        DecisionId(1),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        0.5,
        DecisionContext::Tolerance { measured: 1e-8, threshold: 1e-6 },
    );
    assert_eq!(decision.get_id(), DecisionId(1));
    assert_eq!(*decision.get_kind(), DecisionKind::Exact);
    assert!(decision.is_overridable());
    assert_eq!(decision.get_margin(), 0.5);
}

#[test]
fn decision_id_display() {
    let id = DecisionId(42);
    assert_eq!(format!("{}", id), "decision-42");
}

#[test]
fn decision_log_query_api() {
    let mut log = DecisionLog::new();

    log.record(TracedDecision::new(
        DecisionId(1),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
    ));
    log.record(TracedDecision::new(
        DecisionId(2),
        DecisionKind::Ambiguous { fallback_applied: "snap_to_edge".to_string() },
        DecisionTier::Escalated,
        0.001,
        DecisionContext::Tolerance { measured: 9e-7, threshold: 1e-6 },
    ));
    log.record(TracedDecision::new(
        DecisionId(3),
        DecisionKind::NearBoundary { threshold: 1e-6 },
        DecisionTier::NearBoundary,
        0.1,
        DecisionContext::Tolerance { measured: 8e-7, threshold: 1e-6 },
    ));
    log.record(TracedDecision::new(
        DecisionId(4),
        DecisionKind::PolicyApplied {
            policy: PolicyKind::CoincidentGeometry,
            default_used: true,
        },
        DecisionTier::PolicyApplied,
        0.05,
        DecisionContext::Coincidence {
            entity_a: EntityRef::new(EntityKind::Vertex, 0),
            entity_b: EntityRef::new(EntityKind::Vertex, 1),
        },
    ));

    assert_eq!(log.len(), 4);
    assert!(!log.is_clean());
    assert_eq!(log.ambiguous_only().len(), 1);
    assert_eq!(log.ambiguous_only()[0].get_id(), DecisionId(2));

    let by_margin = log.by_margin_ascending();
    assert_eq!(by_margin[0].get_id(), DecisionId(2));
    assert_eq!(by_margin[1].get_id(), DecisionId(4));

    let summary = log.summary();
    assert_eq!(summary.total, 4);
    assert_eq!(summary.exact, 1);
    assert_eq!(summary.ambiguous, 1);
    assert_eq!(summary.near_boundary, 1);
    assert_eq!(summary.policy_applied, 1);
    assert_eq!(summary.forced, 0);
    assert!((summary.min_margin - 0.001).abs() < 1e-10);
}

#[test]
fn decision_log_is_clean_when_no_ambiguous() {
    let mut log = DecisionLog::new();
    log.record(TracedDecision::new(
        DecisionId(1),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
    ));
    assert!(log.is_clean());
}

#[test]
fn decision_log_merge() {
    let mut log_a = DecisionLog::new();
    log_a.record(TracedDecision::new(
        DecisionId(1), DecisionKind::Exact, DecisionTier::Deterministic, 1.0,
        DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
    ));

    let mut log_b = DecisionLog::new();
    log_b.record(TracedDecision::new(
        DecisionId(2), DecisionKind::Exact, DecisionTier::Deterministic, 0.5,
        DecisionContext::Tolerance { measured: 0.0, threshold: 1e-6 },
    ));

    log_a.merge(log_b);
    assert_eq!(log_a.len(), 2);
}

// =====================================================================
// Phase C: Span-Based Tracing Verification Tests
// =====================================================================

#[test]
fn mismatched_span_close_truncates_stack() {
    let mut log = DecisionLog::new();
    let outer = log.start_span("outer");
    let inner = log.start_span("inner");

    assert_eq!(log.active_span(), Some(inner));

    log.end_span(outer, 100);

    assert_eq!(log.active_span(), None, "Closing outer should truncate inner too");
}

#[test]
fn closing_unknown_span_is_harmless() {
    let mut log = DecisionLog::new();
    let real = log.start_span("real");

    log.end_span(SpanId(999), 50);

    assert_eq!(log.active_span(), Some(real), "Unknown close should not affect stack");

    log.end_span(real, 100);
    assert_eq!(log.active_span(), None);
}

#[test]
fn nested_spans_record_parent_ids() {
    let mut log = DecisionLog::new();
    let outer = log.start_span("outer");
    let inner = log.start_span("inner");
    let deepest = log.start_span("deepest");

    log.end_span(deepest, 10);
    log.end_span(inner, 20);
    log.end_span(outer, 30);

    let starts: Vec<_> = log.get_events().iter().filter_map(|e| match e {
        TraceEvent::StartSpan { id, parent_id, .. } => Some((*id, *parent_id)),
        _ => None,
    }).collect();

    assert_eq!(starts.len(), 3);
    assert_eq!(starts[0], (outer, None));
    assert_eq!(starts[1], (inner, Some(outer)));
    assert_eq!(starts[2], (deepest, Some(inner)));
}

#[test]
fn decisions_stamped_with_active_span() {
    let mut log = DecisionLog::new();
    let span_a = log.start_span("phase_a");

    log.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
    log.end_span(span_a, 100);

    log.record(make_decision(2, DecisionTier::Deterministic, DecisionKind::Exact));

    let decisions: Vec<_> = log.decisions().collect();
    assert_eq!(decisions[0].get_span_id(), Some(span_a));
    assert_eq!(decisions[1].get_span_id(), None);
}

#[test]
fn serde_roundtrip_resets_ephemeral_span_counter() {
    let mut log = DecisionLog::new();
    let _span = log.start_span("test");
    log.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
    log.end_span(_span, 100);

    let json = serde_json::to_string(&log).expect("serialize");
    let restored: DecisionLog = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(restored.active_span(), None, "span_stack is ephemeral, should be empty");

    assert_eq!(
        restored.decisions().count(),
        log.decisions().count(),
        "Decisions should survive serde roundtrip",
    );

    let new_span = restored.clone();
    assert_eq!(new_span.get_events().len(), log.get_events().len());
}

#[test]
fn tier_filtering_returns_only_tier_2_plus() {
    let mut log = DecisionLog::new();

    log.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
    log.record(make_decision(2, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));
    log.record(make_decision(3, DecisionTier::Escalated,
        DecisionKind::Ambiguous { fallback_applied: "snap".into() }));
    log.record(make_decision(4, DecisionTier::Deterministic, DecisionKind::Exact));

    let interesting = log.interesting_only();
    assert_eq!(interesting.len(), 2);
    assert_eq!(interesting[0].get_id(), DecisionId(2));
    assert_eq!(interesting[1].get_id(), DecisionId(3));
}

#[test]
fn display_interesting_empty_for_boring_spans() {
    let mut log = DecisionLog::new();

    for i in 0..10 {
        let span = log.start_span(&format!("boring_{}", i));
        log.record(make_decision(i, DecisionTier::Deterministic, DecisionKind::Exact));
        log.end_span(span, 10);
    }

    let output = log.display_interesting();
    assert!(
        !output.contains("NearBoundary"),
        "All-boring log should have no interesting content in display",
    );
}

#[test]
fn trace_summary_diff_detects_added_decisions() {
    let mut log_old = DecisionLog::new();
    log_old.record(make_decision(1, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));

    let mut log_new = DecisionLog::new();
    log_new.record(make_decision(1, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));
    log_new.record(make_decision(2, DecisionTier::Escalated,
        DecisionKind::Ambiguous { fallback_applied: "snap".into() }));

    let summary_old = log_old.to_summary(0xAAAA);
    let summary_new = log_new.to_summary(0xBBBB);

    let diff = summary_new.diff(&summary_old);

    assert!(diff.state_hash_changed);
    assert_eq!(diff.added.len(), 1);
    assert_eq!(diff.added[0].get_id(), DecisionId(2));
    assert!(diff.removed.is_empty());
    assert!(diff.changed.is_empty());
}

#[test]
fn trace_summary_diff_detects_removed_decisions() {
    let mut log_old = DecisionLog::new();
    log_old.record(make_decision(1, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));
    log_old.record(make_decision(2, DecisionTier::Escalated,
        DecisionKind::Ambiguous { fallback_applied: "snap".into() }));

    let mut log_new = DecisionLog::new();
    log_new.record(make_decision(1, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));

    let summary_old = log_old.to_summary(0xAAAA);
    let summary_new = log_new.to_summary(0xAAAA);

    let diff = summary_new.diff(&summary_old);

    assert!(!diff.state_hash_changed);
    assert!(diff.added.is_empty());
    assert_eq!(diff.removed.len(), 1);
    assert_eq!(diff.removed[0].get_id(), DecisionId(2));
}

#[test]
fn trace_summary_diff_detects_changed_tier() {
    let mut log_old = DecisionLog::new();
    log_old.record(make_decision(1, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));

    let mut log_new = DecisionLog::new();
    log_new.record(make_decision(1, DecisionTier::Escalated,
        DecisionKind::NearBoundary { threshold: 1e-6 }));

    let summary_old = log_old.to_summary(0xAAAA);
    let summary_new = log_new.to_summary(0xAAAA);

    let diff = summary_new.diff(&summary_old);

    assert!(diff.added.is_empty());
    assert!(diff.removed.is_empty());
    assert_eq!(diff.changed.len(), 1);
    assert_eq!(diff.changed[0].0.get_tier(), DecisionTier::NearBoundary);
    assert_eq!(diff.changed[0].1.get_tier(), DecisionTier::Escalated);
}

#[test]
fn trace_summary_diff_identical_is_empty() {
    let mut log = DecisionLog::new();
    log.record(make_decision(1, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));
    log.record(make_decision(2, DecisionTier::Escalated,
        DecisionKind::Ambiguous { fallback_applied: "snap".into() }));

    let summary = log.to_summary(0xAAAA);

    let diff = summary.diff(&summary);

    assert!(!diff.state_hash_changed);
    assert!(diff.is_empty(), "Diffing a summary against itself should be empty");
}

#[test]
fn empty_log_to_summary_is_empty() {
    let log = DecisionLog::new();
    let summary = log.to_summary(0);

    assert!(summary.get_interesting().is_empty());
    assert!(summary.get_span_summaries().is_empty());
}

// =====================================================================
// Phase P3.1: Checkpoint Diffing Tests
// =====================================================================

use super::checkpoint_diff::{diff_decision_logs, CheckpointLog};

#[test]
fn diff_decision_logs_detects_added() {
    let before = DecisionLog::new();
    let mut after = DecisionLog::new();
    after.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
    after.record(make_decision(2, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));

    let delta = diff_decision_logs(&before, &after);

    assert_eq!(delta.get_added().len(), 2);
    assert!(delta.get_removed().is_empty());
    assert!(delta.get_changed().is_empty());
    assert!(!delta.is_empty());
    assert_eq!(delta.total_changes(), 2);
}

#[test]
fn diff_decision_logs_detects_removed() {
    let mut before = DecisionLog::new();
    before.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
    before.record(make_decision(2, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));

    let after = DecisionLog::new();

    let delta = diff_decision_logs(&before, &after);

    assert!(delta.get_added().is_empty());
    assert_eq!(delta.get_removed().len(), 2);
    assert!(delta.get_changed().is_empty());
}

#[test]
fn diff_decision_logs_detects_changed_tier() {
    let mut before = DecisionLog::new();
    before.record(make_decision(1, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));

    let mut after = DecisionLog::new();
    after.record(make_decision(1, DecisionTier::Escalated,
        DecisionKind::NearBoundary { threshold: 1e-6 }));

    let delta = diff_decision_logs(&before, &after);

    assert!(delta.get_added().is_empty());
    assert!(delta.get_removed().is_empty());
    assert_eq!(delta.get_changed().len(), 1);
    assert!(delta.get_changed()[0].is_tier_changed());
    assert!(!delta.get_changed()[0].is_kind_changed());
}

#[test]
fn diff_decision_logs_detects_changed_margin() {
    let mut before = DecisionLog::new();
    before.record(TracedDecision::new(
        DecisionId(1), DecisionKind::Exact, DecisionTier::Deterministic, 0.5,
        DecisionContext::Tolerance { measured: 1e-8, threshold: 1e-6 },
    ));

    let mut after = DecisionLog::new();
    after.record(TracedDecision::new(
        DecisionId(1), DecisionKind::Exact, DecisionTier::Deterministic, 0.9,
        DecisionContext::Tolerance { measured: 1e-8, threshold: 1e-6 },
    ));

    let delta = diff_decision_logs(&before, &after);

    assert_eq!(delta.get_changed().len(), 1);
    assert!(!delta.get_changed()[0].is_kind_changed());
    assert!(!delta.get_changed()[0].is_tier_changed());
    assert!((delta.get_changed()[0].get_margin_delta() - 0.4).abs() < 1e-10);
}

#[test]
fn diff_decision_logs_identical_is_empty() {
    let mut log = DecisionLog::new();
    log.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
    log.record(make_decision(2, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));
    log.record(make_decision(3, DecisionTier::Escalated,
        DecisionKind::Ambiguous { fallback_applied: "snap".into() }));

    let delta = diff_decision_logs(&log, &log);

    assert!(delta.is_empty(), "Diffing a log against itself should be empty");
    assert_eq!(delta.total_changes(), 0);
}

#[test]
fn checkpoint_log_snapshot_and_delta() {
    let mut checkpoint = CheckpointLog::new();

    let mut log = DecisionLog::new();
    log.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
    checkpoint.snapshot(&log);

    log.record(make_decision(2, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));
    checkpoint.snapshot(&log);

    assert_eq!(checkpoint.step_count(), 2);
    assert!(checkpoint.delta_at(0).is_none(), "Step 0 has no predecessor");

    let delta = checkpoint.delta_at(1).expect("Step 1 should have a delta");
    assert_eq!(delta.get_added().len(), 1);
    assert_eq!(delta.get_added()[0].get_id(), DecisionId(2));
    assert!(delta.get_removed().is_empty());
}

#[test]
fn checkpoint_log_temporal_range_query() {
    let mut checkpoint = CheckpointLog::new();

    let mut log = DecisionLog::new();
    checkpoint.snapshot(&log);

    log.record(make_decision(1, DecisionTier::Deterministic, DecisionKind::Exact));
    checkpoint.snapshot(&log);

    log.record(make_decision(2, DecisionTier::NearBoundary,
        DecisionKind::NearBoundary { threshold: 1e-6 }));
    checkpoint.snapshot(&log);

    log.record(make_decision(3, DecisionTier::Escalated,
        DecisionKind::Ambiguous { fallback_applied: "snap".into() }));
    checkpoint.snapshot(&log);

    assert_eq!(checkpoint.step_count(), 4);

    let delta_0_to_3 = checkpoint.delta_between(0, 3).expect("Should have delta");
    assert_eq!(delta_0_to_3.get_added().len(), 3, "All 3 decisions should appear as added");

    let delta_1_to_3 = checkpoint.delta_between(1, 3).expect("Should have delta");
    assert_eq!(delta_1_to_3.get_added().len(), 2, "Decisions 2 and 3 should appear as added");

    assert!(checkpoint.delta_between(0, 99).is_none(), "Out of bounds should return None");
}

// =====================================================================
// Phase P3.2: Delta-Debug Tests
// =====================================================================

use super::delta_debug::delta_debug;

#[test]
fn delta_debug_finds_exact_step() {
    let result = delta_debug(100, |step| Ok(step >= 73)).unwrap();
    assert_eq!(result.get_failing_step(), 73);
    assert_eq!(result.get_total_steps(), 100);
    assert!(result.get_probes_used() <= 10, "Binary search on 100 steps should take ≤ 10 probes");
}

#[test]
fn delta_debug_failure_at_first_step() {
    let result = delta_debug(100, |step| Ok(step >= 0)).unwrap();
    assert_eq!(result.get_failing_step(), 0);
}

#[test]
fn delta_debug_failure_at_last_step() {
    let result = delta_debug(100, |step| Ok(step >= 99)).unwrap();
    assert_eq!(result.get_failing_step(), 99);
}
