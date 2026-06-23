use crate::docs_closeout::{
    current_worth_feature_doc_coverage_matrix, worth_feature_doc_coverage_matrix_for_root,
    WorthFeatureDocCoverageStatus,
};

use super::hostile_workspace::WorthDocsTestWorkspace;

#[test]
fn feature_doc_coverage_matrix_exposes_clean_evidence_on_green_rows() {
    let matrix = current_worth_feature_doc_coverage_matrix()
        .expect("feature doc coverage matrix should build");
    let row = matrix
        .rows()
        .iter()
        .find(|row| row.feature_id() == "primitive-construction")
        .expect("primitive construction row should exist");

    assert_eq!(row.crate_name(), "worth-kernel");
    assert_eq!(row.status(), WorthFeatureDocCoverageStatus::Satisfied);
    assert_eq!(
        row.reason(),
        "feature doc owns one shipped surface with explicit workflow headings"
    );
    assert_eq!(row.evidence().ownership_count(), Some(1));
    assert_eq!(row.evidence().actual_relative_path(), None);
    assert!(row.evidence().missing_headings().is_empty());
    assert!(row.evidence().missing_markdown_fragments().is_empty());
}

#[test]
fn feature_doc_coverage_matrix_blocks_duplicate_feature_ownership() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.copy_file(
        "crates/worth-kernel/docs/features/primitive-construction.md",
        "crates/worth-spatial/docs/features/primitive-construction-duplicate.md",
    );

    let matrix = worth_feature_doc_coverage_matrix_for_root(workspace.root())
        .expect("feature coverage matrix should build");
    let row = matrix
        .rows()
        .iter()
        .find(|row| row.feature_id() == "primitive-construction")
        .expect("primitive construction row should exist");

    assert_eq!(row.status(), WorthFeatureDocCoverageStatus::Blocked);
    assert_eq!(row.evidence().ownership_count(), Some(2));
}

#[test]
fn feature_doc_coverage_matrix_blocks_relative_path_drift() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.rename_path(
        "crates/worth-kernel/docs/features/primitive-construction.md",
        "crates/worth-kernel/docs/features/primitive-construction-drifted.md",
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
        row.evidence().actual_relative_path(),
        Some("features/primitive-construction-drifted.md")
    );
}

#[test]
fn feature_doc_coverage_matrix_blocks_query_proof_docs_missing_required_fragments() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.replace_once(
        "crates/worth-kernel/docs/features/primitive-construction.md",
        "query_proof_required: false",
        "query_proof_required: true",
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
        row.evidence().missing_markdown_fragments(),
        &[
            "evidence-report".to_string(),
            "hard-prohibition".to_string(),
            "support pin".to_string(),
        ]
    );
}
