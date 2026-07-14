use worth_query::facade::runtime::WorthQueryComputedInspectionEvidence;

fn assert_no_neutral_computed_inspection_aspect_aliases(
    evidence: &WorthQueryComputedInspectionEvidence,
) {
    let _ = evidence.dependency_aspects();
    let _ = evidence.produced_aspects();
}

fn main() {}
