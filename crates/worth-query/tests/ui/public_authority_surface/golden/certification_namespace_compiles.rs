use worth_query::facade::certification::{worth_query_intent_admission_certification_output_manifest, worth_query_intent_admission_compile_fail_targets};

fn main() {
    assert!(!worth_query_intent_admission_certification_output_manifest().is_empty());
    assert!(!worth_query_intent_admission_compile_fail_targets().is_empty());
}
