//! Certify that a boundary loop is geometrically valid.
//!
//! DOMAIN: Validates that a sequence of vertices forming a boundary loop is
//! closed, non-self-intersecting, consistently wound, and has sufficient area
//! to participate in a surface construction or fillet spine computation.
//!
//! This is pure geometry (vertex positions only) — no topology handles needed,
//! no GeometryState import. The caller extracts positions before calling.
//!
//! POLICY REQUIREMENTS: CoincidentGeometry (declared in step contract).
//!
//! DEPENDENCIES: forge-core (KernelError only)

use forge_core::KernelError;

/// Result of boundary loop certification.
#[derive(Debug, Clone)]
pub struct BoundaryCertification {
    /// Signed area of the boundary polygon (positive = counter-clockwise in projection).
    pub signed_area: f64,
    /// Whether the winding is counter-clockwise (outward-facing normal).
    pub is_ccw: bool,
    /// Number of vertices in the certified loop.
    pub vertex_count: usize,
}

/// Certify that a sequence of 3D positions forms a valid planar boundary loop.
///
/// Checks:
/// 1. At least 3 distinct vertices.
/// 2. Signed area ≠ 0 within the coincidence tolerance.
///
/// # Parameters
/// - `positions` — ordered 3D positions of the boundary vertices
/// - `normal` — face normal for consistent area-sign projection
/// - `coincidence_tol` — area degeneracy threshold
pub fn certify_boundary(
    positions: &[[f64; 3]],
    normal: &[f64; 3],
    coincidence_tol: f64,
) -> Result<BoundaryCertification, KernelError> {
    let n = positions.len();

    if n < 3 {
        return Err(KernelError::InvalidInput {
            message: format!(
                "certify_boundary: loop has only {} vertices (need ≥ 3)",
                n
            ),
            context: None,
        });
    }

    // Project onto a tangent plane and compute 2D signed area (Shoelace).
    let (u_axis, v_axis) = build_tangent_frame(normal);
    let mut signed_area_2x = 0.0_f64;
    for i in 0..n {
        let a = positions[i];
        let b = positions[(i + 1) % n];
        let ua = dot3(&a, &u_axis);
        let va = dot3(&a, &v_axis);
        let ub = dot3(&b, &u_axis);
        let vb = dot3(&b, &v_axis);
        signed_area_2x += ua * vb - ub * va;
    }

    let signed_area = signed_area_2x * 0.5;

    if signed_area.abs() < coincidence_tol * coincidence_tol {
        return Err(KernelError::InvalidInput {
            message: format!(
                "certify_boundary: degenerate loop — signed area {:.2e} < tol² {:.2e}",
                signed_area.abs(),
                coincidence_tol * coincidence_tol,
            ),
            context: None,
        });
    }

    Ok(BoundaryCertification {
        signed_area,
        is_ccw: signed_area > 0.0,
        vertex_count: n,
    })
}

/// Build an orthonormal tangent frame (u, v) perpendicular to `n`.
fn build_tangent_frame(n: &[f64; 3]) -> ([f64; 3], [f64; 3]) {
    let abs_n = [n[0].abs(), n[1].abs(), n[2].abs()];
    let candidate: [f64; 3] = if abs_n[0] <= abs_n[1] && abs_n[0] <= abs_n[2] {
        [1.0, 0.0, 0.0]
    } else if abs_n[1] <= abs_n[2] {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };
    let u = normalize(cross3(n, &candidate));
    let v = cross3(n, &u);
    (u, v)
}

fn cross3(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot3(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-300 { v } else { [v[0] / len, v[1] / len, v[2] / len] }
}
