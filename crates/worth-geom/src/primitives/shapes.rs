//! Plane definitions for standard primitive solids.
//!
//! This module now serves as the legacy plane-only adapter surface.
//! The authoritative primitive realization logic lives in
//! `shape_realization.rs`; callers that need conditioning/stability truth
//! should use that surface directly.

use worth_math::MathError;

use crate::primitives::plane::Plane;
use crate::primitives::shape_realization::{
    realize_block_support, realize_prism_support, realize_pyramid_support,
    realize_tetrahedron_support, PrimitiveRealizationError,
};

pub fn cube(center: [f64; 3], half_size: f64) -> Result<Vec<Plane>, MathError> {
    block(center, [half_size, half_size, half_size])
}

pub fn block(center: [f64; 3], half_extents: [f64; 3]) -> Result<Vec<Plane>, MathError> {
    Ok(realize_block_support(center, half_extents)?.into_planes())
}

pub fn tetrahedron(center: [f64; 3], scale: f64) -> Result<Vec<Plane>, MathError> {
    Ok(realize_tetrahedron_support(center, scale)?.into_planes())
}

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
        .map(|normal| {
            let normalized = worth_math::linalg::normalize_checked(*normal).ok_or_else(|| {
                MathError::InvalidInput("Zero-length normal in dodecahedron".into())
            })?;
            let point = [
                center[0] + normalized[0] * scale,
                center[1] + normalized[1] * scale,
                center[2] + normalized[2] * scale,
            ];
            Plane::from_point_normal(point, normalized)
        })
        .collect()
}

pub fn prism(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Result<Vec<Plane>, MathError> {
    Ok(realize_prism_support(center, sides, radius, height)?.into_planes())
}

pub fn pyramid(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Result<Vec<Plane>, MathError> {
    realize_pyramid_support(center, sides, radius, height)
        .map(|realization| realization.into_planes())
        .map_err(map_realization_error)
}

fn map_realization_error(error: PrimitiveRealizationError) -> MathError {
    match error {
        PrimitiveRealizationError::Exhausted(report) => {
            MathError::InvalidInput(report.to_string().into())
        }
        PrimitiveRealizationError::Geometry(error) => error,
    }
}

pub fn wedge(center: [f64; 3], dimensions: [f64; 3]) -> Result<Vec<Plane>, MathError> {
    let [wx, wy, hz] = dimensions;
    let hx = wx / 2.0;
    let hy = wy / 2.0;
    let slope_normal = worth_math::linalg::normalize_checked([0.0, hz, wx])
        .ok_or_else(|| MathError::InvalidInput("Degenerate wedge slope normal".into()))?;
    Ok(vec![
        Plane::from_point_normal([center[0], center[1], center[2]], [0.0, 0.0, -1.0])?,
        Plane::from_point_normal([center[0] + hx, center[1], center[2]], [1.0, 0.0, 0.0])?,
        Plane::from_point_normal([center[0] - hx, center[1], center[2]], [-1.0, 0.0, 0.0])?,
        Plane::from_point_normal([center[0], center[1] - hy, center[2]], [0.0, -1.0, 0.0])?,
        Plane::from_point_normal([center[0], center[1] + hy, center[2]], [0.0, 1.0, 0.0])?,
        Plane::from_point_normal([center[0], center[1], center[2] + hz], slope_normal)?,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_returns_six_planes() {
        assert_eq!(cube([0.0, 0.0, 0.0], 1.0).unwrap().len(), 6);
    }

    #[test]
    fn tetrahedron_returns_four_planes() {
        assert_eq!(tetrahedron([0.0, 0.0, 0.0], 1.0).unwrap().len(), 4);
    }

    #[test]
    fn dodecahedron_returns_twelve_planes() {
        assert_eq!(dodecahedron([0.0, 0.0, 0.0], 1.0).unwrap().len(), 12);
    }

    #[test]
    fn prism_hex_returns_eight_planes() {
        assert_eq!(prism([0.0; 3], 6, 1.0, 2.0).unwrap().len(), 8);
    }

    #[test]
    fn pyramid_quad_returns_five_planes() {
        assert_eq!(pyramid([0.0; 3], 4, 1.0, 2.0).unwrap().len(), 5);
    }
}
