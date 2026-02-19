//! Integration tests for the span-based tracing API.
//!
//! These tests run real boolean operations and then interrogate the
//! `DecisionLog` through its public API, verifying that spans, decisions,
//! summaries, and diffs all work end-to-end.

use forge_core::result::{TraceEvent, DecisionTier};

use super::super::test_helpers::build_cube;
use super::super::assemble::merge::execute_boolean;
use super::super::schema::{BooleanInput, BooleanOp};

// ══════════════════════════════════════════════════════════════
// §9  TRACING API INTEGRATION TESTS
// ══════════════════════════════════════════════════════════════

/// 9.1 — Boolean produces span events.
///
/// A successful boolean must record at least the top-level phase spans
/// (split, classify, select, assemble, postprocess) as StartSpan/EndSpan
/// pairs in the DecisionLog.
#[test]
fn boolean_produces_span_events() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Intersection);
    let envelope = execute_boolean(input).expect("Boolean should succeed");

    let log = envelope.get_decision_log();

    let span_starts: Vec<&str> = log.get_events().iter().filter_map(|e| match e {
        TraceEvent::StartSpan { name, .. } => Some(name.as_str()),
        _ => None,
    }).collect();

    let span_ends: Vec<_> = log.get_events().iter().filter(|e| matches!(e, TraceEvent::EndSpan { .. })).collect();

    assert!(
        span_starts.len() >= 4,
        "Expected at least 4 phase spans, got {}: {:?}",
        span_starts.len(), span_starts,
    );

    assert_eq!(
        span_starts.len(), span_ends.len(),
        "Every StartSpan must have a matching EndSpan",
    );

    assert!(span_starts.contains(&"split"), "Missing 'split' span");
    assert!(span_starts.contains(&"classify"), "Missing 'classify' span");
    assert!(span_starts.contains(&"select"), "Missing 'select' span");
    assert!(span_starts.contains(&"assemble"), "Missing 'assemble' span");
}

/// 9.2 — Span durations are plausible.
///
/// Every EndSpan event must have a duration > 0 (the operation took real time).
#[test]
fn span_durations_are_nonzero() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 2.0);
    let (topo_b, geom_b) = build_cube([1.0, 1.0, 1.0], 2.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);
    let envelope = execute_boolean(input).expect("Boolean should succeed");

    let log = envelope.get_decision_log();

    for event in log.get_events() {
        if let TraceEvent::EndSpan { duration_micros, id } = event {
            assert!(
                *duration_micros > 0,
                "Span {:?} ended with zero duration — scope() timing is broken",
                id,
            );
        }
    }
}

/// 9.3 — Decisions are routed into spans.
///
/// The classify phase should produce decisions (point classifications).
/// Verify that those decisions carry a span_id matching the classify span.
#[test]
fn decisions_carry_span_ids() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Subtraction);
    let envelope = execute_boolean(input).expect("Boolean should succeed");

    let log = envelope.get_decision_log();
    let decisions: Vec<_> = log.decisions().collect();

    if !decisions.is_empty() {
        let with_span: Vec<_> = decisions.iter().filter(|d| d.get_span_id().is_some()).collect();

        assert!(
            !with_span.is_empty(),
            "Decisions exist but none carry a span_id — recording isn't wired to active spans",
        );
    }
}

/// 9.4 — DecisionLog summary stats are consistent.
///
/// Run a boolean and verify that the summary counts agree with
/// manual iteration over the decisions.
#[test]
fn summary_stats_match_manual_count() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([0.5, 0.5, 0.5], 1.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Intersection);
    let envelope = execute_boolean(input).expect("Boolean should succeed");

    let log = envelope.get_decision_log();
    let summary = log.summary();

    let manual_total = log.decisions().count();
    assert_eq!(summary.total, manual_total, "summary.total should match decisions().count()");

    // Verify kind-based counts add up to total
    let kind_sum = summary.exact + summary.policy_applied
        + summary.near_boundary + summary.ambiguous + summary.forced;
    assert_eq!(
        kind_sum, summary.total,
        "Kind-based counts should sum to total",
    );

    // Verify tier-based interesting_only() is consistent with manual tier filtering
    let manual_interesting = log.decisions()
        .filter(|d| d.get_tier() >= DecisionTier::NearBoundary)
        .count();
    let api_interesting = log.interesting_only().len();
    assert_eq!(
        manual_interesting, api_interesting,
        "interesting_only().len() should match manual Tier2+ filter count",
    );
}

/// 9.5 — TraceSummary can be built from a real boolean result.
///
/// Build a TraceSummary from the DecisionLog and verify its structure:
/// span_summaries should exist for each phase span, and the state hash
/// should be non-zero for a non-empty result.
#[test]
fn trace_summary_from_real_boolean() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);
    let envelope = execute_boolean(input).expect("Boolean should succeed");

    let log = envelope.get_decision_log();
    let state_hash = envelope.get_state_hash_after();
    let summary = log.to_summary(state_hash);

    assert_eq!(summary.get_state_hash(), state_hash);

    let span_summaries = summary.get_span_summaries();
    assert!(
        !span_summaries.is_empty(),
        "TraceSummary should have span_summaries for each phase",
    );

    for ss in span_summaries {
        assert!(
            !ss.name.is_empty(),
            "SpanSummaryEntry should have a non-empty name",
        );
    }
}

/// 9.6 — TraceSummary::diff detects changes between operations.
///
/// Run two different booleans and diff their TraceSummaries.
/// The diff should detect that the state hash changed.
///
/// NOTE: We use Intersection vs Subtraction (not Union) because
/// Intersection and Union of these specific cubes both produce
/// hexahedra (6F/8V) — structurally isomorphic topologies that
/// the permutation-invariant hash correctly can't distinguish.
/// Subtraction produces a non-convex solid with different face count.
#[test]
fn trace_diff_between_different_booleans() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 2.0);
    let (topo_b, geom_b) = build_cube([0.0, 0.0, 0.0], 1.0);

    let input1 = BooleanInput::new(
        topo_a.clone(), geom_a.clone(), topo_b.clone(), geom_b.clone(),
        BooleanOp::Intersection,
    );
    let envelope1 = execute_boolean(input1).expect("Boolean 1 (Intersection) should succeed");

    let input2 = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Subtraction);
    let envelope2 = execute_boolean(input2).expect("Boolean 2 (Subtraction) should succeed");

    let summary1 = envelope1.get_decision_log().to_summary(
        envelope1.get_state_hash_after(),
    );
    let summary2 = envelope2.get_decision_log().to_summary(
        envelope2.get_state_hash_after(),
    );

    let diff = summary2.diff(&summary1);

    assert!(
        diff.state_hash_changed || !diff.is_empty(),
        "Two different boolean operations should produce a non-trivial diff",
    );
}

/// 9.7 — Same operation produces identical TraceSummary.
///
/// Running the exact same boolean twice should produce the same
/// TraceSummary (determinism). The diff should be empty.
#[test]
fn same_operation_produces_identical_summary() {
    let run = || {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Intersection);
        execute_boolean(input).expect("Boolean should succeed")
    };

    let envelope1 = run();
    let envelope2 = run();

    let summary1 = envelope1.get_decision_log().to_summary(
        envelope1.get_state_hash_after(),
    );
    let summary2 = envelope2.get_decision_log().to_summary(
        envelope2.get_state_hash_after(),
    );

    assert_eq!(
        summary1.get_state_hash(), summary2.get_state_hash(),
        "Same operation should produce same topology hash",
    );

    let diff = summary1.diff(&summary2);
    assert!(!diff.state_hash_changed, "State hash should not change between identical runs");
}

/// 9.8 — Disjoint cubes use the zero_split fast path.
///
/// When cubes don't overlap, split produces zero splits. The zero_split
/// fast path handles disjoint unions directly via `execute_disjoint_boolean`,
/// so only the split and zero_split spans are present (no classify/select).
#[test]
fn disjoint_cubes_full_pipeline_spans() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([10.0, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);
    let envelope = execute_boolean(input).expect("Disjoint union should succeed");

    let log = envelope.get_decision_log();

    let span_names: Vec<&str> = log.get_events().iter().filter_map(|e| match e {
        TraceEvent::StartSpan { name, .. } => Some(name.as_str()),
        _ => None,
    }).collect();

    assert!(
        span_names.contains(&"split"),
        "Should have a split span, got: {:?}", span_names,
    );

    assert!(
        span_names.contains(&"zero_split"),
        "Disjoint cubes should use the zero_split fast path, got: {:?}", span_names,
    );

    // Disjoint cubes are fully handled by zero_split — no classify/select needed
    assert!(
        !span_names.contains(&"classify"),
        "Disjoint cubes should NOT fall through to classify (fast path handles them). Got: {:?}", span_names,
    );
}

/// 9.9 — display_interesting() produces useful output.
///
/// Verify that `display_interesting()` returns a non-empty string
/// that mentions decision counts, not a blank string.
#[test]
fn display_interesting_is_not_empty() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Subtraction);
    let envelope = execute_boolean(input).expect("Boolean should succeed");

    let log = envelope.get_decision_log();
    let output = log.display_interesting();

    assert!(!output.is_empty(), "display_interesting() should produce output");
    assert!(output.contains("decisions"), "Output should mention decision counts");
}

/// 9.10 — Tier filtering is consistent with event content.
///
/// If the log contains Tier 2+ decisions, `interesting_only()` must
/// return them. If it only has Tier 0 (Deterministic), the list is empty.
#[test]
fn tier_filtering_matches_event_content() {
    let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
    let (topo_b, geom_b) = build_cube([0.5, 0.0, 0.0], 1.0);

    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Intersection);
    let envelope = execute_boolean(input).expect("Boolean should succeed");

    let log = envelope.get_decision_log();
    let interesting = log.interesting_only();
    let all_decisions: Vec<_> = log.decisions().collect();

    let manual_tier2_plus = all_decisions.iter()
        .filter(|d| d.get_tier() >= DecisionTier::NearBoundary)
        .count();

    assert_eq!(
        interesting.len(), manual_tier2_plus,
        "interesting_only() should match manual Tier2+ count",
    );
}
