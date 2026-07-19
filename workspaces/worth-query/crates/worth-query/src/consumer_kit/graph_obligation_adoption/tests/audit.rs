use crate::{WorthQueryBoundaryAuditSourceSet, WorthQueryGraphObligationLocalCeremonyAudit};

#[test]
fn local_ceremony_audit_finds_real_bypass_and_ignores_comments_and_literals() {
    let sources = WorthQueryBoundaryAuditSourceSet::new("worth-kernel")
        .source(
            "comment-and-literal-only.rs",
            r####"
// WorthQueryGraphObligationRegistration::blocking_invariant(...)
let label = "WorthQueryGraphTouchSelector::collection";
let raw = r###"select_graph_obligations_for_touch(&touch, &world)"###;
/* WorthQueryGraphObligationIndex::from_catalog(&catalog) */
"####,
        )
        .source(
            "real-bypass.rs",
            r#"
fn install() {
    let index = WorthQueryGraphObligationIndex::from_catalog(&catalog);
    let manual_pre_check = InvariantPack::from(local_legality_graph);
}
"#,
        );

    let audit = WorthQueryGraphObligationLocalCeremonyAudit::evaluate(&sources);

    assert_eq!(audit.findings().len(), 4);
    assert_eq!(audit.findings()[0].source_label(), "real-bypass.rs");
    assert_eq!(
        audit.findings()[0].pattern(),
        "WorthQueryGraphObligationIndex::from_catalog"
    );
    assert_eq!(audit.findings()[0].column(), 17);
    assert!(audit.findings()[0].source_path().is_none());
}

#[test]
fn local_ceremony_audit_preserves_code_after_rust_lifetimes() {
    let sources = WorthQueryBoundaryAuditSourceSet::new("worth-spatial").source(
        "lifetime-heavy-source.rs",
        r#"
pub struct LifetimeHeavy<'a> {
    borrowed: &'a str,
}

fn install_bypass_after_lifetime<'a>() {
    let _ = WorthQueryGraphObligationIndex::from_catalog(&catalog);
}
"#,
    );

    let audit = WorthQueryGraphObligationLocalCeremonyAudit::evaluate(&sources);

    assert_eq!(audit.findings().len(), 1);
    assert_eq!(
        audit.findings()[0].source_label(),
        "lifetime-heavy-source.rs"
    );
    assert_eq!(
        audit.findings()[0].pattern(),
        "WorthQueryGraphObligationIndex::from_catalog"
    );
    assert_eq!(audit.findings()[0].line(), 7);
}
