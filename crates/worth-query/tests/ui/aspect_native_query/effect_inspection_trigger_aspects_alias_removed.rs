use worth_query::facade::WorthQueryEffectInspectionEvidence;

fn assert_no_neutral_trigger_aspect_alias(evidence: &WorthQueryEffectInspectionEvidence) {
    let _ = evidence.trigger_aspects();
}

fn main() {}
