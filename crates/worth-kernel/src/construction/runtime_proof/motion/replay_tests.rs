use super::{
    prepare_primitive_construction_move_replay_parity_report,
    prepare_primitive_construction_points_toward_replay_parity_report_with_catalog,
    prepare_primitive_construction_reorient_replay_parity_report_with_catalog,
    prepare_primitive_construction_rotate_replay_parity_report,
};
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::specs::{RegularPyramidSpec, WireBodySpec};
use crate::facade::authoring::intents::{
    MoveSpatialIntent, ReorientSpatialIntent, RotateSpatialIntent,
};
use crate::test_support::SpatialFixtureWitnessCatalog;
use worth_geom::ParameterSpacePoint;
use worth_spatial::facade::refs::{
    SpatialAnchorRef, SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
    SpatialDirectionWitnessRef, SpatialPointWitnessRef,
};
use worth_spatial::facade::witness_catalog::{
    SpatialCatalogResolvedDirectionWitness, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass,
};
use worth_spatial::facade::witness_resolution::SpatialWitnessResolutionClass;

#[test]
fn motion_replay_parity_reports_preserve_admitted_and_rejected_truth() {
    let admitted = prepare_primitive_construction_move_replay_parity_report(
        MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .to([10.0, 0.0, 3.0]),
    );
    let rejected = prepare_primitive_construction_rotate_replay_parity_report(
        RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
            edge_count: 6,
        }))
        .around([0.0, 0.0, 1.0])
        .by_radians(f64::NAN),
    );

    assert!(admitted.parity_verified());
    assert_eq!(admitted.direct_report(), admitted.replay_report());
    assert!(rejected.parity_verified());
    assert_eq!(rejected.direct_report(), rejected.replay_report());
}

#[test]
fn motion_replay_parity_reports_cover_catalog_backed_direction_and_point_witnesses() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_parameter_space_direction(
            SpatialCarrierKind::Curve,
            "curve-replay",
            ParameterSpacePoint::try_new([0.25, 0.0]).unwrap(),
            SpatialCarrierDirectionRole::Tangent,
            Ok(SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 2.0, 0.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        )
        .with_feature_owned_point(
            "feature-replay",
            SpatialCarrierPointRole::Origin,
            Ok(SpatialCatalogResolvedPointWitness::new(
                [4.0, 5.0, 6.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        );
    let reorient = prepare_primitive_construction_reorient_replay_parity_report_with_catalog(
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 4,
                radius: 1.0,
                height: 2.0,
            },
        ))
        .toward_witness(SpatialDirectionWitnessRef::curve_tangent(
            "curve-replay",
            0.25,
        )),
        &catalog,
    );
    let points = prepare_primitive_construction_points_toward_replay_parity_report_with_catalog(
        ReorientSpatialIntent::shape(PrimitiveConstructionIntent::regular_pyramid(
            RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 1.0,
            },
        ))
        .so(SpatialAnchorRef::shape_origin())
        .points_toward_witness(SpatialPointWitnessRef::feature_origin("feature-replay")),
        &catalog,
    );

    assert!(reorient.parity_verified());
    assert_eq!(reorient.direct_report(), reorient.replay_report());
    assert_eq!(
        reorient.direct_report().resolution_class(),
        Some(SpatialWitnessResolutionClass::CarrierDerived)
    );
    assert!(points.parity_verified());
    assert_eq!(points.direct_report(), points.replay_report());
    assert_eq!(
        points.direct_report().resolution_class(),
        Some(SpatialWitnessResolutionClass::FallbackDerived)
    );
}
