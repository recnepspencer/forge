use hadwiger_research::facade::*;

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

fn cockpit_context() -> (
    HadwigerResearchHandle,
    GraphVersion,
    ResearchEvidenceCorpus,
    DiscoveryFrontier,
    HadwigerResearchInvariantCatalog,
    AgentExplorationAdmissionChecked,
) {
    let handle = handle();
    let version = graph_version("phase10-cockpit");
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
    let corpus = ResearchEvidenceCorpus::builder("phase10-corpus")
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
    let source = AgentSourceRecord::new(
        "codex",
        "local-agent-session",
        "transcript:digest:phase10",
        "tool:digest:hadwiger-cli",
    )
    .unwrap();
    let batch = AgentExplorationBatch::builder("phase10-agent-pass", source)
        .with_experiment_proposal(
            AgentExperimentProposal::new("try-local-edge-rewire", version.reference())
                .with_rationale("test whether the failure is tied to one local edge orbit")
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap();
    let agent = admit_agent_exploration_batch_checked(&handle, &corpus, batch).unwrap();
    (handle, version, corpus, frontier, catalog, agent)
}

fn session_builder(
    corpus: ResearchEvidenceCorpus,
    frontier: DiscoveryFrontier,
    catalog: HadwigerResearchInvariantCatalog,
    agent: AgentExplorationAdmissionChecked,
) -> ResearchCockpitSessionBuilder {
    ResearchCockpitSession::builder("frontier-cockpit-a")
        .with_corpus(corpus)
        .with_frontier(frontier)
        .with_invariant_catalog(catalog)
        .with_agent_admission(agent)
        .finish()
        .unwrap()
}

fn partial_admission_for_missing_checker_evidence(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
) -> HadwigerPartialAdmissionExplanation {
    let blocked = admit_plane_lower_bound_claim_checked(
        handle,
        PlaneLowerBoundClaimRequest::new("phase10-missing-checker-proof", version),
    )
    .expect_err("missing checker evidence should block lower-bound admission");
    let HadwigerProofClaimAdmissionError::Blocked(blocked) = blocked else {
        panic!("expected blocked proof claim");
    };
    explain_partial_admission(
        handle,
        ExplainPartialAdmissionRequest::from_blocked_proof_claim(
            "phase10-missing-checker-partial",
            version,
            &blocked,
        )
        .with_surviving_artifact(version.reference())
        .with_repair_obligation("supply admitted checker evidence before proof admission")
        .unwrap(),
    )
    .unwrap()
}

#[test]
fn equivalent_sessions_replay_to_same_action_packet() {
    let (handle, _version, corpus, frontier, catalog, agent) = cockpit_context();
    let left = assemble_research_cockpit_session_checked(
        &handle,
        session_builder(
            corpus.clone(),
            frontier.clone(),
            catalog.clone(),
            agent.clone(),
        ),
    )
    .unwrap();
    let right = assemble_research_cockpit_session_checked(
        &handle,
        session_builder(corpus, frontier, catalog, agent),
    )
    .unwrap();

    let left_packet = derive_research_cockpit_action_packet_checked(&handle, &left).unwrap();
    let replayed = replay_research_cockpit_session_checked(&handle, &right).unwrap();

    assert_eq!(left.artifact_digest(), right.artifact_digest());
    assert_eq!(left_packet.artifact_digest(), replayed.artifact_digest());
    assert_eq!(left_packet.source_session_digest(), left.session_digest());
    assert!(left_packet.counters().query_readiness_checks() > 0);
    assert!(!left_packet.admits_theorem_authority());
}

#[test]
fn cockpit_blocks_suppressed_and_advisory_actions_without_authority() {
    let (handle, _version, corpus, frontier, catalog, agent) = cockpit_context();
    let session = assemble_research_cockpit_session_checked(
        &handle,
        session_builder(corpus, frontier, catalog, agent),
    )
    .unwrap();

    let packet = derive_research_cockpit_action_packet_checked(&handle, &session).unwrap();

    assert!(packet.actions().iter().any(|action| action.blocker()
        == Some(ResearchCockpitActionBlocker::SuppressedDeadEndEquivalence)));
    assert!(packet
        .actions()
        .iter()
        .any(|action| action.eligibility() == ResearchCockpitActionEligibility::AdvisoryOnly));
    assert!(packet.counters().suppression_hits() > 0);
    assert!(!packet.equivalence_classes().is_empty());
}

#[test]
fn cockpit_classifies_partial_admission_as_missing_checker_proof_work() {
    let (handle, version, _corpus, frontier, catalog, agent) = cockpit_context();
    let partial = partial_admission_for_missing_checker_evidence(&handle, &version);
    let corpus = ResearchEvidenceCorpus::builder("phase10-partial-corpus")
        .with_retained_artifact(version.reference())
        .with_partial_admission(partial)
        .unwrap()
        .finish()
        .unwrap();
    let session = assemble_research_cockpit_session_checked(
        &handle,
        session_builder(corpus, frontier, catalog, agent),
    )
    .unwrap();

    let packet = derive_research_cockpit_action_packet_checked(&handle, &session).unwrap();

    assert!(packet.actions().iter().any(|action| {
        action.kind() == ResearchCockpitActionKind::ProofAdmission
            && action.blocker() == Some(ResearchCockpitActionBlocker::MissingCheckerEvidence)
    }));
    assert!(packet
        .equivalence_classes()
        .iter()
        .any(|class| class.scope() == ResearchCockpitEquivalenceScope::ProofAdmission));
}

#[test]
fn changed_corpus_and_stale_derived_frontier_change_cockpit_identity() {
    let (handle, version, corpus, frontier, catalog, agent) = cockpit_context();
    let derived = recompute_derived_discovery_frontier(&handle, &corpus).unwrap();
    let changed_corpus = ResearchEvidenceCorpus::builder("phase10-corpus-changed")
        .with_graph_version(version.reference())
        .finish()
        .unwrap();
    let baseline = assemble_research_cockpit_session_checked(
        &handle,
        session_builder(
            corpus.clone(),
            frontier.clone(),
            catalog.clone(),
            agent.clone(),
        ),
    )
    .unwrap();
    let stale = assemble_research_cockpit_session_checked(
        &handle,
        session_builder(changed_corpus, frontier, catalog, agent)
            .with_derived_frontier_state(derived)
            .finish()
            .unwrap(),
    )
    .unwrap();

    let packet = derive_research_cockpit_action_packet_checked(&handle, &stale).unwrap();

    assert_ne!(baseline.artifact_digest(), stale.artifact_digest());
    assert!(
        packet
            .actions()
            .iter()
            .any(|action| action.blocker()
                == Some(ResearchCockpitActionBlocker::StaleDerivedFrontier))
    );
}

#[test]
fn tile_contact_equivalence_converges_and_blocks_duplicate_checker_work() {
    let handle = handle();
    let left =
        TileContactGraphSignature::from_edges("tile-a", [("center", "north"), ("center", "east")])
            .unwrap();
    let right =
        TileContactGraphSignature::from_edges("tile-b", [("east", "center"), ("north", "center")])
            .unwrap();
    let changed = TileContactGraphSignature::from_edges("tile-c", [("center", "south")]).unwrap();
    let equivalent = TileEquivalenceWitness::builder(
        "different-shape-same-contact-class",
        TileEquivalenceScope::ContactConstraint,
    )
    .with_left_contact_signature(left)
    .with_right_contact_signature(right)
    .finish()
    .unwrap();
    let unsupported = TileEquivalenceWitness::builder(
        "different-contact-class",
        TileEquivalenceScope::ContactConstraint,
    )
    .with_left_contact_signature(
        TileContactGraphSignature::from_edges("tile-a", [("center", "north"), ("center", "east")])
            .unwrap(),
    )
    .with_right_contact_signature(changed)
    .finish()
    .unwrap();

    let checked = declare_tile_equivalence_witness_checked(&handle, equivalent).unwrap();

    assert!(checked.blocks_duplicate_checker_work());
    assert!(!checked.admits_theorem_authority());
    assert_eq!(unsupported.posture(), TileEquivalencePosture::Unsupported);
    assert_ne!(
        checked.witness().artifact_digest(),
        unsupported.artifact_digest()
    );
}

#[test]
fn tile_witness_in_session_blocks_duplicate_checker_action() {
    let (handle, _version, _corpus, frontier, catalog, agent) = cockpit_context();
    let witness = TileEquivalenceWitness::builder(
        "session-tile-equivalence",
        TileEquivalenceScope::ContactConstraint,
    )
    .with_left_contact_signature(
        TileContactGraphSignature::from_edges("tile-a", [("center", "north")]).unwrap(),
    )
    .with_right_contact_signature(
        TileContactGraphSignature::from_edges("tile-b", [("north", "center")]).unwrap(),
    )
    .finish()
    .unwrap();
    let corpus = ResearchEvidenceCorpus::builder("phase10-tile-corpus")
        .with_retained_artifact(witness.reference())
        .finish()
        .unwrap();
    let session = assemble_research_cockpit_session_checked(
        &handle,
        session_builder(corpus, frontier, catalog, agent),
    )
    .unwrap();

    let packet = derive_research_cockpit_action_packet_checked(&handle, &session).unwrap();

    assert!(packet.actions().iter().any(|action| {
        action.blocker() == Some(ResearchCockpitActionBlocker::TileEquivalenceDuplicateCheckerWork)
    }));
    assert_eq!(packet.counters().tile_equivalence_hits(), 1);
    assert!(packet
        .equivalence_classes()
        .iter()
        .any(|class| class.scope() == ResearchCockpitEquivalenceScope::TileContact));
}

#[test]
fn certification_bundle_retains_digest_inventory_and_scenarios() {
    let (handle, _version, corpus, frontier, catalog, agent) = cockpit_context();
    let session = assemble_research_cockpit_session_checked(
        &handle,
        session_builder(corpus, frontier, catalog, agent),
    )
    .unwrap();

    let bundle = certify_hadwiger_milestone_one_bundle_checked(&handle, &session).unwrap();

    assert!(bundle
        .digest_inventory()
        .contains_discovery_frontier_digest());
    assert!(bundle
        .scenarios()
        .iter()
        .any(|row| row.name() == "dead-end suppression"));
    assert!(bundle
        .scenarios()
        .iter()
        .all(|row| row.retained_digest().len() > 16));
    assert!(!bundle.registers_query_invariant_authority());
    assert!(!bundle.admits_theorem_authority());
    assert!(bundle
        .digest_inventory()
        .rows()
        .iter()
        .any(|(name, _)| name.starts_with("retained_evidence:")));
}
