//! Parametric line-segment geometry utilities.
//!
//! DOMAIN: Point proximity to 3D line segments.
//! DEPENDENCIES: None (forge-math linalg primitives used inline for zero-cost inlining).
//! INVARIANTS: All functions are pure — no topology, no policy, no arena.

/// Project `point` onto the segment `origin → dest` and return proximity data.
///
/// Returns `Some((t, dist_sq))` where:
/// - `t` is the parametric position along the segment (exclusive of endpoints
///   via `endpoint_margin` derived from `tolerance_sq`)
/// - `dist_sq` is the squared perpendicular distance from `point` to the segment
///
/// Returns `None` when:
/// - The segment is degenerate (`len_sq < tolerance_sq`)
/// - `t` is at or beyond an endpoint (within `endpoint_margin` of 0 or 1)
/// - The perpendicular distance exceeds `tolerance_sq`
///
/// `tolerance_sq` should be supplied from `ToleranceConfig` at the kernel layer.
pub fn point_on_segment(
    point: &[f64; 3],
    origin: &[f64; 3],
    dest: &[f64; 3],
    tolerance_sq: f64,
) -> Option<(f64, f64)> {
    let dx = dest[0] - origin[0];
    let dy = dest[1] - origin[1];
    let dz = dest[2] - origin[2];
    let len_sq = dx * dx + dy * dy + dz * dz;

    if len_sq < tolerance_sq {
        return None;
    }

    let px = point[0] - origin[0];
    let py = point[1] - origin[1];
    let pz = point[2] - origin[2];

    let t = (px * dx + py * dy + pz * dz) / len_sq;

    let endpoint_margin = (tolerance_sq / len_sq).sqrt();
    if t <= endpoint_margin || t >= 1.0 - endpoint_margin {
        return None;
    }

    let proj_x = origin[0] + t * dx;
    let proj_y = origin[1] + t * dy;
    let proj_z = origin[2] + t * dz;

    let ex = point[0] - proj_x;
    let ey = point[1] - proj_y;
    let ez = point[2] - proj_z;
    let dist_sq = ex * ex + ey * ey + ez * ez;

    if dist_sq > tolerance_sq {
        return None;
    }

    Some((t, dist_sq))
}

#[cfg(test)]
mod tests {
    use super::point_on_segment;

    const TOL_SQ: f64 = 1e-12;

    #[test]
    fn midpoint_of_unit_segment_returns_t_half() {
        let origin = [0.0, 0.0, 0.0];
        let dest = [1.0, 0.0, 0.0];
        let point = [0.5, 0.0, 0.0];
        let result = point_on_segment(&point, &origin, &dest, TOL_SQ);
        assert!(result.is_some(), "midpoint should be found");
        let (t, dist_sq) = result.unwrap();
        assert!((t - 0.5).abs() < 1e-9, "t should be 0.5, got {t}");
        assert!(dist_sq < TOL_SQ, "dist_sq should be zero, got {dist_sq}");
    }

    #[test]
    fn point_off_to_side_within_tolerance_returns_some() {
        let origin = [0.0, 0.0, 0.0];
        let dest = [1.0, 0.0, 0.0];
        let nudge = 1e-7;
        let point = [0.5, nudge, 0.0];
        let result = point_on_segment(&point, &origin, &dest, nudge * nudge * 4.0);
        assert!(
            result.is_some(),
            "small lateral offset within tolerance should match"
        );
    }

    #[test]
    fn point_beyond_tolerance_returns_none() {
        let origin = [0.0, 0.0, 0.0];
        let dest = [1.0, 0.0, 0.0];
        let point = [0.5, 1.0, 0.0];
        let result = point_on_segment(&point, &origin, &dest, TOL_SQ);
        assert!(
            result.is_none(),
            "point 1 unit off segment should not match"
        );
    }

    #[test]
    fn point_at_origin_returns_none() {
        let origin = [0.0, 0.0, 0.0];
        let dest = [1.0, 0.0, 0.0];
        let point = [0.0, 0.0, 0.0];
        let result = point_on_segment(&point, &origin, &dest, TOL_SQ);
        assert!(
            result.is_none(),
            "point at origin endpoint (t≈0) should return None"
        );
    }

    #[test]
    fn point_at_dest_returns_none() {
        let origin = [0.0, 0.0, 0.0];
        let dest = [1.0, 0.0, 0.0];
        let point = [1.0, 0.0, 0.0];
        let result = point_on_segment(&point, &origin, &dest, TOL_SQ);
        assert!(
            result.is_none(),
            "point at dest endpoint (t≈1) should return None"
        );
    }

    #[test]
    fn degenerate_segment_returns_none() {
        let origin = [0.5, 0.5, 0.5];
        let dest = [0.5, 0.5, 0.5];
        let point = [0.5, 0.5, 0.5];
        let result = point_on_segment(&point, &origin, &dest, TOL_SQ);
        assert!(result.is_none(), "zero-length segment should return None");
    }
}
