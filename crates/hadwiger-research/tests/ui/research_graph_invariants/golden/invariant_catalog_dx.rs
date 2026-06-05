use forge_query::facade::runtime::ForgeQueryLowerRuntimeBoundaryEnvelopeSource;
use hadwiger_research::facade::{
    certify_research_graph_invariant_violation, draft_research_graph_invariant_catalog,
    materialize_research_graph_invariant_denial, plan_research_graph_invariant_registration,
    DiscoveryFrontier, ExperimentBatch, HadwigerResearchHandle, ResearchEvidenceCorpus,
    ResearchGraphInvariantCheckRequest, ResearchGraphInvariantDenialRequest,
    ResearchGraphInvariantError, ResearchGraphInvariantFamily,
    ResearchGraphInvariantRegistrationPosture,
};

fn invariant_catalog_dx(
    handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
    frontier: &DiscoveryFrontier,
    plans: ExperimentBatch,
    lower_runtime_source: &impl ForgeQueryLowerRuntimeBoundaryEnvelopeSource,
) -> Result<(), ResearchGraphInvariantError> {
    let catalog = draft_research_graph_invariant_catalog(handle, corpus, frontier)?;
    assert!(catalog.has_rule_family(ResearchGraphInvariantFamily::FailureResidency));
    assert!(!catalog.registers_query_invariant_authority());

    let violation = certify_research_graph_invariant_violation(
        handle,
        ResearchGraphInvariantCheckRequest::for_experiment_batch(&catalog, plans)
            .with_corpus(corpus),
    )?;
    let denial = materialize_research_graph_invariant_denial(
        handle,
        ResearchGraphInvariantDenialRequest::from_violation(&catalog, &violation)
            .for_lower_runtime_boundary_source(lower_runtime_source),
    )?;
    assert!(denial.query_denial().is_some());

    let plan = plan_research_graph_invariant_registration(handle, &catalog)?;
    assert_eq!(plan.posture(), ResearchGraphInvariantRegistrationPosture::BlockedDraft);
    assert!(plan
        .compatible_query_surfaces()
        .contains("ForgeQueryRuntime::builder().invariant_catalog(...)"));
    assert!(!plan.registers_runtime_invariants());
    Ok(())
}

fn main() {}
