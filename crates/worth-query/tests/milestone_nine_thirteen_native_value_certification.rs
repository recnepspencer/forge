use std::path::PathBuf;

use worth_foundational::facade::AspectValue;
use worth_query::facade::certification::certify_milestone_nine_thirteen_native_values;
use worth_query::facade::mutation::WorthQueryAuthoredAspectValue;
use worth_query::facade::read::{
    ConsumedFieldValueFact, ConsumedNativeRefinementDenial, ConsumedNativeValueShape,
    ConsumedNativeValueView, ConsumedProjectionFactSet,
};

#[test]
fn native_value_closeout_composes_authority_and_documentation_evidence() {
    let bundle = certify_milestone_nine_thirteen_native_values(repository_root())
        .expect("the source-backed native-value certification should execute");

    assert!(bundle.is_closed(), "bundle: {bundle:#?}");
    assert_eq!(bundle.authority_finding_count(), 0);
    assert_eq!(bundle.grammar_gap_count(), 0);
    assert_eq!(bundle.documentation_disagreement_count(), 0);
    assert_eq!(bundle.native_family_count(), 26);
    assert!(!bundle.certification_digest().is_empty());
    assert!(!bundle.native_authority_digest().is_empty());
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

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-query should live below the repository root")
        .to_path_buf()
}
