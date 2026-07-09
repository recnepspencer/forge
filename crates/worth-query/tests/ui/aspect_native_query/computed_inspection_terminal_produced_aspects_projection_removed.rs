use worth_query::facade::WorthQueryComputedInspectionEvidence;

fn assert_no_terminal_produced_projection(evidence: &WorthQueryComputedInspectionEvidence) {
    let _ = evidence.terminal_produced_aspects_projection();
}

fn main() {}
