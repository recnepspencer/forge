//! Display implementation for `KernelWarning`.

use std::fmt;

use crate::envelope::data::KernelWarning;

impl fmt::Display for KernelWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelWarning::SliverFaceCreated {
                face_index,
                area,
                threshold,
            } => {
                write!(
                    f,
                    "Sliver face {} (area {:.2e}, threshold {:.2e})",
                    face_index, area, threshold
                )
            }
            KernelWarning::ShortEdgeCreated {
                halfedge_index,
                length,
                threshold,
            } => {
                write!(
                    f,
                    "Short edge {} (length {:.2e}, threshold {:.2e})",
                    halfedge_index, length, threshold
                )
            }
            KernelWarning::AutoDecision { decision_id } => {
                write!(f, "Automatic tolerance decision: {}", decision_id)
            }
            KernelWarning::ErrorBudgetExceeded {
                accumulated_mm,
                threshold_mm,
            } => {
                write!(
                    f,
                    "Error budget exceeded: {:.2e} mm accumulated (threshold {:.2e} mm)",
                    accumulated_mm, threshold_mm
                )
            }
            KernelWarning::RegimeMismatch {
                healing_tolerance_mm,
                operation_tolerance,
            } => {
                write!(
                    f,
                    "Regime mismatch: healed vertex tol={:.2e} mm in op with tol={:.2e} mm",
                    healing_tolerance_mm, operation_tolerance
                )
            }
        }
    }
}
