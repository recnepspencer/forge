use hadwiger_research::facade::ResearchGraphInvariantDenial;

fn mutate_denial_source(denial: &mut ResearchGraphInvariantDenial) {
    denial.lower_runtime_source_digest = "changed".to_string();
}

fn main() {}
