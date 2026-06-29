use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::firewall_bound_closeout::closeout_from_products;
use super::{
    current_worth_touched_graph_conflict_deletion_closeout,
    WorthTouchedGraphConflictDeletionCloseoutErrorKind,
    WorthTouchedGraphConflictDeletionDisposition,
};
use crate::workload_composition::current_conflict_batch_admission_inventory;
use crate::workload_composition::source_firewall::scan_worth_touched_graph_conflict_source_firewall_region_for_tests;

#[test]
fn deletion_closeout_binds_firewall_report_to_named_phase_twelve_rows() {
    let closeout = current_worth_touched_graph_conflict_deletion_closeout()
        .expect("phase 12 deletion closeout should bind current products");

    assert!(!closeout.inventory_digest().is_empty());
    assert!(!closeout.source_firewall_report_digest().is_empty());
    assert!(!closeout.closeout_digest().is_empty());
    assert!(!closeout.deletion_ledger().rows().is_empty());
    assert_ledger_row(
        &closeout,
        "crates/worth-kernel/src/workload_composition/conflict_batch_admission_inventory/source_firewall.rs",
        "ConflictBatchAdmissionSourceFirewallReport",
        WorthTouchedGraphConflictDeletionDisposition::DeletedAuthority,
    );
    assert_ledger_row(
        &closeout,
        "crates/worth-topo/src/derived_topology/invalidation_plan/migrated_products/traversal_views/old_authority_residue.rs",
        "TraversalViewsOldAuthorityResidue",
        WorthTouchedGraphConflictDeletionDisposition::CappedResidue,
    );
    assert_ledger_row(
        &closeout,
        "crates/worth-spatial/src/workload_platform/evidence_lookup_source_firewall/counters.rs",
        "EvidenceLookupSourceFirewallCounters::broad_receipt_scan_row_count",
        WorthTouchedGraphConflictDeletionDisposition::CappedResidue,
    );
}

#[test]
fn deletion_closeout_rejects_source_firewall_relapse() {
    let inventory = current_conflict_batch_admission_inventory()
        .expect("phase 12 inventory should load for synthetic relapse");
    let workspace = temp_dir("tgc-deletion-closeout");
    let hostile_path = workspace.join(
        "crates/worth-spatial/src/workload_platform/projected_overlap_faces/certified_pair.rs",
    );
    fs::create_dir_all(hostile_path.parent().expect("hostile parent"))
        .expect("create hostile parent");
    fs::write(hostile_path, "fn neutral_relapse_gate() {}\n").expect("write hostile source");
    let firewall_report = scan_worth_touched_graph_conflict_source_firewall_region_for_tests(
        "synthetic_region",
        "synthetic:root",
        &workspace,
    )
    .expect("synthetic region should scan");
    assert!(firewall_report.violations().iter().any(|row| {
        row.region_label() == "synthetic_region"
            && row.source_path().ends_with(
                "crates/worth-spatial/src/workload_platform/projected_overlap_faces/certified_pair.rs",
            )
    }));

    let error = closeout_from_products(&inventory, &firewall_report)
        .expect_err("relapse should fail phase 12 deletion closeout");
    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictDeletionCloseoutErrorKind::SourceFirewallViolation
    );

    let _ = fs::remove_dir_all(workspace);
}

fn temp_dir(prefix: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("{prefix}-{stamp}"));
    fs::create_dir_all(&path).expect("create temp dir");
    path
}

fn assert_ledger_row(
    closeout: &super::firewall_bound_closeout::WorthTouchedGraphConflictDeletionCloseout,
    expected_path: &str,
    expected_surface: &str,
    expected_disposition: WorthTouchedGraphConflictDeletionDisposition,
) {
    assert!(
        closeout.deletion_ledger().rows().iter().any(|row| {
            row.source_path() == expected_path
                && row.surface_name() == expected_surface
                && row.disposition() == expected_disposition
        }),
        "missing deletion ledger row `{expected_path}` `{expected_surface}` `{expected_disposition:?}`"
    );
}
