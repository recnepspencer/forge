//! Modeling context, tolerance policies, and the `check_tolerance!` macro.
//!
//! # Architecture (Doctrine D2)
//!
//! The `ModelingContext` carries all policy configuration and records every
//! tolerance-driven decision. When the kernel encounters ambiguity in curved
//! geometry (Phase 3+), it either:
//! - Applies the configured policy and logs a `ToleranceDecision`, or
//! - Returns `KernelError::PolicyRequired` for the caller to decide
//!
//! Silent heuristic decisions are forbidden. Every judgment call is traceable.
//!
//! # Note on Timing
//!
//! Phases 0–2 use exact predicates on planar geometry — there are zero
//! tolerance decisions. This module exists now to establish the API shape,
//! but the policies won't be actively used until Phase 3 (curved surfaces).

use forge_core::{ToleranceDecision, DecisionKind, DecisionId, DecisionLog};

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
#[derive(Debug, Clone, Default)]
pub struct ModelingContext {
    tolerance: TolerancePolicy,
    tangency: TangencyPolicy,
    sliver: SliverPolicy,
    gap_closure: GapClosurePolicy,
    precision: PrecisionEscalationPolicy,
    tolerance_config: ToleranceConfig,
    decisions: Vec<ToleranceDecision>,
    decision_counter: u64,
}

impl ModelingContext {
    /// Create a modeling context with default policies.
    pub fn new() -> Self {
        Self::default()
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

    /// Record a tolerance decision, auto-assigning a unique DecisionId.
    pub fn log_decision(&mut self, kind: DecisionKind, location: [f64; 3], margin: f64, threshold: f64) {
        self.decision_counter += 1;
        let decision = ToleranceDecision::new(
            DecisionId(self.decision_counter),
            kind,
            location,
            margin,
            threshold,
        );
        self.decisions.push(decision);
    }

    /// Clear the decision log (for starting a fresh operation).
    pub fn clear_decisions(&mut self) {
        self.decisions.clear();
    }

    /// Returns the number of tolerance decisions made.
    pub fn get_decision_count(&self) -> usize {
        self.decisions.len()
    }
}

impl DecisionLog for ModelingContext {
    fn get_decisions(&self) -> &[ToleranceDecision] {
        &self.decisions
    }

    fn get_decision(&self, id: DecisionId) -> Option<&ToleranceDecision> {
        self.decisions.iter().find(|d| d.get_id() == id)
    }

    fn get_decisions_by_kind(&self, kind: &DecisionKind) -> Vec<&ToleranceDecision> {
        self.decisions.iter().filter(|d| d.get_kind() == kind).collect()
    }

    fn get_most_marginal(&self, n: usize) -> Vec<&ToleranceDecision> {
        let mut refs: Vec<&ToleranceDecision> = self.decisions.iter().collect();
        refs.sort_by(|a, b| {
            let ratio_a = a.get_margin() / a.get_threshold();
            let ratio_b = b.get_margin() / b.get_threshold();
            ratio_a.partial_cmp(&ratio_b).unwrap_or(std::cmp::Ordering::Equal)
        });
        refs.truncate(n);
        refs
    }
}



/// Spatial tolerance policy for coincidence detection.
#[derive(Debug, Clone)]
pub struct TolerancePolicy {
    spatial_tolerance: f64,
    angular_tolerance: f64,
}

impl TolerancePolicy {
    /// Create a tolerance policy with explicit values.
    pub fn new(spatial_tolerance: f64, angular_tolerance: f64) -> Self {
        Self {
            spatial_tolerance,
            angular_tolerance,
        }
    }

    /// Distance below which two points are considered coincident (meters).
    pub fn get_spatial_tolerance(&self) -> f64 {
        self.spatial_tolerance
    }

    /// Set the spatial coincidence tolerance.
    pub fn set_spatial_tolerance(&mut self, value: f64) {
        self.spatial_tolerance = value;
    }

    /// Angular tolerance for direction comparisons (radians).
    pub fn get_angular_tolerance(&self) -> f64 {
        self.angular_tolerance
    }

    /// Set the angular comparison tolerance.
    pub fn set_angular_tolerance(&mut self, value: f64) {
        self.angular_tolerance = value;
    }
}

impl Default for TolerancePolicy {
    fn default() -> Self {
        Self {
            spatial_tolerance: DEFAULT_SPATIAL_TOLERANCE,
            angular_tolerance: DEFAULT_ANGULAR_TOLERANCE,
        }
    }
}

/// Default spatial tolerance: 1 micron.
const DEFAULT_SPATIAL_TOLERANCE: f64 = 1e-6;
/// Default angular tolerance: ~0.00006 degrees.
const DEFAULT_ANGULAR_TOLERANCE: f64 = 1e-6;

/// Policy for handling near-tangent surface intersections.
#[derive(Debug, Clone)]
pub struct TangencyPolicy {
    min_transversal_angle: f64,
    max_tangent_gap: f64,
}

impl TangencyPolicy {
    /// Create a tangency policy with explicit values.
    pub fn new(min_transversal_angle: f64, max_tangent_gap: f64) -> Self {
        Self {
            min_transversal_angle,
            max_tangent_gap,
        }
    }

    /// Minimum angle between surfaces to classify as transversal (radians).
    pub fn get_min_transversal_angle(&self) -> f64 {
        self.min_transversal_angle
    }

    /// Set the minimum transversal angle.
    pub fn set_min_transversal_angle(&mut self, value: f64) {
        self.min_transversal_angle = value;
    }

    /// Maximum gap in near-tangent regions before escalating to policy decision.
    pub fn get_max_tangent_gap(&self) -> f64 {
        self.max_tangent_gap
    }

    /// Set the maximum tangent gap.
    pub fn set_max_tangent_gap(&mut self, value: f64) {
        self.max_tangent_gap = value;
    }
}

impl Default for TangencyPolicy {
    fn default() -> Self {
        Self {
            min_transversal_angle: DEFAULT_MIN_TRANSVERSAL_ANGLE,
            max_tangent_gap: DEFAULT_MAX_TANGENT_GAP,
        }
    }
}

/// Default minimum transversal angle: ~0.06 degrees.
const DEFAULT_MIN_TRANSVERSAL_ANGLE: f64 = 1e-3;
/// Default maximum tangent gap: 0.1mm.
const DEFAULT_MAX_TANGENT_GAP: f64 = 1e-4;

/// Policy for sliver face detection and removal.
#[derive(Debug, Clone)]
pub struct SliverPolicy {
    min_face_area: f64,
    max_slivers_per_op: usize,
}

impl SliverPolicy {
    /// Create a sliver policy with explicit values.
    pub fn new(min_face_area: f64, max_slivers_per_op: usize) -> Self {
        Self {
            min_face_area,
            max_slivers_per_op,
        }
    }

    /// Minimum face area (m²) — faces below this are slivers.
    pub fn get_min_face_area(&self) -> f64 {
        self.min_face_area
    }

    /// Set the minimum face area.
    pub fn set_min_face_area(&mut self, value: f64) {
        self.min_face_area = value;
    }

    /// Maximum number of slivers an operation may create before requiring explicit waiver.
    pub fn get_max_slivers_per_op(&self) -> usize {
        self.max_slivers_per_op
    }

    /// Set the maximum slivers per operation.
    pub fn set_max_slivers_per_op(&mut self, value: usize) {
        self.max_slivers_per_op = value;
    }
}

impl Default for SliverPolicy {
    fn default() -> Self {
        Self {
            min_face_area: DEFAULT_MIN_FACE_AREA,
            max_slivers_per_op: DEFAULT_MAX_SLIVERS_PER_OP,
        }
    }
}

/// Default minimum face area: 0.01 mm².
const DEFAULT_MIN_FACE_AREA: f64 = 1e-10;
/// Default maximum slivers per operation.
const DEFAULT_MAX_SLIVERS_PER_OP: usize = 3;

/// Policy for automatic gap closure during sewing.
#[derive(Debug, Clone)]
pub struct GapClosurePolicy {
    max_gap: f64,
}

impl GapClosurePolicy {
    /// Create a gap closure policy with explicit value.
    pub fn new(max_gap: f64) -> Self {
        Self { max_gap }
    }

    /// Maximum gap that will be automatically closed (meters).
    pub fn get_max_gap(&self) -> f64 {
        self.max_gap
    }

    /// Set the maximum gap for automatic closure.
    pub fn set_max_gap(&mut self, value: f64) {
        self.max_gap = value;
    }
}

impl Default for GapClosurePolicy {
    fn default() -> Self {
        Self {
            max_gap: DEFAULT_GAP_CLOSURE_MAX,
        }
    }
}

/// Default maximum gap for closure: 0.1mm.
const DEFAULT_GAP_CLOSURE_MAX: f64 = 1e-4;

/// Policy for precision escalation (Milestone 0.2.3).
#[derive(Debug, Clone)]
pub struct PrecisionEscalationPolicy {
    bit_length_threshold: u32,
}

impl PrecisionEscalationPolicy {
    /// Create a precision escalation policy with explicit value.
    pub fn new(bit_length_threshold: u32) -> Self {
        Self { bit_length_threshold }
    }

    /// Bit-length threshold before escalating.
    pub fn get_bit_length_threshold(&self) -> u32 {
        self.bit_length_threshold
    }

    /// Set the bit-length threshold.
    pub fn set_bit_length_threshold(&mut self, value: u32) {
        self.bit_length_threshold = value;
    }
}

impl Default for PrecisionEscalationPolicy {
    fn default() -> Self {
        Self {
            bit_length_threshold: DEFAULT_BIT_LENGTH_THRESHOLD,
        }
    }
}

/// Default bit-length threshold for precision escalation.
const DEFAULT_BIT_LENGTH_THRESHOLD: u32 = 512;

/// Configurable thresholds for geometry-layer computations.
///
/// These values are used by `forge-geom` functions that accept tolerance
/// parameters (plane intersection degeneracy, overconstrained residual, etc.).
/// Defaults are suitable for unit-scale CAD (meters). Adjust for different
/// model scales or import pipeline tolerance.
#[derive(Debug, Clone)]
pub struct ToleranceConfig {
    /// Maximum acceptable residual for overconstrained vertex verification.
    residual: f64,
    /// Minimum acceptable |det| for 3-plane intersection degeneracy check.
    degeneracy: f64,
}

impl ToleranceConfig {
    /// Create a tolerance config with explicit values.
    pub fn new(residual: f64, degeneracy: f64) -> Self {
        Self { residual, degeneracy }
    }

    /// The residual tolerance for overconstrained verification.
    pub fn get_residual(&self) -> f64 {
        self.residual
    }

    /// Set the residual tolerance.
    pub fn set_residual(&mut self, value: f64) {
        self.residual = value;
    }

    /// The degeneracy threshold for plane intersection.
    pub fn get_degeneracy(&self) -> f64 {
        self.degeneracy
    }

    /// Set the degeneracy threshold.
    pub fn set_degeneracy(&mut self, value: f64) {
        self.degeneracy = value;
    }
}

impl Default for ToleranceConfig {
    fn default() -> Self {
        Self {
            residual: DEFAULT_RESIDUAL_TOLERANCE,
            degeneracy: DEFAULT_DEGENERACY_THRESHOLD,
        }
    }
}

/// Default residual tolerance for overconstrained vertex verification.
const DEFAULT_RESIDUAL_TOLERANCE: f64 = 1e-8;
/// Default degeneracy threshold for plane intersection determinants.
const DEFAULT_DEGENERACY_THRESHOLD: f64 = 1e-12;



/// Macro for cleanly checking tolerance and logging decisions.
///
/// Keeps math code readable while ensuring every tolerance decision
/// is logged (Doctrine D2).
///
/// # Usage
/// ```ignore
/// if check_tolerance!(ctx, spatial_tolerance, distance, DecisionKind::MergedVertices) {
///     // The value was within tolerance — handle accordingly
///     return Ok(TriSign::Zero);
/// }
/// ```
#[macro_export]
macro_rules! check_tolerance {
    ($ctx:expr, $threshold:expr, $value:expr, $location:expr, $kind:expr) => {{
        if $value < $threshold {
            $ctx.log_decision(
                $kind,
                $location,
                $value,
                $threshold,
            );
            true
        } else {
            false
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_context_has_no_decisions() {
        let ctx = ModelingContext::new();
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn decisions_are_recorded() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::MergedVertices,
            [1.0, 2.0, 3.0],
            1e-8,
            1e-6,
        );
        assert_eq!(ctx.get_decision_count(), 1);
        assert_eq!(*ctx.get_decisions()[0].get_kind(), DecisionKind::MergedVertices);
        assert_eq!(ctx.get_decisions()[0].get_id(), DecisionId(1));
    }

    #[test]
    fn check_tolerance_macro_logs_when_within() {
        let mut ctx = ModelingContext::new();
        let distance = 1e-8;
        let threshold = 1e-6;
        let location = [0.0, 0.0, 0.0];

        let within = check_tolerance!(ctx, threshold, distance, location, DecisionKind::MergedVertices);

        assert!(within);
        assert_eq!(ctx.get_decision_count(), 1);
    }

    #[test]
    fn check_tolerance_macro_does_not_log_when_outside() {
        let mut ctx = ModelingContext::new();
        let distance = 1e-3;
        let threshold = 1e-6;
        let location = [0.0, 0.0, 0.0];

        let within = check_tolerance!(ctx, threshold, distance, location, DecisionKind::MergedVertices);

        assert!(!within);
        assert_eq!(ctx.get_decision_count(), 0);
    }
}
