use worth_math::MathError;
use worth_primitives::canonical_simplex_vertices;

use crate::primitives::plane::Plane;
use crate::primitives::shape_realization::{
    conditioning_witness_with_normalization, feature_size_collapsed, PrimitiveRealizationError,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationExhaustionReport,
};
use crate::spatial::coordinate::local_space::LocalCoordinateSpace;

use super::{
    direct_realization, escalated_realization, oriented_triangle_face_normal,
    PrimitiveRealizationStrategy, PrimitiveSupportRealization,
};

const SIMPLEX_ALTITUDE_RATIO_EXHAUSTION_FLOOR: f64 = 1.0e-32;

pub fn realize_tetrahedron_support(
    center: [f64; 3],
    scale: f64,
) -> Result<PrimitiveSupportRealization, PrimitiveRealizationError> {
    realize_tetrahedron_support_with_altitude_component(center, scale, 0.0)
}

pub fn realize_tetrahedron_support_with_altitude_component(
    center: [f64; 3],
    scale: f64,
    auxiliary_altitude_component: f64,
) -> Result<PrimitiveSupportRealization, PrimitiveRealizationError> {
    let all_points = tetrahedron_vertices(center, scale, auxiliary_altitude_component);
    let preflight_witness = conditioning_witness_with_normalization(&all_points, &[], 1.0, false);
    let direct_planes = tetrahedron_planes_from_points(center, &all_points).ok();

    if let Some(planes) = direct_planes {
        if !preflight_witness.needs_local_transform() {
            return Ok(direct_realization("simplex_solid", all_points, planes));
        }
        return local_normalized_tetrahedron_realization(
            &all_points,
            center,
            scale,
            auxiliary_altitude_component,
            vec![
                PrimitiveRealizationStrategy::DirectWorld,
                PrimitiveRealizationStrategy::LocalNormalized,
            ],
        );
    }

    if preflight_witness.needs_local_transform() && !feature_size_collapsed(&all_points) {
        match local_normalized_tetrahedron_realization(
            &all_points,
            center,
            scale,
            auxiliary_altitude_component,
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
                return exact_support_tetrahedron_realization(
                    center,
                    scale,
                    auxiliary_altitude_component,
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

    exact_support_tetrahedron_realization(
        center,
        scale,
        auxiliary_altitude_component,
        all_points,
        vec![
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ],
    )
}

fn tetrahedron_vertices(
    center: [f64; 3],
    scale: f64,
    auxiliary_altitude_component: f64,
) -> Vec<[f64; 3]> {
    canonical_simplex_vertices(scale, auxiliary_altitude_component)
        .local_vertices()
        .iter()
        .map(|vertex| [vertex[0] + center[0], vertex[1] + center[1], vertex[2] + center[2]])
        .collect()
}

fn tetrahedron_planes_from_points(
    center: [f64; 3],
    vertices: &[[f64; 3]],
) -> Result<Vec<Plane>, MathError> {
    let face_indices = [(0usize, 1usize, 2usize), (0, 3, 1), (0, 2, 3), (1, 3, 2)];
    let mut planes = Vec::with_capacity(face_indices.len());
    for (a, b, c) in face_indices {
        let v0 = vertices[a];
        let v1 = vertices[b];
        let v2 = vertices[c];
        let normal = worth_math::linalg::normalize_checked(
            oriented_triangle_face_normal(v0, v1, v2, center)
                .ok_or_else(|| MathError::InvalidInput("Degenerate simplex face normal".into()))?,
        )
        .ok_or_else(|| MathError::InvalidInput("Degenerate simplex face normal".into()))?;
        planes.push(Plane::from_point_normal(v0, normal)?);
    }
    Ok(planes)
}

fn local_normalized_tetrahedron_realization(
    all_points: &[[f64; 3]],
    center: [f64; 3],
    scale: f64,
    auxiliary_altitude_component: f64,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
) -> Result<PrimitiveSupportRealization, PrimitiveRealizationError> {
    let local_space = LocalCoordinateSpace::from_points(all_points);
    let local_center = local_space.to_local(center);
    let local_scale = scale * local_space.get_scale();
    let local_vertices = tetrahedron_vertices(
        local_center,
        local_scale,
        auxiliary_altitude_component * local_space.get_scale(),
    );
    let local_planes = tetrahedron_planes_from_points(local_center, &local_vertices).map_err(
        |error| match error {
            MathError::InvalidInput(message)
                if message.contains("Degenerate simplex face normal") =>
            {
                PrimitiveRealizationError::Exhausted(exhausted_simplex_realization(
                    all_points,
                    local_space.get_scale(),
                    true,
                    attempted_strategies.clone(),
                    PrimitiveRealizationExhaustionReason::DegenerateSupportNormals,
                ))
            }
            other => PrimitiveRealizationError::Geometry(other),
        },
    )?;
    let world_planes = local_planes
        .iter()
        .map(|plane| local_space.inverse_transform_plane_exact(plane))
        .collect::<Vec<_>>();
    Ok(escalated_realization(
        "simplex_solid",
        all_points.to_vec(),
        world_planes,
        local_space.get_scale(),
        PrimitiveRealizationStrategy::LocalNormalized,
        attempted_strategies,
    ))
}

fn exact_support_tetrahedron_realization(
    center: [f64; 3],
    scale: f64,
    auxiliary_altitude_component: f64,
    all_points: Vec<[f64; 3]>,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
) -> Result<PrimitiveSupportRealization, PrimitiveRealizationError> {
    if altitude_ratio_collapsed(scale, auxiliary_altitude_component) {
        return Err(PrimitiveRealizationError::Exhausted(
            exhausted_simplex_realization(
                &all_points,
                LocalCoordinateSpace::from_origin_and_extent(
                    center,
                    scale.abs().max(f64::MIN_POSITIVE),
                )
                .get_scale(),
                true,
                attempted_strategies,
                PrimitiveRealizationExhaustionReason::DegenerateSupportNormals,
            ),
        ));
    }

    let semantic_extent = scale.abs().max(f64::MIN_POSITIVE);
    let semantic_space = LocalCoordinateSpace::from_origin_and_extent(center, semantic_extent);
    let local_center = semantic_space.to_local(center);
    let local_scale = scale * semantic_space.get_scale();
    let local_vertices = tetrahedron_vertices(
        local_center,
        local_scale,
        auxiliary_altitude_component * semantic_space.get_scale(),
    );
    let local_planes = tetrahedron_planes_from_points(local_center, &local_vertices).map_err(
        |error| match error {
            MathError::InvalidInput(message)
                if message.contains("Degenerate simplex face normal") =>
            {
                PrimitiveRealizationError::Exhausted(exhausted_simplex_realization(
                    &all_points,
                    semantic_space.get_scale(),
                    true,
                    attempted_strategies.clone(),
                    PrimitiveRealizationExhaustionReason::DegenerateSupportNormals,
                ))
            }
            other => PrimitiveRealizationError::Geometry(other),
        },
    )?;
    let world_planes = local_planes
        .iter()
        .map(|plane| semantic_space.inverse_transform_plane_exact(plane))
        .collect::<Vec<_>>();
    Ok(escalated_realization(
        "simplex_solid",
        all_points,
        world_planes,
        semantic_space.get_scale(),
        PrimitiveRealizationStrategy::ExactSupport,
        attempted_strategies,
    ))
}

fn altitude_ratio_collapsed(scale: f64, auxiliary_altitude_component: f64) -> bool {
    let semantic_scale = scale.abs();
    let auxiliary_magnitude = auxiliary_altitude_component.abs();
    if semantic_scale == 0.0 || auxiliary_magnitude == 0.0 {
        return true;
    }
    (auxiliary_magnitude / semantic_scale) <= SIMPLEX_ALTITUDE_RATIO_EXHAUSTION_FLOOR
}

fn exhausted_simplex_realization(
    all_points: &[[f64; 3]],
    normalization_scale: f64,
    normalization_applied: bool,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    reason: PrimitiveRealizationExhaustionReason,
) -> PrimitiveRealizationExhaustionReport {
    PrimitiveRealizationExhaustionReport::new(
        "simplex_solid",
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
