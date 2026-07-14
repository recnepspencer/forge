use hadwiger_research::facade::*;

pub fn base_session(label: &str) -> (HadwigerResearchHandle, ResearchCockpitSession) {
    let (handle, version, corpus, frontier, catalog, agent) = cockpit_inputs(label);
    let session = assemble_research_cockpit_session_checked(
        &handle,
        ResearchCockpitSession::builder(format!("{label}-session"))
            .with_corpus(corpus)
            .with_frontier(frontier)
            .with_invariant_catalog(catalog)
            .with_agent_admission(agent)
            .finish()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(version.version_id(), "v1");
    (handle, session)
}

pub fn stale_session(label: &str) -> (HadwigerResearchHandle, ResearchCockpitSession) {
    let (handle, version, corpus, frontier, catalog, agent) = cockpit_inputs(label);
    let derived = recompute_derived_discovery_frontier(&handle, &corpus).unwrap();
    let changed_corpus = ResearchEvidenceCorpus::builder(format!("{label}-changed-corpus"))
        .with_graph_version(version.reference())
        .finish()
        .unwrap();
    let session = assemble_research_cockpit_session_checked(
        &handle,
        ResearchCockpitSession::builder(format!("{label}-stale-session"))
            .with_corpus(changed_corpus)
            .with_frontier(frontier)
            .with_invariant_catalog(catalog)
            .with_agent_admission(agent)
            .with_derived_frontier_state(derived)
            .finish()
            .unwrap(),
    )
    .unwrap();
    (handle, session)
}

fn cockpit_inputs(
    label: &str,
) -> (
    HadwigerResearchHandle,
    GraphVersion,
    ResearchEvidenceCorpus,
    DiscoveryFrontier,
    HadwigerResearchInvariantCatalog,
    AgentExplorationAdmissionChecked,
) {
    let handle = crate::installed_support::installed_hadwiger_research_handle().unwrap();
    let version = graph_version(&handle, label);
    let rejection = checker_rejection(&handle, &version, label);
    let corpus = retained_corpus(&handle, &version, rejection, label);
    let observations = mine_research_patterns(&handle, &corpus).unwrap();
    let hypotheses = propose_invariant_hypotheses(&handle, &corpus, &observations).unwrap();
    let plans = plan_next_experiments(&handle, &corpus, &hypotheses).unwrap();
    let frontier =
        update_discovery_frontier(&handle, &corpus, observations, hypotheses, plans).unwrap();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    let agent = agent_admission(&handle, &corpus, &version, label);
    (handle, version, corpus, frontier, catalog, agent)
}

fn graph_version(handle: &HadwigerResearchHandle, label: &str) -> GraphVersion {
    let graph_id = format!("{label}-graph");
    let declaration =
        declare_research_request_checked(handle, CandidateGraphDeclaration::new(&graph_id))
            .admitted()
            .unwrap();
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
    GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("a")
        .unwrap()
        .with_vertex("b")
        .unwrap()
        .with_undirected_edge("a", "b")
        .unwrap()
        .finish()
        .unwrap()
}

fn checker_rejection(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
    label: &str,
) -> HadwigerRejectionExplanation {
    let embedding = ExactGraphEmbedding::builder(version.reference(), format!("{label}-bad"))
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(2, 0))
        .unwrap()
        .finish()
        .unwrap();
    let checked = verify_unit_distance_embedding_checked(handle, version, embedding).unwrap();
    explain_rejection(
        handle,
        ExplainRejectionRequest::for_checker_rejection(
            format!("{label}-bad-unit-distance-edge"),
            version,
            checked.verification(),
        )
        .with_rejected_aspect(checked.unit_distance_aspect())
        .with_repair_obligation("supply exact unit-distance coordinates")
        .unwrap(),
    )
    .unwrap()
}

fn retained_corpus(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
    rejection: HadwigerRejectionExplanation,
    label: &str,
) -> ResearchEvidenceCorpus {
    let initial = ResearchEvidenceCorpus::builder(format!("{label}-initial-corpus"))
        .with_graph_version(version.reference())
        .with_checker_rejection(rejection.clone())
        .unwrap()
        .finish()
        .unwrap();
    let failure = attach_failure_to_research_graph(
        handle,
        &initial,
        rejection.reusable_negative_evidence().unwrap(),
        FailureScope::edge_local(version.reference(), "a", "b").unwrap(),
    )
    .unwrap();
    ResearchEvidenceCorpus::builder(format!("{label}-corpus"))
        .with_graph_version(version.reference())
        .with_checker_rejection(rejection)
        .unwrap()
        .with_graph_resident_failure(failure)
        .finish()
        .unwrap()
}

fn agent_admission(
    handle: &HadwigerResearchHandle,
    corpus: &ResearchEvidenceCorpus,
    version: &GraphVersion,
    label: &str,
) -> AgentExplorationAdmissionChecked {
    let source = AgentSourceRecord::new(
        "codex",
        "local-agent-session",
        format!("transcript:digest:{label}"),
        "tool:digest:hadwiger-cli",
    )
    .unwrap();
    let batch = AgentExplorationBatch::builder(format!("{label}-agent-pass"), source)
        .with_experiment_proposal(
            AgentExperimentProposal::new(format!("{label}-rewire"), version.reference())
                .with_rationale("try a local edge rewire around the retained failure")
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap();
    admit_agent_exploration_batch_checked(handle, corpus, batch).unwrap()
}
