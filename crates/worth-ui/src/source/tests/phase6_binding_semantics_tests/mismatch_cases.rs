use super::binding_app_fixture::admitted_app;
use super::binding_phase_fixture::legally_structured_artifact_input;
use super::binding_snapshot_support::snapshot_with_support_catalog;
use super::binding_support_catalog_fixture::support_catalog_with_extra;
use crate::capability::{
    CapabilitySupportKind, RegistrationCandidate, COMMAND_FAMILY_NAME,
    COMMAND_PROJECTION_FAMILY_NAME, ICON_FAMILY_NAME, THEME_TOKEN_FAMILY_NAME,
    VIEW_BINDING_FAMILY_NAME,
};
use crate::source::{WorthUiBindingDiagnosticCode, WorthUiBindingSemanticsLowerer};

#[test]
fn binding_family_mismatch_localizes_to_binding_boundary() {
    let app = admitted_app();
    let snapshot = app.capabilities();
    let unsupported_snapshot = snapshot_with_support_catalog(
        snapshot,
        support_catalog_with_extra([RegistrationCandidate::with_support(
            VIEW_BINDING_FAMILY_NAME,
            "workspace.view_binding.selection",
            CapabilitySupportKind::Unsupported,
        )]),
    );
    let legally_structured = legally_structured_artifact_input(snapshot);

    let report = WorthUiBindingSemanticsLowerer::lower(&legally_structured, &unsupported_snapshot)
        .unwrap_err();
    assert!(report
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.code()
            == WorthUiBindingDiagnosticCode::UnsupportedSemanticViewBindingReference));
}

#[test]
fn nested_descriptor_capability_mismatches_localize_to_binding_boundary() {
    let app = admitted_app();
    let snapshot = app.capabilities();
    let platform_internal_snapshot = snapshot_with_support_catalog(
        snapshot,
        support_catalog_with_extra([
            RegistrationCandidate::with_support(
                ICON_FAMILY_NAME,
                "workspace.icon.surface.inspector",
                CapabilitySupportKind::PlatformInternal,
            ),
            RegistrationCandidate::with_support(
                COMMAND_PROJECTION_FAMILY_NAME,
                "workspace.command_projection.inspect_actions",
                CapabilitySupportKind::Unsupported,
            ),
        ]),
    );
    let legally_structured = legally_structured_artifact_input(snapshot);

    let report =
        WorthUiBindingSemanticsLowerer::lower(&legally_structured, &platform_internal_snapshot)
            .unwrap_err();
    let diagnostic_codes = report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect::<Vec<_>>();

    assert!(diagnostic_codes
        .contains(&WorthUiBindingDiagnosticCode::PlatformInternalSemanticSurfaceIconReference));
    assert!(diagnostic_codes
        .contains(&WorthUiBindingDiagnosticCode::UnsupportedSemanticCommandProjectionReference));
    assert_eq!(report.metrics().direct_lookup_count(), 6);
    assert_eq!(report.metrics().families_scanned(), 0);
}

#[test]
fn missing_or_deferred_command_reference_rejected_at_binding_boundary() {
    let app = admitted_app();
    let snapshot = app.capabilities();
    let deferred_snapshot = snapshot_with_support_catalog(
        snapshot,
        support_catalog_with_extra([RegistrationCandidate::with_support(
            COMMAND_FAMILY_NAME,
            "workspace.command.inspect",
            CapabilitySupportKind::Deferred,
        )]),
    );
    let legally_structured = legally_structured_artifact_input(snapshot);

    let report =
        WorthUiBindingSemanticsLowerer::lower(&legally_structured, &deferred_snapshot).unwrap_err();
    assert!(report
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.code()
            == WorthUiBindingDiagnosticCode::DeferredSemanticCommandReference));
    assert_eq!(report.metrics().direct_lookup_count(), 4);
    assert_eq!(report.metrics().families_scanned(), 0);
}

#[test]
fn token_reference_posture_mismatch_localizes_to_binding_boundary() {
    let app = admitted_app();
    let snapshot = app.capabilities();
    let unsupported_snapshot = snapshot_with_support_catalog(
        snapshot,
        support_catalog_with_extra([RegistrationCandidate::with_support(
            THEME_TOKEN_FAMILY_NAME,
            "theme.text.primary",
            CapabilitySupportKind::PlatformInternal,
        )]),
    );
    let legally_structured = legally_structured_artifact_input(snapshot);

    let report = WorthUiBindingSemanticsLowerer::lower(&legally_structured, &unsupported_snapshot)
        .unwrap_err();
    assert!(report
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.code()
            == WorthUiBindingDiagnosticCode::PlatformInternalSemanticThemeTokenReference));
    assert_eq!(report.metrics().direct_lookup_count(), 6);
    assert_eq!(report.metrics().families_scanned(), 0);
}

#[test]
fn mixed_semantic_failures_report_in_deterministic_code_order() {
    let app = admitted_app();
    let snapshot = app.capabilities();
    let rejected_snapshot = snapshot_with_support_catalog(
        snapshot,
        support_catalog_with_extra([
            RegistrationCandidate::with_support(
                ICON_FAMILY_NAME,
                "workspace.icon.surface.inspector",
                CapabilitySupportKind::Unsupported,
            ),
            RegistrationCandidate::with_support(
                COMMAND_PROJECTION_FAMILY_NAME,
                "workspace.command_projection.inspect_actions",
                CapabilitySupportKind::Deferred,
            ),
            RegistrationCandidate::with_support(
                VIEW_BINDING_FAMILY_NAME,
                "workspace.view_binding.selection",
                CapabilitySupportKind::PlatformInternal,
            ),
        ]),
    );
    let legally_structured = legally_structured_artifact_input(snapshot);

    let report =
        WorthUiBindingSemanticsLowerer::lower(&legally_structured, &rejected_snapshot).unwrap_err();
    let diagnostic_codes = report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect::<Vec<_>>();

    assert_eq!(
        diagnostic_codes,
        vec![
            WorthUiBindingDiagnosticCode::UnsupportedSemanticSurfaceIconReference,
            WorthUiBindingDiagnosticCode::DeferredSemanticCommandProjectionReference,
            WorthUiBindingDiagnosticCode::PlatformInternalSemanticViewBindingReference,
        ]
    );
    assert_eq!(report.metrics().direct_lookup_count(), 6);
    assert_eq!(report.metrics().families_scanned(), 0);
}
