use super::structural_legality_body_fixture_support::{
    illegal_region_child_mix_body_atoms, invalid_sizing_body_atoms,
    resolved_artifact_input_from_modules, standard_component_module,
};
use super::structural_legality_capability_fixture_support::{
    standard_app, support_catalog_with_extra,
};
use super::structural_legality_snapshot_support::snapshot_with_support_catalog;
use crate::capability::{
    CapabilitySupportKind, RegistrationCandidate, MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
    MOSAIC_SIZING_CONTRACT_FAMILY_NAME,
};
use crate::source::{WorthUiStructuralLegalityDiagnosticCode, WorthUiStructuralLegalityLowerer};
use worth_ui_dsl::WorthUiRustAuthoredArtifactInputModule;

#[test]
fn illegal_region_or_scroll_or_sizing_shape_rejected_before_artifact_assembly() {
    let app = standard_app();
    let snapshot = app.capabilities();
    let resolved = resolved_artifact_input_from_modules(
        [
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_component_body_atoms(
                "workspace.component.dashboard",
                invalid_sizing_body_atoms(),
            ),
        ],
        snapshot,
    );

    let report = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).unwrap_err();
    let codes: Vec<_> = report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect();
    assert!(
        codes.contains(&WorthUiStructuralLegalityDiagnosticCode::IllegalSizingContractForRegion)
    );
    assert!(codes
        .contains(&WorthUiStructuralLegalityDiagnosticCode::IllegalPinnedStateSlotForRegionRole));
}

#[test]
fn missing_or_deferred_capability_rejected_at_structural_boundary() {
    let app = standard_app();
    let snapshot = app.capabilities();
    let deferred_snapshot = snapshot_with_support_catalog(
        snapshot,
        support_catalog_with_extra([RegistrationCandidate::with_support(
            MOSAIC_SIZING_CONTRACT_FAMILY_NAME,
            "workspace.sizing.fill",
            CapabilitySupportKind::Deferred,
        )]),
    );
    let resolved = resolved_artifact_input_from_modules(
        [super::structural_legality_body_fixture_support::standard_component_module()],
        snapshot,
    );

    let report =
        WorthUiStructuralLegalityLowerer::lower(&resolved, &deferred_snapshot).unwrap_err();
    assert!(report
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.code()
            == WorthUiStructuralLegalityDiagnosticCode::DeferredMosaicSizingContractReference));
    assert_eq!(report.metrics().families_scanned(), 0);
    assert_eq!(report.metrics().renderer_dependent_checks(), 0);
}

#[test]
fn platform_internal_references_fail_here_with_deterministic_sorted_report() {
    let app = standard_app();
    let snapshot = app.capabilities();
    let platform_internal_snapshot = snapshot_with_support_catalog(
        snapshot,
        support_catalog_with_extra([RegistrationCandidate::with_support(
            MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
            "workspace.placement.primary",
            CapabilitySupportKind::PlatformInternal,
        )]),
    );
    let resolved = resolved_artifact_input_from_modules([standard_component_module()], snapshot);

    let report = WorthUiStructuralLegalityLowerer::lower(&resolved, &platform_internal_snapshot)
        .unwrap_err();
    let codes: Vec<_> = report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect();
    let mut sorted_codes = codes.clone();
    sorted_codes.sort();
    assert_eq!(codes, sorted_codes);
    assert!(codes.contains(
        &WorthUiStructuralLegalityDiagnosticCode::PlatformInternalMosaicPlacementPolicyReference
    ));
}

#[test]
fn region_child_rule_violations_reject_nested_region_misuse() {
    let app = standard_app();
    let snapshot = app.capabilities();
    let resolved = resolved_artifact_input_from_modules(
        [
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_component_body_atoms(
                "workspace.component.dashboard",
                illegal_region_child_mix_body_atoms(),
            ),
        ],
        snapshot,
    );

    let report = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).unwrap_err();
    assert!(report
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.code()
            == WorthUiStructuralLegalityDiagnosticCode::IllegalRegionChildMix));
}
