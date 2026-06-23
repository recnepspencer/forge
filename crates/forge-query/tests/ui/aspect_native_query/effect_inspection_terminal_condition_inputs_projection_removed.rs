use forge_query::facade::ForgeQueryEffectInspectionEvidence;

fn assert_no_terminal_condition_inputs_projection(evidence: &ForgeQueryEffectInspectionEvidence) {
    let _ = evidence.terminal_condition_inputs_projection();
}

fn main() {}
