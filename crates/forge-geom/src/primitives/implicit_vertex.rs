//! DOMAIN: Implicit Vertex
//! INVARIANTS:
//! - Every vertex is defined by at least 3 plane references
//! - Position is derived on demand, never stored
//! - Overconstrained vertices (4+ planes) select the best-conditioned triple
//! - Inconsistent vertices return `MathError::Ambiguous` (D2)
//!
//! DEPENDENCIES: `plane`, `forge-math` (predicates, error, GeometrySource)

pub use eval::{resolve_position, select_best_triple};

// =========================================================================
// SCHEMA
// =========================================================================

use serde::{Deserialize, Serialize};

/// An implicit vertex defined by the intersection of 3 or more planes.
///
/// The vertex position is not stored — it is derived on demand by
/// solving the linear system of plane equations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImplicitVertex {
    /// Indices into the plane table. Must contain at least 3 entries.
    defining_planes: Vec<PlaneRef>,
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

impl ImplicitVertex {
    /// Create a new implicit vertex from 3+ plane references.
    ///
    /// Returns `None` if fewer than 3 planes are provided.
    pub fn try_new(planes: Vec<PlaneRef>) -> Option<Self> {
        if planes.len() < 3 {
            return None;
        }
        Some(Self {
            defining_planes: planes,
        })
    }

    /// The plane references defining this vertex.
    pub fn defining_planes(&self) -> &[PlaneRef] {
        &self.defining_planes
    }

    /// The number of defining planes.
    pub fn plane_count(&self) -> usize {
        self.defining_planes.len()
    }

    /// Whether this vertex is overconstrained (4+ planes).
    pub fn is_overconstrained(&self) -> bool {
        self.defining_planes.len() > 3
    }
}
// =========================================================================
// EVALUATION LOGIC
// =========================================================================

mod eval {
use forge_math::{MathError, GeometrySource};
use forge_math::linalg::det3_rows;

use crate::primitives::plane::{Plane, signed_distance, intersect_three_planes};

use super::{ImplicitVertex, PlaneRef};

/// Resolve the 3D position of an implicit vertex.
///
/// - Exactly 3 planes → solve the 3×3 system directly
/// - 4+ planes → select the best-conditioned triple, solve, then
///   verify that all remaining planes are satisfied within tolerance
pub fn resolve_position(
    vertex: &ImplicitVertex,
    geom: &impl GeometrySource,
    residual_tolerance: f64,
    degeneracy: f64,
) -> Result<[f64; 3], MathError> {
    let refs = vertex.defining_planes();

    if refs.len() < 3 {
        return Err(MathError::InvalidInput(
            "ImplicitVertex requires at least 3 defining planes".to_string(),
        ));
    }

    if refs.len() == 3 {
        return intersect_three_planes(
            &get_plane_from_source(geom, refs[0].index())?,
            &get_plane_from_source(geom, refs[1].index())?,
            &get_plane_from_source(geom, refs[2].index())?,
            degeneracy,
        );
    }

    resolve_overconstrained(refs, geom, residual_tolerance, degeneracy)
}

/// Resolve an overconstrained vertex (4+ planes).
fn resolve_overconstrained(
    refs: &[PlaneRef],
    geom: &impl GeometrySource,
    residual_tolerance: f64,
    degeneracy: f64,
) -> Result<[f64; 3], MathError> {
    let (i, j, k) = select_best_triple(refs, geom)?;

    let point = intersect_three_planes(
        &get_plane_from_source(geom, refs[i].index())?,
        &get_plane_from_source(geom, refs[j].index())?,
        &get_plane_from_source(geom, refs[k].index())?,
        degeneracy,
    )?;

    verify_all_planes_satisfied(&point, refs, geom, residual_tolerance)?;

    Ok(point)
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

/// Verify that a point satisfies all defining planes within tolerance.
fn verify_all_planes_satisfied(
    point: &[f64; 3],
    refs: &[PlaneRef],
    geom: &impl GeometrySource,
    residual_tolerance: f64,
) -> Result<(), MathError> {
    let mut max_violation = 0.0_f64;

    for plane_ref in refs {
        let plane = get_plane_from_source(geom, plane_ref.index())?;
        let dist = signed_distance(&plane, point).abs();
        if dist > max_violation {
            max_violation = dist;
        }
    }

    if max_violation > residual_tolerance {
        return Err(MathError::Ambiguous {
            location: *point,
            residual: max_violation,
            context: format!(
                "Overconstrained vertex resolution residual ({:.2e}) exceeds tolerance ({:.2e})",
                max_violation, residual_tolerance
            ),
        });
    }

    Ok(())
}

/// Safely look up a plane from the GeometrySource.
fn get_plane_from_source(geom: &impl GeometrySource, index: usize) -> Result<Plane, MathError> {
    let coeffs = geom.get_plane(index)?;
    let n = coeffs.normal();
    Plane::try_new(n, coeffs.offset())
}
} // end mod eval

#[cfg(test)]
mod tests {
    use crate::primitives::plane::Plane;
    use crate::spatial::bsp::PlaneSet;
    use crate::primitives::implicit_vertex::{ImplicitVertex, PlaneRef, resolve_position, select_best_triple};

    const TEST_RESIDUAL: f64 = 1e-8;
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
    fn reject_fewer_than_three_planes() {
        let result = ImplicitVertex::try_new(vec![PlaneRef::new(0), PlaneRef::new(1)]);
        assert!(result.is_none());
    }

    #[test]
    fn three_axis_aligned_planes_at_origin() {
        let planes = cube_planes();
        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(0), PlaneRef::new(2), PlaneRef::new(4),
        ]).unwrap();

        let pos = resolve_position(&vertex, &PlaneSet::new(planes), TEST_RESIDUAL, TEST_DEGENERACY).unwrap();
        assert!((pos[0]).abs() < 1e-10);
        assert!((pos[1]).abs() < 1e-10);
        assert!((pos[2]).abs() < 1e-10);
    }

    #[test]
    fn cube_vertex_at_one_one_one() {
        let planes = cube_planes();
        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(1), PlaneRef::new(3), PlaneRef::new(5),
        ]).unwrap();

        let pos = resolve_position(&vertex, &PlaneSet::new(planes), TEST_RESIDUAL, TEST_DEGENERACY).unwrap();
        assert!((pos[0] - 1.0).abs() < TEST_TOLERANCE);
        assert!((pos[1] - 1.0).abs() < TEST_TOLERANCE);
        assert!((pos[2] - 1.0).abs() < TEST_TOLERANCE);
    }

    #[test]
    fn all_eight_cube_vertices_resolve() {
        let planes = cube_planes();
        let triples: [(usize, usize, usize); 8] = [
            (0, 2, 4), (0, 2, 5), (0, 3, 4), (0, 3, 5),
            (1, 2, 4), (1, 2, 5), (1, 3, 4), (1, 3, 5),
        ];

        for (a, b, c) in triples {
            let vertex = ImplicitVertex::try_new(vec![
                PlaneRef::new(a), PlaneRef::new(b), PlaneRef::new(c),
            ]).unwrap();
            let pos = resolve_position(&vertex, &PlaneSet::new(planes.clone()), TEST_RESIDUAL, TEST_DEGENERACY);
            assert!(pos.is_ok(), "Failed to resolve vertex ({}, {}, {})", a, b, c);
        }
    }

    #[test]
    fn overconstrained_apex_consistent() {
        let planes = vec![
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap(),
            Plane::try_new([1.0, 1.0, 1.0], 0.0).unwrap(),
        ];

        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(0), PlaneRef::new(1), PlaneRef::new(2), PlaneRef::new(3),
        ]).unwrap();

        assert!(vertex.is_overconstrained());

        let pos = resolve_position(&vertex, &PlaneSet::new(planes), TEST_RESIDUAL, TEST_DEGENERACY).unwrap();
        assert!((pos[0]).abs() < 1e-10);
        assert!((pos[1]).abs() < 1e-10);
        assert!((pos[2]).abs() < 1e-10);
    }

    #[test]
    fn overconstrained_inconsistent_returns_error() {
        let planes = vec![
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap(),
            Plane::try_new([1.0, 0.0, 0.0], -5.0).unwrap(),
        ];

        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(0), PlaneRef::new(1), PlaneRef::new(2), PlaneRef::new(3),
        ]).unwrap();

        let result = resolve_position(&vertex, &PlaneSet::new(planes), TEST_RESIDUAL, TEST_DEGENERACY);
        assert!(result.is_err());
    }

    #[test]
    fn select_best_triple_picks_well_conditioned() {
        let planes = vec![
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap(),
            Plane::try_new([1.0, 1.0, 0.0], 0.0).unwrap(),
        ];

        let refs: Vec<PlaneRef> = (0..4).map(PlaneRef::new).collect();
        let (i, j, k) = select_best_triple(&refs, &PlaneSet::new(planes)).unwrap();

        assert!(i < j);
        assert!(j < k);
    }

    #[test]
    fn implicit_vertex_plane_count() {
        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(0), PlaneRef::new(1), PlaneRef::new(2),
        ]).unwrap();

        assert_eq!(vertex.plane_count(), 3);
        assert!(!vertex.is_overconstrained());
    }

    #[test]
    fn out_of_bounds_plane_ref_returns_error() {
        let planes = vec![
            Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
            Plane::try_new([0.0, 1.0, 0.0], 0.0).unwrap(),
        ];

        let vertex = ImplicitVertex::try_new(vec![
            PlaneRef::new(0), PlaneRef::new(1), PlaneRef::new(99),
        ]).unwrap();

        let result = resolve_position(&vertex, &PlaneSet::new(planes), TEST_RESIDUAL, TEST_DEGENERACY);
        assert!(result.is_err());
    }
}
