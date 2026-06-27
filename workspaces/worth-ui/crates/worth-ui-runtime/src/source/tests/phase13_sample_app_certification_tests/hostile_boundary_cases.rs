use super::sample_app_sources::{
    malformed_file_source_package, sample_file_source_package,
    source_package_with_illegal_structure, source_package_with_missing_component,
    source_package_with_unsupported_component,
};
use super::sample_boundary_rejection_pipeline::{
    bind_file_source_with_view_binding_rejected, resolve_file_source_with_component_unsupported,
};
use super::sample_certification_pipeline::{
    certify_file_source_package, WorthUiSampleCertificationFailure,
};
use crate::source::{
    WorthUiBindingDiagnosticCode, WorthUiParseDiagnosticCode, WorthUiResolutionDiagnosticCode,
    WorthUiStructuralLegalityDiagnosticCode,
};

#[test]
fn hostile_source_rejection_localizes_before_artifact_authority() {
    assert_eq!(
        certify_file_source_package(malformed_file_source_package())
            .expect_err("malformed source must stop at parse"),
        WorthUiSampleCertificationFailure::Parse(vec![
            WorthUiParseDiagnosticCode::UnterminatedBlock
        ])
    );

    assert_eq!(
        certify_file_source_package(source_package_with_missing_component())
            .expect_err("missing component support must stop at resolution"),
        WorthUiSampleCertificationFailure::Resolution(vec![
            WorthUiResolutionDiagnosticCode::MissingComponentReference
        ])
    );

    assert_eq!(
        resolve_file_source_with_component_unsupported(source_package_with_unsupported_component()),
        WorthUiSampleCertificationFailure::Resolution(vec![
            WorthUiResolutionDiagnosticCode::UnsupportedComponentReference
        ])
    );

    assert_eq!(
        certify_file_source_package(source_package_with_illegal_structure())
            .expect_err("illegal structure must stop at structural legality"),
        WorthUiSampleCertificationFailure::StructuralLegality(vec![
            WorthUiStructuralLegalityDiagnosticCode::IllegalRootStructuralStatement
        ])
    );

    assert_eq!(
        bind_file_source_with_view_binding_rejected(sample_file_source_package()),
        WorthUiSampleCertificationFailure::BindingSemantics(vec![
            WorthUiBindingDiagnosticCode::PlatformInternalSemanticViewBindingReference
        ])
    );
}
