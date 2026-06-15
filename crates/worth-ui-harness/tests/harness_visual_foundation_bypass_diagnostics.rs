use worth_ui::facade::{
    CapabilityDiagnosticCode, MeasurementConstraint, MeasurementValue, MosaicMeasurementAuthority,
    MosaicOverflowBehavior, MosaicParentGrowthBehavior, MosaicResizePermission,
    MosaicSizingContractDescriptor, MosaicSizingContractId, MosaicSizingKind,
    MosaicSizingPersistence, MosaicViewportConstraint, NamedMeasurementDefinition,
    NamedMeasurementToken, RawColorOutsideTokenDefinition, RawLayoutMeasurementForDiagnostics,
    RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionId, ThemeTokenDescriptor,
    ThemeTokenId, WorthUi,
};
use worth_ui_harness::facade::{
    HarnessVisualFoundationBundle, HarnessVisualFoundationRegistration, HarnessVisualTokenRole,
};

#[test]
fn raw_visual_value_bypass_reports_typed_diagnostics_without_poisoning_foundation() {
    let prepared = HarnessVisualFoundationBundle::vscode_like_dark()
        .prepare()
        .expect("default visual foundation should prepare");
    let report = WorthUi::app()
        .install_harness_visual_foundation(prepared)
        .register_theme_token(
            ThemeTokenDescriptor::raw_color_outside_token_definition_for_diagnostics(
                ThemeTokenId::new("harness.theme.raw_bypass").unwrap(),
                RawColorOutsideTokenDefinition::new("#ffffff"),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::MissingThemeTokenDefinition,
            CapabilityDiagnosticCode::RawColorOutsideThemeTokenDefinition,
        ],
    );
    assert_eq!(
        report.accepted_snapshot().theme_tokens().len(),
        HarnessVisualTokenRole::REQUIRED.len(),
        "bad raw token must not erase accepted harness theme tokens"
    );
}

#[test]
fn raw_density_measurement_bypass_reports_typed_diagnostics_without_poisoning_foundation() {
    let prepared = HarnessVisualFoundationBundle::vscode_like_dark()
        .prepare()
        .expect("default visual foundation should prepare");
    let report = WorthUi::app()
        .install_harness_visual_foundation(prepared)
        .register_mosaic_sizing_contract(
            complete_sizing_contract("harness.sizing.raw_bypass")
                .with_raw_measurement_for_diagnostics(RawLayoutMeasurementForDiagnostics::width(
                    320,
                )),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::RawMosaicWidthMeasurementOutsideNamedMeasurement],
    );
    assert_eq!(
        report.accepted_snapshot().mosaic_sizing_contracts().len(),
        worth_ui_harness::facade::HarnessDensity::REQUIRED_SIZING_CONTRACT_IDS.len(),
        "bad raw sizing must not erase accepted harness sizing contracts"
    );
}

#[test]
fn local_runtime_status_bypass_reports_typed_diagnostics_without_poisoning_foundation() {
    let prepared = HarnessVisualFoundationBundle::vscode_like_dark()
        .prepare()
        .expect("default visual foundation should prepare");
    let report = WorthUi::app()
        .install_harness_visual_foundation(prepared)
        .register_runtime_outcome_projection(
            RuntimeOutcomeProjectionDescriptor::local_status_enum_for_diagnostics(
                RuntimeOutcomeProjectionId::new("harness.runtime_outcome.local_bypass").unwrap(),
                "Loading | Success | Error",
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::UnknownRuntimeOutcomeFamily,
            CapabilityDiagnosticCode::MissingRuntimeOutcomeSource,
            CapabilityDiagnosticCode::LocalStatusEnumRuntimeOutcomeProjection,
        ],
    );
    assert_eq!(
        report
            .accepted_snapshot()
            .runtime_outcome_projections()
            .len(),
        worth_ui_harness::facade::HarnessRuntimeOutcomeVisualRole::REQUIRED.len(),
        "bad local status enum must not erase accepted Query-backed outcome projections"
    );
}

fn complete_sizing_contract(id: &str) -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        MosaicSizingContractId::new(id).unwrap(),
        MosaicSizingKind::bounded(),
    )
    .with_named_measurement(NamedMeasurementDefinition::new(
        NamedMeasurementToken::new("harness.measurement.raw_bypass.width").unwrap(),
        MeasurementValue::logical_pixels(320),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(240),
            MeasurementValue::logical_pixels(520),
        ),
    ))
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
}

fn assert_diagnostic_codes(
    diagnostics: &[worth_ui::facade::CapabilityRegistrationDiagnostic],
    expected: &[CapabilityDiagnosticCode],
) {
    let actual = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect::<Vec<_>>();
    for code in expected {
        assert!(
            actual.contains(code),
            "expected diagnostic {code:?}, got {actual:?}"
        );
    }
}
