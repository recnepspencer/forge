use std::path::PathBuf;
use worth_query::facade::consumer_kit::{
    downstream_authority_adoption, worth_query_consumer_residue_certification_evidence,
};

#[test]
fn residue_detectors_reject_every_registered_hostile_shape_without_false_positives() {
    for evidence in worth_query_consumer_residue_certification_evidence() {
        assert!(
            evidence.satisfied(),
            "residue detector case {} is not trustworthy",
            evidence.case_id()
        );
    }
}

#[test]
fn worth_ui_production_consumers_have_no_competing_query_authority_residue() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("certification crate remains under workspaces/worth-query/crates")
        .to_path_buf();
    let proof = downstream_authority_adoption("worth-ui-reference-consumer")
        .required_root(root.join("workspaces/worth-ui/crates/worth-ui-query-binding/src"))
        .required_root(root.join("workspaces/worth-ui/crates/worth-ui-runtime/src"))
        .evaluate()
        .expect("reference consumer production sources must parse");

    proof.assert_adopted();
    assert!(proof.deletion_receipt().is_some());
    let report = proof.residue_report();
    for required_suffix in [
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/lib.rs",
        "workspaces/worth-ui/crates/worth-ui-runtime/src/lib.rs",
    ] {
        assert!(
            report
                .audited_source_paths()
                .iter()
                .any(|path| path.ends_with(required_suffix)),
            "the production audit omitted its required source root sentinel {required_suffix}"
        );
    }
}
