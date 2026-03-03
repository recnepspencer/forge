//! Geometric ambiguity result type.

use serde::{Deserialize, Serialize};

/// A geometric result that requires a policy decision.
///
/// This carries ONLY geometric data. No policy categories or modeling
/// concepts are allowed in the math/geom layers (Rule 2.1).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbiguousResult {
    /// 3D location where the ambiguity occurred.
    pub location: [f64; 3],
    /// Geometric metric of ambiguity (e.g. residual, distance).
    pub residual: f64,
    /// Human-readable context describing the ambiguity.
    pub context: String,
}
