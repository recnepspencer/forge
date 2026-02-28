//! Interior sample point generation for polygon faces.
//!
//! DOMAIN: Given a face and its vertex positions, produce a set of 3D sample
//! points that lie in the interior of the face polygon. Used by the
//! point-in-solid classifier to handle edge cases where a single centroid
//! sample lands exactly on a boundary (kissing contact).
//!
//! ALGORITHM: Generates subsets of barycentric-style samples:
//!   1. The centroid itself (if confirmed on-face).
//!   2. For each boundary edge (up to 8): edge midpoint inset 65% toward
//!      centroid, fan-centroid barycentric average, and two vertex-centroid
//!      blends at 60/40 weight.
//!
//! INVARIANTS:
//! - All returned points pass `classify_point_on_face` as `OnFace`.
//! - No duplicate points within `point_coincidence_tol`.
//! - If no interior samples are found, returns the centroid as a fallback.
//! - `point_coincidence_tol` is always supplied from `ToleranceConfig`
//!   at the kernel layer — no magic numbers live here.

use forge_core::KernelError;
use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::queries::polygon::face_loop_vertices;

use crate::classify::point_on_face::{classify_point_on_face, FacePointClassification};
use forge_core::ToleranceProvider;

/// Generate interior sample points for a polygon face.
///
/// # Parameters
/// - `arena`: topology arena
/// - `position_fn`: resolves `VertexId` → 3D position
/// - `face_id`: the face to sample
/// - `centroid`: previously computed centroid of the face (pass-in avoids re-computation)
/// - `tolerance_provider`: tolerance used for `classify_point_on_face` boundary test
/// - `point_coincidence_tol`: distance below which two samples are considered identical;
///   destructure from `ToleranceConfig::get_spatial_tolerance()` in the kernel
///
/// Returns at least `[centroid]` as a fallback if no interior samples are confirmed.
pub fn face_interior_samples(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    face_id: FaceId,
    centroid: [f64; 3],
    tolerance_provider: &dyn ToleranceProvider,
    point_coincidence_tol: f64,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let loops = face_loop_vertices(arena, face_id)?;
    let outer_loop = match loops.first() {
        Some(l) if !l.is_empty() => l,
        _ => return Ok(vec![centroid]),
    };

    let verts: Vec<[f64; 3]> = outer_loop
        .iter()
        .filter_map(|vid| position_fn(*vid))
        .collect();

    if verts.len() < 3 {
        return Ok(vec![centroid]);
    }

    let pos_fn = |v: VertexId| position_fn(v);
    let mut samples: Vec<[f64; 3]> = Vec::new();

    let mut push_if_on_face = |p: [f64; 3]| -> Result<(), KernelError> {
        match classify_point_on_face(arena, face_id, &p, &pos_fn, tolerance_provider)? {
            FacePointClassification::OnFace => {
                if !samples
                    .iter()
                    .any(|q| points_are_coincident(q, &p, point_coincidence_tol))
                {
                    samples.push(p);
                }
            }
            _ => {}
        }
        Ok(())
    };

    push_if_on_face(centroid)?;

    let n = verts.len();
    for i in 0..n.min(8) {
        let a = verts[i];
        let b = verts[(i + 1) % n];

        let edge_mid = lerp3(a, b, 0.5);
        let inset = lerp3(edge_mid, centroid, 0.65);
        push_if_on_face(inset)?;

        let fan = barycentric3(a, b, centroid);
        push_if_on_face(fan)?;

        let toward_a = lerp3(centroid, a, 0.4);
        push_if_on_face(toward_a)?;

        let toward_b = lerp3(centroid, b, 0.4);
        push_if_on_face(toward_b)?;
    }

    if samples.is_empty() {
        samples.push(centroid);
    }

    Ok(samples)
}

// ── Private geometry helpers ─────────────────────────────────────────────────

/// Linear interpolation between two points: `a * (1 - t) + b * t`.
#[inline]
fn lerp3(a: [f64; 3], b: [f64; 3], t: f64) -> [f64; 3] {
    let s = 1.0 - t;
    [
        a[0] * s + b[0] * t,
        a[1] * s + b[1] * t,
        a[2] * s + b[2] * t,
    ]
}

/// Equal-weight barycentric average of three points.
#[inline]
fn barycentric3(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> [f64; 3] {
    [
        (a[0] + b[0] + c[0]) / 3.0,
        (a[1] + b[1] + c[1]) / 3.0,
        (a[2] + b[2] + c[2]) / 3.0,
    ]
}

/// True when two points are within `tol` of each other (Euclidean).
#[inline]
fn points_are_coincident(a: &[f64; 3], b: &[f64; 3], tol: f64) -> bool {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz) <= tol * tol
}
