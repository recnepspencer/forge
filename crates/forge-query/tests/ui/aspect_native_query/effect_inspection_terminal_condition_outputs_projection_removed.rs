use forge_query::facade::ForgeQueryEffectInspectionEvidence;

fn assert_no_terminal_condition_outputs_projection(evidence: &ForgeQueryEffectInspectionEvidence) {
    let _ = evidence.terminal_condition_outputs_projection();
}

fn main() {}
