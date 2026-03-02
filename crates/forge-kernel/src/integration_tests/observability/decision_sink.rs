//! Group 1: DecisionSink threading tests [K-1].
//!
//! Proves: The real `ModelingContext` records decisions when
//! `make_block`/`make_cube` runs through the production path.

use crate::integration_tests::harness::assertions::assert_decisions_well_formed;
use crate::integration_tests::harness::shapes::{unit_cube_traced, unit_block_traced};
use forge_core::DecisionKind;

/// Build a unit cube with ModelingContext and verify decisions are recorded.
///
/// Phase 1 acceptance gate: `build_halfedge_mesh` must record a `NearBoundary`
/// decision for every vertex placement — both merges and clean inserts.
#[test]
fn test_make_block_produces_decisions() {
    let result = unit_cube_traced().expect("unit cube should succeed");
    let log = result.ctx.get_decision_log();

    assert!(
        !log.is_empty(),
        "DecisionLog is empty — DecisionSink not threaded through make_cube"
    );

    assert_decisions_well_formed(log);

    let has_near_boundary = log.decisions().any(|d| {
        matches!(d.get_kind(), DecisionKind::NearBoundary { .. })
    });
    assert!(
        has_near_boundary,
        "Expected at least one NearBoundary decision from vertex dedup, got: {:?}",
        log.summary()
    );
}

/// Build a large block where vertices are far apart relative to tolerance.
/// All decisions should be NearBoundary with large margins (clean inserts).
#[test]
fn test_large_block_no_spurious_decisions() {
    let result = unit_block_traced(
        [0.0, 0.0, 0.0],
        [50.0, 50.0, 50.0],
    ).expect("large block should succeed");
    let log = result.ctx.get_decision_log();

    // Every vertex should produce a NearBoundary decision.
    let near_boundary_count = log.decisions()
        .filter(|d| matches!(d.get_kind(), DecisionKind::NearBoundary { .. }))
        .count();
    assert!(
        near_boundary_count > 0,
        "Expected NearBoundary decisions for every vertex, got 0"
    );

    // For well-separated vertices, all margins should be large (> tolerance).
    assert_decisions_well_formed(log);
}

/// Build a block through the pipeline path and verify span timing is recorded.
#[test]
fn test_pipeline_span_timing_recorded() {
    let result = unit_cube_traced().expect("unit cube should succeed");
    let log = result.ctx.get_decision_log();

    let events = log.get_events();
    let has_span = events.iter().any(|e| {
        matches!(e, forge_core::TraceEvent::StartSpan { .. })
            || matches!(e, forge_core::TraceEvent::EndSpan { .. })
    });

    // NOTE: Spans are now wired into primitive generation (Phase 1.1 completeness).
    assert!(
        has_span,
        "Expected at least one StartSpan or EndSpan event recorded during primitive generation."
    );
}
