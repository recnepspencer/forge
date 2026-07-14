use super::super::{hard_prohibition_boundary_audit, WorthQueryBoundaryAuditSourceSet};

#[test]
fn report_identity_changes_when_findings_change() {
    let write_report = hard_prohibition_boundary_audit()
        .covering_sources(
            WorthQueryBoundaryAuditSourceSet::new("worth-kernel").source(
                "construction.authoring",
                "fn dirty(workspace: &mut WorthQueryWorkspace) { workspace.write(command); }",
            ),
        )
        .evaluate()
        .expect("write source should parse");
    let batch_report = hard_prohibition_boundary_audit()
        .covering_sources(
            WorthQueryBoundaryAuditSourceSet::new("worth-kernel").source(
                "construction.authoring",
                "fn dirty(workspace: &mut WorthQueryWorkspace) { workspace.batch(commands); }",
            ),
        )
        .evaluate()
        .expect("batch source should parse");

    assert_ne!(
        write_report.report_identity(),
        batch_report.report_identity()
    );
    assert_ne!(
        write_report.finding_identities(),
        batch_report.finding_identities()
    );
}
