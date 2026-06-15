use crate::capability::{
    CapabilitySupportKind, RegistrationCandidate, COMPONENT_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};
use crate::source::{
    WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiParsedSourceToArtifactInputLowerer, WorthUiSourcePackage, WorthUiSourceParser,
    WorthUiStructuralLegalityLowerer,
};

use super::super::phase7_identity_seeding_tests::identity_app_fixture::identity_test_app;
use super::sample_certification_pipeline::{
    binding_codes, resolution_codes, WorthUiSampleCertificationFailure,
};
use super::sample_snapshot_support::{
    sample_snapshot_with_support_catalog, sample_support_catalog_with_extra,
};

pub(super) fn bind_file_source_with_view_binding_rejected(
    source_package: WorthUiSourcePackage,
) -> WorthUiSampleCertificationFailure {
    let parsed_package =
        WorthUiSourceParser::parse_package(&source_package).expect("source should parse");
    let artifact_input = WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed_package)
        .expect("authoring entry should lower to artifact input");
    let app = identity_test_app();
    let snapshot = app.capabilities();
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("resolution should succeed before binding rejection");
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("structural legality should succeed before binding rejection");
    let rejected_snapshot = sample_snapshot_with_support_catalog(
        snapshot,
        sample_support_catalog_with_extra([RegistrationCandidate::with_support(
            VIEW_BINDING_FAMILY_NAME,
            "workspace.view_binding.selection",
            CapabilitySupportKind::PlatformInternal,
        )]),
    );
    let report =
        WorthUiBindingSemanticsLowerer::lower(&structured, &rejected_snapshot).unwrap_err();

    WorthUiSampleCertificationFailure::BindingSemantics(binding_codes(&report))
}

pub(super) fn resolve_file_source_with_component_unsupported(
    source_package: WorthUiSourcePackage,
) -> WorthUiSampleCertificationFailure {
    let parsed_package =
        WorthUiSourceParser::parse_package(&source_package).expect("source should parse");
    let artifact_input = WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed_package)
        .expect("authoring entry should lower to artifact input");
    let app = identity_test_app();
    let unsupported_snapshot = sample_snapshot_with_support_catalog(
        app.capabilities(),
        sample_support_catalog_with_extra([RegistrationCandidate::with_support(
            COMPONENT_FAMILY_NAME,
            "workspace.component.unsupported",
            CapabilitySupportKind::Unsupported,
        )]),
    );
    let report = WorthUiArtifactInputResolver::resolve(&artifact_input, &unsupported_snapshot)
        .expect_err("unsupported component must fail at resolution");

    WorthUiSampleCertificationFailure::Resolution(resolution_codes(&report))
}
