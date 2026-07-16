use worth_query::facade::WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget;
use hadwiger_research::facade::{
    HadwigerResearchInvariantCatalog, ResearchGraphInvariantDenialRequest,
    ResearchGraphInvariantViolation,
};

fn denial_request(
    catalog: &HadwigerResearchInvariantCatalog,
    violation: &ResearchGraphInvariantViolation,
    target: &WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget,
) {
    let _ = ResearchGraphInvariantDenialRequest::from_violation(catalog, violation)
        .for_lower_runtime_boundary_source(target);
}

fn main() {}
