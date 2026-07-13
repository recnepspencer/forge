use worth_query::facade::runtime::WorthQueryEffectInspectionEvidence;

fn assert_no_neutral_condition_aspect_aliases(evidence: &WorthQueryEffectInspectionEvidence) {
    let _ = evidence.condition_inputs();
    let _ = evidence.condition_outputs();
}

fn main() {}
