use super::fixture_tree::SourceFirewallFixtureTree;
use crate::workload_platform::evidence_lookup_source_firewall::{
    source_firewall_report_for_snapshot_root, EvidenceLookupForbiddenAuthorityKind,
    EvidenceLookupSourceFirewallExceptionKind, EvidenceLookupSourceFirewallOutcome,
    EvidenceLookupSourceFirewallRowPosture,
};

#[test]
fn firewall_exceptions_are_named_non_authoritative_codecs() {
    let fixture_tree = SourceFirewallFixtureTree::new();
    write_required_snapshot_roots(&fixture_tree);
    fixture_tree.write_file(
        "crates/worth-spatial/src/certification/workload_evidence.rs",
        "ledger.require_boolean_receipt_lookup(receipt)?; WorkloadEvidenceRow::new(stage, row.evidence_identity())",
    );
    fixture_tree.write_file(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/surface_inventory/rows.rs",
        "let _ = \"WorkloadEvidenceRow::new\";",
    );
    fixture_tree.write_file(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission/test_support/fixture.rs",
        "fn copied_query_digest_fixture() { let _ = query_descriptor_digest; let _ = lookup_authority; }",
    );

    let report = source_firewall_report_for_snapshot_root(fixture_tree.root())
        .expect("firewall report from fixture tree");

    assert_eq!(
        report.outcome(),
        EvidenceLookupSourceFirewallOutcome::ExceptionsOnly
    );
    assert_eq!(report.counters().forbidden_row_count(), 0);
    assert_eq!(
        report.counters().allowed_exception_row_count(),
        report.rows().len()
    );
    assert!(report.counters().certification_only_exception_row_count() > 0);
    assert!(report.counters().documentation_report_exception_row_count() > 0);
    assert!(report.counters().test_support_exception_row_count() > 0);
    assert!(report.exception_summaries().iter().any(|summary| {
        summary.kind() == EvidenceLookupSourceFirewallExceptionKind::CertificationOnlyCodec
            && summary.row_count() == report.counters().certification_only_exception_row_count()
    }));
    assert!(report.exception_summaries().iter().any(|summary| {
        summary.kind() == EvidenceLookupSourceFirewallExceptionKind::DocumentationReportCodec
            && summary.row_count() == report.counters().documentation_report_exception_row_count()
    }));
    assert!(report.exception_summaries().iter().any(|summary| {
        summary.kind() == EvidenceLookupSourceFirewallExceptionKind::TestSupportFixture
            && summary.row_count() == report.counters().test_support_exception_row_count()
    }));
    assert!(report.rows().iter().all(|row| {
        row.posture() == EvidenceLookupSourceFirewallRowPosture::AllowedNamedException
            && !row.claims_lookup_execution_authority()
    }));
    assert_exception_kind(
        &report,
        EvidenceLookupForbiddenAuthorityKind::BroadReceiptScan,
        EvidenceLookupSourceFirewallExceptionKind::CertificationOnlyCodec,
    );
    assert_exception_kind(
        &report,
        EvidenceLookupForbiddenAuthorityKind::RawEvidenceVectorAccess,
        EvidenceLookupSourceFirewallExceptionKind::DocumentationReportCodec,
    );
    assert_exception_kind(
        &report,
        EvidenceLookupForbiddenAuthorityKind::CopiedDigestLookup,
        EvidenceLookupSourceFirewallExceptionKind::TestSupportFixture,
    );
}

fn write_required_snapshot_roots(fixture_tree: &SourceFirewallFixtureTree) {
    for relative_path in [
        "crates/worth-spatial/src/facade/workload_vocabulary/mod.rs",
        "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs",
        "crates/worth-spatial/src/workload_platform/evidence_ledger/stage_index/product.rs",
        "crates/worth-spatial/src/workload_platform/evidence_ledger/row.rs",
        "crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission/lookup.rs",
        "crates/worth-spatial/src/workload_platform/evidence_ledger/surface_inventory/rows.rs",
        "crates/worth-spatial/src/certification/workload_evidence.rs",
        "crates/worth-spatial/src/query_adoption.rs",
        "crates/worth-kernel/src/workload_composition/worth_workload.rs",
    ] {
        fixture_tree.write_file(relative_path, "");
    }
}

fn assert_exception_kind(
    report: &crate::workload_platform::evidence_lookup_source_firewall::EvidenceLookupSourceFirewallReport,
    authority_kind: EvidenceLookupForbiddenAuthorityKind,
    exception_kind: EvidenceLookupSourceFirewallExceptionKind,
) {
    assert!(report.rows().iter().any(|row| {
        row.forbidden_authority_kind() == authority_kind
            && row.exception_kind() == Some(exception_kind)
    }));
}
