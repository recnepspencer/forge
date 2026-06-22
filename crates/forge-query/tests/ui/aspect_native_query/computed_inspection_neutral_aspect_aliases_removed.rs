use forge_query::facade::ForgeQueryComputedInspectionEvidence;

fn assert_no_neutral_computed_inspection_aspect_aliases(
    evidence: &ForgeQueryComputedInspectionEvidence,
) {
    let _ = evidence.dependency_aspects();
    let _ = evidence.produced_aspects();
}

fn main() {}
