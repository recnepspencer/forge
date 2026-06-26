use forge_query::facade::ForgeQueryEffectInspectionEvidence;

fn assert_no_neutral_condition_aspect_aliases(evidence: &ForgeQueryEffectInspectionEvidence) {
    let _ = evidence.condition_inputs();
    let _ = evidence.condition_outputs();
}

fn main() {}
