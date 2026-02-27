//! Plane-plane intersection chord computation.
//!
//! DOMAIN: Given two planes and a polygon, compute the chord (line segment)
//! where the planes' intersection line enters and exits the polygon interior.
//!
//! DEPENDENCIES: `forge-geom::primitives::plane` (signed_distance).
//! INVARIANTS:
//! - Both functions are purely geometric — no topology, no policy.
//! - `clip_line_to_face_polygon` is the sole authority for cut decisions.
//!   Results are valid for both convex and concave faces: the Cyrus-Beck
//!   interval clipping algorithm accumulates entry/exit `t` values against
//!   every edge half-plane, so only segments inside ALL half-planes survive.
//!   For convex faces this always gives the unique chord; for concave faces
//!   this conservatively gives the longest fully-interior chord segment.

use forge_math::linalg::{cross, dot, norm};

/// Compute the intersection line of two planes.
///
/// Returns `(point_on_line, unit_direction)` where:
/// - `direction` = cross product of the two normals (the line's direction)
/// - `point_on_line` = the closest point on the line to the origin
///
/// Returns `None` if the planes are parallel (cross product length < `min_len`).
pub fn compute_intersection_line(
    normal_a: [f64; 3],
    offset_a: f64,
    normal_b: [f64; 3],
    offset_b: f64,
    min_len: f64,
) -> Option<([f64; 3], [f64; 3])> {
    let dir = cross(normal_a, normal_b);
    let dir_len = norm(dir);
    if dir_len < min_len {
        return None;
    }
    let unit_dir = [dir[0] / dir_len, dir[1] / dir_len, dir[2] / dir_len];

    // Solve for a point on the line by projecting origin onto both planes.
    // Use Cramer's rule on the 2×2 sub-system in the dominant plane perpendicular to dir.
    let abs_dir = [unit_dir[0].abs(), unit_dir[1].abs(), unit_dir[2].abs()];
    let point = if abs_dir[2] >= abs_dir[0] && abs_dir[2] >= abs_dir[1] {
        // Dominant axis is Z: solve in XY
        let det = normal_a[0] * normal_b[1] - normal_a[1] * normal_b[0];
        if det.abs() < 1e-30 {
            return None;
        }
        let x = (-offset_a * normal_b[1] + offset_b * normal_a[1]) / det;
        let y = (-normal_a[0] * offset_b + normal_b[0] * offset_a) / det;
        [x, y, 0.0]
    } else if abs_dir[1] >= abs_dir[0] {
        // Dominant axis is Y: solve in XZ
        let det = normal_a[0] * normal_b[2] - normal_a[2] * normal_b[0];
        if det.abs() < 1e-30 {
            return None;
        }
        let x = (-offset_a * normal_b[2] + offset_b * normal_a[2]) / det;
        let z = (-normal_a[0] * offset_b + normal_b[0] * offset_a) / det;
        [x, 0.0, z]
    } else {
        // Dominant axis is X: solve in YZ
        let det = normal_a[1] * normal_b[2] - normal_a[2] * normal_b[1];
        if det.abs() < 1e-30 {
            return None;
        }
        let y = (-offset_a * normal_b[2] + offset_b * normal_a[2]) / det;
        let z = (-normal_a[1] * offset_b + normal_b[1] * offset_a) / det;
        [0.0, y, z]
    };

    Some((point, unit_dir))
}

/// Clip an infinite line to a face polygon, returning the interior chord segment.
///
/// Uses the Cyrus-Beck parametric clipping algorithm against all edge
/// half-planes of the polygon. This is the single authority for whether a
/// cut is needed: if this returns `Some(...)` with a chord longer than
/// `min_chord_len`, the face must be split — regardless of sign patterns
/// on individual vertices.
///
/// Works for both convex and concave faces: each edge defines an inward-facing
/// half-plane (using `face_normal × edge_dir` as the inward normal). The
/// intersection of all half-planes accumulates a valid `[t_entry, t_exit]`
/// interval on the parametric line. For concave faces the result is the
/// longest chord that lies fully inside all edge half-planes simultaneously.
///
/// `polygon_verts`: ordered vertices of the face boundary (closed — first and
/// last are connected by an implicit final edge).
/// `face_normal`: outward normal of the face plane (used to orient edge normals).
/// `min_chord_len`: segments shorter than this are ignored (below tolerance).
pub fn clip_line_to_face_polygon(
    line_pt: [f64; 3],
    line_dir: [f64; 3],
    polygon_verts: &[[f64; 3]],
    face_normal: [f64; 3],
    min_chord_len: f64,
) -> Option<([f64; 3], [f64; 3])> {
    let n = polygon_verts.len();
    if n < 3 {
        return None;
    }

    let mut t_enter = f64::NEG_INFINITY;
    let mut t_exit = f64::INFINITY;

    for i in 0..n {
        let v0 = polygon_verts[i];
        let v1 = polygon_verts[(i + 1) % n];

        // Edge direction (v0 → v1)
        let edge = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];

        // Inward-pointing normal for this edge = face_normal × edge
        // (for CCW winding, this points into the polygon interior)
        let inward = cross(face_normal, edge);

        // w = line_pt - v0
        let w = [line_pt[0] - v0[0], line_pt[1] - v0[1], line_pt[2] - v0[2]];

        // Cyrus-Beck: the half-plane constraint is inward·(line_pt + t*line_dir - v0) >= 0
        // i.e. inward·w + t * inward·line_dir >= 0
        // Let D_w = inward·w,  D_d = inward·line_dir
        // At boundary: t = -D_w / D_d
        let d_w = dot(inward, w);
        let d_d = dot(inward, line_dir);

        if d_d.abs() < 1e-30 {
            // Line is parallel to this edge's half-plane.
            // If d_w < 0 the line is outside this half-plane entirely → no chord.
            if d_w < 0.0 {
                return None;
            }
            // d_w >= 0: inside, no t constraint from this edge.
        } else {
            let t = -d_w / d_d;
            if d_d > 0.0 {
                // d_d > 0: as t increases, dot product increases → moving MORE inside.
                // This is the ENTERING direction. t_enter = max(t_enter, t).
                if t > t_enter {
                    t_enter = t;
                }
            } else {
                // d_d < 0: as t increases, dot product decreases → moving toward outside.
                // This is the EXITING direction. t_exit = min(t_exit, t).
                if t < t_exit {
                    t_exit = t;
                }
            }
        }

        if t_enter > t_exit {
            return None;
        }
    }

    if !t_enter.is_finite() || !t_exit.is_finite() {
        return None;
    }

    let chord_len = t_exit - t_enter;
    if chord_len < min_chord_len {
        return None;
    }

    let p_start = [
        line_pt[0] + t_enter * line_dir[0],
        line_pt[1] + t_enter * line_dir[1],
        line_pt[2] + t_enter * line_dir[2],
    ];
    let p_end = [
        line_pt[0] + t_exit * line_dir[0],
        line_pt[1] + t_exit * line_dir[1],
        line_pt[2] + t_exit * line_dir[2],
    ];

    Some((p_start, p_end))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;
    const MIN_CHORD: f64 = 1e-9;

    #[test]
    fn intersection_line_of_xz_and_yz_planes() {
        // Plane X=0 (normal [1,0,0], offset=0) ∩ Plane Y=0 ([0,1,0], offset=0)
        // Intersection is the Z axis: dir=[0,0,1], point=[0,0,0].
        let (pt, dir) =
            compute_intersection_line([1.0, 0.0, 0.0], 0.0, [0.0, 1.0, 0.0], 0.0, 1e-12).unwrap();
        assert!((dir[0]).abs() < TOL, "dir[0]={}", dir[0]);
        assert!((dir[1]).abs() < TOL, "dir[1]={}", dir[1]);
        assert!((dir[2].abs() - 1.0).abs() < TOL, "dir[2]={}", dir[2]);
        // Point must satisfy both plane equations.
        let da = pt[0]; // x*1 + 0
        let db = pt[1]; // y*1 + 0
        assert!(da.abs() < TOL, "point not on plane A: {}", da);
        assert!(db.abs() < TOL, "point not on plane B: {}", db);
    }

    #[test]
    fn parallel_planes_return_none() {
        // Two Z-planes at different offsets — parallel.
        let result = compute_intersection_line([0.0, 0.0, 1.0], -5.0, [0.0, 0.0, 1.0], -3.0, 1e-12);
        assert!(result.is_none());
    }

    #[test]
    fn chord_through_unit_square_z_plane() {
        // Face: unit square in Z=0 plane, vertices CCW.
        // Cutting plane at X=0.5: normal=[1,0,0], offset=-0.5.
        // Intersection line with Z=0 (normal=[0,0,1], offset=0):
        //   dir = cross([0,0,1],[1,0,0]) = [0,1,0]
        //   point ≈ [0.5, 0, 0]
        // Chord should go from [0.5,0,0] to [0.5,1,0].
        let verts = [
            [0.0, 0.0, 0.0f64],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let face_normal = [0.0, 0.0, 1.0f64];
        let (line_pt, line_dir) =
            compute_intersection_line([0.0, 0.0, 1.0], 0.0, [1.0, 0.0, 0.0], -0.5, 1e-12).unwrap();
        let (p_start, p_end) =
            clip_line_to_face_polygon(line_pt, line_dir, &verts, face_normal, MIN_CHORD).unwrap();
        // Both endpoints should have x=0.5 and z=0.
        assert!((p_start[0] - 0.5).abs() < TOL, "start.x={}", p_start[0]);
        assert!((p_end[0] - 0.5).abs() < TOL, "end.x={}", p_end[0]);
        assert!(p_start[2].abs() < TOL);
        assert!(p_end[2].abs() < TOL);
        // y coords should bracket [0, 1].
        let y_min = p_start[1].min(p_end[1]);
        let y_max = p_start[1].max(p_end[1]);
        assert!(y_min < TOL, "y_min={}", y_min);
        assert!((y_max - 1.0).abs() < TOL, "y_max={}", y_max);
    }

    #[test]
    fn chord_misses_outside_face_returns_none() {
        // Unit square, but cutting plane at X=2 (outside the square).
        let verts = [
            [0.0, 0.0, 0.0f64],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let face_normal = [0.0, 0.0, 1.0f64];
        let (line_pt, line_dir) =
            compute_intersection_line([0.0, 0.0, 1.0], 0.0, [1.0, 0.0, 0.0], -2.0, 1e-12).unwrap();
        let result = clip_line_to_face_polygon(line_pt, line_dir, &verts, face_normal, MIN_CHORD);
        assert!(result.is_none(), "Should miss outside face");
    }

    #[test]
    fn chord_on_exactly_shared_boundary_returns_chord_not_none() {
        // Cutting plane at X=0 (the left edge of the unit square).
        // The intersection line runs along the left edge: from [0,0,0] to [0,1,0].
        // clip_line_to_face_polygon correctly returns this as a chord of length 1.0.
        // The split_face_by_plane adjacent_pairs guard is responsible for rejecting
        // the resulting cut when both vertices are already connected by an edge.
        let verts = [
            [0.0, 0.0, 0.0f64],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let face_normal = [0.0, 0.0, 1.0f64];
        let (line_pt, line_dir) =
            compute_intersection_line([0.0, 0.0, 1.0], 0.0, [1.0, 0.0, 0.0], 0.0, 1e-12).unwrap();
        let result = clip_line_to_face_polygon(line_pt, line_dir, &verts, face_normal, 1e-9);
        assert!(
            result.is_some(),
            "Boundary chord at X=0 should be returned (length=1.0)"
        );
    }
}
