use forge_query::facade::ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget;
use hadwiger_research::facade::{
    HadwigerResearchInvariantCatalog, ResearchGraphInvariantDenialRequest,
    ResearchGraphInvariantViolation,
};

fn denial_request(
    catalog: &HadwigerResearchInvariantCatalog,
    violation: &ResearchGraphInvariantViolation,
    target: &ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget,
) {
    let _ = ResearchGraphInvariantDenialRequest::from_violation(catalog, violation)
        .for_lower_runtime_boundary_source(target);
}

fn main() {}
