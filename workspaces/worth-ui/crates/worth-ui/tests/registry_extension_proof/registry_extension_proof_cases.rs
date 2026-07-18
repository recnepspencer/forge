use std::collections::BTreeMap;

use worth_ui::facade::{CapabilityDiagnosticCode, RegistryFamily};

use super::facade_exposure_assertions::{
    assert_every_family_has_facade_exposure_decision,
    assert_registry_family_names_round_trip_through_facade_inventory,
};
use super::fixtures::{
    duplicate_representative_family_registration_report, empty_app, single_command_app,
};
use super::inventory_assertions::{
    assert_empty_report_widths_match_inventory, assert_freeze_report_matches_registry_inventory,
    assert_inventory_audit_reports_duplicate_families,
    assert_inventory_audit_reports_unknown_and_omitted_families,
    assert_registry_inventory_names_are_unique,
    assert_single_command_report_widths_match_inventory,
    assert_unknown_family_name_is_not_inventory_member,
};
use super::lifecycle_assertions::{
    assert_every_family_requires_builder_initialization,
    assert_every_family_requires_diagnostics_aggregation,
    assert_every_family_requires_snapshot_freeze,
};

#[test]
fn new_registry_family_requires_builder_initialization_update() {
    let app = empty_app();
    let freeze_report = app.capabilities().freeze_report();

    assert_every_family_requires_builder_initialization();
    assert_freeze_report_matches_registry_inventory(freeze_report);
    assert_empty_report_widths_match_inventory(freeze_report);
}

#[test]
fn new_registry_family_requires_snapshot_freeze_update() {
    let app = single_command_app();
    let freeze_report = app.capabilities().freeze_report();

    assert_every_family_requires_snapshot_freeze();
    assert_freeze_report_matches_registry_inventory(freeze_report);
    assert_single_command_report_widths_match_inventory(freeze_report);
}

#[test]
fn new_registry_family_requires_diagnostics_aggregation_update() {
    let report = duplicate_representative_family_registration_report();
    let diagnostics = report.registration_diagnostics();

    assert_every_family_requires_diagnostics_aggregation();
    assert_eq!(diagnostics.len(), 8);
    assert_eq!(
        duplicate_diagnostic_counts_by_family(diagnostics),
        BTreeMap::from([
            (RegistryFamily::Command, 2),
            (RegistryFamily::Component, 2),
            (RegistryFamily::Setting, 2),
            (RegistryFamily::TaskPresentation, 2),
        ])
    );
    for diagnostic in diagnostics {
        assert_eq!(
            diagnostic.code(),
            CapabilityDiagnosticCode::DuplicateCapabilityId
        );
        assert!(diagnostic.family_name().is_some());
        assert!(
            RegistryFamily::from_name(diagnostic.family_name().expect("diagnostic family"))
                .is_some()
        );
    }
}

#[test]
fn new_registry_family_requires_facade_exposure_decision() {
    assert_registry_inventory_names_are_unique();
    assert_registry_family_names_round_trip_through_facade_inventory();
    assert_every_family_has_facade_exposure_decision();
}

#[test]
fn unknown_or_omitted_registry_family_reported() {
    let app = empty_app();
    let freeze_report = app.capabilities().freeze_report();

    assert_unknown_family_name_is_not_inventory_member("registry_family.experimental");
    assert_inventory_audit_reports_unknown_and_omitted_families();
    assert_inventory_audit_reports_duplicate_families();
    assert_freeze_report_matches_registry_inventory(freeze_report);
}

fn duplicate_diagnostic_counts_by_family(
    diagnostics: &[worth_ui::facade::CapabilityRegistrationDiagnostic],
) -> BTreeMap<RegistryFamily, usize> {
    let mut counts = BTreeMap::new();
    for diagnostic in diagnostics {
        let registry_family =
            RegistryFamily::from_name(diagnostic.family_name().expect("diagnostic family"))
                .expect("known diagnostic family");
        *counts.entry(registry_family).or_insert(0) += 1;
    }

    counts
}
