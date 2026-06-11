use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleDeclarationFamily,
    PlanarDiagnosticBundleQueryDomain, PlanarDiagnosticSubject,
};
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

#[test]
fn spatial_public_facade_exports_readable_planar_diagnostic_surface() {
    let family = <PlanarDiagnosticBundleDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
        PlanarDiagnosticBundleQueryDomain,
    >>::semantic_family_key();
    assert_eq!(family, "PlanarDiagnosticBundle");
    let diagnostic = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::predicate_failure("predicate:surface"),
    )
    .inspect_failure_locality();
    let _ = diagnostic;
}

#[test]
fn planar_diagnostic_bundle_is_registered_with_query_support_posture() {
    let support = geometry_public_support_matrix()
        .row_for_surface(GeometryPublicSurface::PlanarDiagnosticBundle)
        .expect("planar diagnostic support row")
        .clone();
    assert_eq!(
        support.declared_family_key(),
        Some("PlanarDiagnosticBundle")
    );
    assert_eq!(
        support.admission_rule(),
        "support comes from admitted planar diagnostic bundle certification deriving machine-checkable locality and causal references from typed receipts without changing planar truth"
    );

    let applicability = geometry_applicability_matrix();
    for concern in [
        GeometryRuntimeConcern::LowerRuntimeRouting,
        GeometryRuntimeConcern::HistoricalInspection,
        GeometryRuntimeConcern::BranchLocalInspection,
        GeometryRuntimeConcern::ProjectionConsumption,
        GeometryRuntimeConcern::RecoveryAction,
        GeometryRuntimeConcern::BooleanReadinessCertification,
    ] {
        assert_eq!(
            applicability
                .row(GeometryPublicSurface::PlanarDiagnosticBundle, concern)
                .expect("required planar diagnostic applicability row")
                .status(),
            GeometryApplicabilityStatus::RequiredNow
        );
    }
    for concern in [
        GeometryRuntimeConcern::MutationEvidence,
        GeometryRuntimeConcern::ReplayParity,
    ] {
        assert_eq!(
            applicability
                .row(GeometryPublicSurface::PlanarDiagnosticBundle, concern)
                .expect("denied planar diagnostic applicability row")
                .status(),
            GeometryApplicabilityStatus::DeniedForThisRuntime
        );
    }
}
