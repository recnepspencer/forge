use super::super::{
    hard_prohibition_boundary_audit, ForgeQueryBoundaryAuditCoverageMechanism,
    ForgeQueryBoundaryAuditSourceSet, ForgeQueryBoundaryAuditSyntaxClass,
};
use crate::ForgeQueryProhibitedSeam;

#[test]
fn detects_seeded_method_call_bypass_from_registry() {
    let source = "fn forged_workspace_write(workspace: &mut ForgeQueryWorkspace) {\n    workspace.write(command);\n}\n";
    let report = hard_prohibition_boundary_audit()
        .covering_sources(
            ForgeQueryBoundaryAuditSourceSet::new("worth-kernel").source_file(
                "construction.authoring",
                "src/construction/authoring.rs",
                source,
            ),
        )
        .evaluate()
        .expect("seeded bypass source should parse");

    assert_eq!(report.findings().len(), 1);
    let finding = &report.findings()[0];
    assert_eq!(
        finding.seam(),
        ForgeQueryProhibitedSeam::WorkspaceDirectWrite
    );
    assert_eq!(
        finding.syntax_class(),
        ForgeQueryBoundaryAuditSyntaxClass::MethodCall
    );
    assert_eq!(
        finding.mechanism(),
        ForgeQueryBoundaryAuditCoverageMechanism::AstMethodNameResolved
    );
    assert_eq!(
        finding.site().source_path(),
        Some("src/construction/authoring.rs")
    );
    assert_eq!(finding.line(), 2);
    assert_eq!(finding.column(), 15);
}

#[test]
fn detects_seeded_associated_path_call_bypass_from_registry() {
    let source = "fn forged_workspace_batch() {\n    ForgeQueryWorkspace::batch(commands);\n}\n";
    let report = hard_prohibition_boundary_audit()
        .covering_sources(
            ForgeQueryBoundaryAuditSourceSet::new("worth-kernel")
                .source("construction.authoring", source),
        )
        .evaluate()
        .expect("seeded bypass source should parse");

    assert_eq!(report.findings().len(), 1);
    let finding = &report.findings()[0];
    assert_eq!(
        finding.seam(),
        ForgeQueryProhibitedSeam::WorkspaceDirectBatch
    );
    assert_eq!(
        finding.syntax_class(),
        ForgeQueryBoundaryAuditSyntaxClass::AssociatedPathCall
    );
    assert_eq!(finding.line(), 2);
    assert_eq!(finding.column(), 5);
}

#[test]
fn method_call_detection_is_honest_about_method_name_resolution() {
    let failure = hard_prohibition_boundary_audit()
        .covering_sources(
            ForgeQueryBoundaryAuditSourceSet::new("worth-kernel").source_file(
                "construction.near-miss",
                "src/construction/near_miss.rs",
                "fn method_name_only(other: OtherWorkspace) { other.write(command); }",
            ),
        )
        .try_assert_clean()
        .expect_err("method-name-resolved audit should report this limitation explicitly");

    assert_eq!(failure.findings().len(), 1);
    assert_eq!(
        failure.findings()[0].mechanism(),
        ForgeQueryBoundaryAuditCoverageMechanism::AstMethodNameResolved
    );
    assert_eq!(
        failure.findings()[0].site().source_path(),
        Some("src/construction/near_miss.rs")
    );
}

#[test]
fn associated_path_detection_requires_registry_symbol_suffix() {
    let report = hard_prohibition_boundary_audit()
        .covering_sources(
            ForgeQueryBoundaryAuditSourceSet::new("worth-kernel").source(
                "construction.authoring",
                r#"
                fn unrelated_type_with_same_method_name() {
                    OtherWorkspace::write(command);
                    forge_query::ForgeQueryWorkspace::batch(commands);
                }
            "#,
            ),
        )
        .evaluate()
        .expect("associated path source should parse");

    assert_eq!(report.findings().len(), 1);
    assert_eq!(
        report.findings()[0].seam(),
        ForgeQueryProhibitedSeam::WorkspaceDirectBatch
    );
}
