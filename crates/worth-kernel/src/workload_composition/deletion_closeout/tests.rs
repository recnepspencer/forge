use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::firewall_bound_closeout::closeout_from_products;
use super::ledger::expected_deletion_ledger_rows;
use super::{
    current_worth_touched_graph_conflict_deletion_closeout,
    WorthTouchedGraphConflictDeletionCloseoutErrorKind,
};
use crate::workload_composition::current_conflict_batch_admission_inventory;
use crate::workload_composition::source_firewall::scan_worth_touched_graph_conflict_source_firewall_region_for_tests;

#[test]
fn deletion_closeout_binds_firewall_report_to_exact_phase_fifteen_rows() {
    let closeout = current_worth_touched_graph_conflict_deletion_closeout()
        .expect("phase 15 deletion closeout should bind current products");
    let inventory =
        current_conflict_batch_admission_inventory().expect("phase 15 inventory should load");
    let mut expected_rows = expected_deletion_ledger_rows(&inventory)
        .expect("phase 15 expected deletion rows should lower from production authority");
    expected_rows.sort_by(|left, right| {
        left.source_path()
            .cmp(right.source_path())
            .then(left.surface_name().cmp(right.surface_name()))
    });
    let actual_rows = closeout.deletion_ledger().rows().to_vec();

    assert!(!closeout.inventory_digest().is_empty());
    assert!(!closeout.source_firewall_report_digest().is_empty());
    assert!(!closeout.closeout_digest().is_empty());
    assert_eq!(actual_rows, expected_rows);
}

#[test]
fn deletion_closeout_rejects_source_firewall_relapse() {
    let inventory = current_conflict_batch_admission_inventory()
        .expect("phase 15 inventory should load for synthetic relapse");
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
        .expect_err("relapse should fail phase 15 deletion closeout");
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
