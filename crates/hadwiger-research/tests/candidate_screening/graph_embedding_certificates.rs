use hadwiger_research::facade::*;

use super::support::{complete_graph, handle, path_graph, transcript};

#[test]
fn graph_embedding_screening_has_query_readiness_for_all_seven() {
    let handle = handle();

    assert_ready::<UnitDistanceEmbeddabilityScreeningDeclaration>(&handle);
    assert_ready::<RigidityRealizationScreeningDeclaration>(&handle);
    assert_ready::<ExactArithmeticIntervalScreeningDeclaration>(&handle);
    assert_ready::<SymmetryOrbitReductionScreeningDeclaration>(&handle);
    assert_ready::<ExhaustiveLocalNeighborhoodScreeningDeclaration>(&handle);
    assert_ready::<KnownObstructionContainmentScreeningDeclaration>(&handle);
    assert_ready::<CandidateNoveltyScreeningDeclaration>(&handle);
}

#[test]
fn exact_embedding_lanes_replay_unit_distance_rigidity_and_intervals() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = path_graph(2);
    let embedding = unit_path_embedding(&graph);

    let unit = evaluate_unit_distance_embeddability_screening_checked(
        &handle,
        &catalog,
        &graph,
        UnitDistanceEmbeddabilityCertificate::new(
            "unit-path",
            embedding.clone(),
            transcript("unit-path"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(unit.verdict(), CandidateScreeningVerdict::Passed);
    assert!(unit.evidence().contains("query_declaration_digest="));

    let rigidity = evaluate_rigidity_realization_screening_checked(
        &handle,
        &catalog,
        &graph,
        RigidityRealizationCertificate::new(
            "rigid-edge",
            embedding.clone(),
            RigidityRealizationPosture::LocallyRigid,
            transcript("rigid-edge"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(rigidity.verdict(), CandidateScreeningVerdict::Passed);
    assert!(rigidity.evidence().contains("rigidity_rank=1"));

    let interval = evaluate_exact_arithmetic_interval_screening_checked(
        &handle,
        &catalog,
        graph.reference(),
        Some(&graph),
        ExactArithmeticIntervalCertificate::point_pair(
            "unit-pair",
            embedding,
            "v0",
            "v1",
            ExactArithmeticIntervalExpectation::UnitContained,
            transcript("unit-pair"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(interval.rejects_candidate());
}

#[test]
fn exact_embedding_lanes_reject_corrupt_certificates() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = path_graph(2);

    let non_unit = evaluate_unit_distance_embeddability_screening_checked(
        &handle,
        &catalog,
        &graph,
        UnitDistanceEmbeddabilityCertificate::new(
            "bad-unit",
            non_unit_path_embedding(&graph),
            transcript("bad-unit"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(non_unit.rejects_candidate());

    let err = evaluate_rigidity_realization_screening_checked(
        &handle,
        &catalog,
        &graph,
        RigidityRealizationCertificate::new(
            "bad-rigidity-posture",
            unit_path_embedding(&graph),
            RigidityRealizationPosture::Flexible,
            transcript("bad-rigidity"),
        )
        .unwrap(),
    )
    .expect_err("rank replay must reject wrong rigidity posture");
    assert_replay_error(
        err,
        CandidateScreeningInvariantFamily::RigidityRealizationConsistency,
        "rigidity_posture_mismatch",
    );

    let err = evaluate_exact_arithmetic_interval_screening_checked(
        &handle,
        &catalog,
        graph.reference(),
        Some(&graph),
        ExactArithmeticIntervalCertificate::point_pair(
            "bad-interval",
            unit_path_embedding(&graph),
            "v0",
            "v1",
            ExactArithmeticIntervalExpectation::UnitExcluded,
            transcript("bad-interval"),
        )
        .unwrap(),
    )
    .expect_err("exact interval expectation must replay");
    assert_replay_error(
        err,
        CandidateScreeningInvariantFamily::ExactArithmeticIntervalCertificate,
        "exact_interval_expectation_mismatch",
    );
}

#[test]
fn graph_index_lanes_replay_symmetry_neighborhood_obstruction_and_novelty() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let triangle = complete_graph(3);

    let symmetry = evaluate_symmetry_orbit_reduction_screening_checked(
        &handle,
        &catalog,
        &triangle,
        SymmetryOrbitReductionCertificate::new(
            "triangle-swap",
            vec![vec![
                ("v0".to_string(), "v1".to_string()),
                ("v1".to_string(), "v0".to_string()),
                ("v2".to_string(), "v2".to_string()),
            ]],
            transcript("triangle-symmetry"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(symmetry.verdict(), CandidateScreeningVerdict::Priority);
    assert!(symmetry.evidence().contains("orbit_count=2"));

    let path = path_graph(3);
    let neighborhood = evaluate_exhaustive_local_neighborhood_screening_checked(
        &handle,
        &catalog,
        &path,
        ExhaustiveLocalNeighborhoodCertificate::new(
            "path-neighborhood",
            "v1",
            1,
            vec!["v2".to_string(), "v0".to_string(), "v1".to_string()],
            transcript("path-neighborhood"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(neighborhood.verdict(), CandidateScreeningVerdict::Passed);

    let obstruction = evaluate_known_obstruction_containment_screening_checked(
        &handle,
        &catalog,
        &triangle,
        KnownObstructionContainmentCertificate::new(
            "edge-in-triangle",
            complete_graph(2),
            vec![
                ("v0".to_string(), "v0".to_string()),
                ("v1".to_string(), "v1".to_string()),
            ],
            transcript("edge-in-triangle"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(obstruction.rejects_candidate());

    let novelty = evaluate_candidate_novelty_screening_checked(
        &handle,
        &catalog,
        &triangle,
        CandidateNoveltyCertificate::new(
            "known-triangle",
            "v=3;e=3;degrees=[2, 2, 2];wl2=0.0.0",
            2,
            transcript("known-triangle"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(novelty.rejects_candidate());
}

#[test]
fn graph_index_lanes_reject_invalid_replay() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let path = path_graph(3);

    let err = evaluate_symmetry_orbit_reduction_screening_checked(
        &handle,
        &catalog,
        &path,
        SymmetryOrbitReductionCertificate::new(
            "bad-path-swap",
            vec![vec![
                ("v0".to_string(), "v1".to_string()),
                ("v1".to_string(), "v0".to_string()),
                ("v2".to_string(), "v2".to_string()),
            ]],
            transcript("bad-path-swap"),
        )
        .unwrap(),
    )
    .expect_err("non-automorphism must not replay");
    assert_replay_error(
        err,
        CandidateScreeningInvariantFamily::SymmetryOrbitReduction,
        "symmetry_permutation_not_automorphism",
    );

    let err = evaluate_exhaustive_local_neighborhood_screening_checked(
        &handle,
        &catalog,
        &path,
        ExhaustiveLocalNeighborhoodCertificate::new(
            "bad-neighborhood",
            "v1",
            1,
            vec!["v1".to_string()],
            transcript("bad-neighborhood"),
        )
        .unwrap(),
    )
    .expect_err("wrong neighborhood must not replay");
    assert_replay_error(
        err,
        CandidateScreeningInvariantFamily::ExhaustiveLocalNeighborhood,
        "local_neighborhood_not_exhaustive",
    );
}

#[test]
fn changed_embedding_certificate_changes_evaluation_digest() {
    let handle = handle();
    let catalog = draft_candidate_screening_invariant_catalog_checked(&handle).unwrap();
    let graph = path_graph(2);

    let left = evaluate_unit_distance_embeddability_screening_checked(
        &handle,
        &catalog,
        &graph,
        UnitDistanceEmbeddabilityCertificate::new(
            "unit-path",
            unit_path_embedding(&graph),
            transcript("unit-path"),
        )
        .unwrap(),
    )
    .unwrap();
    let right = evaluate_unit_distance_embeddability_screening_checked(
        &handle,
        &catalog,
        &graph,
        UnitDistanceEmbeddabilityCertificate::new(
            "unit-path-shifted",
            shifted_unit_path_embedding(&graph),
            transcript("unit-path"),
        )
        .unwrap(),
    )
    .unwrap();

    assert_ne!(left.artifact_digest(), right.artifact_digest());
}

fn assert_ready<I: HadwigerResearchDeclarationInput>(handle: &HadwigerResearchHandle) {
    assert!(!research_declaration_entry_readiness::<I>(handle)
        .rows()
        .is_empty());
}

fn assert_replay_error(
    error: CandidateScreeningError,
    family: CandidateScreeningInvariantFamily,
    reason: &'static str,
) {
    assert_eq!(
        error,
        CandidateScreeningError::CertificateReplayRejected { family, reason }
    );
}

fn unit_path_embedding(graph: &GraphVersion) -> ExactGraphEmbedding {
    ExactGraphEmbedding::builder(graph.reference(), "unit-path")
        .with_vertex("v0", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("v1", ExactPoint2::integer(1, 0))
        .unwrap()
        .finish()
        .unwrap()
}

fn shifted_unit_path_embedding(graph: &GraphVersion) -> ExactGraphEmbedding {
    ExactGraphEmbedding::builder(graph.reference(), "unit-path-shifted")
        .with_vertex("v0", ExactPoint2::integer(3, 0))
        .unwrap()
        .with_vertex("v1", ExactPoint2::integer(4, 0))
        .unwrap()
        .finish()
        .unwrap()
}

fn non_unit_path_embedding(graph: &GraphVersion) -> ExactGraphEmbedding {
    ExactGraphEmbedding::builder(graph.reference(), "non-unit-path")
        .with_vertex("v0", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("v1", ExactPoint2::integer(2, 0))
        .unwrap()
        .finish()
        .unwrap()
}
