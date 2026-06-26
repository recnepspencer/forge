use crate::capability::{
    CapabilitySupportCatalog, CapabilitySupportKind, RegistrationCandidate, COMPONENT_FAMILY_NAME,
    SURFACE_FAMILY_NAME, THEME_TOKEN_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};
use crate::source::{
    WorthUiArtifactInputProvenance, WorthUiArtifactInputResolver, WorthUiResolutionDiagnosticCode,
    WorthUiRustCompositionInput, WorthUiRustCompositionModule,
    WorthUiRustCompositionToArtifactInputLowerer,
};

use super::super::phase4_snapshot_resolution_tests::resolution_fixture_support::{
    diagnostic_codes, empty_snapshot, snapshot_with_support_catalog,
};

use super::rust_composition_fixture_support::{
    missing_component_rust_composition, resolution_report_from_composition,
};

#[test]
fn rust_composition_cannot_bypass_snapshot_bound_resolution() {
    let report = resolution_report_from_composition(&missing_component_rust_composition());
    let diagnostics = report.diagnostics();

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0].code(),
        WorthUiResolutionDiagnosticCode::MissingComponentReference
    );
    assert_eq!(
        diagnostics[0].authored_text(),
        "workspace.component.missing"
    );
    assert!(matches!(
        diagnostics[0].provenance(),
        WorthUiArtifactInputProvenance::RustAuthoredDeclaration { .. }
    ));
}

#[test]
fn rust_composition_cannot_bypass_deferred_snapshot_bound_resolution() {
    let report = resolve_rust_composition_against_support_catalog(
        WorthUiRustCompositionInput::from_modules([WorthUiRustCompositionModule::new(
            "app/main.wui",
        )
        .component("workspace.component.deferred")]),
        CapabilitySupportCatalog::from_registration_candidates(&[
            RegistrationCandidate::with_support(
                COMPONENT_FAMILY_NAME,
                "workspace.component.deferred",
                CapabilitySupportKind::Deferred,
            ),
        ]),
    );

    assert_eq!(
        diagnostic_codes(&report),
        [WorthUiResolutionDiagnosticCode::DeferredComponentReference]
    );
    assert_eq!(
        report.diagnostics()[0].authored_text(),
        "workspace.component.deferred"
    );
}

#[test]
fn rust_composition_cannot_bypass_unsupported_snapshot_bound_resolution() {
    let report = resolve_rust_composition_against_support_catalog(
        WorthUiRustCompositionInput::from_modules([WorthUiRustCompositionModule::new(
            "app/main.wui",
        )
        .component("workspace.component.unsupported")
        .surface("workspace.surface.unsupported")
        .binding("workspace.view_binding.unsupported")
        .token("theme.text.unsupported", "ignored")]),
        CapabilitySupportCatalog::from_registration_candidates(&[
            RegistrationCandidate::with_support(
                COMPONENT_FAMILY_NAME,
                "workspace.component.unsupported",
                CapabilitySupportKind::Unsupported,
            ),
            RegistrationCandidate::with_support(
                SURFACE_FAMILY_NAME,
                "workspace.surface.unsupported",
                CapabilitySupportKind::Unsupported,
            ),
            RegistrationCandidate::with_support(
                VIEW_BINDING_FAMILY_NAME,
                "workspace.view_binding.unsupported",
                CapabilitySupportKind::Unsupported,
            ),
            RegistrationCandidate::with_support(
                THEME_TOKEN_FAMILY_NAME,
                "theme.text.unsupported",
                CapabilitySupportKind::Unsupported,
            ),
        ]),
    );

    assert_eq!(
        diagnostic_codes(&report),
        [
            WorthUiResolutionDiagnosticCode::UnsupportedComponentReference,
            WorthUiResolutionDiagnosticCode::UnsupportedSurfaceReference,
            WorthUiResolutionDiagnosticCode::UnsupportedViewBindingReference,
            WorthUiResolutionDiagnosticCode::UnsupportedThemeTokenReference,
        ]
    );
}

fn resolve_rust_composition_against_support_catalog(
    composition: WorthUiRustCompositionInput,
    support_catalog: CapabilitySupportCatalog,
) -> crate::source::WorthUiResolutionReport {
    let snapshot = snapshot_with_support_catalog(empty_snapshot().capabilities(), support_catalog);
    let artifact_input = WorthUiRustCompositionToArtifactInputLowerer::lower(&composition);

    WorthUiArtifactInputResolver::resolve(&artifact_input, &snapshot)
        .expect_err("rust composition should fail at snapshot resolution")
}
