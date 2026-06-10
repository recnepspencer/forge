use hadwiger_research::facade::{
    admit_hadwiger_research_handle, build_frontier_research_projection_graph_checked,
    import_frontier_graph_seed_checked, FrontierGraphSeedImport, FrontierResearchProjectionRequest,
    HadwigerCanonicalArtifact, HadwigerResearchOperatingContext, RetainedFrontierColoringProof,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_510_exact())
            .expect("exact Heule 510 seed imports");
    let proof_manifest = RetainedFrontierColoringProof::heule_510_not_four_colorable()
        .expect("retained proof manifest parses");
    let projection = build_frontier_research_projection_graph_checked(
        &handle,
        FrontierResearchProjectionRequest::new(
            "heule-510-agent-projection",
            imported.seed_artifact(),
            imported.graph_version(),
        )
        .with_retained_proof_manifest(&proof_manifest),
    )
    .expect("projection graph builds");

    println!(
        "projection_digest={}",
        projection.artifact_digest().stable_token()
    );
    println!("nodes={}", projection.nodes().len());
    println!("edges={}", projection.edges().len());
    println!(
        "degree_histogram={}",
        projection
            .degree_histogram()
            .iter()
            .map(|row| format!("{}:{}", row.degree(), row.vertex_count()))
            .collect::<Vec<_>>()
            .join(",")
    );
    for row in projection.top_pressure_vertices(8) {
        println!(
            "pressure_vertex={} degree={} triangles={} score={}",
            row.vertex_label(),
            row.degree(),
            row.triangle_count(),
            row.pressure_score()
        );
    }
    if let Some(motif) = projection.best_pressure_halo_motif() {
        println!(
            "motif={} hub={} score={}",
            motif.novelty_signature(),
            motif.hub_vertex(),
            motif.pressure_score()
        );
        for satellite in motif.satellites() {
            println!(
                "satellite={} degree={} spokes={}",
                satellite.vertex_label(),
                satellite.degree(),
                satellite.common_spokes().join("|")
            );
        }
    }
}
