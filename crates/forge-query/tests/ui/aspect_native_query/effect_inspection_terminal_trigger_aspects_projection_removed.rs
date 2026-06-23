use forge_query::facade::ForgeQueryEffectInspectionEvidence;

fn assert_no_terminal_trigger_aspects_projection(evidence: &ForgeQueryEffectInspectionEvidence) {
    let _ = evidence.terminal_trigger_aspects_projection();
}

fn main() {}
