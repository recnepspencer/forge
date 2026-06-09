use hadwiger_research::facade::ResearchGraphInvariantRuntimeProjection;

fn apply_manually(projection: &ResearchGraphInvariantRuntimeProjection) {
    let _ = projection.apply_invariant_family_manually("failure_residency");
}

fn main() {}
