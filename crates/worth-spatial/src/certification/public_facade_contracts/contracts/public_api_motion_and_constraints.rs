use worth_spatial::facade::{
    constraints::{
        admit_spatial_anchor_match_constraint, admit_spatial_lies_on_constraint,
        admit_spatial_points_toward_constraint,
        apply_admitted_anchor_match_constraint_to_placement,
        apply_admitted_lies_on_constraint_to_placement,
        apply_admitted_points_toward_constraint_to_placement, SpatialAnchorMatchConstraintSpec,
        SpatialLiesOnConstraintSpec, SpatialPointsTowardConstraintSpec,
    },
    frames::admit_spatial_frame,
    lowering::lower_admitted_move_intent,
    motion::{
        admit_spatial_move, admit_spatial_offset, admit_spatial_reorient, admit_spatial_rotate,
        apply_admitted_move_to_placement, apply_admitted_offset_to_placement,
        apply_admitted_reorient_to_placement, apply_admitted_rotate_to_placement,
        SpatialMotionError, SpatialMoveSpec, SpatialOffsetSpec, SpatialReorientSpec,
        SpatialRotateSpec,
    },
    placement::{admit_spatial_placement, SpatialPlacementError, SpatialPlacementSpec},
    refs, witness_resolution,
};

#[test]
fn spatial_public_facade_exports_shared_motion_and_anchor_surface() {
    let moved = admit_spatial_move(
        SpatialMoveSpec::shape_origin()
            .from(refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::W))
            .to([10.0, 0.0, 3.0]),
    )
    .expect("move plan");
    let rotated = admit_spatial_rotate(
        SpatialRotateSpec::shape_origin()
            .about(refs::SpatialAnchorRef::shape_origin())
            .around([0.0, 1.0, 1.0])
            .by_radians(0.5),
    )
    .expect("rotate plan");
    let reoriented = admit_spatial_reorient(
        SpatialReorientSpec::shape_origin()
            .about(refs::SpatialAnchorRef::shape_origin())
            .toward([1.0, 0.0, 1.0]),
    )
    .expect("reorient plan");
    let offset = admit_spatial_offset(
        SpatialOffsetSpec::shape_origin()
            .from(refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::U))
            .offset_by([0.0, 0.0, 2.0]),
    )
    .expect("offset plan");
    let parallel = admit_spatial_reorient(SpatialReorientSpec::shape_origin().parallel_to(
        refs::SpatialFrameRef::workplane("wp-2", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
    ))
    .expect("parallel plan");
    let perpendicular =
        admit_spatial_reorient(SpatialReorientSpec::shape_origin().perpendicular_to(
            refs::SpatialFrameRef::workplane("wp-3", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
        ))
        .expect("perpendicular plan");

    assert_eq!(
        moved.spec().anchor(),
        &refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::W)
    );
    assert_eq!(
        rotated.resolved_axis_witness().resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::DirectWorld
    );
    assert!(rotated.normalized_axis()[1] > 0.70);
    assert!(rotated.normalized_axis()[2] > 0.70);
    assert_eq!(
        reoriented.spec().direction_witness(),
        &refs::SpatialDirectionWitnessRef::world_direction([1.0, 0.0, 1.0])
    );
    assert!(reoriented.normalized_facing()[0] > 0.70);
    assert!(reoriented.normalized_facing()[2] > 0.70);
    assert_eq!(
        offset.spec().anchor(),
        &refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::U)
    );
    assert_eq!(parallel.normalized_facing(), [0.0, 0.0, 1.0]);
    assert_eq!(
        perpendicular
            .resolved_direction_witness()
            .resolution_class(),
        witness_resolution::SpatialWitnessResolutionClass::FallbackDerived
    );
    assert!(perpendicular.normalized_facing()[2].abs() < 1.0e-12);
}

#[test]
fn spatial_public_facade_rejects_ambiguous_and_unsupported_direction_witnesses() {
    let ambiguous = admit_spatial_placement(
        SpatialPlacementSpec::world()
            .facing_witness(refs::SpatialDirectionWitnessRef::ambiguous_curve("curve-1")),
    )
    .expect_err("ambiguous curve witness should fail");
    let unsupported = admit_spatial_reorient(SpatialReorientSpec::shape_origin().toward_witness(
        refs::SpatialDirectionWitnessRef::surface_normal("surface-1", 0.5, 0.5),
    ))
    .expect_err("carrier-level surface witness should fail");

    assert_eq!(
        ambiguous,
        SpatialPlacementError::DirectionWitnessFailure(
            witness_resolution::SpatialWitnessFailureClass::Ambiguous,
        )
    );
    assert_eq!(
        unsupported,
        SpatialMotionError::DirectionWitnessFailure(
            witness_resolution::SpatialWitnessFailureClass::Unsupported,
        )
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
                .about(refs::SpatialAnchorRef::shape_origin())
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
fn spatial_public_facade_lowers_admitted_motion_into_query_declarations() {
    let admitted = admit_spatial_move(
        SpatialMoveSpec::shape_origin()
            .from(refs::SpatialAnchorRef::shape_origin())
            .to([10.0, 0.0, 3.0]),
    )
    .expect("move plan");
    let declaration =
        lower_admitted_move_intent(SpatialPlacementSpec::world(), &admitted).expect("declaration");

    assert_eq!(declaration.name(), "worth.spatial.lowered.move");
    assert_eq!(
        declaration.strategy_name(),
        "worth.spatial.lowering.runtime_handoff"
    );
    assert_eq!(
        declaration.input_contract(),
        "worth.spatial.lowered_runtime_declaration.v1"
    );
    assert!(matches!(
        declaration.input(),
        serde_json::Value::Object(map)
            if matches!(map.get("payload"), Some(serde_json::Value::Object(payload))
                if payload.get("kind") == Some(&serde_json::Value::String("move".to_string())))
    ));
}

#[test]
fn spatial_public_facade_lowers_constraint_style_intents_into_placement_updates() {
    let workplane = refs::SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let on_plane = apply_admitted_lies_on_constraint_to_placement(
        SpatialPlacementSpec::world().at([4.0, -2.0, 1.0]),
        &admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
            refs::SpatialAnchorRef::shape_origin(),
            workplane.clone(),
        ))
        .expect("lies on"),
    )
    .expect("placement on workplane");
    let pointed = apply_admitted_points_toward_constraint_to_placement(
        on_plane,
        &admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
            refs::SpatialAnchorRef::shape_origin(),
            [0.0, 2.0, 5.0],
        ))
        .expect("points toward"),
    )
    .expect("placement points toward");
    let matched = apply_admitted_anchor_match_constraint_to_placement(
        pointed,
        &admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
            refs::SpatialAnchorRef::shape_origin(),
            refs::SpatialAnchorRef::world_origin(),
        ))
        .expect("anchor match"),
    )
    .expect("placement matches world origin");
    let admitted = admit_spatial_placement(matched.clone()).expect("admitted matched placement");

    let matched_frame = admit_spatial_frame(matched.reference_frame().clone()).expect("frame");
    assert_eq!(matched.reference_frame(), &workplane);
    assert_eq!(
        matched_frame.basis().embed_point(matched.origin()),
        [0.0, 0.0, 0.0]
    );
    assert!(matches!(
        matched.direction_witness(),
        refs::SpatialDirectionWitnessRef::WorldDirection(direction)
            if direction.iter().all(|value| value.is_finite())
    ));
    assert!(admitted
        .facing_vector()
        .iter()
        .any(|value| value.abs() > 0.0));
}

#[test]
fn spatial_public_facade_exports_frame_and_constraint_surface() {
    let workplane = refs::SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let frame = admit_spatial_frame(workplane.clone()).expect("frame");
    let lies_on = admit_spatial_lies_on_constraint(SpatialLiesOnConstraintSpec::new(
        refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::W),
        workplane.clone(),
    ))
    .expect("lies on");
    let points = admit_spatial_points_toward_constraint(SpatialPointsTowardConstraintSpec::new(
        refs::SpatialAnchorRef::frame_origin(workplane.clone()),
        [1.0, 0.0, 2.0],
    ))
    .expect("points");
    let matched = admit_spatial_anchor_match_constraint(SpatialAnchorMatchConstraintSpec::new(
        refs::SpatialAnchorRef::shape_origin(),
        refs::SpatialAnchorRef::frame_origin(workplane),
    ))
    .expect("match");

    assert_eq!(frame.basis().origin(), [0.0, 0.0, 5.0]);
    assert_eq!(
        lies_on.spec().anchor(),
        &refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::W)
    );
    assert_eq!(points.target_point(), [1.0, 0.0, 2.0]);
    assert_eq!(
        matched.spec().anchor(),
        &refs::SpatialAnchorRef::shape_origin()
    );
}
