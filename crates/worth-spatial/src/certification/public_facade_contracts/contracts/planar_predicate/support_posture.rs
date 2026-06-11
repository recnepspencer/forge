use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn planar_predicate_authority_has_explicit_support_and_applicability_posture() {
    let applicability = geometry_applicability_matrix();

    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarPredicateAuthority,
                GeometryRuntimeConcern::LowerRuntimeRouting,
            )
            .expect("planar predicate routing row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarPredicateAuthority,
                GeometryRuntimeConcern::MutationEvidence,
            )
            .expect("planar predicate mutation evidence row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
