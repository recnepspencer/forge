use hadwiger_research::facade::ResearchGraphInvariantRuntimeProjection;

fn mutate(projection: &mut ResearchGraphInvariantRuntimeProjection) {
    let _ = projection.relations_mut();
}

fn main() {}
