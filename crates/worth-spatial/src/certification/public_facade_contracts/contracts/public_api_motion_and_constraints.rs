use worth_spatial::facade::{
    anchor_selection::{
        AuthorSpatialAnchorSelectionIntent, SpatialAnchorMatchConstraintSpec,
        SpatialAnchorSelectionDeclarationEntry, SpatialAnchorSelectionFailureKind,
        SpatialAnchorSelectionRequestedInput, SpatialAnchorSelectionStatus,
        SpatialLiesOnConstraintSpec, SpatialMoveSpec, SpatialOffsetSpec,
        SpatialPointsTowardConstraintSpec, SpatialReorientSpec, SpatialRotateSpec,
        SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
    },
    refs,
    refs::EmptySpatialWitnessCatalog,
};

#[test]
fn spatial_public_api_exports_shared_motion_and_anchor_surface() {
    let moved = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::Move(
            SpatialMoveSpec::shape_origin()
                .from(refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::W))
                .to([10.0, 0.0, 3.0]),
        ),
        &EmptySpatialWitnessCatalog,
    );
    let rotated = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::Rotate(
            SpatialRotateSpec::shape_origin()
                .about(refs::SpatialAnchorRef::shape_origin())
                .around([0.0, 1.0, 1.0])
                .by_radians(0.5),
        ),
        &EmptySpatialWitnessCatalog,
    );
    let reoriented = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::Reorient(
            SpatialReorientSpec::shape_origin()
                .about(refs::SpatialAnchorRef::shape_origin())
                .toward([1.0, 0.0, 1.0]),
        ),
        &EmptySpatialWitnessCatalog,
    );
    let offset = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::Offset(
            SpatialOffsetSpec::shape_origin()
                .from(refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::U))
                .offset_by([0.0, 0.0, 2.0]),
        ),
        &EmptySpatialWitnessCatalog,
    );
    let parallel = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::Reorient(
            SpatialReorientSpec::shape_origin().parallel_to(refs::SpatialFrameRef::workplane(
                "wp-2",
                [0.0, 0.0, 5.0],
                [0.0, 0.0, 1.0],
            )),
        ),
        &EmptySpatialWitnessCatalog,
    );
    let perpendicular =
        SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
            AuthorSpatialAnchorSelectionIntent::Reorient(
                SpatialReorientSpec::shape_origin().perpendicular_to(
                    refs::SpatialFrameRef::workplane("wp-3", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
                ),
            ),
            &EmptySpatialWitnessCatalog,
        );

    assert_eq!(
        moved.anchor(),
        &refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::W)
    );
    assert_eq!(moved.status(), SpatialAnchorSelectionStatus::Admitted);
    assert_eq!(
        rotated.resolution_class(),
        Some(SpatialWitnessResolutionClass::DirectWorld)
    );
    assert!(matches!(
        rotated.resolved_witness(),
        Some(worth_spatial::facade::anchor_selection::SpatialResolvedAnchorWitness::Direction(direction))
            if direction[1] > 0.70 && direction[2] > 0.70
    ));
    assert_eq!(
        reoriented.requested_input(),
        &worth_spatial::facade::anchor_selection::SpatialAnchorSelectionRequestedInput::DirectionWitness(
            refs::SpatialDirectionWitnessRef::world_direction([1.0, 0.0, 1.0])
        )
    );
    assert!(matches!(
        reoriented.resolved_witness(),
        Some(worth_spatial::facade::anchor_selection::SpatialResolvedAnchorWitness::Direction(direction))
            if direction[0] > 0.70 && direction[2] > 0.70
    ));
    assert_eq!(
        offset.anchor(),
        &refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::U)
    );
    assert_eq!(offset.status(), SpatialAnchorSelectionStatus::Admitted);
    assert_eq!(
        parallel.resolved_witness(),
        Some(
            worth_spatial::facade::anchor_selection::SpatialResolvedAnchorWitness::Direction([
                0.0, 0.0, 1.0
            ])
        )
    );
    assert_eq!(
        perpendicular.resolution_class(),
        Some(SpatialWitnessResolutionClass::FallbackDerived)
    );
    assert!(matches!(
        perpendicular.resolved_witness(),
        Some(worth_spatial::facade::anchor_selection::SpatialResolvedAnchorWitness::Direction(direction))
            if direction[2].abs() < 1.0e-12
    ));
}

#[test]
fn spatial_public_api_rejects_ambiguous_and_unsupported_direction_witnesses() {
    let ambiguous = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::Reorient(
            SpatialReorientSpec::shape_origin()
                .toward_witness(refs::SpatialDirectionWitnessRef::ambiguous_curve("curve-1")),
        ),
        &EmptySpatialWitnessCatalog,
    );
    let unsupported = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::Reorient(
            SpatialReorientSpec::shape_origin().toward_witness(
                refs::SpatialDirectionWitnessRef::surface_normal("surface-1", 0.5, 0.5),
            ),
        ),
        &EmptySpatialWitnessCatalog,
    );

    assert_eq!(
        ambiguous.failure_kind(),
        Some(SpatialAnchorSelectionFailureKind::Witness(
            SpatialWitnessFailureClass::Ambiguous,
        ))
    );
    assert_eq!(
        unsupported.failure_kind(),
        Some(SpatialAnchorSelectionFailureKind::Witness(
            SpatialWitnessFailureClass::Unsupported,
        ))
    );
}

#[test]
fn spatial_public_api_exports_frame_and_constraint_surface() {
    let workplane = refs::SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]);
    let lies_on = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::LiesOnConstraint(SpatialLiesOnConstraintSpec::new(
            refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::W),
            workplane.clone(),
        )),
        &EmptySpatialWitnessCatalog,
    );
    let points = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::PointsToward(SpatialPointsTowardConstraintSpec::new(
            refs::SpatialAnchorRef::frame_origin(workplane.clone()),
            [1.0, 0.0, 2.0],
        )),
        &EmptySpatialWitnessCatalog,
    );
    let matched = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::AnchorMatchConstraint(
            SpatialAnchorMatchConstraintSpec::new(
                refs::SpatialAnchorRef::shape_origin(),
                refs::SpatialAnchorRef::frame_origin(workplane.clone()),
            ),
        ),
        &EmptySpatialWitnessCatalog,
    );

    assert_eq!(
        lies_on.anchor(),
        &refs::SpatialAnchorRef::shape_axis(refs::SpatialAxis::W)
    );
    assert_eq!(
        lies_on.requested_input(),
        &SpatialAnchorSelectionRequestedInput::Frame(workplane.clone())
    );
    assert_eq!(
        points.resolved_witness(),
        Some(
            worth_spatial::facade::anchor_selection::SpatialResolvedAnchorWitness::Point([
                1.0, 0.0, 2.0
            ])
        )
    );
    assert_eq!(matched.anchor(), &refs::SpatialAnchorRef::shape_origin());
}
