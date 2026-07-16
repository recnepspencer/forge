use worth_query::facade::{
    domain::WorthQueryInstalledDomainHandle,
    runtime::{WorthQueryLowerRuntimeBoundaryEnvelopeSource, WorthQueryWorkspace},
};
use hadwiger_research::facade::{
    certify_research_graph_invariant_violation, draft_research_graph_invariant_catalog,
    materialize_research_graph_invariant_denial,
    project_research_graph_for_invariant_registration_checked,
    DiscoveryFrontier, ExperimentBatch, HadwigerResearchDomainEntry, HadwigerResearchHandle,
    ResearchEvidenceCorpus,
    ResearchGraphInvariantCheckRequest, ResearchGraphInvariantDenialRequest,
    ResearchGraphInvariantError, ResearchGraphInvariantFamily,
};

fn invariant_catalog_dx(
    handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
    frontier: &DiscoveryFrontier,
    plans: ExperimentBatch,
    installed_handle: &WorthQueryInstalledDomainHandle<HadwigerResearchDomainEntry>,
    workspace: &WorthQueryWorkspace,
    lower_runtime_source: &impl WorthQueryLowerRuntimeBoundaryEnvelopeSource,
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
        installed_handle,
        workspace,
        ResearchGraphInvariantDenialRequest::from_violation(&catalog, &violation)
            .for_lower_runtime_boundary_source(lower_runtime_source),
    )?;
    assert!(denial.query_denial().is_some());

    let projection =
        project_research_graph_for_invariant_registration_checked(handle, corpus, frontier)?;
    assert_eq!(projection.source_corpus_digest(), corpus.corpus_digest().stable_token());
    assert_eq!(projection.catalog().rules().len(), 5);
    Ok(())
}

fn main() {}
