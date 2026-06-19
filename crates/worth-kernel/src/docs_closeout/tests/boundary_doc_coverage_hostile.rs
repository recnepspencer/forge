use crate::docs_closeout::{
    current_worth_boundary_doc_coverage_matrix, worth_boundary_doc_coverage_matrix_for_root,
    WorthBoundaryDocCoverageStatus,
};

use super::hostile_workspace::WorthDocsTestWorkspace;

#[test]
fn boundary_doc_coverage_matrix_exposes_clean_evidence_on_green_rows() {
    let matrix = current_worth_boundary_doc_coverage_matrix()
        .expect("boundary doc coverage matrix should build");
    let row = matrix
        .rows()
        .iter()
        .find(|row| row.boundary_id() == "topo-query-runtime-boundary")
        .expect("topo boundary row should exist");

    assert_eq!(row.crate_name(), "worth-topo");
    assert_eq!(row.status(), WorthBoundaryDocCoverageStatus::Satisfied);
    assert_eq!(
        row.reason(),
        "boundary doc teaches the owning handoff explicitly"
    );
    assert_eq!(row.evidence().ownership_count(), Some(1));
    assert_eq!(row.evidence().actual_relative_path(), None);
    assert!(row.evidence().missing_headings().is_empty());
}

#[test]
fn boundary_doc_coverage_matrix_blocks_duplicate_boundary_ownership() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.copy_file(
        "crates/worth-topo/docs/boundaries/topo-query-runtime-boundary.md",
        "crates/worth-kernel/docs/boundaries/topo-query-runtime-boundary-duplicate.md",
    );

    let matrix = worth_boundary_doc_coverage_matrix_for_root(workspace.root())
        .expect("boundary coverage matrix should build");
    let row = matrix
        .rows()
        .iter()
        .find(|row| row.boundary_id() == "topo-query-runtime-boundary")
        .expect("topo boundary row should exist");

    assert_eq!(row.status(), WorthBoundaryDocCoverageStatus::Blocked);
    assert_eq!(row.evidence().ownership_count(), Some(2));
}

#[test]
fn boundary_doc_coverage_matrix_blocks_relative_path_drift() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.rename_path(
        "crates/worth-topo/docs/boundaries/topo-query-runtime-boundary.md",
        "crates/worth-topo/docs/boundaries/topo-query-runtime-boundary-drifted.md",
    );

    let matrix = worth_boundary_doc_coverage_matrix_for_root(workspace.root())
        .expect("boundary coverage matrix should build");
    let row = matrix
        .rows()
        .iter()
        .find(|row| row.boundary_id() == "topo-query-runtime-boundary")
        .expect("topo boundary row should exist");

    assert_eq!(row.status(), WorthBoundaryDocCoverageStatus::Blocked);
    assert_eq!(
        row.evidence().actual_relative_path(),
        Some("boundaries/topo-query-runtime-boundary-drifted.md")
    );
}
