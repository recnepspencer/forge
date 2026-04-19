//! DOMAIN: Implicit Vertex
//! INVARIANTS:
//! - Every vertex is defined by at least 3 plane references
//! - Position is derived on demand, never stored
//! - Overconstrained vertices (4+ planes) select the best-conditioned triple
//! - Inconsistent vertices return `MathError::Ambiguous` (D2)
//!
//! DEPENDENCIES: `plane`, `worth-math` (predicates, error, GeometrySource)

pub use eval::{orient3d_symbolic, resolve_position, select_best_triple};

// =========================================================================
// SCHEMA
// =========================================================================

use serde::{Deserialize, Serialize};
use worth_math::arithmetic::Rational;

/// A geometric vertex.
///
/// Can be defined explicitly by exact rational coordinates, or symbolically
/// by the intersection of three planes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Vertex {
    /// Exact arbitrary-precision coordinates.
    Explicit([Rational; 3]),
    /// An implicit intersection of exactly three planes.
    Symbolic([PlaneRef; 3]),
}

/// Lightweight reference to a plane in a plane table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlaneRef {
    /// Index into the plane table.
    index: usize,
}

impl PlaneRef {
    /// Create a new plane reference.
    pub fn new(index: usize) -> Self {
        Self { index }
    }

    /// The index into the plane table.
    pub fn index(self) -> usize {
        self.index
    }
}

impl Vertex {
    /// Create a new symbolic vertex from exactly 3 plane references.
    pub fn try_new_symbolic(planes: [PlaneRef; 3]) -> Self {
        Self::Symbolic(planes)
    }

    /// The plane references defining this vertex, if symbolic.
    pub fn defining_planes(&self) -> Option<&[PlaneRef; 3]> {
        match self {
            Self::Symbolic(planes) => Some(planes),
            Self::Explicit(_) => None,
        }
    }
}
// =========================================================================
// EVALUATION LOGIC
// =========================================================================

mod eval {
    use std::convert::TryFrom;
    use worth_math::arithmetic::Rational;
    use worth_math::linalg::det3_rows;
    use worth_math::{GeometrySource, MathError};

    use crate::primitives::plane::{intersect_three_planes, signed_distance, Plane};

    use super::{PlaneRef, Vertex};

    /// Compute the 4x4 determinant to test a symbolic vertex against a fourth plane.
    pub fn orient3d_symbolic(
        vertex: &Vertex,
        test_plane: PlaneRef,
        geom: &impl GeometrySource,
    ) -> Result<worth_math::sign::TriSign, MathError> {
        match vertex {
            Vertex::Explicit(coords) => {
                // Evaluate orient3d using exact Rational coordinates against the plane.
                // P4 = A4*x + B4*y + C4*z + D4
                let p4 = get_plane_from_source(geom, test_plane.index())?;
                let n = p4.raw_normal();
                let d = p4.offset();

                let a4 = Rational::try_from_f64(n[0])?;
                let b4 = Rational::try_from_f64(n[1])?;
                let c4 = Rational::try_from_f64(n[2])?;
                let d4 = Rational::try_from_f64(d)?;

                let dist = &a4 * &coords[0] + &b4 * &coords[1] + &c4 * &coords[2] + d4;
                Ok(dist.sign())
            }
            Vertex::Symbolic(planes) => {
                // Evaluate the 4x4 determinant of the plane coefficients
                let p1 = get_plane_from_source(geom, planes[0].index())?;
                let p2 = get_plane_from_source(geom, planes[1].index())?;
                let p3 = get_plane_from_source(geom, planes[2].index())?;
                let p4 = get_plane_from_source(geom, test_plane.index())?;

                let m1 = plane_to_rationals(&p1)?;
                let m2 = plane_to_rationals(&p2)?;
                let m3 = plane_to_rationals(&p3)?;
                let m4 = plane_to_rationals(&p4)?;

                let det = det4_rationals(&m1, &m2, &m3, &m4);
                Ok(det.sign())
            }
        }
    }

    fn plane_to_rationals(p: &Plane) -> Result<[Rational; 4], MathError> {
        let n = p.raw_normal();
        Ok([
            Rational::try_from_f64(n[0])?,
            Rational::try_from_f64(n[1])?,
            Rational::try_from_f64(n[2])?,
            Rational::try_from_f64(p.offset())?,
        ])
    }

    fn det4_rationals(
        r1: &[Rational; 4],
        r2: &[Rational; 4],
        r3: &[Rational; 4],
        r4: &[Rational; 4],
    ) -> Rational {
        // Cofactor expansion along the first row
        let det_sub_1 = det3_rationals(
            [&r2[1], &r2[2], &r2[3]],
            [&r3[1], &r3[2], &r3[3]],
            [&r4[1], &r4[2], &r4[3]],
        );
        let det_sub_2 = det3_rationals(
            [&r2[0], &r2[2], &r2[3]],
            [&r3[0], &r3[2], &r3[3]],
            [&r4[0], &r4[2], &r4[3]],
        );
        let det_sub_3 = det3_rationals(
            [&r2[0], &r2[1], &r2[3]],
            [&r3[0], &r3[1], &r3[3]],
            [&r4[0], &r4[1], &r4[3]],
        );
        let det_sub_4 = det3_rationals(
            [&r2[0], &r2[1], &r2[2]],
            [&r3[0], &r3[1], &r3[2]],
            [&r4[0], &r4[1], &r4[2]],
        );

        (&r1[0] * &det_sub_1) - (&r1[1] * &det_sub_2) + (&r1[2] * &det_sub_3)
            - (&r1[3] * &det_sub_4)
    }

    fn det3_rationals(r1: [&Rational; 3], r2: [&Rational; 3], r3: [&Rational; 3]) -> Rational {
        let t1 = r1[0] * &(r2[1] * r3[2] - r2[2] * r3[1]);
        let t2 = r1[1] * &(r2[0] * r3[2] - r2[2] * r3[0]);
        let t3 = r1[2] * &(r2[0] * r3[1] - r2[1] * r3[0]);
        t1 - t2 + t3
    }

    /// Resolve the 3D position of an implicit vertex strictly for debugging/export.
    pub fn resolve_position(
        vertex: &Vertex,
        geom: &impl GeometrySource,
        degeneracy: f64,
    ) -> Result<[f64; 3], MathError> {
        match vertex {
            Vertex::Explicit(coords) => Ok([
                coords[0].to_f64_approx(),
                coords[1].to_f64_approx(),
                coords[2].to_f64_approx(),
            ]),
            Vertex::Symbolic(refs) => intersect_three_planes(
                &get_plane_from_source(geom, refs[0].index())?,
                &get_plane_from_source(geom, refs[1].index())?,
                &get_plane_from_source(geom, refs[2].index())?,
                degeneracy,
            ),
        }
    }
    /// Select the best-conditioned triple from N planes.
    pub fn select_best_triple(
        refs: &[PlaneRef],
        geom: &impl GeometrySource,
    ) -> Result<(usize, usize, usize), MathError> {
        let count = refs.len();
        let mut best_det = 0.0_f64;
        let mut best_triple: Option<(usize, usize, usize)> = None;

        let planes: Vec<Plane> = refs
            .iter()
            .map(|r| get_plane_from_source(geom, r.index()))
            .collect::<Result<_, _>>()?;

        for i in 0..count {
            for j in (i + 1)..count {
                for k in (j + 1)..count {
                    let n0 = planes[i].raw_normal();
                    let n1 = planes[j].raw_normal();
                    let n2 = planes[k].raw_normal();

                    let det = det3_rows(n0, n1, n2).abs();

                    if det > best_det {
                        best_det = det;
                        best_triple = Some((i, j, k));
                    }
                }
            }
        }

        best_triple.ok_or_else(|| {
            MathError::InvalidInput(
                "All plane triples are degenerate (all determinants ≈ 0)".to_string(),
            )
        })
    }

    fn get_plane_from_source(geom: &impl GeometrySource, index: usize) -> Result<Plane, MathError> {
        let coeffs = geom.get_plane(index)?;
        let n = coeffs.normal();
        Plane::try_new(n, coeffs.offset())
    }
} // end mod eval

#[cfg(test)]
mod tests {
    use crate::primitives::implicit_vertex::{
        resolve_position, select_best_triple, PlaneRef, Vertex,
    };
    use crate::primitives::plane::Plane;
    use crate::spatial::bsp::PlaneSet;

    const TEST_DEGENERACY: f64 = 1e-15;
    const TEST_TOLERANCE: f64 = 1e-10;

    fn cube_planes() -> Vec<Plane> {
        vec![
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
            Plane::try_new([-1.0, 0.0, 0.0], 1.0).unwrap(),
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, -1.0, 0.0], 1.0).unwrap(),
            Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap(),
            Plane::try_new([0.0, 0.0, -1.0], 1.0).unwrap(),
        ]
    }

    #[test]
    fn three_axis_aligned_planes_at_origin() {
        let planes = cube_planes();
        let vertex =
            Vertex::try_new_symbolic([PlaneRef::new(0), PlaneRef::new(2), PlaneRef::new(4)]);

        let pos = resolve_position(&vertex, &PlaneSet::new(planes), TEST_DEGENERACY).unwrap();
        assert!((pos[0]).abs() < 1e-10);
        assert!((pos[1]).abs() < 1e-10);
        assert!((pos[2]).abs() < 1e-10);
    }
}
