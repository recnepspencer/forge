use worth_math::MathError;

use crate::primitives::plane::Plane;
use crate::primitives::shape_realization::schema::{
    PrimitiveRealizationReport, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportRealization,
};
use crate::primitives::shape_realization::{
    build_direct_realization_report, conditioning_witness_with_normalization,
};

mod pyramid;
mod simplex;

pub use pyramid::realize_pyramid_support;
pub use simplex::{
    realize_tetrahedron_support, realize_tetrahedron_support_with_altitude_component,
};

pub fn realize_block_support(
    center: [f64; 3],
    half_extents: [f64; 3],
) -> Result<PrimitiveSupportRealization, MathError> {
    let planes = vec![
        Plane::from_point_normal(
            [center[0] + half_extents[0], center[1], center[2]],
            [1.0, 0.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0] - half_extents[0], center[1], center[2]],
            [-1.0, 0.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1] + half_extents[1], center[2]],
            [0.0, 1.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1] - half_extents[1], center[2]],
            [0.0, -1.0, 0.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1], center[2] + half_extents[2]],
            [0.0, 0.0, 1.0],
        )?,
        Plane::from_point_normal(
            [center[0], center[1], center[2] - half_extents[2]],
            [0.0, 0.0, -1.0],
        )?,
    ];
    Ok(direct_realization(
        "orthotope",
        points_for_block(center, half_extents),
        planes,
    ))
}

pub fn realize_prism_support(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Result<PrimitiveSupportRealization, MathError> {
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
        let normal = [angle.cos(), angle.sin(), 0.0];
        let point = [
            center[0] + normal[0] * radius,
            center[1] + normal[1] * radius,
            center[2],
        ];
        planes.push(Plane::from_point_normal(point, normal)?);
    }
    Ok(direct_realization(
        "regular_prism",
        polygon_vertices(center, sides, radius, 0.0, Some(height)),
        planes,
    ))
}

pub(super) fn direct_realization(
    family: &'static str,
    points: Vec<[f64; 3]>,
    planes: Vec<Plane>,
) -> PrimitiveSupportRealization {
    let report = build_direct_realization_report(family, &points, &planes);
    PrimitiveSupportRealization::new(planes, report)
}

pub(super) fn escalated_realization(
    family: &'static str,
    points: Vec<[f64; 3]>,
    planes: Vec<Plane>,
    normalization_scale: f64,
    strategy: PrimitiveRealizationStrategy,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
) -> PrimitiveSupportRealization {
    let report = PrimitiveRealizationReport::new(
        family,
        strategy,
        attempted_strategies,
        PrimitiveStabilityClass::StableAfterEscalation,
        conditioning_witness_with_normalization(&points, &planes, normalization_scale, true),
        &points,
        &planes,
    );
    PrimitiveSupportRealization::new(planes, report)
}

pub(super) fn polygon_vertices(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    z_offset: f64,
    height: Option<f64>,
) -> Vec<[f64; 3]> {
    let mut vertices = Vec::with_capacity((sides as usize) * if height.is_some() { 2 } else { 1 });
    let angle_step = std::f64::consts::TAU / sides as f64;
    for index in 0..sides {
        let angle = angle_step * index as f64;
        let base = [
            center[0] + angle.cos() * radius,
            center[1] + angle.sin() * radius,
            center[2] + z_offset,
        ];
        vertices.push(base);
        if let Some(height) = height {
            vertices.push([base[0], base[1], base[2] + height]);
        }
    }
    vertices
}

pub(super) fn oriented_triangle_face_normal(
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
    interior: [f64; 3],
) -> Option<[f64; 3]> {
    let edge_a = worth_math::linalg::sub(b, a);
    let edge_b = worth_math::linalg::sub(c, a);
    let mut raw_normal = worth_math::linalg::cross(edge_a, edge_b);
    let face_mid = [
        (a[0] + b[0] + c[0]) / 3.0,
        (a[1] + b[1] + c[1]) / 3.0,
        (a[2] + b[2] + c[2]) / 3.0,
    ];
    let to_face = worth_math::linalg::sub(face_mid, interior);
    let dot = raw_normal[0] * to_face[0] + raw_normal[1] * to_face[1] + raw_normal[2] * to_face[2];
    if dot < 0.0 {
        raw_normal = [-raw_normal[0], -raw_normal[1], -raw_normal[2]];
    }
    Some(raw_normal)
}

fn points_for_block(center: [f64; 3], half_extents: [f64; 3]) -> Vec<[f64; 3]> {
    let [hx, hy, hz] = half_extents;
    vec![
        [center[0] - hx, center[1] - hy, center[2] - hz],
        [center[0] - hx, center[1] - hy, center[2] + hz],
        [center[0] - hx, center[1] + hy, center[2] - hz],
        [center[0] - hx, center[1] + hy, center[2] + hz],
        [center[0] + hx, center[1] - hy, center[2] - hz],
        [center[0] + hx, center[1] - hy, center[2] + hz],
        [center[0] + hx, center[1] + hy, center[2] - hz],
        [center[0] + hx, center[1] + hy, center[2] + hz],
    ]
}
