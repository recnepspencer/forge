use forge_query::facade::ForgeQueryWorkspace;
use worth_geom::ParameterSpacePoint;

use crate::construction::certification::motion::policy_report::PrimitiveConstructionMotionResolutionPolicyCase;
use crate::construction::certification::motion::representative_evidence::{
    PrimitiveConstructionMotionRepresentativeEvidenceError,
    PrimitiveConstructionMotionRepresentativeInputs,
};
use crate::construction::certification::motion::witness_report::{
    prepare_primitive_construction_move_witness_resolution_report,
    prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog,
    prepare_primitive_construction_reorient_witness_resolution_report,
    prepare_primitive_construction_reorient_witness_resolution_report_with_catalog,
    prepare_primitive_construction_rotate_witness_resolution_report_with_catalog,
};
use crate::construction::query::{
    prepare_primitive_construction_query_motion_inspection_parity_report,
    prepare_primitive_construction_query_motion_projection_consumption_receipt_report,
};
use crate::construction::{
    prepare_primitive_construction_move_branch_preview_runtime_report,
    prepare_primitive_construction_move_replay_parity_report,
    prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_points_toward_replay_parity_report_with_catalog,
    prepare_primitive_construction_reorient_branch_preview_runtime_report,
    prepare_primitive_construction_reorient_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_reorient_replay_parity_report,
    prepare_primitive_construction_reorient_replay_parity_report_with_catalog,
    prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog,
    prepare_primitive_construction_rotate_replay_parity_report_with_catalog,
    PrimitiveConstructionIntent, RegularPyramidSpec, WireBodySpec,
};
use crate::spatial_intent::{MoveSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent};
use crate::test_support::SpatialFixtureWitnessCatalog;
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
    SpatialDirectionWitnessRef, SpatialFrameRef, SpatialPointWitnessRef,
};
use worth_spatial::facade::witness_catalog::{
    SpatialCatalogResolvedDirectionWitness, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass,
};
use worth_spatial::facade::witness_resolution::SpatialWitnessFailureClass;

pub(crate) fn required_motion_representative_cases(
) -> &'static [PrimitiveConstructionMotionResolutionPolicyCase] {
    &[
        PrimitiveConstructionMotionResolutionPolicyCase::DirectMove,
        PrimitiveConstructionMotionResolutionPolicyCase::FrameReorient,
        PrimitiveConstructionMotionResolutionPolicyCase::CarrierReorient,
        PrimitiveConstructionMotionResolutionPolicyCase::FallbackPointsToward,
        PrimitiveConstructionMotionResolutionPolicyCase::AmbiguousMove,
        PrimitiveConstructionMotionResolutionPolicyCase::UndefinedReorient,
        PrimitiveConstructionMotionResolutionPolicyCase::UnsupportedReorient,
        PrimitiveConstructionMotionResolutionPolicyCase::ExhaustedRotate,
        PrimitiveConstructionMotionResolutionPolicyCase::CoincidentPointsToward,
    ]
}

pub(crate) fn prepare_motion_representative_inputs(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionMotionResolutionPolicyCase,
) -> Result<
    PrimitiveConstructionMotionRepresentativeInputs,
    PrimitiveConstructionMotionRepresentativeEvidenceError,
> {
    let (witness_report, replay_report, branch_runtime_report) = match case {
        PrimitiveConstructionMotionResolutionPolicyCase::DirectMove => {
            let intent =
                MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 6,
                }))
                .to([10.0, 0.0, 3.0]);
            (
                prepare_primitive_construction_move_witness_resolution_report(intent.clone()),
                prepare_primitive_construction_move_replay_parity_report(intent.clone()),
                prepare_primitive_construction_move_branch_preview_runtime_report(
                    workspace, intent,
                )
                .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Runtime)?,
            )
        }
        PrimitiveConstructionMotionResolutionPolicyCase::FrameReorient => {
            let intent = ReorientSpatialIntent::shape(
                PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                    sides: 4,
                    radius: 1.0,
                    height: 2.0,
                }),
            )
            .parallel_to(SpatialFrameRef::workplane(
                "policy-workplane",
                [0.0, 0.0, 5.0],
                [0.0, 0.0, 1.0],
            ));
            (
                prepare_primitive_construction_reorient_witness_resolution_report(intent.clone()),
                prepare_primitive_construction_reorient_replay_parity_report(intent.clone()),
                prepare_primitive_construction_reorient_branch_preview_runtime_report(
                    workspace, intent,
                )
                .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Runtime)?,
            )
        }
        PrimitiveConstructionMotionResolutionPolicyCase::CarrierReorient => {
            let intent = ReorientSpatialIntent::shape(
                PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                    sides: 4,
                    radius: 1.0,
                    height: 2.0,
                }),
            )
            .toward_witness(SpatialDirectionWitnessRef::curve_tangent(
                "policy-curve",
                0.25,
            ));
            let catalog = SpatialFixtureWitnessCatalog::new().with_parameter_space_direction(
                SpatialCarrierKind::Curve,
                "policy-curve",
                ParameterSpacePoint::try_new([0.25, 0.0]).unwrap(),
                SpatialCarrierDirectionRole::Tangent,
                Ok(SpatialCatalogResolvedDirectionWitness::new(
                    [0.0, 1.0, 0.0],
                    SpatialCatalogWitnessResolutionClass::CarrierDerived,
                )),
            );
            (
                prepare_primitive_construction_reorient_witness_resolution_report_with_catalog(
                    intent.clone(),
                    &catalog,
                ),
                prepare_primitive_construction_reorient_replay_parity_report_with_catalog(
                    intent.clone(),
                    &catalog,
                ),
                prepare_primitive_construction_reorient_branch_preview_runtime_report_with_catalog(
                    workspace, intent, &catalog,
                )
                .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Runtime)?,
            )
        }
        PrimitiveConstructionMotionResolutionPolicyCase::FallbackPointsToward => {
            let intent = ReorientSpatialIntent::shape(
                PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                    sides: 3,
                    radius: 1.0,
                    height: 1.0,
                }),
            )
            .so(SpatialAnchorRef::shape_origin())
            .points_toward_witness(SpatialPointWitnessRef::feature_origin("policy-feature"));
            let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_point(
                "policy-feature",
                SpatialCarrierPointRole::Origin,
                Ok(SpatialCatalogResolvedPointWitness::new(
                    [4.0, 5.0, 6.0],
                    SpatialCatalogWitnessResolutionClass::FallbackDerived,
                )),
            );
            (
                prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog(
                    intent.clone(),
                    &catalog,
                ),
                prepare_primitive_construction_points_toward_replay_parity_report_with_catalog(
                    intent.clone(),
                    &catalog,
                ),
                prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog(
                    workspace, intent, &catalog,
                )
                .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Runtime)?,
            )
        }
        PrimitiveConstructionMotionResolutionPolicyCase::AmbiguousMove => {
            let intent =
                MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 4,
                }))
                .to_witness(SpatialPointWitnessRef::ambiguous_curve_point(
                    "policy-curve",
                ));
            (
                prepare_primitive_construction_move_witness_resolution_report(intent.clone()),
                prepare_primitive_construction_move_replay_parity_report(intent.clone()),
                prepare_primitive_construction_move_branch_preview_runtime_report(
                    workspace, intent,
                )
                .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Runtime)?,
            )
        }
        PrimitiveConstructionMotionResolutionPolicyCase::UndefinedReorient => {
            let intent = ReorientSpatialIntent::shape(
                PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                    sides: 4,
                    radius: 1.0,
                    height: 2.0,
                }),
            )
            .toward_witness(SpatialDirectionWitnessRef::feature_axis(
                "policy-undefined-feature",
            ));
            let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_direction(
                "policy-undefined-feature",
                SpatialCarrierDirectionRole::Axis,
                Err(SpatialWitnessFailureClass::Undefined),
            );
            (
                prepare_primitive_construction_reorient_witness_resolution_report_with_catalog(
                    intent.clone(),
                    &catalog,
                ),
                prepare_primitive_construction_reorient_replay_parity_report_with_catalog(
                    intent.clone(),
                    &catalog,
                ),
                prepare_primitive_construction_reorient_branch_preview_runtime_report_with_catalog(
                    workspace, intent, &catalog,
                )
                .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Runtime)?,
            )
        }
        PrimitiveConstructionMotionResolutionPolicyCase::UnsupportedReorient => {
            let intent = ReorientSpatialIntent::shape(
                PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                    sides: 4,
                    radius: 1.0,
                    height: 2.0,
                }),
            )
            .toward_witness(SpatialDirectionWitnessRef::surface_normal(
                "policy-surface",
                0.5,
                0.5,
            ));
            (
                prepare_primitive_construction_reorient_witness_resolution_report(intent.clone()),
                prepare_primitive_construction_reorient_replay_parity_report(intent.clone()),
                prepare_primitive_construction_reorient_branch_preview_runtime_report(
                    workspace, intent,
                )
                .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Runtime)?,
            )
        }
        PrimitiveConstructionMotionResolutionPolicyCase::ExhaustedRotate => {
            let intent =
                RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                    edge_count: 4,
                }))
                .around_witness(SpatialDirectionWitnessRef::feature_axis(
                    "policy-exhausted-feature",
                ))
                .by_radians(0.5);
            let catalog = SpatialFixtureWitnessCatalog::new().with_feature_owned_direction(
                "policy-exhausted-feature",
                SpatialCarrierDirectionRole::Axis,
                Err(SpatialWitnessFailureClass::Exhausted),
            );
            (
                prepare_primitive_construction_rotate_witness_resolution_report_with_catalog(
                    intent.clone(),
                    &catalog,
                ),
                prepare_primitive_construction_rotate_replay_parity_report_with_catalog(
                    intent.clone(),
                    &catalog,
                ),
                prepare_primitive_construction_rotate_branch_preview_runtime_report_with_catalog(
                    workspace, intent, &catalog,
                )
                .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Runtime)?,
            )
        }
        PrimitiveConstructionMotionResolutionPolicyCase::CoincidentPointsToward => {
            let intent = ReorientSpatialIntent::shape(
                PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                    sides: 3,
                    radius: 1.0,
                    height: 1.0,
                }),
            )
            .so(SpatialAnchorRef::shape_origin())
            .points_toward([0.0, 0.0, 0.0]);
            (
                prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog(
                    intent.clone(),
                    &worth_spatial::facade::witness_catalog::EmptySpatialWitnessCatalog,
                ),
                prepare_primitive_construction_points_toward_replay_parity_report_with_catalog(
                    intent.clone(),
                    &worth_spatial::facade::witness_catalog::EmptySpatialWitnessCatalog,
                ),
                prepare_primitive_construction_points_toward_branch_preview_runtime_report_with_catalog(
                    workspace,
                    intent,
                    &worth_spatial::facade::witness_catalog::EmptySpatialWitnessCatalog,
                )
                .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Runtime)?,
            )
        }
    };
    let inspection_report = prepare_primitive_construction_query_motion_inspection_parity_report(
        workspace,
        witness_report.clone(),
    )
    .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_motion_projection_consumption_receipt_report(
            workspace,
            witness_report.clone(),
        )
        .map_err(PrimitiveConstructionMotionRepresentativeEvidenceError::Projection)?;

    Ok(PrimitiveConstructionMotionRepresentativeInputs {
        witness_report,
        replay_report,
        inspection_report,
        projection_report,
        branch_runtime_report,
    })
}
