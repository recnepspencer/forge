use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        MosaicMeasurementAuthority, MosaicOverflowBehavior, MosaicParentGrowthBehavior,
        MosaicResizePermission, MosaicSizingPersistence, MosaicViewportConstraint,
        RawLayoutMeasurementForDiagnostics,
    },
    diagnostics::CapabilityDiagnosticCode,
};

use super::sizing_assertions::assert_diagnostic_codes;
use super::sizing_fixtures::{
    bounded_sidebar_contract, complete_sizing_contract, inverted_constraint_sizing_contract,
    mixed_unit_constraint_sizing_contract, unitless_constraint_sizing_contract,
    unitless_sizing_contract,
};

#[test]
fn raw_width_value_rejected_outside_named_measurement() {
    assert_raw_measurement_rejected(
        RawLayoutMeasurementForDiagnostics::width(320),
        CapabilityDiagnosticCode::RawMosaicWidthMeasurementOutsideNamedMeasurement,
    );
}

#[test]
fn raw_height_value_rejected_outside_named_measurement() {
    assert_raw_measurement_rejected(
        RawLayoutMeasurementForDiagnostics::height(600),
        CapabilityDiagnosticCode::RawMosaicWidthMeasurementOutsideNamedMeasurement,
    );
}

#[test]
fn raw_gap_value_rejected_outside_named_measurement() {
    assert_raw_measurement_rejected(
        RawLayoutMeasurementForDiagnostics::gap(8),
        CapabilityDiagnosticCode::RawMosaicGapMeasurementOutsideNamedMeasurement,
    );
}

#[test]
fn raw_z_order_value_rejected_outside_named_measurement() {
    assert_raw_measurement_rejected(
        RawLayoutMeasurementForDiagnostics::z_order(10),
        CapabilityDiagnosticCode::RawMosaicZOrderMeasurementOutsideNamedMeasurement,
    );
}

#[test]
fn raw_timing_value_rejected_outside_named_measurement() {
    assert_raw_measurement_rejected(
        RawLayoutMeasurementForDiagnostics::timing(120),
        CapabilityDiagnosticCode::RawMosaicTimingMeasurementOutsideNamedMeasurement,
    );
}

#[test]
fn raw_breakpoint_value_rejected_outside_named_measurement() {
    assert_raw_measurement_rejected(
        RawLayoutMeasurementForDiagnostics::breakpoint(768),
        CapabilityDiagnosticCode::RawMosaicBreakpointMeasurementOutsideNamedMeasurement,
    );
}

#[test]
fn raw_padding_value_rejected_outside_named_measurement() {
    assert_raw_measurement_rejected(
        RawLayoutMeasurementForDiagnostics::padding(16),
        CapabilityDiagnosticCode::RawMosaicGapMeasurementOutsideNamedMeasurement,
    );
}

#[test]
fn unitless_measurement_definition_rejected() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(unitless_sizing_contract("workspace.sizing.unitless"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_sizing_contracts()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnitlessMosaicSizingMeasurementDefinition],
    );
}

#[test]
fn unitless_constraint_measurement_definition_rejected() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(unitless_constraint_sizing_contract(
            "workspace.sizing.unitless_constraint",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_sizing_contracts()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnitlessMosaicSizingMeasurementDefinition],
    );
}

#[test]
fn inverted_measurement_constraint_rejected() {
    assert_invalid_measurement_constraint_rejected(inverted_constraint_sizing_contract(
        "workspace.sizing.inverted_constraint",
    ));
}

#[test]
fn mixed_unit_measurement_constraint_rejected() {
    assert_invalid_measurement_constraint_rejected(mixed_unit_constraint_sizing_contract(
        "workspace.sizing.mixed_unit_constraint",
    ));
}

#[test]
fn sizing_contract_without_overflow_policy_rejected() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(
            complete_sizing_contract("workspace.sizing.no_overflow", sizing_kind())
                .with_overflow_behavior(MosaicOverflowBehavior::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_sizing_contracts()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingMosaicSizingOverflowBehavior],
    );
}

#[test]
fn sizing_contract_reports_every_missing_required_policy() {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(
            bounded_sidebar_contract("workspace.sizing.missing")
                .with_measurement_authority(MosaicMeasurementAuthority::missing_for_diagnostics())
                .with_resize_permission(MosaicResizePermission::missing_for_diagnostics())
                .with_persistence(MosaicSizingPersistence::missing_for_diagnostics())
                .with_overflow_behavior(MosaicOverflowBehavior::missing_for_diagnostics())
                .with_parent_growth_behavior(MosaicParentGrowthBehavior::missing_for_diagnostics())
                .with_viewport_constraint(MosaicViewportConstraint::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_sizing_contracts()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::MissingMosaicSizingMeasurementAuthority,
            CapabilityDiagnosticCode::MissingMosaicSizingResizePermission,
            CapabilityDiagnosticCode::MissingMosaicSizingPersistence,
            CapabilityDiagnosticCode::MissingMosaicSizingOverflowBehavior,
            CapabilityDiagnosticCode::MissingMosaicSizingParentGrowthBehavior,
            CapabilityDiagnosticCode::MissingMosaicSizingViewportConstraint,
        ],
    );
}

fn assert_raw_measurement_rejected(
    raw_measurement: RawLayoutMeasurementForDiagnostics,
    expected_code: CapabilityDiagnosticCode,
) {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(
            bounded_sidebar_contract("workspace.sizing.raw")
                .with_raw_measurement_for_diagnostics(raw_measurement),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_sizing_contracts()
        .is_empty());
    assert_diagnostic_codes(report.registration_diagnostics(), &[expected_code]);
}

fn assert_invalid_measurement_constraint_rejected(
    descriptor: worth_ui::facade::declaration::MosaicSizingContractDescriptor,
) {
    let report = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(descriptor)
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_sizing_contracts()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::InvalidMosaicSizingMeasurementConstraint],
    );
}

fn sizing_kind() -> worth_ui::facade::declaration::MosaicSizingKind {
    worth_ui::facade::declaration::MosaicSizingKind::bounded()
}
