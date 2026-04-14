//! Edge-plane intersection position computation for topology-cutting operations.
//!
//! DOMAIN: Given two plane indices (face + twin) and a cut plane, compute the
//! exact 3D position of the new vertex that lives on the cut. Attempts exact
//! 3-plane intersection first; falls back to f64 edge-plane intersection.
//!
//! CONSUMERS: Boolean parametric split (cut.rs), future fillet arc tangent
//! point computation, NURBS surface trim vertex placement.
//!
//! INVARIANT: Tolerance threshold is supplied as `f64` from `ToleranceConfig`
//! at the kernel layer — not hardcoded here.

use worth_math::arithmetic::Rational;

use crate::geom_facade::{intersect_three_planes_exact, Plane};
use crate::core::ToleranceConfig;

/// Compute the intersection position for a new cut vertex on an edge.
///
/// `planes`: indexed slice of all planes for this solid. The caller resolves
/// `face_plane_idx`, `twin_plane_idx`, and `cut_plane_idx` into entries of this slice.
///
/// When `face_plane_idx != twin_plane_idx`, attempts exact 3-plane
/// intersection using rational arithmetic. On failure (parallel/degenerate)
/// or when the twin is on the same plane, falls back to `f64`
/// edge-plane intersection.
///
/// Returns `(exact_position, f64_position, symbolic_plane_indices)`:
/// - `exact_position`: `Some([Rational;3])` when 3-plane intersection succeeded.
/// - `f64_position`: Always valid; `exact.to_f64_approx()` if exact was used.
/// - `symbolic_plane_indices`: `Some([face, twin, cut])` when exact succeeded.
pub fn compute_edge_plane_intersection_position(
    face_plane_idx: usize,
    twin_plane_idx: usize,
    cut_plane_idx: usize,
    planes: &[Plane],
    cut_plane: &Plane,
    p_origin: &[f64; 3],
    p_dest: &[f64; 3],
    config: &ToleranceConfig,
) -> (Option<[Rational; 3]>, [f64; 3], Option<[usize; 3]>) {
    if face_plane_idx != twin_plane_idx {
        let p0 = &planes[face_plane_idx];
        let p1 = &planes[twin_plane_idx];
        let p2 = &planes[cut_plane_idx];
        match intersect_three_planes_exact(p0, p1, p2) {
            Ok(ep) => {
                let fx = ep[0].to_f64_approx();
                let fy = ep[1].to_f64_approx();
                let fz = ep[2].to_f64_approx();
                let f64_pos = if fx.is_finite() && fy.is_finite() && fz.is_finite() {
                    [fx, fy, fz]
                } else {
                    crate::geom_facade::intersect_edge_plane(
                        cut_plane,
                        p_origin,
                        p_dest,
                        config.get_edge_split_degeneracy(),
                    )
                };
                (
                    Some(ep),
                    f64_pos,
                    Some([face_plane_idx, twin_plane_idx, cut_plane_idx]),
                )
            }
            Err(_) => (
                None,
                crate::geom_facade::intersect_edge_plane(
                    cut_plane,
                    p_origin,
                    p_dest,
                    config.get_edge_split_degeneracy(),
                ),
                None,
            ),
        }
    } else {
        let f64_pos = crate::geom_facade::intersect_edge_plane(
            cut_plane,
            p_origin,
            p_dest,
            config.get_edge_split_degeneracy(),
        );
        let ep = Rational::try_from_f64_3(&f64_pos);
        (ep, f64_pos, None)
    }
}
