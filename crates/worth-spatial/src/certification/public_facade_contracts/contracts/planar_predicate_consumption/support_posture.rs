use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn predicate_certificate_consumption_validator_is_registered_for_boolean_readiness() {
    let support = geometry_public_support_matrix();
    let row = support
        .row_for_surface(GeometryPublicSurface::PredicateCertificateConsumptionValidator)
        .expect("predicate consumption support row");
    assert_eq!(
        row.declared_family_key(),
        Some("PredicateCertificateConsumptionValidator")
    );
    assert!(row
        .admission_rule()
        .contains("consumed worth-math certified signs"));

    let applicability = geometry_applicability_matrix();
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PredicateCertificateConsumptionValidator,
                GeometryRuntimeConcern::BooleanReadinessCertification,
            )
            .expect("boolean readiness row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PredicateCertificateConsumptionValidator,
                GeometryRuntimeConcern::ProjectionConsumption,
            )
            .expect("projection consumption row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PredicateCertificateConsumptionValidator,
                GeometryRuntimeConcern::MutationEvidence,
            )
            .expect("mutation row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}
