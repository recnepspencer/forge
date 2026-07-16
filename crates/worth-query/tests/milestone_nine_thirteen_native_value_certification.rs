use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use worth_foundational::facade::AspectValue;
use worth_query::facade::certification::{
    certify_milestone_nine_thirteen_native_values,
    worth_query_milestone_nine_thirteen_native_value_evidence_rows,
};
use worth_query::facade::mutation::WorthQueryAuthoredAspectValue;
use worth_query::facade::read::{
    ConsumedFieldValueFact, ConsumedNativeRefinementDenial, ConsumedNativeValueShape,
    ConsumedNativeValueView, ConsumedProjectionFactSet,
};

#[test]
fn phases_twenty_one_through_thirty_have_source_backed_closeout_evidence() {
    let root = repository_root();
    let rows = worth_query_milestone_nine_thirteen_native_value_evidence_rows();
    let phases = rows.iter().map(|row| row.phase()).collect::<BTreeSet<_>>();
    assert_eq!(phases, (21..=30).collect::<BTreeSet<_>>());

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
fn native_value_closeout_composes_authority_consumer_and_documentation_evidence() {
    let bundle = certify_milestone_nine_thirteen_native_values(repository_root())
        .expect("the source-backed native-value certification should execute");

    assert!(bundle.is_closed(), "bundle: {bundle:#?}");
    assert_eq!(bundle.authority_finding_count(), 0);
    assert_eq!(bundle.grammar_gap_count(), 0);
    assert_eq!(bundle.consumer_residue_count(), 0);
    assert_eq!(bundle.documentation_disagreement_count(), 0);
    assert_eq!(bundle.phase_manifest_gap_count(), 0);
    assert_eq!(bundle.native_family_count(), 26);
    assert_eq!(bundle.compile_fail_fixture_count(), 216);
    assert!(!bundle.certification_digest().is_empty());
    assert!(!bundle.evidence_digest().is_empty());
    assert!(!bundle.native_authority_digest().is_empty());
    assert!(!bundle.consumer_source_digest().is_empty());
    assert!(!bundle.documentation_digest().is_empty());
}

#[test]
fn ordinary_facades_expose_native_authoring_and_consumption_roles() {
    let authored = WorthQueryAuthoredAspectValue::native(AspectValue::UInt32(7));
    assert_eq!(
        authored,
        WorthQueryAuthoredAspectValue::from(AspectValue::UInt32(7))
    );
    assert!(std::mem::size_of::<ConsumedFieldValueFact>() > 0);
    assert!(std::mem::size_of::<ConsumedNativeValueView<'static>>() > 0);
    assert!(std::mem::size_of::<ConsumedNativeRefinementDenial>() > 0);
    assert!(std::mem::size_of::<ConsumedNativeValueShape>() > 0);
    assert!(std::mem::size_of::<ConsumedProjectionFactSet>() > 0);
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
