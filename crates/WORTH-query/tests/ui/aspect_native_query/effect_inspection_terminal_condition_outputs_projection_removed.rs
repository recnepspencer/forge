use worth_query::facade::WorthQueryEffectInspectionEvidence;

fn assert_no_terminal_condition_outputs_projection(evidence: &WorthQueryEffectInspectionEvidence) {
    let _ = evidence.terminal_condition_outputs_projection();
}

fn main() {}
