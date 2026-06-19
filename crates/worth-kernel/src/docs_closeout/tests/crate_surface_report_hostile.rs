use crate::docs_closeout::{
    current_worth_crate_docs_surface_report, worth_crate_docs_surface_report_for_root,
    WorthCrateDocsSurfaceStatus,
};

use super::hostile_workspace::WorthDocsTestWorkspace;

#[test]
fn crate_docs_surface_report_exposes_clean_evidence_on_green_rows() {
    let report =
        current_worth_crate_docs_surface_report().expect("crate docs surface report should build");
    let row = report
        .rows()
        .iter()
        .find(|row| row.crate_name() == "worth-topo")
        .expect("worth-topo row should exist");

    assert_eq!(row.status(), WorthCrateDocsSurfaceStatus::Satisfied);
    assert_eq!(
        row.reason(),
        "crate README, categories, and reader graph are machine-checkable"
    );
    assert!(row.evidence().missing_headings().is_empty());
    assert!(row.evidence().missing_metadata_entries().is_empty());
    assert!(row.evidence().missing_readme_fragments().is_empty());
    assert!(row.evidence().missing_directories().is_empty());
}

#[test]
fn crate_docs_surface_report_blocks_doc_style_metadata_drift() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.replace_once(
        "crates/worth-topo/docs/README.md",
        "doc_style: authority-first",
        "doc_style: workflow-first",
    );

    let report = worth_crate_docs_surface_report_for_root(workspace.root())
        .expect("crate docs surface report should build");
    let row = report
        .rows()
        .iter()
        .find(|row| row.crate_name() == "worth-topo")
        .expect("worth-topo row should exist");

    assert_eq!(row.status(), WorthCrateDocsSurfaceStatus::Blocked);
    assert!(row
        .evidence()
        .missing_metadata_entries()
        .contains(&"doc_style=authority-first".to_string()));
}

#[test]
fn crate_docs_surface_report_blocks_missing_reader_graph_links() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.remove_line("crates/worth-topo/docs/README.md", "./features/");

    let report = worth_crate_docs_surface_report_for_root(workspace.root())
        .expect("crate docs surface report should build");
    let row = report
        .rows()
        .iter()
        .find(|row| row.crate_name() == "worth-topo")
        .expect("worth-topo row should exist");

    assert_eq!(row.status(), WorthCrateDocsSurfaceStatus::Blocked);
    assert!(row
        .evidence()
        .missing_readme_fragments()
        .contains(&"./features/".to_string()));
}

#[test]
fn crate_docs_surface_report_blocks_neighbor_metadata_drift() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.replace_once(
        "crates/worth-topo/docs/README.md",
        "neighbor_crates: worth-kernel, worth-spatial, worth-geom, forge-query",
        "neighbor_crates: worth-kernel, worth-spatial, forge-query",
    );

    let report = worth_crate_docs_surface_report_for_root(workspace.root())
        .expect("crate docs surface report should build");
    let row = report
        .rows()
        .iter()
        .find(|row| row.crate_name() == "worth-topo")
        .expect("worth-topo row should exist");

    assert_eq!(row.status(), WorthCrateDocsSurfaceStatus::Blocked);
    assert!(row
        .evidence()
        .missing_metadata_entries()
        .contains(&"neighbor:worth-geom".to_string()));
}
