use hadwiger_research::facade::*;

#[derive(Clone, Debug)]
struct IterationSummary {
    index: usize,
    seed_label: String,
    motif_digest: String,
    observations: usize,
    hypotheses: usize,
    plans: usize,
    suppression_hits: usize,
    actions: usize,
    packet_actions: usize,
}

#[test]
fn run_five_iteration_edge_local_motif_loop() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger research handle should admit");
    let graph_version = graph_version(&handle, "edge-local-unit-distance-seed");
    let motif = edge_local_unit_distance_motif(&handle);
    let mut summaries = Vec::new();

    for (index, bad_length) in [2, 3, 4, 2, 3].into_iter().enumerate() {
        summaries.push(run_iteration(
            &handle,
            &graph_version,
            &motif,
            index + 1,
            bad_length,
        ));
    }

    println!("seed=edge-local-unit-distance-failure");
    for summary in &summaries {
        println!(
            "iteration={} label={} motif={} observations={} hypotheses={} plans={} suppression_hits={} cockpit_actions={} packet_actions={}",
            summary.index,
            summary.seed_label,
            &summary.motif_digest[..16],
            summary.observations,
            summary.hypotheses,
            summary.plans,
            summary.suppression_hits,
            summary.actions,
            summary.packet_actions
        );
    }

    assert_eq!(summaries.len(), 5);
    assert!(summaries.iter().all(|summary| summary.observations >= 2));
    assert!(summaries.iter().all(|summary| summary.hypotheses >= 2));
    assert!(summaries.iter().all(|summary| summary.suppression_hits > 0));
    assert!(summaries.iter().all(|summary| summary.packet_actions > 0));
}

fn run_iteration(
    handle: &HadwigerResearchHandle,
    graph_version: &GraphVersion,
    motif: &MotifArtifact,
    index: usize,
    bad_length: i64,
) -> IterationSummary {
    let label = format!("motif-loop-{index}-d{bad_length}");
    let rejection = checker_rejection(handle, graph_version, &label, bad_length);
    let initial = ResearchEvidenceCorpus::builder(format!("{label}-initial"))
        .with_graph_version(graph_version.reference())
        .with_retained_artifact(motif.reference())
        .with_checker_rejection(rejection.clone())
        .unwrap()
        .finish()
        .unwrap();
    let failure = attach_failure_to_research_graph(
        handle,
        &initial,
        rejection.reusable_negative_evidence().unwrap(),
        FailureScope::edge_local(graph_version.reference(), "a", "b").unwrap(),
    )
    .unwrap();
    let corpus = ResearchEvidenceCorpus::builder(format!("{label}-corpus"))
        .with_graph_version(graph_version.reference())
        .with_retained_artifact(motif.reference())
        .with_checker_rejection(rejection)
        .unwrap()
        .with_graph_resident_failure(failure)
        .finish()
        .unwrap();

    let observations = mine_research_patterns(handle, &corpus).unwrap();
    let hypotheses = propose_invariant_hypotheses(handle, &corpus, &observations).unwrap();
    let plans = plan_next_experiments(handle, &corpus, &hypotheses).unwrap();
    let frontier = update_discovery_frontier(
        handle,
        &corpus,
        observations.clone(),
        hypotheses.clone(),
        plans.clone(),
    )
    .unwrap();
    let catalog = draft_research_graph_invariant_catalog(handle, &corpus, &frontier).unwrap();
    let registration = register_research_graph_invariants_checked(handle, &catalog).unwrap();
    let source = AgentSourceRecord::new(
        "codex",
        "local-agent-session",
        format!("transcript:digest:{label}"),
        "tool:digest:exploration-loop",
    )
    .unwrap();
    let agent_batch = AgentExplorationBatch::builder(format!("{label}-agent"), source)
        .with_motif_suggestion(
            AgentMotifSuggestion::new(format!("{label}-motif-suggestion"), motif.reference())
                .with_observation("unit-distance edge failures remain edge-local after mutation")
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap();
    let agent = admit_agent_exploration_batch_checked(handle, &corpus, agent_batch).unwrap();
    let session = assemble_research_cockpit_session_checked(
        handle,
        ResearchCockpitSession::builder(format!("{label}-session"))
            .with_corpus(corpus)
            .with_frontier(frontier)
            .with_invariant_catalog(catalog)
            .with_agent_admission(agent)
            .finish()
            .unwrap(),
    )
    .unwrap();
    let packet = derive_research_cockpit_action_packet_checked(handle, &session).unwrap();
    let iteration = derive_tiling_iteration_packet_checked(
        handle,
        TilingIterationPacketRequest::lower_bound_obstruction(format!("{label}-packet"))
            .from_cockpit_session(&session)
            .with_evidence_basis("retained edge-local unit-distance rejection")
            .with_required_checker_lane("exact_unit_distance_embedding")
            .with_required_checker_lane("motif_terminal_forcing_study")
            .with_reactivation_obligation("supply exact coordinates for the failed unit edge")
            .with_expected_information_gain("extract reusable edge-local terminal motif")
            .unwrap(),
    )
    .unwrap();

    assert_eq!(registration.custom_invariant_registrations().len(), 5);
    assert!(session.frontier().research_graph_legality().is_enforced());
    assert!(!packet.admits_theorem_authority());
    assert!(!iteration.executes_checker_work());

    IterationSummary {
        index,
        seed_label: label,
        motif_digest: motif.artifact_digest().stable_token().to_string(),
        observations: observations.len(),
        hypotheses: hypotheses.len(),
        plans: plans.experiment_plans().len(),
        suppression_hits: session.frontier().scorecard().suppression_hits(),
        actions: packet.actions().len(),
        packet_actions: iteration.actions().len(),
    }
}

fn graph_version(handle: &HadwigerResearchHandle, graph_id: &str) -> GraphVersion {
    let declaration = declare_research_request_checked(
        handle,
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

fn edge_local_unit_distance_motif(handle: &HadwigerResearchHandle) -> MotifArtifact {
    let declaration = declare_research_request_checked(
        handle,
        MotifSeedDeclaration::new("edge-local-unit-distance-failure")
            .with_source_family("checker_rejection")
            .with_novelty_signature("unit_edge_length_mismatch:a:b"),
    )
    .admitted()
    .expect("motif seed declaration should admit");
    let source = declaration.clone().into();
    let builder = MotifArtifact::builder("edge-local-unit-distance-failure", source)
        .with_source_family("checker_rejection")
        .unwrap()
        .with_novelty_signature("unit_edge_length_mismatch:a:b")
        .unwrap()
        .with_geometry_template(
            MotifGeometryTemplateReference::new("exact-point-edge-local-template").unwrap(),
        )
        .with_vertex(MotifVertex::new("a").unwrap())
        .unwrap()
        .with_vertex(MotifVertex::new("b").unwrap())
        .unwrap()
        .with_terminal(MotifTerminal::new("left").unwrap())
        .unwrap()
        .with_terminal(MotifTerminal::new("right").unwrap())
        .unwrap()
        .with_parameter(MotifParameterBinding::new("required_squared_distance", "1").unwrap())
        .unwrap()
        .with_unit_edge(MotifUnitEdge::new("a", "b").unwrap())
        .unwrap()
        .with_forbidden_same_color_pair(MotifForbiddenSameColorPair::new("left", "right").unwrap())
        .unwrap();
    build_motif_from_seed_declaration_checked(handle, declaration, builder).unwrap()
}

fn checker_rejection(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
    label: &str,
    bad_length: i64,
) -> HadwigerRejectionExplanation {
    let embedding = ExactGraphEmbedding::builder(version.reference(), format!("{label}-embedding"))
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(bad_length as i128, 0))
        .unwrap()
        .finish()
        .unwrap();
    let checked = verify_unit_distance_embedding_checked(handle, version, embedding).unwrap();
    explain_rejection(
        handle,
        ExplainRejectionRequest::for_checker_rejection(
            format!("{label}-unit-distance-rejection"),
            version,
            checked.verification(),
        )
        .with_rejected_aspect(checked.unit_distance_aspect())
        .with_repair_obligation("replace the failed edge coordinates with squared distance 1")
        .unwrap(),
    )
    .unwrap()
}
