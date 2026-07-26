use crate::capability::{
    CapabilitySupportCatalog, CapabilitySupportKind, RegistrationCandidate, COMPONENT_FAMILY_NAME,
    SURFACE_FAMILY_NAME, THEME_TOKEN_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};
use crate::source::{WorthUiArtifactInputResolver, WorthUiResolutionDiagnosticCode};
use worth_ui_dsl::{
    WorthUiArtifactInputProvenance, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule,
};

use super::resolution_fixture_support::{
    admitted_app, component_descriptor, diagnostic_codes, empty_snapshot,
    snapshot_with_support_catalog, support_catalog_with_extra,
};

#[test]
fn missing_or_deferred_capability_rejected_at_resolution_boundary() {
    let snapshot = snapshot_with_support_catalog(
        empty_snapshot().capabilities(),
        CapabilitySupportCatalog::from_registration_candidates(&[
            RegistrationCandidate::with_support(
                COMPONENT_FAMILY_NAME,
                "workspace.component.deferred",
                CapabilitySupportKind::Deferred,
            ),
        ]),
    );
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_component("workspace.component.deferred")
                .with_surface("workspace.surface.missing"),
        ]),
    );

    let report = WorthUiArtifactInputResolver::resolve(&artifact_input, &snapshot)
        .expect_err("resolution should fail at phase 4");

    assert_eq!(
        diagnostic_codes(&report),
        [
            WorthUiResolutionDiagnosticCode::DeferredComponentReference,
            WorthUiResolutionDiagnosticCode::MissingSurfaceReference,
        ]
    );
}

#[test]
fn unsupported_references_localize_to_phase_4_with_exact_codes() {
    let snapshot = snapshot_with_support_catalog(
        empty_snapshot().capabilities(),
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
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_component("workspace.component.unsupported")
                .with_surface("workspace.surface.unsupported")
                .with_binding("workspace.view_binding.unsupported")
                .with_token("theme.text.unsupported", "ignored"),
        ]),
    );

    let report = WorthUiArtifactInputResolver::resolve(&artifact_input, &snapshot)
        .expect_err("resolution should fail");

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

#[test]
fn platform_internal_references_fail_here_and_never_silently_degrade() {
    let snapshot = snapshot_with_support_catalog(
        empty_snapshot().capabilities(),
        CapabilitySupportCatalog::from_registration_candidates(&[
            RegistrationCandidate::with_support(
                COMPONENT_FAMILY_NAME,
                "workspace.component.internal",
                CapabilitySupportKind::PlatformInternal,
            ),
            RegistrationCandidate::with_support(
                SURFACE_FAMILY_NAME,
                "workspace.surface.internal",
                CapabilitySupportKind::PlatformInternal,
            ),
            RegistrationCandidate::with_support(
                VIEW_BINDING_FAMILY_NAME,
                "workspace.view_binding.internal",
                CapabilitySupportKind::PlatformInternal,
            ),
            RegistrationCandidate::with_support(
                THEME_TOKEN_FAMILY_NAME,
                "theme.text.internal",
                CapabilitySupportKind::PlatformInternal,
            ),
        ]),
    );
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_component("workspace.component.internal")
                .with_surface("workspace.surface.internal")
                .with_binding("workspace.view_binding.internal")
                .with_token("theme.text.internal", "ignored"),
        ]),
    );

    let report = WorthUiArtifactInputResolver::resolve(&artifact_input, &snapshot)
        .expect_err("resolution should fail");

    assert_eq!(
        diagnostic_codes(&report),
        [
            WorthUiResolutionDiagnosticCode::PlatformInternalComponentReference,
            WorthUiResolutionDiagnosticCode::PlatformInternalSurfaceReference,
            WorthUiResolutionDiagnosticCode::PlatformInternalViewBindingReference,
            WorthUiResolutionDiagnosticCode::PlatformInternalThemeTokenReference,
        ]
    );
}

#[test]
fn invalid_reference_ids_fail_as_structured_resolution_diagnostics() {
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_component("BadComponent")
                .with_surface("BadSurface")
                .with_binding("BadBinding")
                .with_token("BadToken", "ignored"),
        ]),
    );

    let report =
        WorthUiArtifactInputResolver::resolve(&artifact_input, empty_snapshot().capabilities())
            .expect_err("invalid ids should fail as diagnostics, not panics");

    assert_eq!(
        diagnostic_codes(&report),
        [
            WorthUiResolutionDiagnosticCode::InvalidComponentReferenceId,
            WorthUiResolutionDiagnosticCode::InvalidSurfaceReferenceId,
            WorthUiResolutionDiagnosticCode::InvalidViewBindingReferenceId,
            WorthUiResolutionDiagnosticCode::InvalidThemeTokenReferenceId,
        ]
    );
}

#[test]
fn mixed_success_and_failure_resolution_reports_are_deterministically_sorted() {
    let app = admitted_app();
    let snapshot = snapshot_with_support_catalog(
        app.capabilities(),
        support_catalog_with_extra([
            RegistrationCandidate::with_support(
                COMPONENT_FAMILY_NAME,
                "workspace.component.unsupported",
                CapabilitySupportKind::Unsupported,
            ),
            RegistrationCandidate::with_support(
                SURFACE_FAMILY_NAME,
                "workspace.surface.missing_later",
                CapabilitySupportKind::Deferred,
            ),
        ]),
    );
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/zeta.wui")
                .with_surface("workspace.surface.missing_later"),
            WorthUiRustAuthoredArtifactInputModule::new("app/alpha.wui")
                .with_component("workspace.component.unsupported")
                .with_component("workspace.component.dashboard"),
        ]),
    );

    let report = WorthUiArtifactInputResolver::resolve(&artifact_input, &snapshot)
        .expect_err("resolution should fail");

    let ordered = report
        .diagnostics()
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.code(),
                diagnostic.module_id().as_str().to_owned(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        ordered,
        vec![
            (
                WorthUiResolutionDiagnosticCode::UnsupportedComponentReference,
                "app/alpha.wui".to_owned(),
            ),
            (
                WorthUiResolutionDiagnosticCode::DeferredSurfaceReference,
                "app/zeta.wui".to_owned(),
            ),
        ]
    );
    assert_eq!(
        report.diagnostics()[0].authored_text(),
        "workspace.component.unsupported"
    );
    assert!(matches!(
        report.diagnostics()[0].provenance(),
        WorthUiArtifactInputProvenance::RustAuthoredDeclaration { .. }
    ));
}

#[test]
fn resolution_never_falls_back_to_mutable_builder_state() {
    let _unregistered_component = component_descriptor("workspace.component.only_in_scope");
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_component("workspace.component.only_in_scope"),
        ]),
    );

    let report =
        WorthUiArtifactInputResolver::resolve(&artifact_input, empty_snapshot().capabilities())
            .expect_err("resolution should not consult caller-local descriptor state");

    assert_eq!(
        diagnostic_codes(&report),
        [WorthUiResolutionDiagnosticCode::MissingComponentReference]
    );
}

#[test]
fn resolution_does_not_scan_broad_registry_families_for_direct_lookup() {
    let app = admitted_app();
    let artifact_input = crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_component("workspace.component.dashboard")
                .with_surface("workspace.surface.inspector")
                .with_binding("workspace.view_binding.selection")
                .with_token("theme.text.default", "theme.text.primary")
                .with_component("workspace.component.missing"),
        ]),
    );

    let report = WorthUiArtifactInputResolver::resolve(&artifact_input, app.capabilities())
        .expect_err("resolution should fail only for the missing final component");

    assert_eq!(
        diagnostic_codes(&report),
        [WorthUiResolutionDiagnosticCode::MissingComponentReference]
    );
    assert_eq!(report.metrics().direct_lookup_count(), 5);
    assert_eq!(report.metrics().families_scanned(), 0);
    assert!(report.metrics().total_family_width() >= report.metrics().direct_lookup_count());
}
