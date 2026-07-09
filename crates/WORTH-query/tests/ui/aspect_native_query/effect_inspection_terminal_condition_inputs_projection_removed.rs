use worth_query::facade::WorthQueryEffectInspectionEvidence;

fn assert_no_terminal_condition_inputs_projection(evidence: &WorthQueryEffectInspectionEvidence) {
    let _ = evidence.terminal_condition_inputs_projection();
}

fn main() {}
