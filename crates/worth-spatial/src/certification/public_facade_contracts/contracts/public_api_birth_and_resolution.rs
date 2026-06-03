use worth_spatial::facade::{
    bindings::{
        evaluate_primitive_construction_birth_consequence, plan_primitive_construction_birth,
        PrimitiveConstructionBirthFamily, PrimitiveConstructionBirthScaffoldInput,
        SpatialConstructionBirthConsequence, SpatialConstructionBirthMappingKind,
    },
    frames::admit_spatial_frame,
    placement::{admit_spatial_placement, apply_spatial_placement, SpatialPlacementSpec},
    refs, witness_resolution,
};

#[test]
fn spatial_public_facade_exports_primitive_birth_plan_and_consequence_surface() {
    let input = PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionBirthFamily::WireBody,
        "planar_wire_body",
        "scaffold".to_string(),
        vec![
            worth_geom::facade::Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
                .expect("plane"),
        ],
        vec![
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
        4,
        4,
        1,
        1,
        0,
        0,
        1,
    );
    let plan = plan_primitive_construction_birth(input.clone()).expect("birth plan");
    let consequence = evaluate_primitive_construction_birth_consequence(&input, &plan);

    assert_eq!(plan.topology_birth_class(), "planar_wire_body");
    assert_eq!(plan.supported_wire_count(), 1);
    assert_eq!(
        plan.realization_strategy(),
        worth_geom::facade::PrimitiveRealizationStrategy::DirectWorld
    );
    match consequence {
        SpatialConstructionBirthConsequence::Admitted(admitted) => {
            assert_eq!(admitted.birth_digest(), plan.birth_digest());
            assert_eq!(
                admitted
                    .row_for(SpatialConstructionBirthMappingKind::Wire)
                    .expect("wire row")
                    .mapped_count(),
                1
            );
            assert_eq!(
                admitted
                    .row_for(SpatialConstructionBirthMappingKind::Wire)
                    .expect("wire row")
                    .support_plane_count(),
                1
            );
        }
        SpatialConstructionBirthConsequence::Rejected(_) => {
            panic!("expected admitted consequence")
        }
    }
    let mismatched = PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionBirthFamily::WireBody,
        "bad_birth_class",
        "scaffold".to_string(),
        vec![
            worth_geom::facade::Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
                .expect("plane"),
        ],
        vec![
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
        4,
        4,
        1,
        1,
        0,
        0,
        1,
    );
    let rejection = evaluate_primitive_construction_birth_consequence(&mismatched, &plan);
    match rejection {
        SpatialConstructionBirthConsequence::Rejected(rejected) => {
            assert!(rejected.reason().contains("topology birth class"));
        }
        SpatialConstructionBirthConsequence::Admitted(_) => {
            panic!("expected rejected consequence")
        }
    }
}

#[test]
fn spatial_public_facade_exports_shared_spatial_placement_surface() {
    let workplane = refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]);
    let placement = admit_spatial_placement(
        SpatialPlacementSpec::world()
            .between([0.0, 0.0, 1.0], [0.0, 0.0, 3.0])
            .relative_to(workplane.clone())
            .aligned_with(workplane.clone()),
    )
    .expect("placement");
    let base_plane = worth_geom::facade::Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0])
        .expect("plane");
    let geometry = apply_spatial_placement(&placement, &[base_plane], &[[0.0, 0.0, 2.0]])
        .expect("placed geometry");

    let admitted_frame = admit_spatial_frame(workplane).expect("frame");
    assert_eq!(
        placement.reference_frame().basis().origin(),
        admitted_frame.basis().origin()
    );
    assert_eq!(placement.origin(), [12.0, 0.0, 3.0]);
    assert_eq!(
        placement.spec().direction_witness(),
        &refs::SpatialDirectionWitnessRef::frame_axis(
            refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]),
            refs::SpatialAxis::W,
        )
    );
    assert_eq!(
        placement.resolved_direction_witness().resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::FrameDerived
    );
    assert_eq!(placement.facing_vector(), placement.frame().w_axis());
    assert_eq!(
        geometry.vertex_positions()[0],
        placement.embed_point([0.0, 0.0, 2.0])
    );
    assert_eq!(
        geometry.support_planes()[0].normal(),
        placement.facing_vector()
    );
}

#[test]
fn spatial_public_facade_exports_full_prepositional_vocabulary_surface() {
    let workplane = refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]);
    let placement = admit_spatial_placement(
        SpatialPlacementSpec::world()
            .on(workplane.clone())
            .between([0.0, 0.0, 1.0], [0.0, 0.0, 3.0])
            .r#in(workplane.clone())
            .parallel_to(workplane.clone()),
    )
    .expect("placement");
    let perpendicular = admit_spatial_placement(
        SpatialPlacementSpec::world()
            .inside(workplane.clone())
            .perpendicular_to(workplane),
    )
    .expect("perpendicular placement");

    assert_eq!(
        placement.reference_frame().basis().origin(),
        [10.0, 0.0, 3.0]
    );
    assert_eq!(
        placement.spec().direction_witness(),
        &refs::SpatialDirectionWitnessRef::frame_axis(
            refs::SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]),
            refs::SpatialAxis::W,
        )
    );
    assert_eq!(
        perpendicular
            .resolved_direction_witness()
            .resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::FallbackDerived
    );
    assert!((perpendicular.frame().w_axis()[0]).abs() < 1.0e-12);
    assert!(
        (perpendicular.frame().w_axis()[0] * perpendicular.frame().w_axis()[0]
            + perpendicular.frame().w_axis()[1] * perpendicular.frame().w_axis()[1]
            + perpendicular.frame().w_axis()[2] * perpendicular.frame().w_axis()[2]
            - 1.0)
            .abs()
            < 1.0e-12
    );
}

#[test]
fn spatial_public_facade_exports_witness_resolution_truth_not_local_progression_products() {
    let point = witness_resolution::resolve_spatial_point_witness(
        refs::SpatialPointWitnessRef::world_point([3.0, 4.0, 5.0]),
    )
    .expect("resolved point witness");
    let direction = witness_resolution::resolve_spatial_direction_witness(
        refs::SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 2.0]),
    )
    .expect("resolved direction witness");

    assert_eq!(
        point.requested(),
        &refs::SpatialPointWitnessRef::world_point([3.0, 4.0, 5.0])
    );
    assert_eq!(point.resolved_world_point(), [3.0, 4.0, 5.0]);
    assert_eq!(
        point.resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::DirectWorld
    );
    assert_eq!(
        direction.requested(),
        &refs::SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 2.0])
    );
    assert_eq!(direction.resolved_world_direction(), [0.0, 0.0, 1.0]);
    assert_eq!(
        direction.resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::DirectWorld
    );
}
