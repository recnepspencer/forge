use super::spatial_fixture_witness_catalog::SpatialFixtureWitnessCatalog;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::ParameterSpacePoint;
use worth_kernel::facade::authoring::{construction::*, intents::*};
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialAxis, SpatialCarrierDirectionRole, SpatialCarrierKind,
    SpatialCarrierPointRole, SpatialDirectionWitnessRef, SpatialFrameRef,
};
use worth_spatial::facade::witness_catalog::{
    SpatialCatalogResolvedDirectionWitness, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass, SpatialGeometricTagFailureClass,
};
use worth_spatial::facade::witness_resolution::SpatialWitnessFailureClass;

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
fn kernel_public_facade_lowers_catalog_backed_anchor_paths_through_query_entry() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_feature_owned_point(
            "feature-anchor",
            SpatialCarrierPointRole::Anchor,
            Ok(SpatialCatalogResolvedPointWitness::new(
                [4.0, 0.0, 0.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        )
        .with_geometric_tag_point(
            "tag-anchor",
            Ok(SpatialCatalogResolvedPointWitness::new(
                [4.0, 0.0, 0.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        )
        .with_feature_owned_direction(
            "feature-axis",
            SpatialCarrierDirectionRole::Axis,
            Ok(SpatialCatalogResolvedDirectionWitness::new(
                [1.0, 0.0, 0.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        );

    let feature_owned_move =
        with_authoring_session("worth-kernel.anchor.feature-move", |session| {
            session
                .author_with_catalog(
                    MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(
                        WireBodySpec { edge_count: 6 },
                    ))
                    .from(SpatialAnchorRef::feature_owned("feature-anchor"))
                    .to([10.0, 0.0, 3.0]),
                    &catalog,
                )
                .and_then(|entry| entry.prepare_result())
        });
    let feature_owned_offset =
        with_authoring_session("worth-kernel.anchor.feature-offset", |session| {
            session
                .author_with_catalog(
                    OffsetSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(
                        WireBodySpec { edge_count: 6 },
                    ))
                    .from(SpatialAnchorRef::feature_owned("feature-anchor"))
                    .by([2.0, -1.0, 0.5]),
                    &catalog,
                )
                .and_then(|entry| entry.prepare_result())
        });
    let feature_owned_match =
        with_authoring_session("worth-kernel.anchor.feature-match", |session| {
            session
                .author_with_catalog(
                    MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(
                        WireBodySpec { edge_count: 6 },
                    ))
                    .so(SpatialAnchorRef::feature_owned("feature-anchor"))
                    .matches(SpatialAnchorRef::world_origin()),
                    &catalog,
                )
                .map(|entry| entry.prepare_outcome())
        });
    let feature_owned_lies_on =
        with_authoring_session("worth-kernel.anchor.feature-lies-on", |session| {
            session
                .author_with_catalog(
                    MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(
                        WireBodySpec { edge_count: 6 },
                    ))
                    .so(SpatialAnchorRef::feature_owned("feature-anchor"))
                    .lies_on(SpatialFrameRef::workplane(
                        "wp-feature",
                        [0.0, 0.0, 5.0],
                        [0.0, 0.0, 1.0],
                    )),
                    &catalog,
                )
                .map(|entry| entry.prepare_outcome())
        });
    let geometric_tag_move = with_authoring_session("worth-kernel.anchor.tag-move", |session| {
        session
            .author_with_catalog(
                MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 6,
                }))
                .from(SpatialAnchorRef::geometric_tag("tag-anchor"))
                .to([10.0, 0.0, 3.0]),
                &catalog,
            )
            .and_then(|entry| entry.prepare_result())
    });
    let directional_reorient =
        with_authoring_session("worth-kernel.anchor.directional-reorient", |session| {
            session
                .author_with_catalog(
                    ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                        RegularPyramidSpec {
                            sides: 4,
                            radius: 2.0,
                            height: 5.0,
                        },
                    ))
                    .about(SpatialAnchorRef::feature_owned("feature-axis"))
                    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0])),
                    &catalog,
                )
                .and_then(|entry| entry.prepare_result())
        });

    assert!(feature_owned_move.is_ok());
    assert!(feature_owned_offset.is_ok());
    assert!(feature_owned_match.is_ok());
    assert!(feature_owned_lies_on.is_ok());
    assert!(geometric_tag_move.is_ok());
    assert!(directional_reorient.is_ok());
}

#[test]
fn kernel_public_facade_lowers_external_reference_anchor_paths_through_query_entry() {
    let move_result = with_authoring_session("worth-kernel.anchor.external-move", |session| {
        session
            .author(
                MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 6,
                }))
                .from(SpatialAnchorRef::world_origin())
                .to([10.0, 0.0, 3.0]),
            )
            .and_then(|entry| entry.prepare_result())
    });
    let offset_result = with_authoring_session("worth-kernel.anchor.external-offset", |session| {
        session
            .author(
                OffsetSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 6,
                }))
                .from(SpatialAnchorRef::frame_origin(SpatialFrameRef::workplane(
                    "wp-external",
                    [10.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0],
                )))
                .by([2.0, -1.0, 0.5]),
            )
            .and_then(|entry| entry.prepare_result())
    });
    let shape_u = with_authoring_session("worth-kernel.anchor.shape-u", |session| {
        session
            .author(
                ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 4,
                        radius: 2.0,
                        height: 5.0,
                    },
                ))
                .about(SpatialAnchorRef::shape_axis(SpatialAxis::U))
                .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0])),
            )
            .and_then(|entry| entry.prepare_result())
    });
    let frame_axis = with_authoring_session("worth-kernel.anchor.frame-axis", |session| {
        session
            .author(
                ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                    RegularPyramidSpec {
                        sides: 4,
                        radius: 2.0,
                        height: 5.0,
                    },
                ))
                .about(SpatialAnchorRef::frame_axis(
                    SpatialFrameRef::world(),
                    SpatialAxis::U,
                ))
                .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0])),
            )
            .and_then(|entry| entry.prepare_result())
    });

    assert!(move_result.is_ok());
    assert!(offset_result.is_ok());
    assert!(shape_u.is_ok());
    assert!(frame_axis.is_ok());
}

#[test]
fn kernel_public_facade_preserves_anchor_lowering_failure_truth_at_query_entry() {
    let geometric_tag_failure =
        with_authoring_session("worth-kernel.anchor.tag-failure", |session| {
            session
                .author_with_catalog(
                    MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(
                        WireBodySpec { edge_count: 6 },
                    ))
                    .so(SpatialAnchorRef::shape_origin())
                    .matches(SpatialAnchorRef::geometric_tag("tag-anchor")),
                    &SpatialFixtureWitnessCatalog::new().with_geometric_tag_point(
                        "tag-anchor",
                        Err(SpatialWitnessFailureClass::Ambiguous),
                    ),
                )
                .and_then(|entry| entry.prepare_result())
        });
    let lies_on_failure =
        with_authoring_session("worth-kernel.anchor.lies-on-failure", |session| {
            session
                .author_with_catalog(
                    MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(
                        WireBodySpec { edge_count: 6 },
                    ))
                    .so(SpatialAnchorRef::feature_owned("feature-anchor"))
                    .lies_on(SpatialFrameRef::workplane(
                        "wp-feature",
                        [0.0, 0.0, 5.0],
                        [0.0, 0.0, 1.0],
                    )),
                    &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
                        "feature-anchor",
                        SpatialCarrierPointRole::Anchor,
                        Err(SpatialWitnessFailureClass::Undefined),
                    ),
                )
                .and_then(|entry| entry.prepare_result())
        });
    let target_match_failure =
        with_authoring_session("worth-kernel.anchor.target-match-failure", |session| {
            session
                .author_with_catalog(
                    MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(
                        WireBodySpec { edge_count: 6 },
                    ))
                    .so(SpatialAnchorRef::shape_origin())
                    .matches(SpatialAnchorRef::feature_owned("target-anchor")),
                    &SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
                        "target-anchor",
                        SpatialCarrierPointRole::Anchor,
                        Err(SpatialWitnessFailureClass::Exhausted),
                    ),
                )
                .and_then(|entry| entry.prepare_result())
        });
    let parameter_space_failure =
        with_authoring_session("worth-kernel.anchor.parameter-space-failure", |session| {
            session
                .author_with_catalog(
                    RotateSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                        RegularPyramidSpec {
                            sides: 4,
                            radius: 2.0,
                            height: 5.0,
                        },
                    ))
                    .about(SpatialAnchorRef::parameter_space(
                        "surface-anchor",
                        "0.25,0.75",
                    ))
                    .around([0.0, 0.0, 1.0])
                    .by_radians(std::f64::consts::FRAC_PI_2),
                    &SpatialFixtureWitnessCatalog::new().with_parameter_space_point(
                        SpatialCarrierKind::Surface,
                        "surface-anchor",
                        ParameterSpacePoint::try_new([0.25, 0.75]).unwrap(),
                        Ok(SpatialCatalogResolvedPointWitness::new(
                            [4.0, 0.0, 0.0],
                            SpatialCatalogWitnessResolutionClass::CarrierDerived,
                        )),
                    ),
                )
                .and_then(|entry| entry.prepare_result())
        });
    let directional_ambiguity =
        with_authoring_session("worth-kernel.anchor.directional-ambiguity", |session| {
            session
                .author_with_catalog(
                    ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                        RegularPyramidSpec {
                            sides: 4,
                            radius: 2.0,
                            height: 5.0,
                        },
                    ))
                    .about(SpatialAnchorRef::feature_owned("feature-axis"))
                    .toward_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0])),
                    &SpatialFixtureWitnessCatalog::new()
                        .with_feature_owned_point(
                            "feature-axis",
                            SpatialCarrierPointRole::Anchor,
                            Ok(SpatialCatalogResolvedPointWitness::new(
                                [0.0, 0.0, 0.0],
                                SpatialCatalogWitnessResolutionClass::FallbackDerived,
                            )),
                        )
                        .with_feature_owned_direction(
                            "feature-axis",
                            SpatialCarrierDirectionRole::Axis,
                            Ok(SpatialCatalogResolvedDirectionWitness::new(
                                [1.0, 0.0, 0.0],
                                SpatialCatalogWitnessResolutionClass::CarrierDerived,
                            )),
                        ),
                )
                .and_then(|entry| entry.prepare_result())
        });

    assert!(matches!(
        geometric_tag_failure,
        Err(PrimitiveConstructionQueryEntryError::Lowering(
            PrimitiveConstructionSpatialIntentError::ConstraintLowering(
                worth_spatial::facade::placement::SpatialPlacementConstraintError::AnchorTagFailure(
                    SpatialGeometricTagFailureClass::Resolution(
                        SpatialWitnessFailureClass::Ambiguous
                    ),
                ),
            ),
        ))
    ));
    assert!(matches!(
        lies_on_failure,
        Err(PrimitiveConstructionQueryEntryError::Lowering(
            PrimitiveConstructionSpatialIntentError::ConstraintLowering(
                worth_spatial::facade::placement::SpatialPlacementConstraintError::AnchorWitnessFailure(
                    SpatialWitnessFailureClass::Undefined
                ),
            ),
        ))
    ));
    assert!(matches!(
        target_match_failure,
        Err(PrimitiveConstructionQueryEntryError::Lowering(
            PrimitiveConstructionSpatialIntentError::ConstraintLowering(
                worth_spatial::facade::placement::SpatialPlacementConstraintError::AnchorWitnessFailure(
                    SpatialWitnessFailureClass::Exhausted
                ),
            ),
        ))
    ));
    assert!(matches!(
        parameter_space_failure,
        Err(PrimitiveConstructionQueryEntryError::Lowering(
            PrimitiveConstructionSpatialIntentError::PlacementLowering(_)
        ))
    ));
    assert!(matches!(
        directional_ambiguity,
        Err(PrimitiveConstructionQueryEntryError::Lowering(
            PrimitiveConstructionSpatialIntentError::PlacementLowering(
                worth_spatial::facade::placement::SpatialPlacementMotionError::AmbiguousReorientAnchorMeaning
            )
        ))
    ));
}
