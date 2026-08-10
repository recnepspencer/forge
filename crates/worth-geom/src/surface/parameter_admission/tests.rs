use super::{ParameterDomain, ParameterDomainError, PolygonalTrimmedParameterRegion};
use crate::primitives::parameter_space::ParameterSpacePoint;

#[test]
fn canonicalize_wraps_periodic_coordinates() {
    let domain = ParameterDomain::cylinder();
    let point = ParameterSpacePoint::try_new([std::f64::consts::TAU + 0.25, 3.0]).unwrap();
    let canonical = domain.canonicalize(point).unwrap();
    assert_eq!(canonical.point().as_array(), [0.25, 3.0]);
}

#[test]
fn admit_rejects_out_of_domain_coordinates() {
    let domain = ParameterDomain::sphere();
    let point = ParameterSpacePoint::try_new([1.0, std::f64::consts::PI]).unwrap();
    assert!(matches!(
        domain.admit(point),
        Err(ParameterDomainError::OutsideDomain { .. })
    ));
}

#[test]
fn polygonal_trim_region_admits_interior_points() {
    let domain = ParameterDomain::plane();
    let region = PolygonalTrimmedParameterRegion::new(
        domain.clone(),
        vec![
            ParameterSpacePoint::try_new([0.0, 0.0]).unwrap(),
            ParameterSpacePoint::try_new([2.0, 0.0]).unwrap(),
            ParameterSpacePoint::try_new([2.0, 2.0]).unwrap(),
            ParameterSpacePoint::try_new([0.0, 2.0]).unwrap(),
        ],
        vec![],
    )
    .unwrap();
    let point = domain
        .canonicalize(ParameterSpacePoint::try_new([1.0, 1.0]).unwrap())
        .unwrap();
    assert!(region.admit(point).is_ok());
}

#[test]
fn polygonal_trim_region_accepts_outer_boundary_points() {
    let domain = ParameterDomain::plane();
    let region = PolygonalTrimmedParameterRegion::new(
        domain.clone(),
        vec![
            ParameterSpacePoint::try_new([0.0, 0.0]).unwrap(),
            ParameterSpacePoint::try_new([2.0, 0.0]).unwrap(),
            ParameterSpacePoint::try_new([2.0, 2.0]).unwrap(),
            ParameterSpacePoint::try_new([0.0, 2.0]).unwrap(),
        ],
        vec![],
    )
    .unwrap();
    let point = domain
        .canonicalize(ParameterSpacePoint::try_new([1.0, 0.0]).unwrap())
        .unwrap();
    assert!(region.admit(point).is_ok());
}
