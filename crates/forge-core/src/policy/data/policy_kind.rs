//! Categories of policy decisions the kernel may request.

use serde::{Deserialize, Serialize};

/// Categories of policy decisions the kernel may request.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PolicyKind {
    /// Two geometric entities are within tolerance of coincident
    CoincidentGeometry,
    /// Two surfaces are nearly tangent over a region
    NearTangency,
    /// A face would be created below the sliver area threshold
    SliverFace,
    /// A gap exceeds the automatic sewing threshold
    GapClosure,
    /// Precision escalation budget exceeded
    PrecisionBudget,
}
