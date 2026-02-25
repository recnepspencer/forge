//! Trim curve operation contracts for UV-space merge qualification.
//!
//! DOMAIN: Defines the operations needed to qualify and resolve trim
//! curve overlap during curved same-support merge. The kernel's merge
//! eligibility path uses these to determine whether two adjacent faces
//! on the same support surface have compatible trim boundaries.

/// Result of a trim curve overlap check in UV space.
#[derive(Debug, Clone, PartialEq)]
pub enum TrimOverlapResult {
    /// Trims do not overlap in UV (no shared boundary portion).
    Disjoint,
    /// Trims share a portion of their UV path within tolerance.
    Overlap {
        /// Parameter range on `self` where overlap occurs.
        self_range: (f64, f64),
        /// Parameter range on `other` where overlap occurs.
        other_range: (f64, f64),
    },
    /// Overlap classification could not be safely decided.
    /// Kernel must treat as fail-closed.
    Undetermined,
}

/// Trim curve operations for UV-space merge qualification.
///
/// DESIGN TARGET: Not yet implemented by `Coedge` or `ParametricCurve2D`.
/// Implementation will be added when curved merge execution is built.
pub trait TrimCurveOps {
    /// UV-space endpoints of this trim curve.
    fn uv_endpoints(&self) -> ([f64; 2], [f64; 2]);

    /// Unit direction vector in UV space at the start of the trim.
    fn uv_direction(&self) -> [f64; 2];

    /// Check whether this trim curve overlaps another in UV space.
    ///
    /// `tol` is the maximum UV-space distance for coincidence.
    /// Returns `Undetermined` if the result cannot be safely decided.
    fn uv_overlap(&self, other: &Self, tol: f64) -> TrimOverlapResult;
}
