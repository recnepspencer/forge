use std::path::{Path, PathBuf};

use worth_query::facade::consumer_kit::{
    hard_prohibition_boundary_audit, query_boundary_source_inventory, query_consumer_residue_audit,
};

#[test]
fn product_backend_support_rows_are_query_owned_and_digest_pinned() {
    let report =
        worth_ui_query_binding::certification::certify_product_projection_support_contract()
            .expect("the production projection backend must satisfy its Query support pins");

    assert!(report.satisfied());
    assert_eq!(report.requirement_count(), 5);
    assert_eq!(report.matched_required_count(), 5);
    assert_eq!(report.blocking_finding_count(), 0);
    assert!(!report.contract_digest().is_empty());
    assert!(!report.report_digest().is_empty());
}

#[test]
fn query_owned_prohibitions_cover_all_projection_consumer_sources() {
    let roots = projection_authority_roots();
    let inventory = roots.iter().fold(
        query_boundary_source_inventory("worth-ui-projection-consumers"),
        |inventory, root| inventory.required_root(root),
    );
    let inventory = inventory
        .include_rs_files()
        .seal()
        .expect("production projection source inventory must be readable");
    let report = hard_prohibition_boundary_audit()
        .covering_sources(inventory.boundary_sources())
        .try_assert_clean()
        .expect("production projection consumers must avoid Query hard prohibitions");

    assert_eq!(report.source_labels().len(), inventory.source_count());
    assert!(!report.source_labels().is_empty());
    assert!(!inventory
        .inventory_identity()
        .terminal_projection_for_reporting()
        .is_empty());
    assert!(!report
        .report_identity()
        .terminal_projection_for_reporting()
        .is_empty());
}

#[test]
fn query_consumer_residue_report_covers_the_same_production_roots() {
    let roots = projection_downstream_roots();
    let report = roots
        .iter()
        .fold(
            query_consumer_residue_audit("worth-ui-projection-consumers"),
            |audit, root| audit.required_root(root),
        )
        .evaluate()
        .expect("Query consumer residue audit must parse production roots");

    report.assert_clean();
    assert_eq!(report.audited_roots().len(), roots.len());
    assert!(report.scanned_file_count() > 0);
    assert_eq!(
        report.scanned_file_count(),
        report.source_inventory().audited_source_count()
    );
    assert!(!report.source_inventory_digest().is_empty());
    assert!(!report
        .report_identity()
        .terminal_projection_for_reporting()
        .is_empty());
}

fn projection_authority_roots() -> [PathBuf; 6] {
    let workspace = worth_ui_workspace_root();
    [
        workspace.join("apps/platform-pulse/src"),
        workspace.join("crates/worth-ui/src"),
        workspace.join("crates/worth-ui-host-contract/src"),
        workspace.join("crates/worth-ui-inspection/src"),
        workspace.join("crates/worth-ui-query-binding/src"),
        workspace.join("crates/worth-ui-runtime/src"),
    ]
}

fn projection_downstream_roots() -> [PathBuf; 5] {
    let workspace = worth_ui_workspace_root();
    [
        workspace.join("apps/platform-pulse/src"),
        workspace.join("crates/worth-ui/src"),
        workspace.join("crates/worth-ui-host-contract/src"),
        workspace.join("crates/worth-ui-inspection/src"),
        workspace.join("crates/worth-ui-runtime/src"),
    ]
}

fn worth_ui_workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("certification crate must remain under the Worth UI workspace")
        .to_path_buf()
}
