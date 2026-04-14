//! Exact 3D position backed by rational coordinates with a cached f64 approximation.
//!
//! DOMAIN: Vertex positions derived from 3-plane intersection are stored exactly.
//! The f64 cache is derived from the rationals at construction time and
//! is used for BVH, AABB, and rendering — never for topology decisions.

use serde::{Deserialize, Serialize};

use worth_math::arithmetic::Rational;

/// Exact 3D position backed by rational coordinates with a cached f64 approximation.
///
/// Vertex positions derived from 3-plane intersection are stored exactly.
/// The f64 cache is derived from the rationals at construction time and
/// is used for BVH, AABB, and rendering — never for topology decisions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExactPosition {
    /// Exact rational coordinates.
    exact: [Rational; 3],
    /// Cached f64 approximation (derived from exact).
    pub(crate) approx: [f64; 3],
    /// Whether this position was computed via genuine exact arithmetic
    /// (e.g. intersect_three_planes_exact) or promoted from f64.
    /// Only exact positions should be used with classify_point_exact.
    is_exact: bool,
    /// Indices of the 3 planes defining this vertex, if known symbolically.
    symbolic_planes: Option<[usize; 3]>,
}

impl ExactPosition {
    /// Create from exact rational coordinates (genuine exact arithmetic).
    ///
    /// If the rational-to-f64 conversion overflows (producing ±inf or NaN),
    /// the f64 approximation is clamped to 0.0. The exact rationals remain
    /// intact for topology decisions via `classify_point_exact`.
    pub fn from_exact(exact: [Rational; 3]) -> Self {
        let raw = [
            exact[0].to_f64_approx(),
            exact[1].to_f64_approx(),
            exact[2].to_f64_approx(),
        ];
        let approx = [
            if raw[0].is_finite() { raw[0] } else { 0.0 },
            if raw[1].is_finite() { raw[1] } else { 0.0 },
            if raw[2].is_finite() { raw[2] } else { 0.0 },
        ];
        Self {
            exact,
            approx,
            is_exact: true,
            symbolic_planes: None,
        }
    }

    /// Create from exact rationals with an explicit f64 fallback.
    ///
    /// When the rational-to-f64 conversion overflows, the fallback is used
    /// for AABB/BVH/rendering while the exact rationals drive topology.
    pub fn from_exact_with_fallback(exact: [Rational; 3], fallback: [f64; 3]) -> Self {
        let approx = [
            exact[0].to_f64_approx(),
            exact[1].to_f64_approx(),
            exact[2].to_f64_approx(),
        ];
        let safe_approx = if approx[0].is_finite() && approx[1].is_finite() && approx[2].is_finite()
        {
            approx
        } else {
            fallback
        };
        Self {
            exact,
            approx: safe_approx,
            is_exact: true,
            symbolic_planes: None,
        }
    }

    /// Create from f64 coordinates (lossless IEEE754 → Rational conversion).
    /// NOT marked as exact — f64-promoted positions may not satisfy
    /// rational plane equations exactly.
    pub fn from_f64(pos: [f64; 3]) -> Self {
        let exact = [
            Rational::try_from_f64(pos[0]).unwrap_or_else(|_| Rational::zero()),
            Rational::try_from_f64(pos[1]).unwrap_or_else(|_| Rational::zero()),
            Rational::try_from_f64(pos[2]).unwrap_or_else(|_| Rational::zero()),
        ];
        Self {
            exact,
            approx: pos,
            is_exact: false,
            symbolic_planes: None,
        }
    }

    /// The cached f64 approximation.
    pub fn approx(&self) -> &[f64; 3] {
        &self.approx
    }

    /// The exact rational coordinates (only meaningful when `is_exact()` is true).
    pub fn exact(&self) -> &[Rational; 3] {
        &self.exact
    }

    /// Whether this position was computed via genuine exact arithmetic.
    pub fn is_exact(&self) -> bool {
        self.is_exact
    }

    /// Create from exact rationals, preserving the symbolic planes that defined it.
    pub fn from_symbolic(exact: [Rational; 3], fallback: [f64; 3], planes: [usize; 3]) -> Self {
        let approx = [
            exact[0].to_f64_approx(),
            exact[1].to_f64_approx(),
            exact[2].to_f64_approx(),
        ];
        let safe_approx = if approx[0].is_finite() && approx[1].is_finite() && approx[2].is_finite()
        {
            approx
        } else {
            fallback
        };
        Self {
            exact,
            approx: safe_approx,
            is_exact: true,
            symbolic_planes: Some(planes),
        }
    }

    /// Retrieve the symbolic bounding planes if they form a precise 3-plane intersection.
    pub fn symbolic_planes(&self) -> Option<&[usize; 3]> {
        self.symbolic_planes.as_ref()
    }

    /// Transform this position to local coordinates in-place.
    ///
    /// Updates both exact rationals (via `to_local_exact`) and the f64 cache
    /// (via `to_local`), keeping them in sync.
    pub fn transform_in_place(&mut self, space: &worth_geom::facade::LocalCoordinateSpace) {
        self.exact = space.to_local_exact(&self.exact);
        self.approx = space.to_local(self.approx);
    }

    /// Transform this position from local back to world coordinates in-place.
    ///
    /// Updates both exact rationals (via `from_local_exact`) and the f64 cache
    /// (via `from_local`), keeping them in sync.
    pub fn inverse_transform_in_place(&mut self, space: &worth_geom::facade::LocalCoordinateSpace) {
        self.exact = space.from_local_exact(&self.exact);
        self.approx = space.from_local(self.approx);
    }
}
