use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use worth_query::facade::certification::{
    certify_milestone_nine_thirteen_installed_domain,
    worth_query_milestone_nine_thirteen_installed_domain_evidence_rows,
};

#[test]
fn phases_thirteen_through_twenty_have_source_backed_closeout_evidence() {
    let root = repository_root();
    let rows = worth_query_milestone_nine_thirteen_installed_domain_evidence_rows();
    let phases = rows
        .iter()
        .map(|row| row.phase())
        .collect::<BTreeSet<_>>();
    assert_eq!(phases, (13..=20).collect::<BTreeSet<_>>());

    for row in rows {
        let source = read_source(&root, row.path());
        assert_eq!(
            source.match_indices(row.probe()).count(),
            1,
            "phase {} evidence probe drifted in {}: {}",
            row.phase(),
            row.path(),
            row.probe()
        );
    }
}

#[test]
fn installed_domain_closeout_composes_real_consumer_and_authority_evidence() {
    let bundle = certify_milestone_nine_thirteen_installed_domain(repository_root())
        .expect("the source-backed installed-domain certification should execute");

    assert!(bundle.is_closed(), "bundle: {bundle:#?}");
    assert_eq!(bundle.authority_finding_count(), 0);
    assert_eq!(bundle.missing_compile_fail_boundary_count(), 0);
    assert_eq!(bundle.missing_consumer_residue_class_count(), 0);
    assert!(!bundle.certification_digest().is_empty());
    assert!(!bundle.domain_capability_certification_digest().is_empty());
    assert!(!bundle.reference_consumer_journey_digest().is_empty());
    assert!(!bundle.consumer_source_inventory_digest().is_empty());
}

fn read_source(root: &Path, relative: &str) -> String {
    std::fs::read_to_string(root.join(relative))
        .unwrap_or_else(|error| panic!("failed to read {relative}: {error}"))
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("worth-query should live below the repository root")
        .to_path_buf()
}
