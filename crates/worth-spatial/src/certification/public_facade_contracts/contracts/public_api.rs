mod public_api_anchor_lowering;
mod public_api_arbitration;
mod public_api_carrier_witnesses;
mod public_api_continuity;
mod public_api_preview;

use worth_spatial::facade::{
    admit_spatial_anchor_match_constraint, admit_spatial_frame, admit_spatial_lies_on_constraint,
    admit_spatial_move, admit_spatial_offset, admit_spatial_placement,
    admit_spatial_points_toward_constraint, admit_spatial_reorient, admit_spatial_rotate,
    apply_admitted_anchor_match_constraint_to_placement,
    apply_admitted_lies_on_constraint_to_placement, apply_admitted_move_to_placement,
    apply_admitted_offset_to_placement, apply_admitted_points_toward_constraint_to_placement,
    apply_admitted_reorient_to_placement, apply_admitted_rotate_to_placement,
    apply_spatial_placement, build_primitive_construction_birth_mapping_report,
    certify_primitive_construction_birth_completeness, construction_birth_authority,
    impossible_primitive_construction_birth_attachment, plan_primitive_construction_birth,
    PrimitiveConstructionBirthFamily, PrimitiveConstructionBirthScaffoldInput,
    SpatialAnchorMatchConstraintSpec, SpatialAnchorRef, SpatialAxis,
    SpatialConstructionBirthMappingKind, SpatialDirectionWitnessRef, SpatialFrameRef,
    SpatialLiesOnConstraintSpec, SpatialMotionError, SpatialMoveSpec, SpatialOffsetSpec,
    SpatialPlacementError, SpatialPlacementSpec, SpatialPointsTowardConstraintSpec,
    SpatialReorientSpec, SpatialRotateSpec, SpatialWitnessFailureClass,
    SpatialWitnessResolutionClass,
};

#[test]
fn spatial_public_facade_exports_construction_birth_authority() {
    let authority = construction_birth_authority();
    assert_eq!(
        authority.boundary_name(),
        "worth-spatial.construction-birth-authority"
    );
}

#[test]
fn spatial_public_facade_exports_primitive_birth_planning_surface() {
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
    let completeness =
        certify_primitive_construction_birth_completeness(&input, &plan).expect("completeness");

    assert_eq!(plan.topology_birth_class(), "planar_wire_body");
    assert_eq!(plan.supported_wire_count(), 1);
    assert_eq!(
        plan.realization_strategy(),
        worth_geom::facade::PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(completeness.birth_digest(), plan.birth_digest());
    assert_eq!(completeness.support_plane_count(), 1);
    let mapping = build_primitive_construction_birth_mapping_report(&completeness);
    assert_eq!(
        mapping
            .row_for(SpatialConstructionBirthMappingKind::Wire)
            .expect("wire row")
            .mapped_count(),
        1
    );
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
    let rejection =
        impossible_primitive_construction_birth_attachment(&mismatched, &plan).expect("rejection");
    assert!(rejection.reason().contains("topology birth class"));
}

#[test]
fn spatial_public_facade_exports_shared_spatial_placement_surface() {
    let workplane = SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]);
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
        &SpatialDirectionWitnessRef::frame_axis(
            SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]),
            SpatialAxis::W,
        )
    );
    assert_eq!(
        placement.resolved_direction_witness().resolution_class(),
        SpatialWitnessResolutionClass::FrameDerived
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
    let workplane = SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]);
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
        &SpatialDirectionWitnessRef::frame_axis(
            SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]),
            SpatialAxis::W,
        )
    );
    assert_eq!(
        perpendicular
            .resolved_direction_witness()
            .resolution_class(),
        SpatialWitnessResolutionClass::FallbackDerived
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
fn spatial_public_facade_exports_shared_motion_and_anchor_surface() {
    let moved = admit_spatial_move(
        SpatialMoveSpec::shape_origin()
            .from(SpatialAnchorRef::shape_axis(SpatialAxis::W))
            .to([10.0, 0.0, 3.0]),
    )
    .expect("move plan");
    let rotated = admit_spatial_rotate(
        SpatialRotateSpec::shape_origin()
            .about(SpatialAnchorRef::shape_origin())
            .around([0.0, 1.0, 1.0])
            .by_radians(0.5),
    )
    .expect("rotate plan");
    let reoriented = admit_spatial_reorient(
        SpatialReorientSpec::shape_origin()
            .about(SpatialAnchorRef::shape_origin())
            .toward([1.0, 0.0, 1.0]),
    )
    .expect("reorient plan");
    let offset = admit_spatial_offset(
        SpatialOffsetSpec::shape_origin()
            .from(SpatialAnchorRef::shape_axis(SpatialAxis::U))
            .offset_by([0.0, 0.0, 2.0]),
    )
    .expect("offset plan");
    let parallel = admit_spatial_reorient(SpatialReorientSpec::shape_origin().parallel_to(
        SpatialFrameRef::workplane("wp-2", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
    ))
    .expect("parallel plan");
    let perpendicular =
        admit_spatial_reorient(SpatialReorientSpec::shape_origin().perpendicular_to(
            SpatialFrameRef::workplane("wp-3", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
        ))
        .expect("perpendicular plan");

    assert_eq!(
        moved.spec().anchor(),
        &SpatialAnchorRef::shape_axis(SpatialAxis::W)
    );
    assert_eq!(
        rotated.resolved_axis_witness().resolution_class(),
        SpatialWitnessResolutionClass::DirectWorld
    );
    assert!(rotated.normalized_axis()[1] > 0.70);
    assert!(rotated.normalized_axis()[2] > 0.70);
    assert_eq!(
        reoriented.spec().direction_witness(),
        &SpatialDirectionWitnessRef::world_direction([1.0, 0.0, 1.0])
    );
    assert!(reoriented.normalized_facing()[0] > 0.70);
    assert!(reoriented.normalized_facing()[2] > 0.70);
    assert_eq!(
        offset.spec().anchor(),
        &SpatialAnchorRef::shape_axis(SpatialAxis::U)
    );
    assert_eq!(parallel.normalized_facing(), [0.0, 0.0, 1.0]);
    assert_eq!(
        perpendicular
            .resolved_direction_witness()
            .resolution_class(),
        SpatialWitnessResolutionClass::FallbackDerived
    );
    assert!(perpendicular.normalized_facing()[2].abs() < 1.0e-12);
}

#[test]
fn spatial_public_facade_rejects_ambiguous_and_unsupported_direction_witnesses() {
    let ambiguous = admit_spatial_placement(
        SpatialPlacementSpec::world()
            .facing_witness(SpatialDirectionWitnessRef::ambiguous_curve("curve-1")),
    )
    .expect_err("ambiguous curve witness should fail");
    let unsupported = admit_spatial_reorient(SpatialReorientSpec::shape_origin().toward_witness(
        SpatialDirectionWitnessRef::surface_normal("surface-1", 0.5, 0.5),
    ))
    .expect_err("carrier-level surface witness should fail");

    assert_eq!(
        ambiguous,
        SpatialPlacementError::DirectionWitnessFailure(SpatialWitnessFailureClass::Ambiguous)
    );
    assert_eq!(
        unsupported,
        SpatialMotionError::DirectionWitnessFailure(SpatialWitnessFailureClass::Unsupported)
    );
}

#[test]
fn spatial_public_facade_lowers_admitted_motion_into_placement_updates() {
    let moved = apply_admitted_move_to_placement(
        SpatialPlacementSpec::world(),
        &admit_spatial_move(SpatialMoveSpec::shape_origin().to([10.0, 0.0, 3.0]))
            .expect("move plan"),
    )
    .expect("moved placement");
    let offset = apply_admitted_offset_to_placement(
        moved,
        &admit_spatial_offset(SpatialOffsetSpec::shape_origin().by([2.0, -1.0, 0.5]))
            .expect("offset plan"),
    )
    .expect("offset placement");
    let reoriented = apply_admitted_reorient_to_placement(
        offset,
        &admit_spatial_reorient(SpatialReorientSpec::shape_origin().toward([0.0, 1.0, 1.0]))
            .expect("reorient plan"),
    )
    .expect("reoriented placement");
    let rotated = apply_admitted_rotate_to_placement(
        reoriented,
        &admit_spatial_rotate(
            SpatialRotateSpec::shape_origin()
                .about(SpatialAnchorRef::shape_origin())
                .around([1.0, 0.0, 0.0])
                .by_radians(std::f64::consts::FRAC_PI_2),
        )
        .expect("rotate plan"),
    )
    .expect("rotated placement");
    let admitted_rotated = admit_spatial_placement(rotated.clone()).expect("admitted rotated");

    assert_eq!(rotated.origin(), [12.0, -1.0, 3.5]);
    assert!(admitted_rotated.facing_vector()[1] < -0.70);
    assert!(admitted_rotated.facing_vector()[2] > 0.70);
}

#[test]
fn spatial_public_facade_lowers_constraint_style_intents_into_placement_updates() {
    let workplane = SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let on_plane = apply_admitted_lies_on_constraint_to_placement(
        SpatialPlacementSpec::world().at([4.0, -2.0, 1.0]),
        &admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            workplane.clone(),
        ))
        .expect("lies on"),
    )
    .expect("placement on workplane");
    let pointed = apply_admitted_points_toward_constraint_to_placement(
        on_plane,
        &admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            [0.0, 2.0, 5.0],
        ))
        .expect("points toward"),
    )
    .expect("placement points toward");
    let matched = apply_admitted_anchor_match_constraint_to_placement(
        pointed,
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            SpatialAnchorRef::shape_origin(),
            SpatialAnchorRef::world_origin(),
        ))
        .expect("anchor match"),
    )
    .expect("placement matches world origin");
    let admitted = admit_spatial_placement(matched.clone()).expect("admitted matched placement");

    assert_eq!(matched.origin(), [0.0, 0.0, 0.0]);
    assert_eq!(matched.reference_frame(), &SpatialFrameRef::world());
    assert!(matches!(
        matched.direction_witness(),
        SpatialDirectionWitnessRef::WorldDirection(direction)
            if direction.iter().all(|value| value.is_finite())
    ));
    assert!(admitted
        .facing_vector()
        .iter()
        .any(|value| value.abs() > 0.0));
}

#[test]
fn spatial_public_facade_exports_frame_and_constraint_surface() {
    let workplane = SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let frame = admit_spatial_frame(workplane.clone()).expect("frame");
    let lies_on = admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
        SpatialAnchorRef::shape_axis(SpatialAxis::W),
        workplane.clone(),
    ))
    .expect("lies on");
    let points = admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
        SpatialAnchorRef::frame_origin(workplane.clone()),
        [1.0, 0.0, 2.0],
    ))
    .expect("points");
    let matched = admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
        SpatialAnchorRef::shape_origin(),
        SpatialAnchorRef::frame_origin(workplane),
    ))
    .expect("match");

    assert_eq!(frame.basis().origin(), [0.0, 0.0, 5.0]);
    assert_eq!(
        lies_on.spec().anchor(),
        &SpatialAnchorRef::shape_axis(SpatialAxis::W)
    );
    assert_eq!(points.target_point(), [1.0, 0.0, 2.0]);
    assert_eq!(matched.spec().anchor(), &SpatialAnchorRef::shape_origin());
}
