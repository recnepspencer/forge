use crate::docs_closeout::{
    current_worth_docs_backfill_report, worth_docs_backfill_report_for_root,
    WorthDocsBackfillStatus,
};

use super::hostile_workspace::WorthDocsTestWorkspace;

#[test]
fn docs_backfill_report_exposes_clean_evidence_on_green_rows() {
    let report = current_worth_docs_backfill_report().expect("docs backfill report should build");
    let row = report
        .rows()
        .iter()
        .find(|row| row.surface_name() == "topology-graph-authority")
        .expect("topology graph authority row should exist");

    assert_eq!(row.status(), WorthDocsBackfillStatus::Satisfied);
    assert_eq!(
        row.reason(),
        "older public surface has one owning doc and README graph exposure"
    );
    assert_eq!(row.evidence().actual_relative_path(), None);
    assert!(row.evidence().missing_markdown_fragments().is_empty());
    assert!(row.evidence().missing_readme_fragments().is_empty());
    assert!(row.evidence().missing_headings().is_empty());
}

#[test]
fn docs_backfill_report_blocks_relative_path_drift() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.rename_path(
        "crates/worth-topo/docs/features/topology-graph-authority.md",
        "crates/worth-topo/docs/features/topology-graph-authority-drifted.md",
    );

    let report = worth_docs_backfill_report_for_root(workspace.root())
        .expect("backfill report should build");
    let row = report
        .rows()
        .iter()
        .find(|row| row.surface_name() == "topology-graph-authority")
        .expect("topology graph authority row should exist");

    assert_eq!(row.status(), WorthDocsBackfillStatus::Blocked);
    assert_eq!(
        row.evidence().actual_relative_path(),
        Some("features/topology-graph-authority-drifted.md")
    );
}

#[test]
fn docs_backfill_report_blocks_missing_readme_exposure() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.remove_line(
        "crates/worth-topo/docs/README.md",
        "./features/topology-graph-authority.md",
    );

    let report = worth_docs_backfill_report_for_root(workspace.root())
        .expect("backfill report should build");
    let row = report
        .rows()
        .iter()
        .find(|row| row.surface_name() == "topology-graph-authority")
        .expect("topology graph authority row should exist");

    assert_eq!(row.status(), WorthDocsBackfillStatus::Blocked);
    assert_eq!(
        row.evidence().missing_readme_fragments(),
        &["./features/topology-graph-authority.md".to_string()]
    );
}

#[test]
fn docs_backfill_report_blocks_missing_related_docs_heading() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.remove_line(
        "crates/worth-topo/docs/features/topology-graph-authority.md",
        "## Related Docs",
    );

    let report = worth_docs_backfill_report_for_root(workspace.root())
        .expect("backfill report should build");
    let row = report
        .rows()
        .iter()
        .find(|row| row.surface_name() == "topology-graph-authority")
        .expect("topology graph authority row should exist");

    assert_eq!(row.status(), WorthDocsBackfillStatus::Blocked);
    assert_eq!(
        row.evidence().missing_headings(),
        &["Related Docs".to_string()]
    );
}
