use forge_query::facade::ForgeQueryComputedInspectionEvidence;

fn assert_no_terminal_produced_projection(evidence: &ForgeQueryComputedInspectionEvidence) {
    let _ = evidence.terminal_produced_aspects_projection();
}

fn main() {}
