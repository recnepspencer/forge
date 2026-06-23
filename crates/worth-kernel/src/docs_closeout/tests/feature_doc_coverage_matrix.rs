use crate::docs_closeout::{
    current_worth_feature_doc_coverage_matrix, worth_feature_doc_coverage_matrix_for_root,
    WorthFeatureDocCoverageStatus,
};

use super::hostile_workspace::WorthDocsTestWorkspace;

#[test]
fn feature_doc_coverage_matrix_keeps_one_doc_per_shipped_feature() {
    let matrix = current_worth_feature_doc_coverage_matrix()
        .expect("feature doc coverage matrix should build");

    let expected_feature_ids = [
        "primitive-construction",
        "shell-with-hole-construction",
        "wire-body-construction",
        "construction-simulation",
        "construction-replay",
        "construction-results-and-diagnostics",
        "construction-time-birth-bindings",
        "birth-completeness-and-impossibility",
        "birth-truth-artifacts",
        "topology-graph-authority",
        "topology-certification-and-parity",
        "topology-workloads-and-seeds",
        "domain-reads",
        "runtime-support",
        "analytic-primitives-and-planes",
        "curve-and-surface-schema",
        "spatial-acceleration-and-matching",
        "boundary-certification-and-intersection",
        "primitive-realization-strategies",
    ];

    assert_eq!(matrix.rows().len(), expected_feature_ids.len());
    assert!(!matrix.coverage_matrix_digest().is_empty());

    let actual_feature_ids = matrix
        .rows()
        .iter()
        .map(|row| row.feature_id())
        .collect::<Vec<_>>();
    assert_eq!(actual_feature_ids, expected_feature_ids);

    for row in matrix.rows() {
        assert_eq!(row.status(), WorthFeatureDocCoverageStatus::Satisfied);
    }
}

#[test]
fn feature_doc_coverage_matrix_blocks_missing_query_integration_heading() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.remove_line(
        "crates/worth-kernel/docs/features/primitive-construction.md",
        "## Query Integration",
    );

    let matrix = worth_feature_doc_coverage_matrix_for_root(workspace.root())
        .expect("feature coverage matrix should build");
    let row = matrix
        .rows()
        .iter()
        .find(|row| row.feature_id() == "primitive-construction")
        .expect("primitive construction row should exist");

    assert_eq!(row.status(), WorthFeatureDocCoverageStatus::Blocked);
    assert_eq!(
        row.evidence().missing_headings(),
        &["Query Integration".to_string()]
    );
}
