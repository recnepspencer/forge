use forge_query::facade::ForgeQueryEffectInspectionEvidence;

fn assert_no_neutral_trigger_aspect_alias(evidence: &ForgeQueryEffectInspectionEvidence) {
    let _ = evidence.trigger_aspects();
}

fn main() {}
