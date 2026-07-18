use std::path::PathBuf;

use worth_query::facade::certification::certify_milestone_nine_thirteen_installed_domain;

#[test]
fn installed_domain_closeout_composes_authority_evidence() {
    let bundle = certify_milestone_nine_thirteen_installed_domain(repository_root())
        .expect("the installed-domain certification should execute");

    assert!(bundle.is_closed(), "bundle: {bundle:#?}");
    assert_eq!(bundle.authority_finding_count(), 0);
    assert_eq!(bundle.missing_consumer_residue_class_count(), 0);
    assert!(!bundle.certification_digest().is_empty());
    assert!(!bundle.domain_capability_certification_digest().is_empty());
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-query should live below the repository root")
        .to_path_buf()
}
