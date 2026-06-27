use super::super::{
    covered_evidence_lookup_surfaces, evidence_lookup_discovered_surface_report_for_roots,
    fixture_surface, validate_discovered_evidence_lookup_surfaces, EvidenceLookupDiscoveryScanRoot,
    EvidenceLookupInventoryErrorKind, EvidenceLookupInventoryRowScope,
};

#[test]
fn fixture_root_unclassified_lookup_surface_fails_closeout() {
    let covered = covered_evidence_lookup_surfaces();
    let report = evidence_lookup_discovered_surface_report_for_roots(&[
        EvidenceLookupDiscoveryScanRoot::production(
            "crates/worth-spatial/fixtures/evidence_lookup_inventory/unclassified_lookup_surface.rs",
        ),
    ]);

    let error = validate_discovered_evidence_lookup_surfaces(report.surfaces(), &covered)
        .expect_err("unclassified production lookup-shaped file is denied");
    assert_eq!(
        error.kind(),
        EvidenceLookupInventoryErrorKind::UnclassifiedEvidenceLookupSurface
    );
}

#[test]
fn fixture_root_classified_lookup_surface_passes_with_concrete_catalog_row() {
    let source_path =
        "crates/worth-spatial/fixtures/evidence_lookup_inventory/classified_lookup_surface.rs";
    let covered = vec![fixture_surface(
        source_path,
        EvidenceLookupInventoryRowScope::ConcreteSource,
    )];
    let report = evidence_lookup_discovered_surface_report_for_roots(&[
        EvidenceLookupDiscoveryScanRoot::production(source_path).classified_as(source_path),
    ]);

    let guard = validate_discovered_evidence_lookup_surfaces(report.surfaces(), &covered)
        .expect("concrete source row covers concrete discovered lookup file");
    assert_eq!(guard.discovered_surface_count(), 1);
    assert_eq!(guard.covered_surface_count(), 1);
}

#[test]
fn fixture_test_support_lookup_surface_reports_test_support_error() {
    let covered = covered_evidence_lookup_surfaces();
    let report = evidence_lookup_discovered_surface_report_for_roots(&[
        EvidenceLookupDiscoveryScanRoot::test_support(
            "crates/worth-spatial/fixtures/evidence_lookup_inventory/unclassified_lookup_surface.rs",
        ),
    ]);

    let error = validate_discovered_evidence_lookup_surfaces(report.surfaces(), &covered)
        .expect_err("unclassified test-support lookup-shaped file is denied separately");
    assert_eq!(
        error.kind(),
        EvidenceLookupInventoryErrorKind::ProductionShapedTestSupportUnclassified
    );
}

#[test]
fn directory_family_summary_cannot_cover_concrete_fixture_sources() {
    let covered = vec![fixture_surface(
        "crates/worth-spatial/fixtures/evidence_lookup_inventory/family",
        EvidenceLookupInventoryRowScope::FamilySummary,
    )];
    let report = evidence_lookup_discovered_surface_report_for_roots(&[
        EvidenceLookupDiscoveryScanRoot::production(
            "crates/worth-spatial/fixtures/evidence_lookup_inventory/family",
        ),
    ]);

    let error = validate_discovered_evidence_lookup_surfaces(report.surfaces(), &covered)
        .expect_err("family summary must not satisfy concrete file coverage");
    assert_eq!(
        error.kind(),
        EvidenceLookupInventoryErrorKind::UnclassifiedEvidenceLookupSurface
    );
}
