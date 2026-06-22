use std::path::PathBuf;

use forge_query::facade::consumer_kit::query_test_backend_residue_audit;

const AUDITED_DOWNSTREAM_ROOTS: &[&str] =
    &["worth-kernel/src/construction", "hadwiger-research/src"];

#[test]
fn in_memory_test_backend_downstream_adoption_has_no_adapter_or_receipt_residue() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_crates_dir = manifest_dir
        .parent()
        .expect("forge-query crate should live under crates");
    let audit = AUDITED_DOWNSTREAM_ROOTS.iter().fold(
        query_test_backend_residue_audit("downstream-query-consumers"),
        |audit, root| audit.required_root(workspace_crates_dir.join(root)),
    );

    let report = audit
        .evaluate()
        .expect("all audited downstream roots must exist and be readable");

    assert_eq!(report.audited_roots().len(), AUDITED_DOWNSTREAM_ROOTS.len());
    assert!(report.scanned_file_count() > 0);
    assert!(
        !report
            .report_identity()
            .terminal_projection_for_reporting()
            .is_empty(),
        "residue audit must publish a canonical report identity"
    );
    report.assert_clean();
}
