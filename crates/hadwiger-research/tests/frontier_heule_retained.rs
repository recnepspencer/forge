use hadwiger_research::facade::{
    admit_hadwiger_research_handle, build_frontier_research_projection_graph_checked,
    import_frontier_graph_seed_checked, run_frontier_seed_exploration_iterations_checked,
    verify_algebraic_unit_distance_embedding_checked,
    verify_k_colorability_with_certificate_checked, AlgebraicGraphEmbedding,
    ColorabilityVerificationPosture, FrontierExplorationRunRequest, FrontierGraphSeedImport,
    FrontierResearchProjectionRequest, HadwigerResearchHandle, HadwigerResearchOperatingContext,
    RetainedFrontierColoringProof,
};

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("Hadwiger handle admits")
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
fn heule_510_exact_seed_imports_with_retained_coordinates() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_510_exact())
            .expect("public exact 510 seed imports");

    assert_eq!(imported.graph_version().vertex_count(), 510);
    assert_eq!(imported.graph_version().edge_count(), 2504);
    assert_eq!(
        imported.seed_artifact().source_family(),
        "heule_510_exact_algebraic"
    );
    assert!(imported
        .seed_artifact()
        .algebraic_embedding_certificate()
        .is_some());
}

#[test]
fn heule_510_exact_seed_replays_all_algebraic_unit_edges() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_510_exact())
            .expect("public exact 510 seed imports");
    let embedding = AlgebraicGraphEmbedding::from_seed_certificate(imported.seed_artifact())
        .expect("retained exact coordinates parse");

    let checked = verify_algebraic_unit_distance_embedding_checked(
        &handle,
        imported.graph_version(),
        embedding,
    )
    .expect("all retained exact edges replay");

    assert!(checked.verification().is_admitted());
    assert!(checked
        .unit_distance_aspect()
        .satisfies_mathematical_dependency());
}

#[test]
fn heule_510_retained_proof_manifest_is_available_without_loading_large_payload() {
    let proof = RetainedFrontierColoringProof::heule_510_not_four_colorable()
        .expect("retained proof manifest parses");

    assert_eq!(proof.proof_id(), "heule-510-not-4-colorable-varisat-native");
    assert_eq!(proof.seed_id(), "heule-510-exact");
    assert_eq!(proof.color_count(), 4);
    assert_eq!(
        proof.cnf_digest(),
        "006adc4a1a31cb89ea248074fb8af7b5d087d280dad15d86839e5f0e72b69692"
    );
    assert_eq!(
        proof.proof_sha256(),
        "a7aaea46876f67c0fc0ee04e94e03550f16343589a500825da6bd3f94f6af62f"
    );
    assert_eq!(proof.proof_byte_length(), 801_960_073);
}

#[test]
fn heule_510_projection_graph_exposes_pressure_halo_motif() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_510_exact())
            .expect("public exact 510 seed imports");

    let projection = build_frontier_research_projection_graph_checked(
        &handle,
        FrontierResearchProjectionRequest::new(
            "heule-510-agent-projection",
            imported.seed_artifact(),
            imported.graph_version(),
        ),
    )
    .expect("projection graph builds");
    let top = projection.top_pressure_vertices(1);
    let motif = projection
        .best_pressure_halo_motif()
        .expect("pressure halo motif exists");
    let satellites = motif
        .satellites()
        .iter()
        .map(|row| row.vertex_label().to_string())
        .collect::<Vec<_>>();

    assert_eq!(top[0].vertex_label(), "1");
    assert_eq!(top[0].degree(), 36);
    assert_eq!(top[0].triangle_count(), 30);
    assert_eq!(motif.hub_vertex(), "1");
    assert_eq!(satellites, ["100", "85", "88", "91", "94", "97"]);
    assert!(motif
        .satellites()
        .iter()
        .all(|row| row.degree() == 24 && row.common_spokes().len() == 2));
    assert!(!projection.admits_theorem_authority());
}

#[test]
#[ignore = "replays the 802 MB retained Heule 510 proof payload"]
fn heule_510_retained_proof_replays_non_four_colorability_when_payload_is_present() {
    let handle = handle();
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_510_exact())
            .expect("public exact 510 seed imports");
    let proof = RetainedFrontierColoringProof::heule_510_not_four_colorable()
        .expect("retained proof manifest parses");
    assert!(
        proof.proof_file_available(),
        "generate the retained proof with: cargo run -p hadwiger-research --example generate_heule_510_proof"
    );
    let certificate = proof
        .load_certificate()
        .expect("retained proof hash verifies and certificate builds");

    let checked = verify_k_colorability_with_certificate_checked(
        &handle,
        imported.graph_version(),
        proof.color_count(),
        certificate,
    )
    .expect("retained Heule proof replays");

    assert_eq!(
        checked.colorability_verification().posture(),
        ColorabilityVerificationPosture::UnsatVerified
    );
    assert!(checked
        .not_k_colorable_aspect()
        .satisfies_mathematical_dependency());
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
