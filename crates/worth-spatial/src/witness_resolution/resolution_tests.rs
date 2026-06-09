use crate::facade::{
    refs::{
        SpatialAxis, SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
        SpatialDirectionWitnessRef, SpatialFrameRef, SpatialPointWitnessRef,
    },
    refs::{
        SpatialCatalogParameterAdmission, SpatialCatalogResolvedDirectionWitness,
        SpatialCatalogResolvedPointWitness, SpatialCatalogTrimmedAdmissionPosture,
        SpatialCatalogWitnessResolutionClass,
    },
};
use crate::test_support::SpatialFixtureWitnessCatalog;
use crate::witness_resolution::witness_resolution::{
    resolve_spatial_direction_witness, resolve_spatial_direction_witness_with_catalog,
    resolve_spatial_point_witness, resolve_spatial_point_witness_with_catalog,
};
use crate::witness_resolution::{SpatialWitnessFailureClass, SpatialWitnessResolutionClass};
use worth_geom::{ParameterDomain, ParameterSpacePoint, PolygonalTrimmedParameterRegion};

#[test]
fn point_witness_resolution_preserves_direct_and_frame_truth() {
    let direct =
        resolve_spatial_point_witness(SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]))
            .expect("direct");
    let frame = resolve_spatial_point_witness(SpatialPointWitnessRef::frame_origin(
        SpatialFrameRef::workplane("wp-1", [4.0, 5.0, 6.0], [0.0, 0.0, 1.0]),
    ))
    .expect("frame");

    assert_eq!(
        direct.resolution_class(),
        SpatialWitnessResolutionClass::DirectWorld
    );
    assert_eq!(
        frame.resolution_class(),
        SpatialWitnessResolutionClass::FrameDerived
    );
    assert_eq!(direct.resolved_world_point(), [1.0, 2.0, 3.0]);
    assert_eq!(frame.resolved_world_point(), [4.0, 5.0, 6.0]);
}

#[test]
fn point_witness_resolution_rejects_ambiguous_nonfinite_and_unsupported_refs() {
    assert_eq!(
        resolve_spatial_point_witness(SpatialPointWitnessRef::ambiguous_curve_point("curve-1"))
            .expect_err("ambiguous"),
        SpatialWitnessFailureClass::Ambiguous
    );
    assert_eq!(
        resolve_spatial_point_witness(SpatialPointWitnessRef::world_point([f64::NAN, 0.0, 0.0]))
            .expect_err("non-finite"),
        SpatialWitnessFailureClass::NonFinite
    );
    assert_eq!(
        resolve_spatial_point_witness(SpatialPointWitnessRef::curve_point("curve-2", 0.25))
            .expect_err("unsupported"),
        SpatialWitnessFailureClass::Unsupported
    );
}

#[test]
fn point_witness_resolution_supports_catalog_backed_carrier_and_feature_truth() {
    let requested = ParameterSpacePoint::try_new([0.5, 0.25]).unwrap();
    let admission = SpatialCatalogParameterAdmission::new(
        requested,
        ParameterDomain::plane().admit(requested).unwrap(),
        ParameterDomain::plane().canonicalize(requested).unwrap(),
    );
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_parameter_space_point(
            SpatialCarrierKind::Surface,
            "surface-2",
            requested,
            Ok(
                SpatialCatalogResolvedPointWitness::with_parameter_admission(
                    [8.0, 9.0, 10.0],
                    SpatialCatalogWitnessResolutionClass::CarrierDerived,
                    admission.clone(),
                ),
            ),
        )
        .with_feature_owned_point(
            "feature-2",
            SpatialCarrierPointRole::Origin,
            Ok(SpatialCatalogResolvedPointWitness::new(
                [1.0, 1.5, 2.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        );

    let surface = resolve_spatial_point_witness_with_catalog(
        SpatialPointWitnessRef::surface_point("surface-2", 0.5, 0.25),
        &catalog,
    )
    .expect("surface point");
    let feature = resolve_spatial_point_witness_with_catalog(
        SpatialPointWitnessRef::feature_origin("feature-2"),
        &catalog,
    )
    .expect("feature point");

    assert_eq!(
        surface.resolution_class(),
        SpatialWitnessResolutionClass::CarrierDerived
    );
    assert_eq!(surface.resolved_world_point(), [8.0, 9.0, 10.0]);
    assert_eq!(surface.parameter_admission(), Some(&admission));
    assert_eq!(
        feature.resolution_class(),
        SpatialWitnessResolutionClass::FallbackDerived
    );
    assert_eq!(feature.resolved_world_point(), [1.0, 1.5, 2.0]);
    assert_eq!(feature.parameter_admission(), None);
}

#[test]
fn point_witness_resolution_preserves_trimmed_parameter_posture() {
    let requested = ParameterSpacePoint::try_new([0.5, 0.25]).unwrap();
    let domain = ParameterDomain::plane();
    let trimmed_region = PolygonalTrimmedParameterRegion::new(
        domain.clone(),
        vec![
            ParameterSpacePoint::try_new([0.0, 0.0]).unwrap(),
            ParameterSpacePoint::try_new([1.0, 0.0]).unwrap(),
            ParameterSpacePoint::try_new([1.0, 1.0]).unwrap(),
            ParameterSpacePoint::try_new([0.0, 1.0]).unwrap(),
        ],
        vec![],
    )
    .unwrap();
    let admission = SpatialCatalogParameterAdmission::new(
        requested,
        domain.admit(requested).unwrap(),
        domain.canonicalize(requested).unwrap(),
    )
    .with_trimmed_posture(SpatialCatalogTrimmedAdmissionPosture::PolygonalRegion);
    let catalog = SpatialFixtureWitnessCatalog::new().with_parameter_space_point(
        SpatialCarrierKind::Surface,
        "face-1",
        requested,
        Ok(
            SpatialCatalogResolvedPointWitness::with_parameter_admission(
                [3.0, 4.0, 5.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
                admission.clone(),
            ),
        ),
    );

    let resolved = resolve_spatial_point_witness_with_catalog(
        SpatialPointWitnessRef::surface_parameter_point("face-1", requested),
        &catalog,
    )
    .expect("resolved point witness");

    let parameter_admission = resolved.parameter_admission().expect("parameter admission");
    assert_eq!(
        parameter_admission.trimmed_posture(),
        Some(SpatialCatalogTrimmedAdmissionPosture::PolygonalRegion)
    );
    assert!(trimmed_region
        .admit(parameter_admission.canonical_point().clone())
        .is_ok());
}

#[test]
fn direction_witness_resolution_preserves_direct_frame_and_fallback_truth() {
    let world = resolve_spatial_direction_witness(SpatialDirectionWitnessRef::world_direction([
        0.0, 1.0, 1.0,
    ]))
    .expect("world");
    let frame = resolve_spatial_direction_witness(SpatialDirectionWitnessRef::frame_axis(
        SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [1.0, 0.0, 0.0]),
        SpatialAxis::W,
    ))
    .expect("frame");
    let perpendicular =
        resolve_spatial_direction_witness(SpatialDirectionWitnessRef::frame_perpendicular_axis(
            SpatialFrameRef::workplane("wp-2", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
            SpatialAxis::W,
        ))
        .expect("perpendicular");

    assert_eq!(
        world.resolution_class(),
        SpatialWitnessResolutionClass::DirectWorld
    );
    assert_eq!(
        frame.resolution_class(),
        SpatialWitnessResolutionClass::FrameDerived
    );
    assert_eq!(
        perpendicular.resolution_class(),
        SpatialWitnessResolutionClass::FallbackDerived
    );
    assert_eq!(frame.resolved_world_direction(), [1.0, 0.0, 0.0]);
    assert!(perpendicular.resolved_world_direction()[2].abs() < 1.0e-12);
}

#[test]
fn direction_witness_resolution_distinguishes_ambiguous_undefined_and_unsupported() {
    assert_eq!(
        resolve_spatial_direction_witness(SpatialDirectionWitnessRef::ambiguous_curve("curve-1"))
            .expect_err("ambiguous"),
        SpatialWitnessFailureClass::Ambiguous
    );
    assert_eq!(
        resolve_spatial_direction_witness(SpatialDirectionWitnessRef::world_direction([
            0.0, 0.0, 0.0,
        ]))
        .expect_err("undefined"),
        SpatialWitnessFailureClass::Undefined
    );
    assert_eq!(
        resolve_spatial_direction_witness(SpatialDirectionWitnessRef::surface_normal(
            "surface-1",
            0.5,
            0.5,
        ))
        .expect_err("unsupported"),
        SpatialWitnessFailureClass::Unsupported
    );
}

#[test]
fn direction_witness_resolution_supports_catalog_backed_carrier_and_feature_truth() {
    let catalog = SpatialFixtureWitnessCatalog::new()
        .with_parameter_space_direction(
            SpatialCarrierKind::Curve,
            "curve-2",
            ParameterSpacePoint::try_new([0.25, 0.0]).unwrap(),
            SpatialCarrierDirectionRole::Tangent,
            Ok(SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 1.0, 0.0],
                SpatialCatalogWitnessResolutionClass::CarrierDerived,
            )),
        )
        .with_feature_owned_direction(
            "feature-1",
            SpatialCarrierDirectionRole::Axis,
            Ok(SpatialCatalogResolvedDirectionWitness::new(
                [0.0, 0.0, 4.0],
                SpatialCatalogWitnessResolutionClass::FallbackDerived,
            )),
        );

    let curve = resolve_spatial_direction_witness_with_catalog(
        SpatialDirectionWitnessRef::curve_tangent("curve-2", 0.25),
        &catalog,
    )
    .expect("curve tangent");
    let feature = resolve_spatial_direction_witness_with_catalog(
        SpatialDirectionWitnessRef::feature_axis("feature-1"),
        &catalog,
    )
    .expect("feature axis");

    assert_eq!(
        curve.resolution_class(),
        SpatialWitnessResolutionClass::CarrierDerived
    );
    assert_eq!(curve.resolved_world_direction(), [0.0, 1.0, 0.0]);
    assert_eq!(
        feature.resolution_class(),
        SpatialWitnessResolutionClass::FallbackDerived
    );
    assert_eq!(feature.resolved_world_direction(), [0.0, 0.0, 1.0]);
}
