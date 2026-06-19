use crate::docs_closeout::{
    current_worth_crate_docs_surface_report, worth_crate_docs_surface_report_for_root,
    WorthCrateDocsSurfaceStatus,
};

use super::hostile_workspace::WorthDocsTestWorkspace;

#[test]
fn crate_docs_surface_report_closes_touched_crate_entrypoints() {
    let report =
        current_worth_crate_docs_surface_report().expect("crate docs surface report should build");

    assert_eq!(report.rows().len(), 4);
    assert!(!report.report_digest().is_empty());
    for row in report.rows() {
        assert_eq!(row.status(), WorthCrateDocsSurfaceStatus::Satisfied);
    }
}

#[test]
fn crate_docs_surface_report_blocks_missing_foundations_directory() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.remove_directory("crates/worth-geom/docs/foundations");

    let report = worth_crate_docs_surface_report_for_root(workspace.root())
        .expect("crate docs surface report should build");
    let row = report
        .rows()
        .iter()
        .find(|row| row.crate_name() == "worth-geom")
        .expect("worth-geom row should exist");

    assert_eq!(row.status(), WorthCrateDocsSurfaceStatus::Blocked);
    assert!(row
        .evidence()
        .missing_directories()
        .contains(&"foundations".to_string()));
}
