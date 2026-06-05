use hadwiger_research::facade::{
    HadwigerResearchInvariantCatalog, ResearchGraphInvariantDenialRequest,
    ResearchGraphInvariantViolation,
};

fn denial_request(
    catalog: &HadwigerResearchInvariantCatalog,
    violation: &ResearchGraphInvariantViolation,
) {
    let _ = ResearchGraphInvariantDenialRequest::from_violation(catalog, violation)
        .for_lower_runtime_boundary_envelope("raw-envelope");
}

fn main() {}
