//! Modeling context — the policy engine for kernel operations.
//!
//! DOMAIN: Main entry point for kernel policy decisions and tracing.
//! INVARIANTS: Every tolerance decision is logged (Doctrine D2).
//! DEPENDENCIES: `forge-core` (DecisionLog, TracedDecision), `tolerance` (policy structs)
//!
//! # Arena Snapshots
//!
//! `ArenaSnapshot` captures the slot counts of a `TopologyArena` at a point
//! in time. By comparing two snapshots, `compute_topology_delta` produces a
//! `TopologyDelta` listing every entity created between them.
//!
//! # Architecture (Doctrine D2)
//!
//! The `ModelingContext` carries all policy configuration and records every
//! tolerance-driven decision. When the kernel encounters ambiguity in curved
//! geometry (Phase 3+), it either:
//! - Applies the configured policy and logs a `TracedDecision`, or
//! - Returns `KernelError::PolicyRequired` for the caller to decide
//!
//! Silent heuristic decisions are forbidden. Every judgment call is traceable.

use std::collections::BTreeMap;

use forge_core::{
    TracedDecision, DecisionKind, DecisionContext, DecisionId, DecisionLog, DecisionTier,
    TopologyDelta,
};
use forge_topo::arena::TopologyArena;

use crate::operations::boolean::FaceClassification;

use crate::analysis::proof_validation::checkpoint::ValidationConfig;

use super::tolerance::{
    TolerancePolicy, TangencyPolicy, SliverPolicy,
    GapClosurePolicy, PrecisionEscalationPolicy, ToleranceConfig,
};

/// The modeling context that governs all policy decisions.
///
/// Passed to operations that may encounter ambiguity. Records every
/// tolerance-driven decision for traceability (D2) and replay (D1).
///
/// # Example
/// ```
/// use forge_kernel::core::ModelingContext;
///
/// let ctx = ModelingContext::default();
/// assert_eq!(ctx.get_decision_count(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct ModelingContext {
    tolerance: TolerancePolicy,
    tangency: TangencyPolicy,
    sliver: SliverPolicy,
    gap_closure: GapClosurePolicy,
    precision: PrecisionEscalationPolicy,
    tolerance_config: ToleranceConfig,
    validation_config: ValidationConfig,
    decision_log: DecisionLog,
    decision_counter: u64,
    /// When true, Drop persists the DecisionLog as an error trace
    /// if take_decision_log() was never called (i.e. the operation failed).
    auto_persist: bool,
    /// Set by take_decision_log() to indicate the success path was taken.
    log_drained: bool,
    /// Forced classification overrides for counterfactual replay.
    ///
    /// Keyed by `DecisionId` raw value (face index). When the classify
    /// phase encounters a matching decision, it uses the forced
    /// `FaceClassification` instead of the computed result.
    classification_overrides: BTreeMap<u64, FaceClassification>,
}

impl ModelingContext {
    /// Create a modeling context with default policies.
    pub fn new() -> Self {
        Self {
            tolerance: TolerancePolicy::default(),
            tangency: TangencyPolicy::default(),
            sliver: SliverPolicy::default(),
            gap_closure: GapClosurePolicy::default(),
            precision: PrecisionEscalationPolicy::default(),
            tolerance_config: ToleranceConfig::default(),
            validation_config: ValidationConfig::default(),
            decision_log: DecisionLog::new(),
            decision_counter: 0,
            auto_persist: false,
            log_drained: false,
            classification_overrides: BTreeMap::new(),
        }
    }

    /// Enable auto-persist on Drop. Call this on contexts used by
    /// top-level operations (e.g. `execute_boolean`) so that error
    /// traces are captured when the operation fails.
    pub fn enable_auto_persist(&mut self) {
        self.auto_persist = true;
    }

    /// Get the spatial tolerance policy.
    pub fn get_tolerance(&self) -> &TolerancePolicy {
        &self.tolerance
    }

    /// Set the spatial tolerance policy.
    pub fn set_tolerance(&mut self, policy: TolerancePolicy) {
        self.tolerance = policy;
    }

    /// Get the tangency handling policy.
    pub fn get_tangency(&self) -> &TangencyPolicy {
        &self.tangency
    }

    /// Set the tangency handling policy.
    pub fn set_tangency(&mut self, policy: TangencyPolicy) {
        self.tangency = policy;
    }

    /// Get the sliver face policy.
    pub fn get_sliver(&self) -> &SliverPolicy {
        &self.sliver
    }

    /// Set the sliver face policy.
    pub fn set_sliver(&mut self, policy: SliverPolicy) {
        self.sliver = policy;
    }

    /// Get the gap closure policy.
    pub fn get_gap_closure(&self) -> &GapClosurePolicy {
        &self.gap_closure
    }

    /// Set the gap closure policy.
    pub fn set_gap_closure(&mut self, policy: GapClosurePolicy) {
        self.gap_closure = policy;
    }

    /// Get the precision escalation policy.
    pub fn get_precision(&self) -> &PrecisionEscalationPolicy {
        &self.precision
    }

    /// Set the precision escalation policy.
    pub fn set_precision(&mut self, policy: PrecisionEscalationPolicy) {
        self.precision = policy;
    }

    /// Get the geometry-layer tolerance configuration.
    pub fn get_tolerance_config(&self) -> &ToleranceConfig {
        &self.tolerance_config
    }

    /// Set the geometry-layer tolerance configuration.
    pub fn set_tolerance_config(&mut self, config: ToleranceConfig) {
        self.tolerance_config = config;
    }

    /// Get the validation checkpoint configuration.
    pub fn get_validation_config(&self) -> &ValidationConfig {
        &self.validation_config
    }

    /// Set the validation checkpoint configuration.
    pub fn set_validation_config(&mut self, config: ValidationConfig) {
        self.validation_config = config;
    }

    /// Record a tolerance decision, auto-assigning a unique ID.
    pub fn log_decision(
        &mut self,
        kind: DecisionKind,
        tier: DecisionTier,
        _location: [f64; 3],
        margin: f64,
        threshold: f64,
    ) {
        self.decision_counter += 1;
        let decision = TracedDecision::new(
            DecisionId(self.decision_counter),
            kind,
            tier,
            margin,
            DecisionContext::Tolerance { measured: margin, threshold },
        );
        self.decision_log.record(decision);
    }

    /// Record a precision escalation decision, auto-assigning a unique ID.
    pub fn log_escalation(
        &mut self,
        escalation: forge_math::arithmetic::precision::PrecisionEscalation,
    ) {
        if escalation.resolved_at > forge_math::arithmetic::precision::PrecisionMode::Float64 {
            self.decision_counter += 1;
            let decision = TracedDecision::new(
                DecisionId(self.decision_counter),
                DecisionKind::Exact,
                DecisionTier::Escalated,
                escalation.disagreement_magnitude.unwrap_or(0.0),
                DecisionContext::PrecisionEscalation { escalation },
            );
            self.decision_log.record(decision);
        }
    }

    /// Execute `f` within a named span, recording start/end events.
    pub fn scope<F, R>(&mut self, name: &'static str, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let span_id = self.decision_log.start_span(name);
        let start = std::time::Instant::now();
        let result = f(self);
        let duration_micros = start.elapsed().as_micros() as u64;
        self.decision_log.end_span(span_id, duration_micros);
        result
    }

    /// Clear the decision log (for starting a fresh operation).
    pub fn clear_decisions(&mut self) {
        self.decision_log = DecisionLog::new();
    }

    /// Returns the number of tolerance decisions made.
    pub fn get_decision_count(&self) -> usize {
        self.decision_log.len()
    }

    /// Get the decision log.
    pub fn get_decision_log(&self) -> &DecisionLog {
        &self.decision_log
    }

    /// Get mutable access to the decision log.
    pub fn get_decision_log_mut(&mut self) -> &mut DecisionLog {
        &mut self.decision_log
    }

    /// Take ownership of the decision log, replacing it with an empty one.
    ///
    /// Marks the log as drained so `Drop` knows the success path was taken.
    pub fn take_decision_log(&mut self) -> DecisionLog {
        self.log_drained = true;
        std::mem::take(&mut self.decision_log)
    }

    /// Set a forced classification override for counterfactual replay.
    ///
    /// When the classify phase encounters a decision with this ID,
    /// it uses the forced `FaceClassification` instead of computing
    /// the result from ray-casting. This enables re-executing the
    /// Boolean pipeline with different classification outcomes.
    pub fn set_classification_override(
        &mut self,
        decision_id: DecisionId,
        classification: FaceClassification,
    ) {
        self.classification_overrides.insert(decision_id.0, classification);
    }

    /// Check if a classification override exists for a decision ID.
    ///
    /// Returns the forced `FaceClassification` if one was set via
    /// `set_classification_override`, or `None` for normal execution.
    pub fn get_classification_override(
        &self,
        decision_id: DecisionId,
    ) -> Option<FaceClassification> {
        self.classification_overrides.get(&decision_id.0).copied()
    }

    /// Remove all classification overrides.
    pub fn clear_classification_overrides(&mut self) {
        self.classification_overrides.clear();
    }

    /// Generate a divergence report from the current decision log (P2.3).
    ///
    /// Scans all decisions for cases where the f64 fast-path disagreed
    /// with the higher-precision answer. Returns a structured report
    /// with divergence rate, topology impact, and per-decision details.
    pub fn generate_divergence_report(&self) -> forge_core::DivergenceReport {
        forge_core::scan_for_divergences(&self.decision_log)
    }
}

// ── Arena Snapshot for topology delta capture ──────────────────────────────

/// A lightweight snapshot of arena slot counts at a point in time.
///
/// Used to compute `TopologyDelta` — the set of entities created
/// between two snapshots (pre-op and post-op).
#[derive(Debug, Clone)]
pub struct ArenaSnapshot {
    face_slots: usize,
    half_edge_slots: usize,
    vertex_slots: usize,
}

impl ArenaSnapshot {
    /// Capture the current slot counts of an arena.
    pub fn capture(arena: &TopologyArena) -> Self {
        Self {
            face_slots: arena.face_slot_count(),
            half_edge_slots: arena.half_edge_slot_count(),
            vertex_slots: arena.vertex_slot_count(),
        }
    }
}

/// Compute the topology delta between a pre-operation snapshot and the
/// current arena state.
///
/// Any slot indices in `[snapshot.X_slots .. arena.X_slot_count())` are
/// entities created since the snapshot was taken.
pub fn compute_topology_delta(snapshot: &ArenaSnapshot, arena: &TopologyArena) -> TopologyDelta {
    let created_faces: Vec<u32> = (snapshot.face_slots..arena.face_slot_count())
        .filter(|&i| arena.face_generation(i).is_some())
        .map(|i| i as u32)
        .collect();

    let created_halfedges: Vec<u32> = (snapshot.half_edge_slots..arena.half_edge_slot_count())
        .filter(|&i| arena.half_edge_generation(i).is_some())
        .map(|i| i as u32)
        .collect();

    let created_vertices: Vec<u32> = (snapshot.vertex_slots..arena.vertex_slot_count())
        .filter(|&i| arena.vertex_generation(i).is_some())
        .map(|i| i as u32)
        .collect();

    TopologyDelta {
        created_faces,
        created_halfedges,
        created_vertices,
        deleted_faces: Vec::new(),
        deleted_halfedges: Vec::new(),
        deleted_vertices: Vec::new(),
    }
}

impl Drop for ModelingContext {
    /// Auto-persist the DecisionLog on error or panic.
    ///
    /// Only fires when `auto_persist` is enabled (top-level operations)
    /// AND `take_decision_log()` was never called (the error path).
    fn drop(&mut self) {
        if !self.auto_persist || self.log_drained || self.decision_log.is_empty() {
            return;
        }

        let dir = match forge_core::resolve_trace_dir() {
            Some(d) => d,
            None => return,
        };

        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            forge_core::write_trace_file(&dir, &self.decision_log, 0, "error");
        }));
    }
}

impl Default for ModelingContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check_tolerance;

    #[test]
    fn default_context_has_no_decisions() {
        let ctx = ModelingContext::new();
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn decisions_are_recorded() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::NearBoundary { threshold: 1e-6 },
            DecisionTier::NearBoundary,
            [1.0, 2.0, 3.0],
            1e-8,
            1e-6,
        );
        assert_eq!(ctx.get_decision_count(), 1);
        let decisions: Vec<_> = ctx.get_decision_log().decisions().collect();
        assert_eq!(decisions[0].get_id(), DecisionId(1));
    }

    #[test]
    fn check_tolerance_macro_logs_when_within() {
        let mut ctx = ModelingContext::new();
        let distance = 1e-8;
        let threshold = 1e-6;
        let location = [0.0, 0.0, 0.0];

        let within = check_tolerance!(ctx, threshold, distance, location, DecisionKind::NearBoundary { threshold });

        assert!(within);
        assert_eq!(ctx.get_decision_count(), 1);
    }

    #[test]
    fn check_tolerance_macro_does_not_log_when_outside() {
        let mut ctx = ModelingContext::new();
        let distance = 1e-3;
        let threshold = 1e-6;
        let location = [0.0, 0.0, 0.0];

        let within = check_tolerance!(ctx, threshold, distance, location, DecisionKind::NearBoundary { threshold });

        assert!(!within);
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn take_decision_log_drains() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1e-6,
        );
        assert_eq!(ctx.get_decision_count(), 1);

        let log = ctx.take_decision_log();
        assert_eq!(log.len(), 1);
        assert_eq!(ctx.get_decision_count(), 0);
    }
}
