use crate::docs_closeout::{
    current_worth_docs_backfill_report, worth_docs_backfill_report_for_root,
    WorthDocsBackfillStatus,
};

use super::hostile_workspace::WorthDocsTestWorkspace;

#[test]
fn docs_backfill_report_stays_honest_about_pre_milestone_foundations() {
    let report = current_worth_docs_backfill_report().expect("docs backfill report should build");

    let expected_surfaces = [
        "topology-graph-authority",
        "topology-certification-and-parity",
        "topology-workloads-and-seeds",
        "topo-query-runtime-boundary",
        "analytic-primitives-and-planes",
        "curve-and-surface-schema",
        "spatial-acceleration-and-matching",
        "boundary-certification-and-intersection",
        "primitive-realization-strategies",
        "geom-to-spatial-authority-boundary",
    ];

    assert_eq!(report.rows().len(), expected_surfaces.len());
    assert!(!report.report_digest().is_empty());

    let actual_surfaces = report
        .rows()
        .iter()
        .map(|row| row.surface_name())
        .collect::<Vec<_>>();
    assert_eq!(actual_surfaces, expected_surfaces);

    for row in report.rows() {
        assert_eq!(row.status(), WorthDocsBackfillStatus::Satisfied);
    }
}

#[test]
fn docs_backfill_report_blocks_missing_required_jump_link() {
    let workspace = WorthDocsTestWorkspace::new();
    workspace.remove_line(
        "crates/worth-topo/docs/features/topology-graph-authority.md",
        "./runtime-support.md",
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
        row.evidence().missing_markdown_fragments(),
        &["./runtime-support.md".to_string()]
    );
}
