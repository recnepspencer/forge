use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn planar_contract_bundle_validator_is_registered_with_boolean_readiness_posture() {
    let support = geometry_public_support_matrix();
    let row = support
        .row_for_surface(GeometryPublicSurface::PlanarContractBundleValidator)
        .expect("bundle validator support row");
    assert_eq!(
        row.declared_family_key(),
        Some("PlanarContractBundleValidator")
    );
    assert!(row
        .admission_rule()
        .contains("boolean-readiness input without computing boolean topology"));

    let applicability = geometry_applicability_matrix();
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarContractBundleValidator,
                GeometryRuntimeConcern::BooleanReadinessCertification,
            )
            .expect("readiness row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarContractBundleValidator,
                GeometryRuntimeConcern::MutationEvidence,
            )
            .expect("mutation row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
