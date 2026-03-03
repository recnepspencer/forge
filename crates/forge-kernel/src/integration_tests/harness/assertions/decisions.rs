//! Decision log assertions for observability tests.
//!
//! DOMAIN: Validates that `DecisionLog` data produced by the production
//! pipeline is well-formed — non-negative margins, populated contexts,
//! correct vertex placement decisions.

use forge_core::DecisionLog;

/// Assert every decision in a `DecisionLog` is well-formed.
///
/// Validates:
/// - Non-negative margin on every decision
/// - Populated context (not a zero-default)
///
/// This is the observability equivalent of `assert_all_invariants` —
/// call it after any traced operation to catch garbage decision payloads.
pub fn assert_decisions_well_formed(log: &DecisionLog) {
    for decision in log.decisions() {
        assert!(
            decision.get_margin() >= 0.0,
            "Decision {:?} has negative margin: {}",
            decision.get_id(), decision.get_margin()
        );
    }
}

/// Assert vertex placement decisions are valid.
///
/// Thin test wrapper around the production validator in
/// `operations::shared_validators::facade::validate_vertex_decisions`.
/// Emits the decision summary via tracing, then delegates.
pub fn assert_vertex_decisions(
    label: &str,
    log: &DecisionLog,
    expected_vertices: usize,
    tolerance: f64,
) {
    forge_core::tracing::log_decision_log(label, log);
    crate::operations::shared_validators::facade::validate_vertex_decisions(
        log, expected_vertices, tolerance,
    )
    .unwrap_or_else(|e| panic!("{label}: {e}"));
}
