use worth_ui::facade::{CapabilityDiagnosticCode, RawLayoutMeasurementForDiagnostics};

use crate::app_fixtures::{
    minimal_app_builder, minimal_command_descriptor, minimal_illegal_mosaic_placement_policy,
    minimal_mosaic_sizing_contract,
};
use crate::diagnostic_assertions::{
    assert_diagnostic_codes, assert_diagnostic_codes_and_identities,
};
use crate::snapshot_assertions::{
    assert_minimal_app_snapshot_does_not_name_illegal_placement_policy,
    assert_minimal_app_snapshot_does_not_name_raw_sizing_contract,
    assert_minimal_app_snapshot_names_registered_capabilities,
    assert_minimal_app_snapshot_preserves_non_command_capabilities,
    assert_minimal_app_snapshot_rejects_duplicate_command,
};

#[test]
fn minimal_structural_app_registers_representative_capabilities() {
    let app = minimal_app_builder().freeze();

    assert_minimal_app_snapshot_names_registered_capabilities(app.capabilities());
}

#[test]
fn minimal_structural_app_duplicate_command_rejected() {
    let report = minimal_app_builder()
        .register_command(minimal_command_descriptor())
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[
            (
                CapabilityDiagnosticCode::DuplicateCapabilityId,
                "minimal.command.save",
            ),
            (
                CapabilityDiagnosticCode::DuplicateCapabilityId,
                "minimal.command.save",
            ),
        ],
    );
    assert_minimal_app_snapshot_rejects_duplicate_command(report.accepted_snapshot());
    assert_minimal_app_snapshot_preserves_non_command_capabilities(report.accepted_snapshot());
}

#[test]
fn minimal_structural_app_raw_layout_number_rejected() {
    let report = minimal_app_builder()
        .register_mosaic_sizing_contract(
            minimal_mosaic_sizing_contract("minimal.sizing.raw")
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
    assert_minimal_app_snapshot_names_registered_capabilities(report.accepted_snapshot());
    assert_minimal_app_snapshot_does_not_name_raw_sizing_contract(report.accepted_snapshot());
}

#[test]
fn minimal_structural_app_illegal_mosaic_placement_rejected() {
    let report = minimal_app_builder()
        .register_mosaic_placement_policy(minimal_illegal_mosaic_placement_policy(
            "minimal.placement.illegal",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::IllegalMosaicPlacementSourceTarget],
    );
    assert_minimal_app_snapshot_names_registered_capabilities(report.accepted_snapshot());
    assert_minimal_app_snapshot_does_not_name_illegal_placement_policy(report.accepted_snapshot());
}

#[test]
fn minimal_structural_app_snapshot_inspection_names_registered_capabilities() {
    let app = minimal_app_builder().freeze();

    assert_minimal_app_snapshot_names_registered_capabilities(app.capabilities());
}
