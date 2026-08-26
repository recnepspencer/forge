use super::*;
use crate::package::{
    WorthQueryPortableDefinition, WorthQueryPortableDomainIdentity, WorthQueryPortableDomainPackage,
};

#[test]
fn export_inventory_is_complete_and_canonical_for_root_families() {
    let canonical = fixture(false);
    let reordered = fixture(true);
    let left = canonical.export_typed_records().unwrap();
    let right = reordered.export_typed_records().unwrap();

    assert_eq!(left, right);
    assert_eq!(left.manifest().record_count(), 7);
    for family in [
        WorthQueryPortablePackageRecordFamily::DomainIdentity,
        WorthQueryPortablePackageRecordFamily::CapabilityRequirement,
        WorthQueryPortablePackageRecordFamily::ConfigurationRequirement,
        WorthQueryPortablePackageRecordFamily::OperatingRequirement,
        WorthQueryPortablePackageRecordFamily::ContributionPolicy,
    ] {
        assert_eq!(left.manifest().family_count(family), 1);
    }
    assert_eq!(
        left.manifest()
            .family_count(WorthQueryPortablePackageRecordFamily::Definition),
        2
    );
    assert!(left
        .views()
        .enumerate()
        .all(|(index, view)| view.canonical_index() as usize == index));
}

#[test]
fn source_closure_rejects_dropped_and_duplicated_records() {
    let package = fixture(false);
    let exported = package.export_typed_records().unwrap();

    let mut dropped = exported.records.clone();
    dropped.pop();
    assert_eq!(
        validate_source_closure(&package, &exported.manifest, &dropped)
            .unwrap_err()
            .kind(),
        DenialKind::IncompleteRecordClosure
    );

    let mut duplicated = exported.records.clone();
    duplicated.push(duplicated[0].clone());
    assert_eq!(
        validate_source_closure(&package, &exported.manifest, &duplicated)
            .unwrap_err()
            .kind(),
        DenialKind::IncompleteRecordClosure
    );
}

#[test]
fn each_export_budget_denies_before_returning_a_record_set() {
    let package = fixture(false);
    let exported = package.export_typed_records().unwrap();
    let record_denial = package
        .export_typed_records_with_limits(WorthQueryPortablePackageExportLimits::new(
            exported.manifest().record_count() - 1,
            u64::MAX,
        ))
        .unwrap_err();
    assert_eq!(record_denial.kind(), DenialKind::RecordCountExceeded);

    let byte_denial = package
        .export_typed_records_with_limits(WorthQueryPortablePackageExportLimits::new(
            u32::MAX,
            exported.manifest().logical_export_bytes() - 1,
        ))
        .unwrap_err();
    assert_eq!(byte_denial.kind(), DenialKind::LogicalExportBytesExceeded);
}

#[test]
fn one_requirement_mutation_changes_its_typed_record() {
    let left = fixture_with_capability("query-read")
        .export_typed_records()
        .unwrap();
    let right = fixture_with_capability("query-write")
        .export_typed_records()
        .unwrap();
    let capability = |set: &WorthQueryPortablePackageRecordSet| {
        set.records().iter().find_map(|record| match record {
            WorthQueryPortablePackageRecord::CapabilityRequirement(value) => {
                Some(value.as_str().to_owned())
            }
            _ => None,
        })
    };
    assert_eq!(capability(&left).as_deref(), Some("query-read"));
    assert_eq!(capability(&right).as_deref(), Some("query-write"));
}

fn fixture(reverse: bool) -> WorthQueryValidatedPortableDomainPackage {
    let definitions = [
        WorthQueryPortableDefinition::invariant("connected", "one-outgoing"),
        WorthQueryPortableDefinition::graph_read_operation("read", "direct-edge"),
    ];
    let mut package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "portable-export",
        1,
        0,
    ));
    if reverse {
        for definition in definitions.into_iter().rev() {
            package = package.definition(definition);
        }
    } else {
        for definition in definitions {
            package = package.definition(definition);
        }
    }
    package
        .requires_capability("query-read")
        .requires_configuration("query")
        .requires_operating_posture("bounded")
        .permits_contribution("query-index")
        .validate()
        .unwrap()
}

fn fixture_with_capability(capability: &str) -> WorthQueryValidatedPortableDomainPackage {
    WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "portable-export-mutation",
        1,
        0,
    ))
    .requires_capability(capability)
    .validate()
    .unwrap()
}
