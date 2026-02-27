//! Plane definitions for standard primitive solids.
//!
//! DOMAIN: Pure geometry — axis-aligned and polyhedral plane sets.
//! DEPENDENCIES: `Plane`, `forge_math::linalg`
//! INVARIANTS: All returned planes are valid (non-zero normals). Results are
//! deterministic and independent of kernel/topology state.

use forge_math::MathError;

use crate::primitives::plane::Plane;

/// Planes defining an axis-aligned cube (uniform box).
///
/// Returns 6 planes (±X, ±Y, ±Z) with outward-pointing normals,
/// centered at `center` with the given `half_size`.
pub fn cube(center: [f64; 3], half_size: f64) -> Result<Vec<Plane>, MathError> {
    block(center, [half_size, half_size, half_size])
}

/// Planes defining an axis-aligned block (non-uniform box).
///
/// Returns 6 planes (±X, ±Y, ±Z) with outward-pointing normals,
/// centered at `center` with independent half-extents `[hx, hy, hz]`.
pub fn block(center: [f64; 3], half_extents: [f64; 3]) -> Result<Vec<Plane>, MathError> {
    let [hx, hy, hz] = half_extents;
    Ok(vec![
        Plane::from_point_normal(
            [center[0] + hx, center[1], center[2]],
            [1.0, 0.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0] - hx, center[1], center[2]],
            [-1.0, 0.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1] + hy, center[2]],
            [0.0, 1.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1] - hy, center[2]],
            [0.0, -1.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1], center[2] + hz],
            [0.0, 0.0, 1.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1], center[2] - hz],
            [0.0, 0.0, -1.0],
        )?,
    ])
}

/// Planes defining a regular tetrahedron.
///
/// Returns 4 planes centered at `center` and scaled by `scale`.
/// The tetrahedron has one face pointing up (+Z) and three faces
/// angled downward.
pub fn tetrahedron(center: [f64; 3], scale: f64) -> Result<Vec<Plane>, MathError> {
    let s = scale;
    Ok(vec![
        Plane::from_point_normal([center[0], center[1], center[2] + s], [0.0, 0.0, 1.0])?,
        Plane::from_point_normal(
            [center[0], center[1] + s, center[2] - s],
            [0.0, 0.8164965809, -0.5773502692],
        )?,
        Plane::from_point_normal(
            [center[0] - s * 0.7071, center[1] - s * 0.5, center[2] - s],
            [-0.8164965809, -0.4714045208, -0.3333333333],
        )?,
        Plane::from_point_normal(
            [center[0] + s * 0.7071, center[1] - s * 0.5, center[2] - s],
            [0.8164965809, -0.4714045208, -0.3333333333],
        )?,
    ])
}

/// Planes defining a regular dodecahedron (12 pentagonal faces).
///
/// Uses golden-ratio face normals. Returns 12 planes centered at
/// `center` and scaled by `scale`.
pub fn dodecahedron(center: [f64; 3], scale: f64) -> Result<Vec<Plane>, MathError> {
    let phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let inv_phi = 1.0 / phi;

    let raw_normals: [[f64; 3]; 12] = [
        [0.0, phi, inv_phi],
        [0.0, phi, -inv_phi],
        [0.0, -phi, inv_phi],
        [0.0, -phi, -inv_phi],
        [inv_phi, 0.0, phi],
        [-inv_phi, 0.0, phi],
        [inv_phi, 0.0, -phi],
        [-inv_phi, 0.0, -phi],
        [phi, inv_phi, 0.0],
        [phi, -inv_phi, 0.0],
        [-phi, inv_phi, 0.0],
        [-phi, -inv_phi, 0.0],
    ];

    raw_normals
        .iter()
        .map(|n| {
            let norm = forge_math::linalg::normalize_checked(*n).ok_or(
                MathError::InvalidInput("Zero-length normal in dodecahedron".into()),
            )?;
            let pt = [
                center[0] + norm[0] * scale,
                center[1] + norm[1] * scale,
                center[2] + norm[2] * scale,
            ];
            Plane::from_point_normal(pt, norm)
        })
        .collect()
}

/// Planes defining a regular prism (n-gon extruded along Z).
///
/// Returns `n` side planes at uniform angular spacing around the Z axis,
/// plus 2 cap planes (top at `+height/2`, bottom at `-height/2`).
/// All centered at `center`.
///
/// `sides` must be ≥ 3. `radius` and `height` must be > 0.
pub fn prism(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Result<Vec<Plane>, MathError> {
    let half_h = height / 2.0;
    let mut planes = Vec::with_capacity(sides as usize + 2);

    planes.push(Plane::from_point_normal(
        [center[0], center[1], center[2] + half_h],
        [0.0, 0.0, 1.0],
    )?);
    planes.push(Plane::from_point_normal(
        [center[0], center[1], center[2] - half_h],
        [0.0, 0.0, -1.0],
    )?);

    let angle_step = std::f64::consts::TAU / sides as f64;
    for i in 0..sides {
        let angle = angle_step * i as f64;
        let nx = angle.cos();
        let ny = angle.sin();
        let pt = [center[0] + nx * radius, center[1] + ny * radius, center[2]];
        planes.push(Plane::from_point_normal(pt, [nx, ny, 0.0])?);
    }

    Ok(planes)
}

/// Planes defining a pyramid (n-gon base with apex).
///
/// Returns `n` angled side planes converging at the apex, plus 1 base plane.
/// The apex is at `center + [0, 0, height]`, the base at `center - [0, 0, 0]`.
///
/// `sides` must be ≥ 3. `radius` and `height` must be > 0.
pub fn pyramid(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Result<Vec<Plane>, MathError> {
    let mut planes = Vec::with_capacity(sides as usize + 1);

    planes.push(Plane::from_point_normal(center, [0.0, 0.0, -1.0])?);

    let apex = [center[0], center[1], center[2] + height];
    let angle_step = std::f64::consts::TAU / sides as f64;

    let interior = [center[0], center[1], center[2] + height / 3.0];

    for i in 0..sides {
        let a0 = angle_step * i as f64;
        let a1 = angle_step * ((i + 1) % sides) as f64;

        let v0 = [center[0] + a0.cos() * radius, center[1] + a0.sin() * radius, center[2]];
        let v1 = [center[0] + a1.cos() * radius, center[1] + a1.sin() * radius, center[2]];

        let edge_a = forge_math::linalg::sub(v1, v0);
        let edge_b = forge_math::linalg::sub(apex, v0);
        let mut raw_normal = forge_math::linalg::cross(edge_a, edge_b);

        let face_mid = [(v0[0] + v1[0] + apex[0]) / 3.0, (v0[1] + v1[1] + apex[1]) / 3.0, (v0[2] + v1[2] + apex[2]) / 3.0];
        let to_face = forge_math::linalg::sub(face_mid, interior);
        let dot = raw_normal[0] * to_face[0] + raw_normal[1] * to_face[1] + raw_normal[2] * to_face[2];
        if dot < 0.0 {
            raw_normal = [-raw_normal[0], -raw_normal[1], -raw_normal[2]];
        }

        let normal = forge_math::linalg::normalize_checked(raw_normal)
            .ok_or(MathError::InvalidInput(
                "Degenerate pyramid face normal".into(),
            ))?;

        planes.push(Plane::from_point_normal(v0, normal)?);
    }

    Ok(planes)
}

/// Planes defining a wedge (triangular cross-section extruded along Y).
///
/// The wedge sits on the XY base plane, with width `wx` along X,
/// depth `wy` along Y, and height `hz` along Z. The sloped face
/// connects the top-front edge to the bottom-back edge.
///
/// All dimensions must be > 0.
pub fn wedge(center: [f64; 3], dimensions: [f64; 3]) -> Result<Vec<Plane>, MathError> {
    let [wx, wy, hz] = dimensions;
    let hx = wx / 2.0;
    let hy = wy / 2.0;

    let slope_normal = forge_math::linalg::normalize_checked([0.0, hz, wx])
        .ok_or(MathError::InvalidInput(
            "Degenerate wedge slope normal".into(),
        ))?;

    Ok(vec![
        Plane::from_point_normal(
            [center[0], center[1], center[2]],
            [0.0, 0.0, -1.0],
        )?,
        Plane::from_point_normal(
            [center[0] + hx, center[1], center[2]],
            [1.0, 0.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0] - hx, center[1], center[2]],
            [-1.0, 0.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1] - hy, center[2]],
            [0.0, -1.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1] + hy, center[2]],
            [0.0, 1.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1], center[2] + hz],
            slope_normal,
        )?,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_returns_six_planes() {
        let planes = cube([0.0, 0.0, 0.0], 1.0).unwrap();
        assert_eq!(planes.len(), 6);
    }

    #[test]
    fn tetrahedron_returns_four_planes() {
        let planes = tetrahedron([0.0, 0.0, 0.0], 1.0).unwrap();
        assert_eq!(planes.len(), 4);
    }

    #[test]
    fn dodecahedron_returns_twelve_planes() {
        let planes = dodecahedron([0.0, 0.0, 0.0], 1.0).unwrap();
        assert_eq!(planes.len(), 12);
    }

    #[test]
    fn cube_offset_center() {
        let planes = cube([5.0, 3.0, -1.0], 2.0).unwrap();
        assert_eq!(planes.len(), 6);
    }

    #[test]
    fn block_three_half_extents() {
        let planes = block([0.0; 3], [1.0, 2.0, 3.0]).unwrap();
        assert_eq!(planes.len(), 6);
    }

    #[test]
    fn prism_hex_returns_eight_planes() {
        let planes = prism([0.0; 3], 6, 1.0, 2.0).unwrap();
        assert_eq!(planes.len(), 8);
    }

    #[test]
    fn pyramid_quad_returns_five_planes() {
        let planes = pyramid([0.0; 3], 4, 1.0, 2.0).unwrap();
        assert_eq!(planes.len(), 5);
    }

    #[test]
    fn wedge_returns_six_planes() {
        let planes = wedge([0.0; 3], [2.0, 3.0, 1.0]).unwrap();
        assert_eq!(planes.len(), 6);
    }
}
