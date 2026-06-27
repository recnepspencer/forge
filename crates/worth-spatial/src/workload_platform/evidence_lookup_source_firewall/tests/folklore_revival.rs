use super::fixture_tree::SourceFirewallFixtureTree;
use crate::workload_platform::evidence_lookup_source_firewall::{
    source_firewall_report_for_snapshot_root, EvidenceLookupForbiddenAuthorityKind,
    EvidenceLookupSourceFirewallOutcome, EvidenceLookupSourceFirewallRowPosture,
};

#[test]
fn source_firewall_rejects_lookup_folklore_revival() {
    let fixture_tree = SourceFirewallFixtureTree::new();
    write_required_snapshot_roots(&fixture_tree);
    fixture_tree.write_file(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs",
        "fn revive_boolean_receipt_lookup() { let _ = require_boolean_receipt_lookup; let _ = row_for_stage; }",
    );
    fixture_tree.write_file(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/row.rs",
        "impl WorkloadEvidenceRow { pub fn new(stage: WorkloadEvidenceStage, identity: &str) -> Self { todo!() } }",
    );
    fixture_tree.write_file(
        "crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission/lookup.rs",
        "fn revive_digest_bridge() { let _ = query_descriptor_digest; let _ = lookup_authority; }",
    );
    fixture_tree.write_file(
        "crates/worth-spatial/src/query_adoption.rs",
        "fn revive_query_descriptor_bridge() { let _ = \"query descriptor\"; let _ = lookup_product_digest; }",
    );

    let report = source_firewall_report_for_snapshot_root(fixture_tree.root())
        .expect("firewall report from fixture tree");

    assert_eq!(
        report.outcome(),
        EvidenceLookupSourceFirewallOutcome::ForbiddenAuthorityPresent
    );
    assert!(report.covered_roots().contains(
        &"crates/worth-spatial/src/workload_platform/evidence_ledger/ledger.rs".to_string()
    ));
    assert_forbidden_kind(
        &report,
        EvidenceLookupForbiddenAuthorityKind::BroadReceiptScan,
    );
    assert_forbidden_kind(
        &report,
        EvidenceLookupForbiddenAuthorityKind::StageLocalNearbyLookup,
    );
    assert_forbidden_kind(
        &report,
        EvidenceLookupForbiddenAuthorityKind::CopiedDigestLookup,
    );
    assert_forbidden_kind(
        &report,
        EvidenceLookupForbiddenAuthorityKind::RawEvidenceVectorAccess,
    );
    assert_forbidden_kind(
        &report,
        EvidenceLookupForbiddenAuthorityKind::QueryLookupProductSubstitution,
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

fn assert_forbidden_kind(
    report: &crate::workload_platform::evidence_lookup_source_firewall::EvidenceLookupSourceFirewallReport,
    kind: EvidenceLookupForbiddenAuthorityKind,
) {
    assert!(report.rows().iter().any(|row| {
        row.forbidden_authority_kind() == kind
            && row.posture() == EvidenceLookupSourceFirewallRowPosture::ForbiddenProductionAuthority
    }));
}
