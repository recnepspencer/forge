use std::collections::BTreeSet;

use super::completeness_support::{
    representative_runtime_stop_errors, runtime_error_variant_key, stop_class_variant_key,
};

const PHASE_THREE_COVERED_RUNTIME_ERROR_VARIANT_COUNT: usize = 45;
const PHASE_THREE_STOP_CLASS_VARIANT_COUNT: usize = 23;

#[test]
fn runtime_stop_class_classifier_is_complete_for_covered_runtime_error_variants() {
    let representative_errors = representative_runtime_stop_errors();
    assert_eq!(
        representative_errors.len(),
        PHASE_THREE_COVERED_RUNTIME_ERROR_VARIANT_COUNT
    );

    let runtime_variant_keys = representative_errors
        .iter()
        .map(runtime_error_variant_key)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        runtime_variant_keys.len(),
        PHASE_THREE_COVERED_RUNTIME_ERROR_VARIANT_COUNT,
        "phase-3 covered runtime error representatives must include one unique example per variant"
    );

    let stop_class_variant_keys = representative_errors
        .iter()
        .map(|error| stop_class_variant_key(error.stop_class()))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        stop_class_variant_keys.len(),
        PHASE_THREE_STOP_CLASS_VARIANT_COUNT,
        "phase-3 stop-class closure must classify through the full typed taxonomy with no fallback bucket"
    );

    let preview_promotion_count = representative_errors
        .iter()
        .filter(|error| stop_class_variant_key(error.stop_class()) == "preview_promotion_denied")
        .count();
    assert_eq!(
        preview_promotion_count, 4,
        "all preview-promotion runtime variants must converge on one typed stop class"
    );

    let runtime_lookup_count = representative_errors
        .iter()
        .filter(|error| stop_class_variant_key(error.stop_class()) == "runtime_lookup_failed")
        .count();
    assert_eq!(
        runtime_lookup_count, 2,
        "lookup failures should converge on one typed runtime-lookup stop class"
    );

    let runtime_artifact_count = representative_errors
        .iter()
        .filter(|error| stop_class_variant_key(error.stop_class()) == "missing_runtime_artifact")
        .count();
    assert_eq!(
        runtime_artifact_count, 5,
        "missing-artifact runtime variants should converge on one typed stop class family"
    );

    let runtime_declaration_count = representative_errors
        .iter()
        .filter(|error| stop_class_variant_key(error.stop_class()) == "runtime_declaration_failed")
        .count();
    assert_eq!(
        runtime_declaration_count, 6,
        "runtime declaration failures should converge on one typed declaration stop class family"
    );

    let classifier_source = include_str!("../../error/stop_classify.rs");
    assert!(
        !classifier_source.contains("_ =>"),
        "phase-3 stop-class classifier must not contain a wildcard escape hatch"
    );
}
