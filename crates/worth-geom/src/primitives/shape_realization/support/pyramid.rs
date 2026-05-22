use worth_math::MathError;

use crate::primitives::plane::Plane;
use crate::primitives::shape_realization::{
    conditioning_witness_with_normalization, feature_size_collapsed, PrimitiveRealizationError,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationExhaustionReport,
};
use crate::spatial::coordinate::local_space::LocalCoordinateSpace;

use super::{
    direct_realization, escalated_realization, oriented_triangle_face_normal, polygon_vertices,
    PrimitiveRealizationStrategy, PrimitiveSupportRealization,
};

pub fn realize_pyramid_support(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Result<PrimitiveSupportRealization, PrimitiveRealizationError> {
    let world_points = polygon_vertices(center, sides, radius, 0.0, None);
    let apex = [center[0], center[1], center[2] + height];
    let mut all_points = world_points;
    all_points.push(apex);
    let preflight_witness = conditioning_witness_with_normalization(&all_points, &[], 1.0, false);
    let direct_planes = try_direct_pyramid_planes(center, sides, radius, height);

    if let Some(planes) = direct_planes {
        if !preflight_witness.needs_local_transform() {
            return Ok(direct_realization("regular_pyramid", all_points, planes));
        }
        return local_normalized_pyramid_realization(
            &all_points,
            center,
            sides,
            radius,
            height,
            vec![
                PrimitiveRealizationStrategy::DirectWorld,
                PrimitiveRealizationStrategy::LocalNormalized,
            ],
        );
    }

    if preflight_witness.needs_local_transform() && !feature_size_collapsed(&all_points) {
        match local_normalized_pyramid_realization(
            &all_points,
            center,
            sides,
            radius,
            height,
            vec![
                PrimitiveRealizationStrategy::DirectWorld,
                PrimitiveRealizationStrategy::LocalNormalized,
            ],
        ) {
            Ok(realization) => return Ok(realization),
            Err(PrimitiveRealizationError::Exhausted(report))
                if report.exhaustion_reason()
                    == PrimitiveRealizationExhaustionReason::DegenerateSupportNormals =>
            {
                return exact_support_pyramid_realization(
                    center,
                    sides,
                    radius,
                    height,
                    all_points,
                    vec![
                        PrimitiveRealizationStrategy::DirectWorld,
                        PrimitiveRealizationStrategy::LocalNormalized,
                        PrimitiveRealizationStrategy::ExactSupport,
                    ],
                );
            }
            Err(other) => return Err(other),
        }
    }

    exact_support_pyramid_realization(
        center,
        sides,
        radius,
        height,
        all_points,
        vec![
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ],
    )
}

fn try_direct_pyramid_planes(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
) -> Option<Vec<Plane>> {
    let apex = [center[0], center[1], center[2] + height];
    let base_points = polygon_vertices(center, sides, radius, 0.0, None);
    let mut planes = Vec::with_capacity(sides as usize + 1);
    planes.push(Plane::from_point_normal(center, [0.0, 0.0, -1.0]).ok()?);
    let interior = [center[0], center[1], center[2] + height / 3.0];
    for index in 0..sides as usize {
        let v0 = base_points[index];
        let v1 = base_points[(index + 1) % base_points.len()];
        let mut normal = oriented_triangle_face_normal(v0, v1, apex, interior)?;
        normal = worth_math::linalg::normalize_checked(normal)?;
        planes.push(Plane::from_point_normal(v0, normal).ok()?);
    }
    Some(planes)
}

fn pyramid_planes_from_points(
    center: [f64; 3],
    base_points: &[[f64; 3]],
    apex: [f64; 3],
) -> Result<Vec<Plane>, MathError> {
    let mut planes = Vec::with_capacity(base_points.len() + 1);
    planes.push(Plane::from_point_normal(center, [0.0, 0.0, -1.0])?);
    let interior = [center[0], center[1], (center[2] + apex[2]) / 2.0];
    for index in 0..base_points.len() {
        let v0 = base_points[index];
        let v1 = base_points[(index + 1) % base_points.len()];
        let normal = worth_math::linalg::normalize_checked(
            oriented_triangle_face_normal(v0, v1, apex, interior)
                .ok_or_else(|| MathError::InvalidInput("Degenerate pyramid face normal".into()))?,
        )
        .ok_or_else(|| MathError::InvalidInput("Degenerate pyramid face normal".into()))?;
        planes.push(Plane::from_point_normal(v0, normal)?);
    }
    Ok(planes)
}

fn local_normalized_pyramid_realization(
    all_points: &[[f64; 3]],
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
) -> Result<PrimitiveSupportRealization, PrimitiveRealizationError> {
    let apex = [center[0], center[1], center[2] + height];
    let local_space = LocalCoordinateSpace::from_points(all_points);
    let local_center = local_space.to_local(center);
    let local_radius = radius * local_space.get_scale();
    let local_points = polygon_vertices(local_center, sides, local_radius, 0.0, None);
    let local_apex = local_space.to_local(apex);
    let local_planes = pyramid_planes_from_points(local_center, &local_points, local_apex)
        .map_err(|error| match error {
            MathError::InvalidInput(message)
                if message.contains("Degenerate pyramid face normal") =>
            {
                PrimitiveRealizationError::Exhausted(exhausted_pyramid_realization(
                    all_points,
                    local_space.get_scale(),
                    true,
                    attempted_strategies.clone(),
                    PrimitiveRealizationExhaustionReason::DegenerateSupportNormals,
                ))
            }
            other => PrimitiveRealizationError::Geometry(other),
        })?;
    let world_planes = local_planes
        .iter()
        .map(|plane| local_space.inverse_transform_plane_exact(plane))
        .collect::<Vec<_>>();
    Ok(escalated_realization(
        "regular_pyramid",
        all_points.to_vec(),
        world_planes,
        local_space.get_scale(),
        PrimitiveRealizationStrategy::LocalNormalized,
        attempted_strategies,
    ))
}

fn exact_support_pyramid_realization(
    center: [f64; 3],
    sides: u32,
    radius: f64,
    height: f64,
    all_points: Vec<[f64; 3]>,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
) -> Result<PrimitiveSupportRealization, PrimitiveRealizationError> {
    let semantic_extent = radius.abs().max(height.abs()).max(f64::MIN_POSITIVE);
    let semantic_space = LocalCoordinateSpace::from_origin_and_extent(center, semantic_extent);
    let local_center = semantic_space.to_local(center);
    let local_radius = radius * semantic_space.get_scale();
    let local_height = height * semantic_space.get_scale();
    let local_base_points = polygon_vertices(local_center, sides, local_radius, 0.0, None);
    let local_apex = [
        local_center[0],
        local_center[1],
        local_center[2] + local_height,
    ];
    let local_planes = pyramid_planes_from_points(local_center, &local_base_points, local_apex)
        .map_err(|error| match error {
            MathError::InvalidInput(message)
                if message.contains("Degenerate pyramid face normal") =>
            {
                PrimitiveRealizationError::Exhausted(exhausted_pyramid_realization(
                    &all_points,
                    semantic_space.get_scale(),
                    true,
                    attempted_strategies.clone(),
                    PrimitiveRealizationExhaustionReason::DegenerateSupportNormals,
                ))
            }
            other => PrimitiveRealizationError::Geometry(other),
        })?;
    let world_planes = local_planes
        .iter()
        .map(|plane| semantic_space.inverse_transform_plane_exact(plane))
        .collect::<Vec<_>>();
    Ok(escalated_realization(
        "regular_pyramid",
        all_points,
        world_planes,
        semantic_space.get_scale(),
        PrimitiveRealizationStrategy::ExactSupport,
        attempted_strategies,
    ))
}

fn exhausted_pyramid_realization(
    all_points: &[[f64; 3]],
    normalization_scale: f64,
    normalization_applied: bool,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    reason: PrimitiveRealizationExhaustionReason,
) -> PrimitiveRealizationExhaustionReport {
    PrimitiveRealizationExhaustionReport::new(
        "regular_pyramid",
        attempted_strategies,
        conditioning_witness_with_normalization(
            all_points,
            &[],
            normalization_scale,
            normalization_applied,
        ),
        reason,
    )
}
