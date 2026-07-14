use super::super::{
    hard_prohibition_boundary_audit, WorthQueryBoundaryAuditErrorKind,
    WorthQueryBoundaryAuditSourceSet,
};

#[test]
fn source_set_validation_localizes_invalid_inputs() {
    let error = hard_prohibition_boundary_audit()
        .covering_sources(
            WorthQueryBoundaryAuditSourceSet::new(" ")
                .source("construction.authoring", "fn clean() {}"),
        )
        .evaluate()
        .expect_err("blank crate names should fail validation");
    assert_eq!(
        error.kind(),
        WorthQueryBoundaryAuditErrorKind::EmptyCrateName
    );

    let error = hard_prohibition_boundary_audit()
        .covering_sources(
            WorthQueryBoundaryAuditSourceSet::new("worth-kernel")
                .source("construction.authoring", "fn first() {}")
                .source("construction.authoring", "fn second() {}"),
        )
        .evaluate()
        .expect_err("duplicate source labels should fail validation");
    assert_eq!(
        error.kind(),
        WorthQueryBoundaryAuditErrorKind::DuplicateSourceLabel
    );
    assert_eq!(error.source_label(), Some("construction.authoring"));

    let error = hard_prohibition_boundary_audit()
        .covering_sources(
            WorthQueryBoundaryAuditSourceSet::new("worth-kernel").source_file(
                "construction.authoring",
                " ",
                "fn clean() {}",
            ),
        )
        .evaluate()
        .expect_err("blank source paths should fail validation");
    assert_eq!(
        error.kind(),
        WorthQueryBoundaryAuditErrorKind::EmptySourcePath
    );
    assert_eq!(error.source_label(), Some("construction.authoring"));

    let error = hard_prohibition_boundary_audit()
        .covering_sources(
            WorthQueryBoundaryAuditSourceSet::new("worth-kernel")
                .source("construction.authoring", "fn broken("),
        )
        .evaluate()
        .expect_err("parse failures should localize the source label");
    assert_eq!(
        error.kind(),
        WorthQueryBoundaryAuditErrorKind::RustParseFailed
    );
    assert_eq!(error.source_label(), Some("construction.authoring"));
}
