//! Operation result envelope and decision types (Milestone 1B.2).
//!
//! DOMAIN: Structured return types for topology operations.
//!
//! INVARIANTS:
//! - `OperationResult` wraps every `apply_op` return value
//! - `ToleranceDecision` records are immutable once created
//! - `DecisionKind` is exhaustive for all tolerance-driven decisions
//!
//! DEPENDENCIES: None (data-only)

use std::fmt;
use std::time::Duration;

/// Categories of tolerance decisions the kernel may make.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionKind {
    /// Two vertices were merged because they were within tolerance.
    MergedVertices,
    /// Two edges were merged because they were within tolerance.
    MergedEdges,
    /// Two faces were merged as coplanar.
    MergedFaces,
    /// A near-tangent surface pair was classified.
    TangencyClassification,
    /// A sliver face was removed or merged.
    SliverRemoval,
    /// A gap was automatically closed.
    GapClosed,
    /// Precision was escalated due to bit-length budget.
    PrecisionEscalated,
}

/// Unique identifier for a tolerance decision (Milestone 1B.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecisionId(pub u64);

impl fmt::Display for DecisionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "decision-{}", self.0)
    }
}

/// A recorded tolerance-driven decision (Doctrine D2).
///
/// Every time the kernel makes a judgment call based on tolerance thresholds,
/// it creates one of these. The user can query "show every decision in this
/// model" and override any of them.
#[derive(Debug, Clone)]
pub struct ToleranceDecision {
    id: DecisionId,
    kind: DecisionKind,
    location: [f64; 3],
    margin: f64,
    threshold: f64,
    overridable: bool,
}

impl ToleranceDecision {
    /// Create a new tolerance decision record.
    pub fn new(id: DecisionId, kind: DecisionKind, location: [f64; 3], margin: f64, threshold: f64) -> Self {
        Self {
            id,
            kind,
            location,
            margin,
            threshold,
            overridable: true,
        }
    }

    /// The unique decision identifier.
    pub fn get_id(&self) -> DecisionId {
        self.id
    }

    /// The kind of decision that was made.
    pub fn get_kind(&self) -> &DecisionKind {
        &self.kind
    }

    /// The 3D location where the decision was made.
    pub fn get_location(&self) -> &[f64; 3] {
        &self.location
    }

    /// How close to the threshold (lower = more marginal).
    pub fn get_margin(&self) -> f64 {
        self.margin
    }

    /// The threshold that was applied.
    pub fn get_threshold(&self) -> f64 {
        self.threshold
    }

    /// Whether the user can override this decision.
    pub fn is_overridable(&self) -> bool {
        self.overridable
    }

    /// Set whether the user can override this decision.
    pub fn set_overridable(&mut self, overridable: bool) {
        self.overridable = overridable;
    }
}

/// Non-fatal warning emitted during an operation.
#[derive(Debug, Clone)]
pub enum KernelWarning {
    /// A sliver face was created (area below threshold).
    SliverFaceCreated {
        face_index: u32,
        area: f64,
        threshold: f64,
    },
    /// A near-degenerate edge was created (length below threshold).
    ShortEdgeCreated {
        halfedge_index: u32,
        length: f64,
        threshold: f64,
    },
    /// A tolerance decision was made automatically.
    AutoDecision {
        decision_id: DecisionId,
    },
}

impl fmt::Display for KernelWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelWarning::SliverFaceCreated { face_index, area, threshold } => {
                write!(f, "Sliver face {} (area {:.2e}, threshold {:.2e})", face_index, area, threshold)
            }
            KernelWarning::ShortEdgeCreated { halfedge_index, length, threshold } => {
                write!(f, "Short edge {} (length {:.2e}, threshold {:.2e})", halfedge_index, length, threshold)
            }
            KernelWarning::AutoDecision { decision_id } => {
                write!(f, "Automatic tolerance decision: {}", decision_id)
            }
        }
    }
}

/// Performance metrics for an operation.
#[derive(Debug, Clone, Default)]
pub struct OperationMetrics {
    /// Wall-clock duration of the operation.
    pub duration: Duration,
    /// Number of entities created during the operation.
    pub entities_created: u32,
    /// Number of entities deleted during the operation.
    pub entities_deleted: u32,
    /// Number of entities modified during the operation.
    pub entities_modified: u32,
}

/// Summary of lineage changes from an operation.
#[derive(Debug, Clone, Default)]
pub struct LineageDelta {
    /// Number of faces created.
    pub faces_created: u32,
    /// Number of faces deleted.
    pub faces_deleted: u32,
    /// Number of half-edges created.
    pub half_edges_created: u32,
    /// Number of half-edges deleted.
    pub half_edges_deleted: u32,
    /// Number of vertices created.
    pub vertices_created: u32,
    /// Number of vertices deleted.
    pub vertices_deleted: u32,
    /// Number of loops created.
    pub loops_created: u32,
    /// Number of loops deleted.
    pub loops_deleted: u32,
}

/// Structured envelope wrapping every operation's return value.
///
/// Carries the primary result alongside warnings, tolerance decisions,
/// performance metrics, and lineage changes.
///
/// # Example
/// ```
/// use forge_core::result::{OperationResult, OperationMetrics, LineageDelta};
///
/// let result: OperationResult<i32> = OperationResult::new(42);
/// assert_eq!(*result.get_value(), 42);
/// assert!(result.get_warnings().is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct OperationResult<T> {
    value: T,
    warnings: Vec<KernelWarning>,
    decisions: Vec<ToleranceDecision>,
    metrics: OperationMetrics,
    lineage_delta: LineageDelta,
}

impl<T> OperationResult<T> {
    /// Create a new operation result with no warnings or decisions.
    pub fn new(value: T) -> Self {
        Self {
            value,
            warnings: Vec::new(),
            decisions: Vec::new(),
            metrics: OperationMetrics::default(),
            lineage_delta: LineageDelta::default(),
        }
    }

    /// Create an operation result with full metadata.
    pub fn with_metadata(
        value: T,
        warnings: Vec<KernelWarning>,
        decisions: Vec<ToleranceDecision>,
        metrics: OperationMetrics,
        lineage_delta: LineageDelta,
    ) -> Self {
        Self { value, warnings, decisions, metrics, lineage_delta }
    }

    /// The primary return value of the operation.
    pub fn get_value(&self) -> &T {
        &self.value
    }

    /// Consume the result and return the inner value.
    pub fn into_value(self) -> T {
        self.value
    }

    /// Non-fatal warnings emitted during the operation.
    pub fn get_warnings(&self) -> &[KernelWarning] {
        &self.warnings
    }

    /// Tolerance decisions made during the operation.
    pub fn get_decisions(&self) -> &[ToleranceDecision] {
        &self.decisions
    }

    /// Performance metrics for the operation.
    pub fn get_metrics(&self) -> &OperationMetrics {
        &self.metrics
    }

    /// Summary of lineage changes from the operation.
    pub fn get_lineage_delta(&self) -> &LineageDelta {
        &self.lineage_delta
    }

    /// Whether any warnings were emitted.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Whether any tolerance decisions were made.
    pub fn has_decisions(&self) -> bool {
        !self.decisions.is_empty()
    }

    /// Set the operation metrics.
    pub fn set_metrics(&mut self, metrics: OperationMetrics) {
        self.metrics = metrics;
    }

    /// Set the lineage delta.
    pub fn set_lineage_delta(&mut self, delta: LineageDelta) {
        self.lineage_delta = delta;
    }

    /// Add a warning.
    pub fn add_warning(&mut self, warning: KernelWarning) {
        self.warnings.push(warning);
    }

    /// Add a tolerance decision.
    pub fn add_decision(&mut self, decision: ToleranceDecision) {
        self.decisions.push(decision);
    }
}

/// Trait for querying the decision log (Milestone 1B.3).
///
/// Implemented by `ModelingContext` in `forge-kernel`.
pub trait DecisionLog {
    /// All tolerance decisions recorded.
    fn get_decisions(&self) -> &[ToleranceDecision];

    /// Look up a specific decision by ID.
    fn get_decision(&self, id: DecisionId) -> Option<&ToleranceDecision>;

    /// Filter decisions by kind.
    fn get_decisions_by_kind(&self, kind: &DecisionKind) -> Vec<&ToleranceDecision>;

    /// Return the N most marginal decisions (smallest margin/threshold ratio).
    fn get_most_marginal(&self, n: usize) -> Vec<&ToleranceDecision>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_result_new_has_empty_metadata() {
        let result: OperationResult<i32> = OperationResult::new(42);
        assert_eq!(*result.get_value(), 42);
        assert!(result.get_warnings().is_empty());
        assert!(result.get_decisions().is_empty());
        assert_eq!(result.get_metrics().entities_created, 0);
        assert!(!result.has_warnings());
        assert!(!result.has_decisions());
    }

    #[test]
    fn operation_result_into_value() {
        let result = OperationResult::new(String::from("hello"));
        let value = result.into_value();
        assert_eq!(value, "hello");
    }

    #[test]
    fn operation_result_add_warning() {
        let mut result = OperationResult::new(0);
        result.add_warning(KernelWarning::SliverFaceCreated {
            face_index: 3,
            area: 1e-12,
            threshold: 1e-10,
        });
        assert!(result.has_warnings());
        assert_eq!(result.get_warnings().len(), 1);
    }

    #[test]
    fn tolerance_decision_creation() {
        let decision = ToleranceDecision::new(
            DecisionId(1),
            DecisionKind::MergedVertices,
            [1.0, 2.0, 3.0],
            1e-8,
            1e-6,
        );
        assert_eq!(decision.get_id(), DecisionId(1));
        assert_eq!(*decision.get_kind(), DecisionKind::MergedVertices);
        assert!(decision.is_overridable());
    }

    #[test]
    fn decision_id_display() {
        let id = DecisionId(42);
        assert_eq!(format!("{}", id), "decision-42");
    }

    #[test]
    fn with_metadata_constructor() {
        let result = OperationResult::with_metadata(
            99,
            vec![KernelWarning::AutoDecision { decision_id: DecisionId(1) }],
            vec![],
            OperationMetrics { duration: Duration::from_millis(5), entities_created: 3, entities_deleted: 1, entities_modified: 0 },
            LineageDelta { faces_created: 1, ..LineageDelta::default() },
        );
        assert_eq!(*result.get_value(), 99);
        assert!(result.has_warnings());
        assert_eq!(result.get_metrics().entities_created, 3);
        assert_eq!(result.get_lineage_delta().faces_created, 1);
    }
}
