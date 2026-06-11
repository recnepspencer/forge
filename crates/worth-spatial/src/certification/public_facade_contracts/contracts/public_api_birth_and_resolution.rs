use worth_spatial::facade::{
    placement::SpatialPlacementSpec, refs, tolerance::ToleranceAndPrecisionRealizationPosture,
};

#[test]
fn spatial_public_facade_exports_tolerance_realization_posture_without_birth_assessment_runtime() {
    let posture = worth_spatial::facade::tolerance::ToleranceAndPrecisionRealizationPosture::from_direct_planar_support(
        "wire_body",
        &[
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
        &[worth_geom::facade::Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
            .expect("plane")],
    );

    assert_eq!(
        posture.selected_strategy(),
        worth_geom::facade::PrimitiveRealizationStrategy::DirectWorld
    );
    let _: ToleranceAndPrecisionRealizationPosture = posture.clone();
    assert_eq!(posture.attempted_strategies().len(), 1);
}

#[test]
fn spatial_public_facade_exports_full_prepositional_vocabulary_surface() {
    let workplane = refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]);
    let placement = SpatialPlacementSpec::world()
        .on(workplane.clone())
        .between([0.0, 0.0, 1.0], [0.0, 0.0, 3.0])
        .r#in(workplane.clone())
        .parallel_to(workplane.clone());
    let perpendicular = SpatialPlacementSpec::world()
        .inside(workplane.clone())
        .perpendicular_to(workplane.clone());

    assert_eq!(placement.reference_frame(), &workplane);
    assert_eq!(placement.origin(), [0.0, 0.0, 2.0]);
    assert_eq!(
        placement.direction_witness(),
        &refs::SpatialDirectionWitnessRef::frame_axis(
            refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]),
            refs::SpatialAxis::W,
        )
    );
    assert_eq!(
        perpendicular.direction_witness(),
        &refs::SpatialDirectionWitnessRef::frame_perpendicular_axis(
            refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]),
            refs::SpatialAxis::W,
        )
    );
}

#[test]
fn spatial_public_birth_facade_no_longer_exports_birth_proof_support_shelf() {
    let certification = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/certification.rs"));

    assert!(!certification.contains("birth_proof_support"));
}
