use worth_query::facade::WorthQueryComputedInspectionEvidence;

fn assert_no_terminal_dependency_projection(evidence: &WorthQueryComputedInspectionEvidence) {
    let _ = evidence.terminal_dependency_aspects_projection();
}

fn main() {}
