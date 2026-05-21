use super::{
    prepare_primitive_construction_move_witness_resolution_report,
    prepare_primitive_construction_move_witness_resolution_report_with_catalog,
    prepare_primitive_construction_points_toward_witness_resolution_report,
    prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog,
    prepare_primitive_construction_reorient_witness_resolution_report,
    prepare_primitive_construction_reorient_witness_resolution_report_with_catalog,
    prepare_primitive_construction_rotate_witness_resolution_report,
    PrimitiveConstructionMotionWitnessResolutionFailureKind,
    PrimitiveConstructionMotionWitnessResolutionKind,
    PrimitiveConstructionMotionWitnessResolutionStatus,
    PrimitiveConstructionRequestedMotionWitness, PrimitiveConstructionResolvedMotionWitness,
};
use crate::construction::{PrimitiveConstructionIntent, RegularPyramidSpec, WireBodySpec};
use crate::facade::{MoveSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent};
use std::collections::BTreeSet;
use worth_spatial::facade::{
    SpatialAnchorRef, SpatialAxis, SpatialCarrierDirectionRole, SpatialCarrierKind,
    SpatialCarrierPointRole, SpatialCatalogResolvedDirectionWitness,
    SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
    SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog, SpatialPointWitnessRef,
    SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

#[test]
fn motion_witness_resolution_reports_preserve_requested_and_resolved_witness_truth() {
    let move_report = prepare_primitive_construction_move_witness_resolution_report(
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .from(SpatialAnchorRef::shape_axis(SpatialAxis::W))
        .to([10.0, 0.0, 3.0]),
    );
    let rotate_report = prepare_primitive_construction_rotate_witness_resolution_report(
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .about(SpatialAnchorRef::shape_origin())
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
        .parallel_to(worth_spatial::facade::SpatialFrameRef::workplane(
            "wp-1",
            [0.0, 0.0, 5.0],
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
        .points_toward([1.0, 2.0, 3.0]),
    );

    assert_eq!(
        move_report.kind(),
        PrimitiveConstructionMotionWitnessResolutionKind::Move
    );
    assert_eq!(
        move_report.requested_witness(),
        &PrimitiveConstructionRequestedMotionWitness::Point(SpatialPointWitnessRef::world_point([
            10.0, 0.0, 3.0
        ]))
    );
    assert_eq!(
        move_report.status(),
        PrimitiveConstructionMotionWitnessResolutionStatus::Admitted
    );
    assert_eq!(move_report.resolved_target_point(), Some([10.0, 0.0, 3.0]));
    assert_eq!(
        move_report.resolved_witness(),
        Some(PrimitiveConstructionResolvedMotionWitness::Point([
            10.0, 0.0, 3.0
        ]))
    );
    assert_eq!(
        move_report.resolution_class(),
        Some(SpatialWitnessResolutionClass::DirectWorld)
    );
    assert_eq!(
        rotate_report.kind(),
        PrimitiveConstructionMotionWitnessResolutionKind::Rotate
    );
    assert_eq!(
        rotate_report.requested_witness(),
        &PrimitiveConstructionRequestedMotionWitness::Direction(
            SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 1.0])
        )
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
    assert_eq!(
        points_report.resolution_class(),
        Some(SpatialWitnessResolutionClass::DirectWorld)
    );
    assert_eq!(
        [
            move_report.report_digest(),
            rotate_report.report_digest(),
            reorient_report.report_digest(),
            points_report.report_digest(),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
        .len(),
        4
    );
}

#[test]
fn motion_witness_resolution_reports_preserve_witness_and_input_failure_truth() {
    let move_report = prepare_primitive_construction_move_witness_resolution_report(
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 4,
        }))
        .to_witness(SpatialPointWitnessRef::ambiguous_curve_point("curve-1")),
    );
    let rotate_report = prepare_primitive_construction_rotate_witness_resolution_report(
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 4,
        }))
        .around_witness(SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]))
        .by_radians(f64::NAN),
    );
    let points_report = prepare_primitive_construction_points_toward_witness_resolution_report(
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 1.0,
            },
        ))
        .so(SpatialAnchorRef::shape_origin())
        .points_toward_witness(SpatialPointWitnessRef::ambiguous_surface_point("surface-1")),
    );

    assert_eq!(
        move_report.status(),
        PrimitiveConstructionMotionWitnessResolutionStatus::Rejected
    );
    assert_eq!(
        move_report.failure_kind(),
        Some(
            PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(
                SpatialWitnessFailureClass::Ambiguous
            )
        )
    );
    assert_eq!(
        rotate_report.failure_kind(),
        Some(PrimitiveConstructionMotionWitnessResolutionFailureKind::NonFiniteRotationAngle)
    );
    assert_eq!(
        points_report.failure_kind(),
        Some(
            PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(
                SpatialWitnessFailureClass::Ambiguous
            )
        )
    );
}

#[test]
fn motion_witness_resolution_reports_support_catalog_backed_carrier_and_feature_truth() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_parameter_space_direction(
            SpatialCarrierKind::Curve,
            "curve-2",
            [0.25, 0.0],
            SpatialCarrierDirectionRole::Tangent,
            Ok(SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 1.0, 0.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        )
        .with_feature_owned_direction(
            "feature-rotate",
            SpatialCarrierDirectionRole::Axis,
            Err(SpatialWitnessFailureClass::Exhausted),
        )
        .with_feature_owned_point(
            "feature-1",
            SpatialCarrierPointRole::Origin,
            Ok(SpatialCatalogResolvedPointWitness::new(
                [4.0, 5.0, 6.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        );
    let reorient_report =
        prepare_primitive_construction_reorient_witness_resolution_report_with_catalog(
            ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                RegularPyramidSpec {
                    sides: 4,
                    radius: 1.0,
                    height: 2.0,
                },
            ))
            .toward_witness(SpatialDirectionWitnessRef::curve_tangent("curve-2", 0.25)),
            &catalog,
        );
    let points_report =
        prepare_primitive_construction_points_toward_witness_resolution_report_with_catalog(
            ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
                RegularPyramidSpec {
                    sides: 4,
                    radius: 1.0,
                    height: 2.0,
                },
            ))
            .so(SpatialAnchorRef::shape_origin())
            .points_toward_witness(SpatialPointWitnessRef::feature_origin("feature-1")),
            &catalog,
        );
    let rotate_report = prepare_primitive_construction_move_witness_resolution_report_with_catalog(
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 4,
        }))
        .to_witness(SpatialPointWitnessRef::world_point([0.0, 0.0, 0.0])),
        &catalog,
    );
    let exhausted_rotate =
        super::prepare_primitive_construction_rotate_witness_resolution_report_with_catalog(
            RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                edge_count: 4,
            }))
            .around_witness(SpatialDirectionWitnessRef::feature_axis("feature-rotate"))
            .by_radians(0.5),
            &catalog,
        );

    assert_eq!(
        reorient_report.resolution_class(),
        Some(SpatialWitnessResolutionClass::CarrierDerived)
    );
    assert_eq!(
        points_report.resolution_class(),
        Some(SpatialWitnessResolutionClass::FallbackDerived)
    );
    assert_eq!(points_report.resolved_target_point(), Some([4.0, 5.0, 6.0]));
    assert_eq!(
        exhausted_rotate.failure_kind(),
        Some(
            PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(
                SpatialWitnessFailureClass::Exhausted
            )
        )
    );
    assert_eq!(
        rotate_report.resolution_class(),
        Some(SpatialWitnessResolutionClass::DirectWorld)
    );
}
