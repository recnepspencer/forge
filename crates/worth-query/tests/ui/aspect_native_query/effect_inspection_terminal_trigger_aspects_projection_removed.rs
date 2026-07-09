use worth_query::facade::WorthQueryEffectInspectionEvidence;

fn assert_no_terminal_trigger_aspects_projection(evidence: &WorthQueryEffectInspectionEvidence) {
    let _ = evidence.terminal_trigger_aspects_projection();
}

fn main() {}
