use std::collections::BTreeSet;

use worth_ui::facade::{RegistryFamily, RegistryFamilyInventoryAudit, SnapshotFreezeReport};

pub(crate) fn assert_registry_inventory_names_are_unique() {
    let inventory_names = registry_inventory_names();

    assert_eq!(inventory_names.len(), RegistryFamily::all().len());
}

pub(crate) fn assert_freeze_report_matches_registry_inventory(report: &SnapshotFreezeReport) {
    assert_eq!(report.families().len(), RegistryFamily::all().len());
    assert_eq!(reported_family_names(report), registry_inventory_names());
    assert!(report.has_complete_registry_family_inventory());
    assert!(report.omitted_registry_families().is_empty());
}

pub(crate) fn assert_unknown_family_name_is_not_inventory_member(family_name: &str) {
    assert_eq!(RegistryFamily::from_name(family_name), None);
}

pub(crate) fn assert_inventory_audit_reports_unknown_and_omitted_families() {
    let reported_names = RegistryFamily::all()
        .iter()
        .copied()
        .filter(|registry_family| *registry_family != RegistryFamily::ThemeToken)
        .map(|registry_family| registry_family.name())
        .chain(["registry_family.experimental"])
        .collect::<Vec<_>>();
    let audit = RegistryFamilyInventoryAudit::from_reported_family_names(reported_names);

    assert!(!audit.is_complete());
    assert_eq!(audit.omitted_families(), &[RegistryFamily::ThemeToken]);
    assert_eq!(
        audit.unknown_family_names(),
        &["registry_family.experimental".to_owned()]
    );
    assert!(audit.duplicate_family_names().is_empty());
}

pub(crate) fn assert_inventory_audit_reports_duplicate_families() {
    let reported_names = RegistryFamily::all()
        .iter()
        .map(|registry_family| registry_family.name())
        .chain([
            RegistryFamily::Command.name(),
            RegistryFamily::Command.name(),
        ])
        .collect::<Vec<_>>();
    let audit = RegistryFamilyInventoryAudit::from_reported_family_names(reported_names);

    assert!(!audit.is_complete());
    assert!(audit.omitted_families().is_empty());
    assert!(audit.unknown_family_names().is_empty());
    assert_eq!(
        audit.duplicate_family_names(),
        &[RegistryFamily::Command.name().to_owned()]
    );
}

pub(crate) fn assert_empty_report_widths_match_inventory(report: &SnapshotFreezeReport) {
    for registry_family in RegistryFamily::all() {
        assert_eq!(report.registry_family_width(*registry_family), Some(0));
    }
}

pub(crate) fn assert_single_command_report_widths_match_inventory(report: &SnapshotFreezeReport) {
    for registry_family in RegistryFamily::all() {
        let expected_width = if *registry_family == RegistryFamily::Command {
            1
        } else {
            0
        };
        assert_eq!(
            report.registry_family_width(*registry_family),
            Some(expected_width)
        );
    }
}

fn registry_inventory_names() -> BTreeSet<&'static str> {
    RegistryFamily::all()
        .iter()
        .map(|registry_family| registry_family.name())
        .collect()
}

fn reported_family_names(report: &SnapshotFreezeReport) -> BTreeSet<&'static str> {
    report
        .families()
        .iter()
        .map(|family| family.family_name())
        .collect()
}
