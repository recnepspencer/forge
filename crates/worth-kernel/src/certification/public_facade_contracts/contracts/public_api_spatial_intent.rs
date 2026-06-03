use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    authoring::{construction::*, create::CreateSpatialIntent, intents::*},
    diagnostics::motion::*,
};
use worth_spatial::facade::constraints::{
    admit_spatial_anchor_match_constraint, admit_spatial_lies_on_constraint,
    admit_spatial_points_toward_constraint,
};
use worth_spatial::facade::motion::{
    admit_spatial_move, admit_spatial_offset, admit_spatial_reorient, admit_spatial_rotate,
};
use worth_spatial::facade::placement::admit_spatial_placement;
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialAxis, SpatialDirectionWitnessRef, SpatialFrameRef,
    SpatialPointWitnessRef,
};
use worth_spatial::facade::witness_resolution::{
    SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

fn with_authoring_session<T>(
    workspace_name: &str,
    run: impl FnOnce(&mut PrimitiveConstructionAuthoringSession<'_>) -> T,
) -> T {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        workspace_name.to_string(),
    )
    .expect("workspace");
    let mut session = primitive_construction_authoring(&mut workspace).expect("authoring session");
    run(&mut session)
}

#[test]
fn kernel_public_facade_exports_prepositional_create_placement_surface() {
    let workplane = SpatialFrameRef::workplane("wp-1", [10.0, 0.0, 3.0], [1.0, 0.0, 0.0]);
    let created = CreateSpatialIntent::new(PrimitiveConstructionIntent::regular_pyramid(
        RegularPyramidSpec {
            sides: 4,
            radius: 1.0,
            height: 2.0,
        },
    ))
    .on(workplane.clone())
    .between([0.0, 0.0, 1.0], [0.0, 0.0, 3.0])
    .r#in(workplane.clone())
    .parallel_to(workplane.clone());
    let intent = created.clone().finish();
    let admitted_placement =
        admit_spatial_placement(intent.placement_spec()).expect("admitted placement");

    assert_eq!(created.placement_spec().origin(), [0.0, 0.0, 2.0]);
    assert_eq!(created.placement_spec().reference_frame(), &workplane);
    assert_eq!(
        created.placement_spec().direction_witness(),
        &SpatialDirectionWitnessRef::frame_axis(workplane.clone(), SpatialAxis::W)
    );
    assert_eq!(intent.placement_spec().reference_frame(), &workplane);
    assert_eq!(
        admitted_placement
            .resolved_direction_witness()
            .resolution_class(),
        SpatialWitnessResolutionClass::FrameDerived
    );
}

#[test]
fn kernel_public_facade_exports_remaining_prepositional_create_aliases() {
    let workplane = SpatialFrameRef::workplane("wp-2", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let intent = CreateSpatialIntent::new(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 4,
    }))
    .inside(workplane.clone())
    .perpendicular_to(workplane.clone())
    .finish();
    let admitted_placement = admit_spatial_placement(intent.placement_spec()).expect("placement");
    let facing = admitted_placement.facing_vector();

    assert_eq!(intent.placement_spec().reference_frame(), &workplane);
    assert_eq!(
        admitted_placement
            .resolved_direction_witness()
            .resolution_class(),
        SpatialWitnessResolutionClass::FallbackDerived
    );
    assert!(
        (facing[0] * facing[0] + facing[1] * facing[1] + facing[2] * facing[2] - 1.0).abs()
            < 1.0e-12
    );
    assert!(facing[2].abs() < 1.0e-12);
}

#[test]
fn kernel_public_facade_exports_authored_motion_verbs() {
    let workplane = SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let moved = MoveSpatialIntent::shape("shape-1")
        .from(SpatialAnchorRef::shape_axis(SpatialAxis::W))
        .to_witness(SpatialPointWitnessRef::frame_origin(workplane.clone()));
    let rotated = RotateSpatialIntent::shape("shape-1")
        .about(SpatialAnchorRef::shape_origin())
        .rotated_about([0.0, 1.0, 1.0], 0.5);
    let reoriented = ReorientSpatialIntent::shape("shape-1")
        .about(SpatialAnchorRef::shape_origin())
        .parallel_to(workplane.clone());
    let offset = OffsetSpatialIntent::shape("shape-1")
        .from(SpatialAnchorRef::shape_axis(SpatialAxis::U))
        .offset_by([0.0, 0.0, 2.0]);
    let lies_on = MoveSpatialIntent::shape("shape-1")
        .so(SpatialAnchorRef::shape_axis(SpatialAxis::W))
        .lies_on(workplane.clone());
    let points_toward = ReorientSpatialIntent::shape("shape-1")
        .so(SpatialAnchorRef::shape_origin())
        .points_toward([1.0, 2.0, 3.0]);
    let matched = MoveSpatialIntent::shape("shape-1")
        .so(SpatialAnchorRef::shape_origin())
        .matches(SpatialAnchorRef::frame_origin(workplane.clone()));
    let admitted_move = admit_spatial_move(moved.motion_spec()).expect("move plan");
    let admitted_rotate = admit_spatial_rotate(rotated.motion_spec()).expect("rotate plan");
    let admitted_reorient =
        admit_spatial_reorient(reoriented.motion_spec()).expect("reorient plan");
    let admitted_offset = admit_spatial_offset(offset.motion_spec()).expect("offset plan");
    let admitted_lies_on =
        admit_spatial_lies_on_constraint(lies_on.constraint_spec().clone()).expect("lies on");
    let admitted_points_toward =
        admit_spatial_points_toward_constraint(points_toward.constraint_spec().clone())
            .expect("points");
    let admitted_match =
        admit_spatial_anchor_match_constraint(matched.constraint_spec().clone()).expect("match");

    assert_eq!(moved.subject(), &"shape-1");
    assert_eq!(
        admitted_move.spec().anchor(),
        &SpatialAnchorRef::shape_axis(SpatialAxis::W)
    );
    assert_eq!(
        admitted_move
            .resolved_destination_witness()
            .resolution_class(),
        SpatialWitnessResolutionClass::FrameDerived
    );
    assert_eq!(rotated.subject(), &"shape-1");
    assert_eq!(
        admitted_rotate.resolved_axis_witness().resolution_class(),
        SpatialWitnessResolutionClass::DirectWorld
    );
    assert!(admitted_rotate.normalized_axis()[1] > 0.70);
    assert!(admitted_rotate.normalized_axis()[2] > 0.70);
    assert_eq!(reoriented.subject(), &"shape-1");
    assert_eq!(
        admitted_reorient
            .resolved_direction_witness()
            .resolution_class(),
        SpatialWitnessResolutionClass::FrameDerived
    );
    assert_eq!(admitted_reorient.normalized_facing(), [0.0, 0.0, 1.0]);
    assert_eq!(offset.subject(), &"shape-1");
    assert_eq!(
        admitted_offset.spec().anchor(),
        &SpatialAnchorRef::shape_axis(SpatialAxis::U)
    );
    assert_eq!(lies_on.subject(), &"shape-1");
    assert_eq!(admitted_lies_on.frame().basis().origin(), [0.0, 0.0, 5.0]);
    assert_eq!(points_toward.subject(), &"shape-1");
    assert_eq!(admitted_points_toward.target_point(), [1.0, 2.0, 3.0]);
    assert_eq!(matched.subject(), &"shape-1");
    assert_eq!(
        admitted_match.spec().anchor(),
        &SpatialAnchorRef::shape_origin()
    );
}

#[test]
fn kernel_public_facade_lowers_primitive_motion_through_query_entry_session() {
    let moved = with_authoring_session("worth-kernel.public-motion-move", |session| {
        session
            .author(
                MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 6,
                }))
                .to([10.0, 0.0, 3.0]),
            )
            .and_then(|entry| entry.prepare_result())
    });
    let reoriented = with_authoring_session("worth-kernel.public-motion-reorient", |session| {
        session
            .author(
                ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 4,
                        radius: 1.0,
                        height: 2.0,
                    },
                ))
                .perpendicular_to(SpatialFrameRef::workplane(
                    "wp-3",
                    [0.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0],
                )),
            )
            .and_then(|entry| entry.prepare_result())
    });
    let rotated = with_authoring_session("worth-kernel.public-motion-rotate", |session| {
        session
            .author(
                RotateSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 4,
                        radius: 1.0,
                        height: 2.0,
                    },
                ))
                .about(SpatialAnchorRef::shape_origin())
                .rotated_about([1.0, 0.0, 0.0], std::f64::consts::FRAC_PI_2),
            )
            .and_then(|entry| entry.prepare_result())
    });
    let lies_on = with_authoring_session("worth-kernel.public-motion-lies-on", |session| {
        session
            .author(
                MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 6,
                }))
                .so(SpatialAnchorRef::shape_origin())
                .lies_on(SpatialFrameRef::workplane(
                    "wp-1",
                    [0.0, 0.0, 5.0],
                    [0.0, 0.0, 1.0],
                )),
            )
            .map(|entry| entry.prepare_outcome())
    });
    let pointed = with_authoring_session("worth-kernel.public-motion-points", |session| {
        session
            .author(
                ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 4,
                        radius: 1.0,
                        height: 2.0,
                    },
                ))
                .so(SpatialAnchorRef::world_origin())
                .points_toward([0.0, 3.0, 0.0]),
            )
            .and_then(|entry| entry.prepare_result())
    });
    let matched = with_authoring_session("worth-kernel.public-motion-match", |session| {
        session
            .author(
                MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 6,
                }))
                .so(SpatialAnchorRef::shape_origin())
                .matches(SpatialAnchorRef::world_origin()),
            )
            .map(|entry| entry.prepare_outcome())
    });
    let rotated_about_frame_origin =
        with_authoring_session("worth-kernel.public-motion-frame-origin", |session| {
            session
                .author(
                    RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(
                        WireBodySpec { edge_count: 6 },
                    ))
                    .about(SpatialAnchorRef::frame_origin(SpatialFrameRef::workplane(
                        "pivot-1",
                        [4.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0],
                    )))
                    .around([0.0, 0.0, 1.0])
                    .by_radians(std::f64::consts::FRAC_PI_2),
                )
                .and_then(|entry| entry.prepare_result())
        });
    let unsupported_anchor =
        with_authoring_session("worth-kernel.public-motion-unsupported-anchor", |session| {
            session
                .author(
                    MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(
                        WireBodySpec { edge_count: 6 },
                    ))
                    .from(SpatialAnchorRef::shape_axis(SpatialAxis::W))
                    .to([10.0, 0.0, 3.0]),
                )
                .and_then(|entry| entry.prepare_result())
        });

    assert!(moved.is_ok());
    assert!(reoriented.is_ok());
    assert!(rotated.is_ok());
    assert!(lies_on.is_ok());
    assert!(pointed.is_ok());
    assert!(matched.is_ok());
    assert!(rotated_about_frame_origin.is_ok());
    assert!(matches!(
        unsupported_anchor,
        Err(PrimitiveConstructionQueryEntryError::Lowering(
            PrimitiveConstructionSpatialIntentError::PlacementLowering(_)
        ))
    ));

    let undefined = admit_spatial_reorient(
        ReorientSpatialIntent::shape("shape-2")
            .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 0.0]))
            .motion_spec(),
    )
    .expect_err("undefined witness should fail");
    let ambiguous_target = admit_spatial_move(
        MoveSpatialIntent::shape("shape-3")
            .to_witness(SpatialPointWitnessRef::ambiguous_curve_point("curve-1"))
            .motion_spec(),
    )
    .expect_err("ambiguous target should fail");
    assert_eq!(
        undefined,
        worth_spatial::facade::motion::SpatialMotionError::DirectionWitnessFailure(
            SpatialWitnessFailureClass::Undefined
        )
    );
    assert_eq!(
        ambiguous_target,
        worth_spatial::facade::motion::SpatialMotionError::DestinationWitnessFailure(
            SpatialWitnessFailureClass::Ambiguous
        )
    );
}

#[test]
fn kernel_public_facade_exports_motion_witness_resolution_reports() {
    let move_report = prepare_primitive_construction_move_witness_resolution_report(
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .to_witness(SpatialPointWitnessRef::frame_origin(
            SpatialFrameRef::workplane("wp-9", [3.0, 4.0, 5.0], [0.0, 0.0, 1.0]),
        )),
    );
    let rotate_report = prepare_primitive_construction_rotate_witness_resolution_report(
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .around([0.0, 1.0, 1.0])
        .by_radians(0.5),
    );
    let reorient_report = prepare_primitive_construction_reorient_witness_resolution_report(
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 4,
                radius: 1.0,
                height: 2.0,
            },
        ))
        .parallel_to(SpatialFrameRef::workplane(
            "wp-10",
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
        )),
    );
    let points_report = prepare_primitive_construction_points_toward_witness_resolution_report(
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 4,
                radius: 1.0,
                height: 2.0,
            },
        ))
        .so(SpatialAnchorRef::shape_origin())
        .points_toward_witness(SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0])),
    );

    assert_eq!(
        move_report.kind(),
        PrimitiveConstructionMotionWitnessResolutionKind::Move
    );
    assert_eq!(
        move_report.requested_witness(),
        &PrimitiveConstructionRequestedMotionWitness::Point(SpatialPointWitnessRef::frame_origin(
            SpatialFrameRef::workplane("wp-9", [3.0, 4.0, 5.0], [0.0, 0.0, 1.0],)
        ))
    );
    assert_eq!(
        move_report.resolution_class(),
        Some(SpatialWitnessResolutionClass::FrameDerived)
    );
    assert_eq!(move_report.resolved_target_point(), Some([3.0, 4.0, 5.0]));
    assert_eq!(
        rotate_report.kind(),
        PrimitiveConstructionMotionWitnessResolutionKind::Rotate
    );
    assert_eq!(
        rotate_report.resolution_class(),
        Some(SpatialWitnessResolutionClass::DirectWorld)
    );
    assert!(rotate_report.resolved_world_direction().expect("direction")[1] > 0.70);
    assert_eq!(
        reorient_report.kind(),
        PrimitiveConstructionMotionWitnessResolutionKind::Reorient
    );
    assert_eq!(
        reorient_report.resolution_class(),
        Some(SpatialWitnessResolutionClass::FrameDerived)
    );
    assert_eq!(
        points_report.kind(),
        PrimitiveConstructionMotionWitnessResolutionKind::PointsToward
    );
    assert_eq!(points_report.resolved_target_point(), Some([1.0, 2.0, 3.0]));
}

#[test]
fn kernel_public_facade_exports_motion_witness_resolution_failure_truth() {
    let rotate_report = prepare_primitive_construction_rotate_witness_resolution_report(
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 3,
        }))
        .around([0.0, 0.0, 1.0])
        .by_radians(f64::NAN),
    );

    assert_eq!(
        rotate_report.failure_kind(),
        Some(PrimitiveConstructionMotionWitnessResolutionFailureKind::NonFiniteRotationAngle)
    );
}
