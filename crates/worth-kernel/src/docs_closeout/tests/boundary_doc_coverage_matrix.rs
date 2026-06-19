use crate::docs_closeout::{
    current_worth_boundary_doc_coverage_matrix, worth_boundary_doc_coverage_matrix_for_root,
    WorthBoundaryDocCoverageStatus,
};

use super::hostile_workspace::WorthDocsTestWorkspace;

#[test]
fn boundary_doc_coverage_matrix_keeps_one_doc_per_handoff_surface() {
    let matrix = current_worth_boundary_doc_coverage_matrix()
        .expect("boundary doc coverage matrix should build");

    assert_eq!(matrix.rows().len(), 6);
    assert!(!matrix.coverage_matrix_digest().is_empty());
    for row in matrix.rows() {
        assert_eq!(row.status(), WorthBoundaryDocCoverageStatus::Satisfied);
    }
}

#[test]
fn boundary_doc_coverage_matrix_blocks_missing_query_usage_heading() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.remove_line(
        "crates/worth-topo/docs/boundaries/topo-query-runtime-boundary.md",
        "## Query Usage",
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
        row.evidence().missing_headings(),
        &["Query Usage".to_string()]
    );
}
