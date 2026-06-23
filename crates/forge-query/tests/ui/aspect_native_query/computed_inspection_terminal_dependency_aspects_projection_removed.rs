use forge_query::facade::ForgeQueryComputedInspectionEvidence;

fn assert_no_terminal_dependency_projection(evidence: &ForgeQueryComputedInspectionEvidence) {
    let _ = evidence.terminal_dependency_aspects_projection();
}

fn main() {}
