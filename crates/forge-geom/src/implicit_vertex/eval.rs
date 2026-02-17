//! Evaluation logic for the ImplicitVertex primitive.

use forge_core::{KernelError, AmbiguousResult, GeometrySource};
use forge_math::linalg::det3_rows;

use crate::plane::Plane;
use crate::plane::signed_distance;
use crate::plane::intersect_three_planes;

use super::schema::{ImplicitVertex, PlaneRef};

/// Resolve the 3D position of an implicit vertex.
///
/// - Exactly 3 planes → solve the 3×3 system directly
/// - 4+ planes → select the best-conditioned triple, solve, then
///   verify that all remaining planes are satisfied within tolerance
///
/// # Parameters
///
/// - `residual_tolerance`: max acceptable signed distance for overconstrained planes
/// - `degeneracy`: minimum acceptable |det| for the 3-plane solve
///
/// # Errors
///
/// - `KernelError::InvalidInput` if planes are degenerate (parallel/coincident)
///   or a `PlaneRef` index is out of bounds
/// - `KernelError::PolicyRequired` if overconstrained planes disagree beyond tolerance
pub fn resolve_position(
    vertex: &ImplicitVertex,
    geom: &impl GeometrySource,
    residual_tolerance: f64,
    degeneracy: f64,
) -> Result<[f64; 3], KernelError> {
    let refs = vertex.defining_planes();

    if refs.len() < 3 {
        return Err(KernelError::InvalidInput {
            message: "ImplicitVertex requires at least 3 defining planes".to_string(),
            context: None,
        });
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
///
/// Selects the best-conditioned triple (max |det|), solves for the
/// intersection point, then verifies all planes are satisfied.
fn resolve_overconstrained(
    refs: &[PlaneRef],
    geom: &impl GeometrySource,
    residual_tolerance: f64,
    degeneracy: f64,
) -> Result<[f64; 3], KernelError> {
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
///
/// Iterates all (N choose 3) combinations and returns the indices
/// (into the `refs` slice) of the triple with the largest |det|.
pub fn select_best_triple(
    refs: &[PlaneRef],
    geom: &impl GeometrySource,
) -> Result<(usize, usize, usize), KernelError> {
    let count = refs.len();
    let mut best_det = 0.0_f64;
    let mut best_triple: Option<(usize, usize, usize)> = None;

    // Cache planes to avoid re-fetching/re-constructing in the inner loop
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
        KernelError::InvalidInput {
            message: "All plane triples are degenerate (all determinants ≈ 0)".to_string(),
            context: None,
        }
    })
}

/// Verify that a point satisfies all defining planes within tolerance.
fn verify_all_planes_satisfied(
    point: &[f64; 3],
    refs: &[PlaneRef],
    geom: &impl GeometrySource,
    residual_tolerance: f64,
) -> Result<(), KernelError> {
    let mut max_violation = 0.0_f64;

    for plane_ref in refs {
        let plane = get_plane_from_source(geom, plane_ref.index())?;
        let dist = signed_distance(&plane, point).abs();
        if dist > max_violation {
            max_violation = dist;
        }
    }

    if max_violation > residual_tolerance {
        return Err(KernelError::AmbiguousResult {
            result: AmbiguousResult {
                location: *point,
                residual: max_violation,
                context: format!(
                    "Overconstrained vertex resolution residual ({:.2e}) exceeds tolerance ({:.2e})",
                    max_violation, residual_tolerance
                ),
            },
            context: None,
        });
    }

    Ok(())
}

/// Safely look up a plane from the GeometrySource.
fn get_plane_from_source(geom: &impl GeometrySource, index: usize) -> Result<Plane, KernelError> {
    let eq = geom.get_plane(index)?;
    Plane::try_new([eq[0], eq[1], eq[2]], eq[3])
}
