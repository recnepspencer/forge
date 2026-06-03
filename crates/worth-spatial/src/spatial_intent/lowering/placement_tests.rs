use super::{admit_spatial_placement, apply_spatial_placement, SpatialPlacementSpec};
use crate::facade::{refs, witness_resolution};
use worth_geom::facade::{realize_tetrahedron_support_with_altitude_component, Plane};

#[test]
fn admitted_spatial_placement_builds_deterministic_frame_from_requested_witness() {
    let placement = admit_spatial_placement(
        SpatialPlacementSpec::world()
            .at([10.0, 0.0, 3.0])
            .facing_witness(refs::SpatialDirectionWitnessRef::world_direction([
                0.0, 1.0, 1.0,
            ])),
    )
    .expect("placement");

    assert_eq!(placement.origin(), [10.0, 0.0, 3.0]);
    assert_eq!(
        placement.resolved_direction_witness().resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::DirectWorld
    );
    let facing = placement.facing_vector();
    assert!(facing[1] > 0.70);
    assert!(facing[2] > 0.70);
}

#[test]
fn relative_frame_and_alignment_preserve_requested_vs_resolved_witness_truth() {
    let workplane = refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]);
    let placement = admit_spatial_placement(
        SpatialPlacementSpec::world()
            .between([0.0, 0.0, 1.0], [0.0, 0.0, 3.0])
            .relative_to(workplane.clone())
            .aligned_with(workplane),
    )
    .expect("placement");

    assert_eq!(placement.origin(), [12.0, 0.0, 3.0]);
    assert_eq!(placement.facing_vector(), [1.0, 0.0, 0.0]);
    assert_eq!(
        placement.resolved_direction_witness().resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::FrameDerived
    );
    assert_eq!(
        placement.spec().direction_witness(),
        &refs::SpatialDirectionWitnessRef::frame_axis(
            refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]),
            crate::facade::refs::SpatialAxis::W,
        )
    );
}

#[test]
fn prepositional_aliases_preserve_requested_witness_meaning() {
    let workplane = refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]);
    let parallel = SpatialPlacementSpec::world().parallel_to(workplane.clone());
    let perpendicular = admit_spatial_placement(
        SpatialPlacementSpec::world()
            .inside(workplane.clone())
            .perpendicular_to(workplane.clone()),
    )
    .expect("perpendicular");

    assert_eq!(parallel.reference_frame(), &workplane);
    assert_eq!(
        parallel.direction_witness(),
        &refs::SpatialDirectionWitnessRef::frame_axis(
            refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]),
            crate::facade::refs::SpatialAxis::W,
        )
    );
    assert_eq!(
        perpendicular
            .resolved_direction_witness()
            .resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::FallbackDerived
    );
}

#[test]
fn spatial_placement_embeds_points_and_planes() {
    let placement = admit_spatial_placement(
        SpatialPlacementSpec::world()
            .at([10.0, 0.0, 3.0])
            .facing([1.0, 0.0, 0.0]),
    )
    .expect("placement");
    let geometry = apply_spatial_placement(
        &placement,
        &[Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("plane")],
        &[[0.0, 0.0, 2.0]],
    )
    .expect("embedded geometry");

    let point = geometry.vertex_positions()[0];
    assert!((point[0] - 12.0).abs() < 1.0e-12);
    assert!((point[1] - 0.0).abs() < 1.0e-12);
    assert!((point[2] - 3.0).abs() < 1.0e-12);

    let plane_normal = geometry.support_planes()[0].normal();
    assert!((plane_normal[0] - 1.0).abs() < 1.0e-12);
    assert!(plane_normal[1].abs() < 1.0e-12);
    assert!(plane_normal[2].abs() < 1.0e-12);
}

#[test]
fn placement_rejects_ambiguous_direction_witnesses_honestly() {
    let error = admit_spatial_placement(SpatialPlacementSpec::world().facing_witness(
        refs::SpatialDirectionWitnessRef::ambiguous_surface("surface-1"),
    ))
    .expect_err("ambiguous witness should fail");

    assert_eq!(
        error,
        super::SpatialPlacementError::DirectionWitnessFailure(
            witness_resolution::SpatialWitnessFailureClass::Ambiguous
        )
    );
}

#[test]
fn placement_embeds_world_collapsed_exact_support_planes_without_invalidating_them() {
    let placement = admit_spatial_placement(SpatialPlacementSpec::world().at([
        2.0f64.powi(548),
        -2.0f64.powi(548),
        2.0f64.powi(548),
    ]))
    .expect("placement");
    let realization =
        realize_tetrahedron_support_with_altitude_component([0.0, 0.0, 0.0], 1.0e-200, 1.0e-220)
            .expect("simplex realization");
    let geometry = apply_spatial_placement(
        &placement,
        realization.planes(),
        &[
            [0.0, 0.0, 1.0e-200],
            [0.0, 1.0e-200, -1.0e-200],
            [-1.0e-200 * 0.7071, -1.0e-200 * 0.5, -1.0e-200],
            [1.0e-200 * 0.7071, -1.0e-200 * 0.5, -1.0e-200 + 1.0e-220],
        ],
    )
    .expect("embedded geometry");

    assert_eq!(geometry.support_planes().len(), 4);
    assert_eq!(geometry.vertex_positions().len(), 4);
    assert!(geometry
        .support_planes()
        .iter()
        .all(|plane| plane.normal().iter().all(|value| value.is_finite())));
    assert!(geometry
        .vertex_positions()
        .iter()
        .all(|point| point.iter().all(|value| value.is_finite())));
}
