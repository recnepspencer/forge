//! Non-fatal warning types emitted during kernel operations.

use serde::{Deserialize, Serialize};

use crate::tracing::DecisionId;

/// Non-fatal warning emitted during an operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KernelWarning {
    /// A sliver face was created (area below threshold).
    SliverFaceCreated {
        /// Index of the sliver face.
        face_index: u32,
        /// Computed area of the face.
        area: f64,
        /// Threshold below which a face is a sliver.
        threshold: f64,
    },
    /// A near-degenerate edge was created (length below threshold).
    ShortEdgeCreated {
        /// Index of the short edge.
        halfedge_index: u32,
        /// Computed length.
        length: f64,
        /// Minimum acceptable length threshold.
        threshold: f64,
    },
    /// A tolerance decision was made automatically.
    AutoDecision {
        /// The decision that was auto-applied.
        decision_id: DecisionId,
    },
    /// The cumulative floating-point error budget has been exhausted.
    ///
    /// Accumulated from per-vertex tolerance deltas across the operation chain.
    /// Non-fatal: the operation still succeeds; the caller decides whether to abort.
    ErrorBudgetExceeded {
        /// Total accumulated error (mm) across all operations so far.
        accumulated_mm: f64,
        /// Configured budget threshold (mm) that was exceeded.
        threshold_mm: f64,
    },
    /// A healed vertex with large tolerance participated in a tight-tolerance operation.
    ///
    /// Emitted when a vertex with `ToleranceRegime::Healed` enters an operation
    /// whose `global_default()` is tighter than the vertex's healing tolerance.
    RegimeMismatch {
        /// The healing tolerance of the incoming vertex (mm).
        healing_tolerance_mm: f64,
        /// The operation's `global_default()` tolerance (mm).
        operation_tolerance: f64,
    },
}
