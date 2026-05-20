use worth_math::MathError;

use crate::primitives::plane::Plane;
use crate::primitives::shape_realization::schema::{
    PrimitiveRealizationReport, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportRealization,
};
use crate::primitives::shape_realization::{
    build_direct_realization_report, conditioning_witness_with_normalization,
    feature_size_collapsed, PrimitiveRealizationError, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationExhaustionReport,
};
use crate::spatial::coordinate::local_space::LocalCoordinateSpace;

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

pub fn realize_tetrahedron_support(
    center: [f64; 3],
    scale: f64,
) -> Result<PrimitiveSupportRealization, MathError> {
    let planes = vec![
        Plane::from_point_normal([center[0], center[1], center[2] + scale], [0.0, 0.0, 1.0])?,
        Plane::from_point_normal(
            [center[0], center[1] + scale, center[2] - scale],
            [0.0, 0.8164965809, -0.5773502692],
        )?,
        Plane::from_point_normal(
            [
                center[0] - scale * 0.7071,
                center[1] - scale * 0.5,
                center[2] - scale,
            ],
            [-0.8164965809, -0.4714045208, -0.3333333333],
        )?,
        Plane::from_point_normal(
            [
                center[0] + scale * 0.7071,
                center[1] - scale * 0.5,
                center[2] - scale,
            ],
            [0.8164965809, -0.4714045208, -0.3333333333],
        )?,
    ];
    let points = vec![
        [center[0], center[1], center[2] + scale],
        [center[0], center[1] + scale, center[2] - scale],
        [
            center[0] - scale * 0.7071,
            center[1] - scale * 0.5,
            center[2] - scale,
        ],
        [
            center[0] + scale * 0.7071,
            center[1] - scale * 0.5,
            center[2] - scale,
        ],
    ];
    Ok(direct_realization("simplex_solid", points, planes))
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
        let mut normal = oriented_face_normal(v0, v1, apex, interior)?;
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
            oriented_face_normal(v0, v1, apex, interior)
                .ok_or_else(|| MathError::InvalidInput("Degenerate pyramid face normal".into()))?,
        )
        .ok_or_else(|| MathError::InvalidInput("Degenerate pyramid face normal".into()))?;
        planes.push(Plane::from_point_normal(v0, normal)?);
    }
    Ok(planes)
}

fn oriented_face_normal(
    v0: [f64; 3],
    v1: [f64; 3],
    apex: [f64; 3],
    interior: [f64; 3],
) -> Option<[f64; 3]> {
    let edge_a = worth_math::linalg::sub(v1, v0);
    let edge_b = worth_math::linalg::sub(apex, v0);
    let mut raw_normal = worth_math::linalg::cross(edge_a, edge_b);
    let face_mid = [
        (v0[0] + v1[0] + apex[0]) / 3.0,
        (v0[1] + v1[1] + apex[1]) / 3.0,
        (v0[2] + v1[2] + apex[2]) / 3.0,
    ];
    let to_face = worth_math::linalg::sub(face_mid, interior);
    let dot = raw_normal[0] * to_face[0] + raw_normal[1] * to_face[1] + raw_normal[2] * to_face[2];
    if dot < 0.0 {
        raw_normal = [-raw_normal[0], -raw_normal[1], -raw_normal[2]];
    }
    Some(raw_normal)
}

fn direct_realization(
    family: &'static str,
    points: Vec<[f64; 3]>,
    planes: Vec<Plane>,
) -> PrimitiveSupportRealization {
    let report = build_direct_realization_report(family, &points, &planes);
    PrimitiveSupportRealization::new(planes, report)
}

fn escalated_realization(
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
    );
    PrimitiveSupportRealization::new(planes, report)
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

fn polygon_vertices(
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
