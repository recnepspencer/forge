use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("Hadwiger handle should admit")
}

fn complete_graph(vertex_count: usize) -> GraphVersion {
    let handle = handle();
    let graph_id = format!("screening-k{vertex_count}");
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(&graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate declaration should admit");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
    let labels = (0..vertex_count)
        .map(|index| format!("v{index}"))
        .collect::<Vec<_>>();
    let mut builder = GraphVersion::builder(graph.reference(), "v1");
    for label in &labels {
        builder = builder.with_vertex(label).unwrap();
    }
    for left in 0..labels.len() {
        for right in (left + 1)..labels.len() {
            builder = builder
                .with_undirected_edge(&labels[left], &labels[right])
                .unwrap();
        }
    }
    builder.finish().unwrap()
}

fn path_graph(vertex_count: usize) -> GraphVersion {
    let handle = handle();
    let graph_id = format!("screening-path-{vertex_count}");
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(&graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate declaration should admit");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
    let labels = (0..vertex_count)
        .map(|index| format!("v{index}"))
        .collect::<Vec<_>>();
    let mut builder = GraphVersion::builder(graph.reference(), "v1");
    for label in &labels {
        builder = builder.with_vertex(label).unwrap();
    }
    for pair in labels.windows(2) {
        builder = builder.with_undirected_edge(&pair[0], &pair[1]).unwrap();
    }
    builder.finish().unwrap()
}

#[test]
fn screening_catalog_materializes_all_solved_filter_nodes() {
    let handle = handle();

    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();

    assert_eq!(catalog.nodes().len(), 35);
    for family in CandidateScreeningInvariantFamily::all() {
        assert!(catalog.has_family(*family), "missing {family:?}");
    }
    assert!(catalog.has_family(CandidateScreeningInvariantFamily::ExactUnitDistanceConflict));
    assert!(catalog.has_family(CandidateScreeningInvariantFamily::MinkowskiDifferenceGeometry));
    assert!(catalog.has_family(CandidateScreeningInvariantFamily::CandidateNoveltyNonIsomorphism));
    assert!(!catalog.admits_theorem_authority());
    assert!(!catalog.registers_query_invariant_authority());
}

#[test]
fn screening_nodes_preserve_authority_and_promotion_requirements() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();

    let exact = node(
        &catalog,
        CandidateScreeningInvariantFamily::ExactUnitDistanceConflict,
    );
    let ranking = node(
        &catalog,
        CandidateScreeningInvariantFamily::MaximumDegreeSanityCheck,
    );
    let sat = node(
        &catalog,
        CandidateScreeningInvariantFamily::SatIlpSixColorability,
    );

    assert_eq!(
        exact.authority(),
        CandidateScreeningInvariantAuthority::ExactCheckerReady
    );
    assert_eq!(
        ranking.authority(),
        CandidateScreeningInvariantAuthority::HeuristicRanking
    );
    assert_eq!(
        sat.applicability(),
        CandidateScreeningApplicability::FiniteConflictGraph
    );
    assert!(sat.promotion_requirement().contains("checked refutation"));
    assert!(!ranking.admits_theorem_authority());
    assert!(!ranking.registers_query_invariant_authority());
}

#[test]
fn equivalent_screening_catalogs_converge_and_nodes_can_live_on_corpus() {
    let handle = handle();
    let left = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let right = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let exact = node(
        &left,
        CandidateScreeningInvariantFamily::ExactUnitDistanceConflict,
    );
    let novelty = node(
        &left,
        CandidateScreeningInvariantFamily::CandidateNoveltyNonIsomorphism,
    );

    let corpus = ResearchEvidenceCorpus::builder("screening-node-corpus")
        .with_retained_artifact(exact.reference())
        .with_retained_artifact(novelty.reference())
        .finish()
        .unwrap();

    assert_eq!(left.artifact_digest(), right.artifact_digest());
    assert!(corpus.has_reference(&exact.reference()));
    assert!(corpus.has_reference(&novelty.reference()));
    assert!(!corpus.admits_theorem_authority());
}

#[test]
fn direct_graph_screening_filters_reject_complete_seven_obstruction() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = complete_graph(7);

    let clique = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::CliqueNumberLowerBound,
        &graph,
    )
    .unwrap();
    let independence = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::IndependenceNumberLowerBound,
        &graph,
    )
    .unwrap();
    let sat = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::SatIlpSixColorability,
        &graph,
    )
    .unwrap();
    let weighted = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::WeightedIndependenceNumberBound,
        &graph,
    )
    .unwrap();
    let spectral = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::SpectralHoffmanBound,
        &graph,
    )
    .unwrap();
    let critical = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::CriticalSubgraphExtraction,
        &graph,
    )
    .unwrap();

    assert!(clique.rejects_candidate());
    assert!(independence.rejects_candidate());
    assert!(sat.rejects_candidate());
    assert!(weighted.rejects_candidate());
    assert!(spectral.rejects_candidate());
    assert_eq!(critical.verdict(), CandidateScreeningVerdict::Priority);
    assert_eq!(
        clique.mode(),
        CandidateScreeningEvaluationMode::DirectGraphAlgorithm
    );
    assert!(weighted.evidence().contains("unit_weights=true"));
    assert!(spectral.evidence().contains("hoffman_bound=7"));
    assert!(critical
        .evidence()
        .contains("smaller_non_colorable_subgraph=false"));
    assert!(sat.evidence().contains("exact_replay_colorable=false"));
    assert!(sat.evidence().contains("verification_posture="));
}

#[test]
fn perfect_graph_subclass_sanity_rejects_bipartite_lower_bound_candidates() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = path_graph(7);

    let perfect = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::PerfectGraphSanityCheck,
        &graph,
    )
    .unwrap();

    assert!(perfect.rejects_candidate());
    assert!(perfect.evidence().contains("perfect_subclass=bipartite"));
    assert!(perfect.evidence().contains("detected=true"));
}

#[test]
fn every_screening_family_produces_checked_evaluation_artifact() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let subject = complete_graph(2).reference();
    let mut evaluations = Vec::new();

    for family in CandidateScreeningInvariantFamily::all() {
        let certificate = CandidateScreeningCertificate::checked(
            *family,
            subject.clone(),
            format!("certificate:{}", family.as_str()),
            CandidateScreeningVerdict::Passed,
            format!("checked basis for {}", family.title()),
        )
        .unwrap();
        let evaluation = evaluate_certificate_screening_invariant_checked(
            &handle,
            &catalog,
            *family,
            certificate,
        )
        .unwrap();
        assert_eq!(evaluation.family(), *family);
        assert_eq!(
            evaluation.mode(),
            CandidateScreeningEvaluationMode::CheckedCertificate
        );
        assert!(!evaluation.admits_theorem_authority());
        evaluations.push(evaluation);
    }

    let report = assemble_candidate_screening_report_checked(&handle, &catalog, evaluations)
        .expect("screening report should assemble");

    assert_eq!(report.evaluations().len(), 35);
    assert_eq!(report.rejected_count(), 0);
    assert!(!report.admits_theorem_authority());
}

#[test]
fn direct_graph_screening_reports_explicit_budget_limits() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = complete_graph(21);

    let err = evaluate_graph_screening_invariant_checked(
        &handle,
        &catalog,
        CandidateScreeningInvariantFamily::CliqueNumberLowerBound,
        &graph,
    )
    .expect_err("large exact clique screening should expose a budget stop");

    assert_eq!(
        err,
        CandidateScreeningError::GraphScreeningBudgetExceeded {
            vertex_count: 21,
            exact_limit: 20
        }
    );
}

fn node(
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
) -> &CandidateScreeningInvariantNode {
    catalog
        .nodes()
        .iter()
        .find(|node| node.family() == family)
        .expect("screening node should exist")
}
