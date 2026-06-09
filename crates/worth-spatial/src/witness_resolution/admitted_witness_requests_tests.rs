use crate::authored_refs::SpatialCarrierKind;
use crate::facade::refs;
use crate::test_support::SpatialFixtureWitnessCatalog;
use crate::witness_resolution::{SpatialWitnessFailureClass, SpatialWitnessResolutionClass};
use worth_geom::{ParameterDomain, ParameterSpacePoint};

use super::super::resolution::{
    resolve_admitted_spatial_direction_witness_request,
    resolve_admitted_spatial_point_witness_request,
};
use super::{admit_spatial_direction_witness_request, admit_spatial_point_witness_request};

#[test]
fn point_request_admission_preserves_direct_world_success() {
    let requested = refs::SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]);
    let admitted = admit_spatial_point_witness_request(requested.clone()).expect("admitted point");
    let resolved = resolve_admitted_spatial_point_witness_request(
        admitted,
        &crate::facade::refs::EmptySpatialWitnessCatalog,
    )
    .expect("resolved point");

    assert_eq!(resolved.requested(), &requested);
    assert_eq!(resolved.resolved_world_point(), [1.0, 2.0, 3.0]);
    assert_eq!(
        resolved.resolution_class(),
        SpatialWitnessResolutionClass::DirectWorld
    );
}

#[test]
fn point_request_admission_allows_carrier_request_but_denies_at_resolution() {
    let requested = refs::SpatialPointWitnessRef::ambiguous_curve_point("curve-7");
    let admitted = admit_spatial_point_witness_request(requested.clone()).expect("admitted point");
    let denied = resolve_admitted_spatial_point_witness_request(
        admitted,
        &crate::facade::refs::EmptySpatialWitnessCatalog,
    )
    .expect_err("carrier point should deny without catalog interpretation");

    assert_eq!(
        requested,
        refs::SpatialPointWitnessRef::ambiguous_curve_point("curve-7")
    );
    assert_eq!(denied, SpatialWitnessFailureClass::Ambiguous);
}

#[test]
fn direction_request_admission_preserves_frame_fallback_path() {
    let requested = refs::SpatialDirectionWitnessRef::frame_perpendicular_axis(
        refs::SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        crate::facade::refs::SpatialAxis::W,
    );
    let admitted = admit_spatial_direction_witness_request(requested.clone()).expect("admitted");
    let resolved = resolve_admitted_spatial_direction_witness_request(
        admitted,
        &crate::facade::refs::EmptySpatialWitnessCatalog,
    )
    .expect("resolved direction");

    assert_eq!(resolved.requested(), &requested);
    assert_eq!(
        resolved.resolution_class(),
        SpatialWitnessResolutionClass::FallbackDerived
    );
    assert!(resolved.resolved_world_direction()[2].abs() < 1.0e-12);
}

#[test]
fn admitted_request_preserves_parameter_space_meaning_until_catalog_resolution() {
    let requested_parameter = ParameterSpacePoint::try_new([0.5, 0.25]).unwrap();
    let admission = crate::facade::refs::SpatialCatalogParameterAdmission::new(
        requested_parameter,
        ParameterDomain::plane().admit(requested_parameter).unwrap(),
        ParameterDomain::plane()
            .canonicalize(requested_parameter)
            .unwrap(),
    );
    let catalog = SpatialFixtureWitnessCatalog::new().with_parameter_space_point(
        SpatialCarrierKind::Surface,
        "surface-1",
        requested_parameter,
        Ok(
            refs::SpatialCatalogResolvedPointWitness::with_parameter_admission(
                [8.0, 9.0, 10.0],
                refs::SpatialCatalogWitnessResolutionClass::CarrierDerived,
                admission.clone(),
            ),
        ),
    );
    let requested =
        refs::SpatialPointWitnessRef::surface_parameter_point("surface-1", requested_parameter);
    let admitted = admit_spatial_point_witness_request(requested.clone()).expect("admitted point");
    let resolved =
        resolve_admitted_spatial_point_witness_request(admitted, &catalog).expect("resolved");

    assert_eq!(resolved.requested(), &requested);
    assert_eq!(resolved.resolved_world_point(), [8.0, 9.0, 10.0]);
    assert_eq!(resolved.parameter_admission(), Some(&admission));
}
