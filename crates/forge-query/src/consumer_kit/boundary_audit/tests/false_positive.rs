use super::super::{hard_prohibition_boundary_audit, ForgeQueryBoundaryAuditSourceSet};

#[test]
fn ignores_comments_docs_and_string_literals() {
    let report = hard_prohibition_boundary_audit()
        .covering_sources(
            ForgeQueryBoundaryAuditSourceSet::new("worth-kernel").source(
                "construction.authoring",
                r#"
                /// `workspace.write(command)` is documentation, not executable bypass.
                #[doc = "ForgeQueryWorkspace::batch(commands) is documentation too"]
                fn clean_reference_only() {
                    let comment_like = "workspace.delete_existing(binding)";
                    // workspace.update_existing(binding);
                }
            "#,
            ),
        )
        .evaluate()
        .expect("reference-only source should parse");

    assert!(report.findings().is_empty());
    assert!(report.parsed_item_count() > 0);
}
