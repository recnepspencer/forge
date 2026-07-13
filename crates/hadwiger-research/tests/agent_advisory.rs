use hadwiger_research::facade::*;
use worth_query::facade::foundation::WorthQueryGroupedDeclarationInput;
use worth_query::facade::runtime::WorthQuerySupportContributionAuthoring;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("Hadwiger handle should admit")
}

fn graph_version(graph_id: &str) -> GraphVersion {
    let handle = handle();
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration should admit");
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

fn clean_context() -> (
    HadwigerResearchHandle,
    GraphVersion,
    ResearchEvidenceCorpus,
    DiscoveryFrontier,
    HadwigerResearchInvariantCatalog,
) {
    let handle = handle();
    let version = graph_version("phase9-agent");
    let embedding = ExactGraphEmbedding::builder(version.reference(), "bad-embedding")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(2, 0))
        .unwrap()
        .finish()
        .unwrap();
    let unit_checked = verify_unit_distance_embedding_checked(&handle, &version, embedding)
        .expect("unit-distance checker should run");
    let rejection = explain_rejection(
        &handle,
        ExplainRejectionRequest::for_checker_rejection(
            "bad-unit-distance-edge",
            &version,
            unit_checked.verification(),
        )
        .with_rejected_aspect(unit_checked.unit_distance_aspect())
        .with_repair_obligation("provide exact coordinates for every unit edge")
        .unwrap(),
    )
    .unwrap();
    let corpus = ResearchEvidenceCorpus::builder("phase9-corpus")
        .with_graph_version(version.reference())
        .with_checker_rejection(rejection)
        .unwrap()
        .finish()
        .unwrap();
    let observations = mine_research_patterns(&handle, &corpus).unwrap();
    let hypotheses = propose_invariant_hypotheses(&handle, &corpus, &observations).unwrap();
    let plans = plan_next_experiments(&handle, &corpus, &hypotheses).unwrap();
    let frontier =
        update_discovery_frontier(&handle, &corpus, observations, hypotheses, plans).unwrap();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    (handle, version, corpus, frontier, catalog)
}

fn source(transcript: &str) -> AgentSourceRecord {
    AgentSourceRecord::new(
        "codex",
        "local-agent-session",
        transcript,
        "tool:digest:test",
    )
    .unwrap()
}

#[test]
fn agent_batch_admits_deterministic_non_authoritative_advisory_artifacts() {
    let (handle, version, corpus, _frontier, _catalog) = clean_context();
    let left = AgentExplorationBatch::builder("frontier-agent-pass-a", source("transcript:a"))
        .with_experiment_proposal(
            AgentExperimentProposal::new("try-local-edge-rewire", version.reference())
                .with_rationale("test whether the failure is tied to one local edge orbit")
                .unwrap(),
        )
        .unwrap()
        .with_motif_suggestion(
            AgentMotifSuggestion::new("motif-a", version.reference())
                .with_observation("edge-local unit-distance failures recur")
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap();
    let right = AgentExplorationBatch::builder("frontier-agent-pass-a", source("transcript:a"))
        .with_motif_suggestion(
            AgentMotifSuggestion::new("motif-a", version.reference())
                .with_observation("edge-local unit-distance failures recur")
                .unwrap(),
        )
        .unwrap()
        .with_experiment_proposal(
            AgentExperimentProposal::new("try-local-edge-rewire", version.reference())
                .with_rationale("test whether the failure is tied to one local edge orbit")
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap();

    let admitted = admit_agent_exploration_batch_checked(&handle, &corpus, left).unwrap();

    assert_eq!(right.artifact_digest(), admitted.batch().artifact_digest());
    assert_eq!(admitted.advisory_artifacts().len(), 2);
    assert!(!admitted.admits_theorem_authority());
    assert!(!admitted.registers_query_invariant_authority());
    assert!(admitted
        .advisory_artifacts()
        .iter()
        .all(|artifact| artifact.source().agent_identity() == "codex"));
}

#[test]
fn changed_agent_transcript_changes_batch_digest() {
    let (_handle, version, _corpus, _frontier, _catalog) = clean_context();
    let build = |transcript| {
        AgentExplorationBatch::builder("frontier-agent-pass-a", source(transcript))
            .with_motif_suggestion(
                AgentMotifSuggestion::new("motif-a", version.reference())
                    .with_observation("edge-local unit-distance failures recur")
                    .unwrap(),
            )
            .unwrap()
            .finish()
            .unwrap()
    };

    assert_ne!(
        build("transcript:a").artifact_digest(),
        build("transcript:b").artifact_digest()
    );
}

#[test]
fn agent_batch_rejects_evidence_outside_corpus() {
    let (handle, _version, corpus, _frontier, _catalog) = clean_context();
    let outside = graph_version("phase9-outside");
    let batch = AgentExplorationBatch::builder("frontier-agent-pass-a", source("transcript:a"))
        .with_motif_suggestion(
            AgentMotifSuggestion::new("motif-a", outside.reference())
                .with_observation("outside reference should not pass")
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap();

    let error = admit_agent_exploration_batch_checked(&handle, &corpus, batch).unwrap_err();

    assert!(matches!(
        error,
        AgentAdvisoryError::EvidenceNotInCorpus { .. }
    ));
}

#[test]
fn declaration_admission_advisory_materializes_query_contribution_digest() {
    let (handle, _version, _corpus, _frontier, _catalog) = clean_context();
    let advisory = AgentAdmissionAdvisory::caution(
        "candidate-a",
        "geometry evidence is suggestive but not checker-admitted",
    )
    .unwrap();

    let materialized = materialize_agent_declaration_advisory_checked(
        &handle,
        AdvisoryNoteDeclaration::new("candidate-a", "agent-caution"),
        advisory,
    )
    .unwrap();

    assert!(materialized.query_contribution_digest().is_some());
    assert!(!materialized.advisory_artifact().admits_theorem_authority());
    assert_eq!(
        materialized.advisory_artifact().advisory_kind(),
        AgentAdvisoryKind::AdmissionCaution
    );
}

#[test]
fn grouped_agent_advisory_uses_query_grouped_contribution_surface() {
    let (handle, version, corpus, _frontier, _catalog) = clean_context();
    let batch = AgentExplorationBatch::builder("frontier-agent-pass-a", source("transcript:a"))
        .with_motif_suggestion(
            AgentMotifSuggestion::new("motif-a", version.reference())
                .with_observation("edge-local pattern applies across this neighborhood")
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap();
    let admitted = admit_agent_exploration_batch_checked(&handle, &corpus, batch).unwrap();
    let advisory_artifact = admitted.advisory_artifacts()[0].clone();
    let grouped = WorthQueryGroupedDeclarationInput::<
        HadwigerResearchDomainEntry,
        CandidateGraphDeclaration,
    >::local_neighborhood(
        CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1")
    )
    .with_member(CandidateGraphDeclaration::new("candidate-b").with_graph_version("v1"))
    .with_shared_support_contribution(
        WorthQuerySupportContributionAuthoring::declaration_support(
            "hadwiger.agent.grouped_support",
            "agent support applies to the candidate neighborhood",
        ),
    );

    let materialized =
        materialize_agent_grouped_advisory_checked(&handle, grouped, advisory_artifact).unwrap();

    assert!(materialized
        .query_contribution_digest()
        .unwrap()
        .starts_with("grouped_query_contributions:"));
    assert!(!materialized.advisory_artifact().admits_theorem_authority());
}

#[test]
fn equal_length_declaration_advisories_do_not_collapse_source_identity() {
    let (handle, _version, _corpus, _frontier, _catalog) = clean_context();
    let left = materialize_agent_declaration_advisory_checked(
        &handle,
        AdvisoryNoteDeclaration::new("candidate-a", "agent-caution-left"),
        AgentAdmissionAdvisory::caution("candidate-a", "alpha note").unwrap(),
    )
    .unwrap();
    let right = materialize_agent_declaration_advisory_checked(
        &handle,
        AdvisoryNoteDeclaration::new("candidate-a", "agent-caution-right"),
        AgentAdmissionAdvisory::caution("candidate-a", "omega note").unwrap(),
    )
    .unwrap();

    assert_ne!(
        left.advisory_artifact().source().transcript_digest(),
        right.advisory_artifact().source().transcript_digest()
    );
    assert_ne!(left.artifact_digest(), right.artifact_digest());
}

#[test]
fn experiment_proposals_remain_advisory_until_screened() {
    let (handle, version, corpus, frontier, catalog) = clean_context();
    let batch = AgentExplorationBatch::builder("frontier-agent-pass-a", source("transcript:a"))
        .with_experiment_proposal(
            AgentExperimentProposal::new("try-local-edge-rewire", version.reference())
                .with_rationale("test whether the failure is tied to one local edge orbit")
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap();
    let admitted = admit_agent_exploration_batch_checked(&handle, &corpus, batch).unwrap();

    let screening =
        screen_agent_experiment_proposals_checked(&handle, &corpus, &frontier, &catalog, admitted)
            .unwrap();

    assert_eq!(screening.blocked_proposals().len(), 1);
    assert_eq!(
        screening.blocked_reasons(),
        &["phase7_suppression_or_reactivation_required".to_string()]
    );
    assert!(!screening.admits_theorem_authority());
}
