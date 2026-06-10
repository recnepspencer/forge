use hadwiger_research::facade::{
    admit_hadwiger_research_handle, generate_k_colorability_certificate_with_varisat_checked,
    import_frontier_graph_seed_checked, run_frontier_seed_exploration_iterations_checked,
    verify_algebraic_unit_distance_embedding_checked, verify_k_colorability_checked,
    verify_k_colorability_with_certificate_checked, AlgebraicGraphEmbedding, AlgebraicPoint2,
    ColorabilityVerificationPosture, ColoringProofCertificate, ExactRational,
    FrontierExplorationEvidenceBundle, FrontierExplorationEvidencePosture,
    FrontierExplorationRunRequest, FrontierGraphSeedImport, FrontierMutationPolicy,
    HadwigerAlgebraicGeometryError, HadwigerCanonicalArtifact, HadwigerColorabilityError,
    HadwigerResearchHandle, HadwigerResearchOperatingContext, QuadraticFieldElement,
};

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("Hadwiger handle admits")
}

fn unit_edge_seed() -> FrontierGraphSeedImport {
    FrontierGraphSeedImport::dimacs_edge_list("unit-edge-seed", "v1", "p edge 2 1\ne 1 2\n")
        .expect("seed shape admits")
        .with_source_digest("sha256:unit-edge")
}

fn unit_edge_seed_with_algebraic_certificate() -> FrontierGraphSeedImport {
    unit_edge_seed().with_algebraic_embedding_certificate(
        "embedding retained-unit-edge\nv 1 0/1+0/1*sqrt(0) 0/1+0/1*sqrt(0)\nv 2 1/1+0/1*sqrt(0) 0/1+0/1*sqrt(0)\n",
    )
}

#[test]
fn heule_parts_517_seed_imports_with_retained_provenance() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_parts_517())
            .expect("public 517 seed imports");

    assert_eq!(imported.graph_version().vertex_count(), 517);
    assert_eq!(imported.graph_version().edge_count(), 2579);
    assert_eq!(imported.seed_artifact().source_family(), "heule_parts_517");
    assert!(!imported.seed_artifact().admits_theorem_authority());
}

#[test]
fn heule_parts_517_seed_can_run_candidate_virtual_edge_iterations() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_parts_517())
            .expect("public 517 seed imports");

    let run = run_frontier_seed_exploration_iterations_checked(
        &handle,
        FrontierExplorationRunRequest::new(
            "heule-parts-517-candidate-pass",
            imported.seed_artifact(),
        )
        .with_iteration_count(5)
        .expect("iteration count admits"),
    )
    .expect("candidate frontier loop runs");

    assert_eq!(run.iterations().len(), 5);
    assert!(run
        .motif_reports()
        .iter()
        .all(|report| report.terminal_forcing_motif().is_none()));
    assert!(run
        .motif_reports()
        .iter()
        .all(|report| report.contains_virtual_edge_candidate()));
    assert!(!run.admits_theorem_authority());
}

#[test]
fn algebraic_quadratic_unit_distance_replay_is_exact() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, unit_edge_seed()).expect("seed imports");
    let half = ExactRational::fraction(1, 2).expect("half");
    let sqrt_three_over_two =
        QuadraticFieldElement::quadratic(ExactRational::integer(0), half, 3).expect("sqrt(3)/2");
    let embedding = AlgebraicGraphEmbedding::builder(
        imported.graph_version().reference(),
        "quadratic-unit-edge",
    )
    .with_vertex("1", AlgebraicPoint2::rational_integer(0, 0))
    .expect("left coordinate")
    .with_vertex(
        "2",
        AlgebraicPoint2::new(
            QuadraticFieldElement::rational(ExactRational::fraction(1, 2).expect("half")),
            sqrt_three_over_two,
        ),
    )
    .expect("right coordinate")
    .finish()
    .expect("embedding shape admits");

    let checked = verify_algebraic_unit_distance_embedding_checked(
        &handle,
        imported.graph_version(),
        embedding,
    )
    .expect("algebraic replay admits");

    assert!(checked.verification().is_admitted());
    assert!(checked
        .unit_distance_aspect()
        .satisfies_mathematical_dependency());
}

#[test]
fn algebraic_embedding_can_be_replayed_from_retained_seed_certificate() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, unit_edge_seed_with_algebraic_certificate())
            .expect("seed imports");

    let embedding = AlgebraicGraphEmbedding::from_seed_certificate(imported.seed_artifact())
        .expect("retained algebraic certificate parses");
    let checked = verify_algebraic_unit_distance_embedding_checked(
        &handle,
        imported.graph_version(),
        embedding,
    )
    .expect("retained certificate replay admits");

    assert!(checked.verification().is_admitted());
}

#[test]
fn algebraic_replay_rejects_unsupported_mixed_fields() {
    let sqrt_two =
        QuadraticFieldElement::quadratic(ExactRational::integer(0), ExactRational::integer(1), 2)
            .expect("sqrt2");
    let sqrt_three =
        QuadraticFieldElement::quadratic(ExactRational::integer(0), ExactRational::integer(1), 3)
            .expect("sqrt3");
    let left = AlgebraicPoint2::new(sqrt_two, QuadraticFieldElement::integer(0));
    let right = AlgebraicPoint2::new(sqrt_three, QuadraticFieldElement::integer(0));
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, unit_edge_seed()).expect("seed imports");
    let embedding = AlgebraicGraphEmbedding::builder(imported.graph_version().reference(), "bad")
        .with_vertex("1", left)
        .expect("left")
        .with_vertex("2", right)
        .expect("right")
        .finish()
        .expect("embedding");

    assert!(matches!(
        verify_algebraic_unit_distance_embedding_checked(
            &handle,
            imported.graph_version(),
            embedding
        ),
        Err(HadwigerAlgebraicGeometryError::UnsupportedMixedField { .. })
    ));
}

#[test]
fn certificate_replay_admits_non_one_colorability_and_rejects_bad_digest() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, unit_edge_seed()).expect("seed imports");
    let baseline = verify_k_colorability_checked(&handle, imported.graph_version(), 1)
        .expect("small exhaustive path provides encoding");
    let certificate = ColoringProofCertificate::from_rup_clauses(
        baseline.encoding().cnf_digest_token(),
        vec![vec![-2], vec![]],
    )
    .expect("certificate shape admits");

    let checked = verify_k_colorability_with_certificate_checked(
        &handle,
        imported.graph_version(),
        1,
        certificate,
    )
    .expect("RUP certificate replays");

    assert_eq!(
        checked.colorability_verification().posture(),
        ColorabilityVerificationPosture::UnsatVerified
    );

    let bad = ColoringProofCertificate::from_rup_clauses("wrong-digest", vec![vec![-2], vec![]])
        .expect("bad cert shape");
    assert_eq!(
        verify_k_colorability_with_certificate_checked(&handle, imported.graph_version(), 1, bad)
            .unwrap_err(),
        HadwigerColorabilityError::CertificateDigestMismatch
    );
}

#[test]
fn varisat_native_certificate_generation_is_retained_and_replayed() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, unit_edge_seed()).expect("seed imports");
    let certificate =
        generate_k_colorability_certificate_with_varisat_checked(imported.graph_version(), 1)
            .expect("native proof generated");

    assert_eq!(
        certificate.format(),
        hadwiger_research::facade::ColoringProofCertificateFormat::VarisatNative
    );
    assert!(!certificate.proof_bytes().is_empty());

    let checked = verify_k_colorability_with_certificate_checked(
        &handle,
        imported.graph_version(),
        1,
        certificate,
    )
    .expect("native proof replays");

    assert_eq!(
        checked.colorability_verification().posture(),
        ColorabilityVerificationPosture::UnsatVerified
    );
}

#[test]
fn varisat_native_certificate_generation_refuses_satisfiable_graphs() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, unit_edge_seed()).expect("seed imports");

    assert_eq!(
        generate_k_colorability_certificate_with_varisat_checked(imported.graph_version(), 2)
            .unwrap_err(),
        HadwigerColorabilityError::SatisfiableFormula
    );
}

#[test]
fn frontier_evidence_bundle_reports_missing_and_ready_evidence() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, unit_edge_seed()).expect("seed imports");
    let candidate = FrontierExplorationEvidenceBundle::new(imported.seed_artifact());

    assert_eq!(
        candidate.posture(),
        FrontierExplorationEvidencePosture::CandidateOnly
    );
    assert_eq!(candidate.missing_evidence().len(), 2);

    let embedding = AlgebraicGraphEmbedding::builder(imported.graph_version().reference(), "edge")
        .with_vertex("1", AlgebraicPoint2::rational_integer(0, 0))
        .expect("left")
        .with_vertex("2", AlgebraicPoint2::rational_integer(1, 0))
        .expect("right")
        .finish()
        .expect("embedding");
    let unit = verify_algebraic_unit_distance_embedding_checked(
        &handle,
        imported.graph_version(),
        embedding,
    )
    .expect("unit distance admits");
    let certificate =
        generate_k_colorability_certificate_with_varisat_checked(imported.graph_version(), 1)
            .expect("native proof generated");
    let color = verify_k_colorability_with_certificate_checked(
        &handle,
        imported.graph_version(),
        1,
        certificate,
    )
    .expect("native proof replays");

    let ready = candidate
        .with_unit_distance_verification(&unit)
        .with_colorability_verification(&color);
    assert_eq!(
        ready.posture(),
        FrontierExplorationEvidencePosture::TerminalForcingReady
    );
    assert!(ready.missing_evidence().is_empty());

    let request = FrontierExplorationRunRequest::from_evidence_bundle("bundle-loop", &ready)
        .with_iteration_count(5)
        .expect("iteration count admits");
    let run = run_frontier_seed_exploration_iterations_checked(&handle, request)
        .expect("bundle-backed loop runs");

    assert!(run
        .motif_reports()
        .iter()
        .all(|report| report.terminal_forcing_motif().is_some()));
}

#[test]
fn five_iteration_frontier_loop_records_query_lowered_motif_work() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, unit_edge_seed()).expect("seed imports");
    let embedding = AlgebraicGraphEmbedding::builder(imported.graph_version().reference(), "edge")
        .with_vertex("1", AlgebraicPoint2::rational_integer(0, 0))
        .expect("left")
        .with_vertex("2", AlgebraicPoint2::rational_integer(1, 0))
        .expect("right")
        .finish()
        .expect("embedding");
    let unit = verify_algebraic_unit_distance_embedding_checked(
        &handle,
        imported.graph_version(),
        embedding,
    )
    .expect("unit distance admits");
    let baseline = verify_k_colorability_checked(&handle, imported.graph_version(), 1)
        .expect("small exhaustive path provides encoding");
    let certificate = ColoringProofCertificate::from_rup_clauses(
        baseline.encoding().cnf_digest_token(),
        vec![vec![-2], vec![]],
    )
    .expect("certificate shape admits");
    let color = verify_k_colorability_with_certificate_checked(
        &handle,
        imported.graph_version(),
        1,
        certificate,
    )
    .expect("certificate replay admits");

    let run = run_frontier_seed_exploration_iterations_checked(
        &handle,
        FrontierExplorationRunRequest::new("frontier-loop-smoke", imported.seed_artifact())
            .with_unit_distance_verification(&unit)
            .with_colorability_verification(&color)
            .with_mutation_policy(FrontierMutationPolicy::CoreMinimizationAndVirtualEdges)
            .with_iteration_count(5)
            .expect("iteration count admits"),
    )
    .expect("frontier loop admits");

    assert_eq!(run.iterations().len(), 5);
    assert_eq!(run.motif_reports().len(), 5);
    assert!(run
        .motif_reports()
        .iter()
        .all(|report| report.contains_virtual_edge_candidate()));
    assert!(run
        .iterations()
        .iter()
        .skip(1)
        .all(|iteration| iteration.suppression_hits() > 0));
    assert!(!run.admits_theorem_authority());
}
