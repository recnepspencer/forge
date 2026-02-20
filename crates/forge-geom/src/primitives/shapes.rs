//! Plane definitions for standard primitive solids.
//!
//! DOMAIN: Pure geometry — axis-aligned and polyhedral plane sets.
//! DEPENDENCIES: `Plane`, `forge_math::linalg`
//! INVARIANTS: All returned planes are valid (non-zero normals). Results are
//! deterministic and independent of kernel/topology state.

use crate::primitives::plane::Plane;

/// Planes defining an axis-aligned cube.
///
/// Returns 6 planes (±X, ±Y, ±Z) with outward-pointing normals,
/// centered at `center` with the given `half_size`.
pub fn cube(center: [f64; 3], half_size: f64) -> Vec<Plane> {
    vec![
        Plane::from_point_normal(
            [center[0] + half_size, center[1], center[2]],
            [1.0, 0.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0] - half_size, center[1], center[2]],
            [-1.0, 0.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1] + half_size, center[2]],
            [0.0, 1.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1] - half_size, center[2]],
            [0.0, -1.0, 0.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1], center[2] + half_size],
            [0.0, 0.0, 1.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1], center[2] - half_size],
            [0.0, 0.0, -1.0],
        ).unwrap(),
    ]
}

/// Planes defining a regular tetrahedron.
///
/// Returns 4 planes centered at `center` and scaled by `scale`.
/// The tetrahedron has one face pointing up (+Z) and three faces
/// angled downward.
pub fn tetrahedron(center: [f64; 3], scale: f64) -> Vec<Plane> {
    let s = scale;
    vec![
        Plane::from_point_normal(
            [center[0], center[1], center[2] + s],
            [0.0, 0.0, 1.0],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0], center[1] + s, center[2] - s],
            [0.0, 0.8164965809, -0.5773502692],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0] - s * 0.7071, center[1] - s * 0.5, center[2] - s],
            [-0.8164965809, -0.4714045208, -0.3333333333],
        ).unwrap(),
        Plane::from_point_normal(
            [center[0] + s * 0.7071, center[1] - s * 0.5, center[2] - s],
            [0.8164965809, -0.4714045208, -0.3333333333],
        ).unwrap(),
    ]
}

/// Planes defining a regular dodecahedron (12 pentagonal faces).
///
/// Uses golden-ratio face normals. Returns 12 planes centered at
/// `center` and scaled by `scale`.
pub fn dodecahedron(center: [f64; 3], scale: f64) -> Vec<Plane> {
    let phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let inv_phi = 1.0 / phi;

    let raw_normals: [[f64; 3]; 12] = [
        [0.0,  phi,  inv_phi],
        [0.0,  phi, -inv_phi],
        [0.0, -phi,  inv_phi],
        [0.0, -phi, -inv_phi],
        [ inv_phi, 0.0,  phi],
        [-inv_phi, 0.0,  phi],
        [ inv_phi, 0.0, -phi],
        [-inv_phi, 0.0, -phi],
        [ phi,  inv_phi, 0.0],
        [ phi, -inv_phi, 0.0],
        [-phi,  inv_phi, 0.0],
        [-phi, -inv_phi, 0.0],
    ];

    raw_normals.iter().map(|n| {
        let norm = forge_math::linalg::normalize_checked(*n).unwrap();
        let pt = [
            center[0] + norm[0] * scale,
            center[1] + norm[1] * scale,
            center[2] + norm[2] * scale,
        ];
        Plane::from_point_normal(pt, norm).unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_returns_six_planes() {
        let planes = cube([0.0, 0.0, 0.0], 1.0);
        assert_eq!(planes.len(), 6);
    }

    #[test]
    fn tetrahedron_returns_four_planes() {
        let planes = tetrahedron([0.0, 0.0, 0.0], 1.0);
        assert_eq!(planes.len(), 4);
    }

    #[test]
    fn dodecahedron_returns_twelve_planes() {
        let planes = dodecahedron([0.0, 0.0, 0.0], 1.0);
        assert_eq!(planes.len(), 12);
    }

    #[test]
    fn cube_offset_center() {
        let planes = cube([5.0, 3.0, -1.0], 2.0);
        assert_eq!(planes.len(), 6);
    }
}
