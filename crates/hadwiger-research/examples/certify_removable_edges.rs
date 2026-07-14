//! Proof-bearing confirmation for removable-edge candidates on the exact
//! Heule-510 seed.
//!
//! For each edge in `CERTIFY_EDGES` (comma-separated `u-v`), this generates
//! a varisat-native UNSAT certificate for the *raw deletion graph* `G - e`
//! at k = 4 and replays it, so the claim "`G - e` is still 5-chromatic on
//! the inherited exact embedding" carries its own independently replayed
//! proof rather than relying on the contraction implication. Certificates
//! are written next to the retained Heule proof under
//! `crates/hadwiger-research/src/frontier_seeds/`.

use std::fs;
use std::time::Instant;

use hadwiger_research::facade::{
    admit_hadwiger_research_handle, generate_k_colorability_certificate_with_varisat_checked,
    import_frontier_graph_seed_checked, verify_k_colorability_with_certificate_checked,
    ColorabilityVerificationPosture, FrontierGraphSeedImport, GraphVersion,
    HadwigerCanonicalArtifact, HadwigerResearchOperatingContext,
};

fn main() {
    let edges = std::env::var("CERTIFY_EDGES").expect("CERTIFY_EDGES lists u-v edge labels");
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let imported =
        import_frontier_graph_seed_checked(&handle, FrontierGraphSeedImport::heule_510_exact())
            .expect("Heule 510 imports");
    let graph = imported.graph_version();
    println!(
        "baseline vertices={} edges={} digest={}",
        graph.vertex_count(),
        graph.edge_count(),
        graph.artifact_digest().stable_token()
    );
    for spec in edges.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let Some((left, right)) = spec.split_once('-') else {
            println!("edge={spec} finding=MALFORMED-SPEC");
            continue;
        };
        let mutated = rebuild_without_edge(graph, left, right);
        let started = Instant::now();
        let certificate =
            match generate_k_colorability_certificate_with_varisat_checked(&mutated, 4) {
                Ok(certificate) => certificate,
                Err(error) => {
                    println!("edge={left}-{right} finding=CERTIFICATE-FAILED error={error:?}");
                    continue;
                }
            };
        let generation_seconds = started.elapsed().as_secs_f64();
        let replayed = verify_k_colorability_with_certificate_checked(
            &handle,
            &mutated,
            4,
            certificate.clone(),
        )
        .expect("generated certificate replays through the checked lane");
        let posture = replayed.colorability_verification().posture();
        assert_eq!(posture, ColorabilityVerificationPosture::UnsatVerified);
        let proof_path = format!(
            "crates/hadwiger-research/src/frontier_seeds/heule_510_minus_{left}_{right}.varisat"
        );
        fs::write(&proof_path, certificate.proof_bytes()).expect("proof file writes");
        println!(
            "edge={left}-{right} finding=REMOVABLE-CERTIFIED posture={posture:?} \
             aspect_dependency={} cnf_digest={} proof_bytes={} generation_seconds={:.1} \
             proof_path={proof_path} mutated_digest={}",
            replayed
                .not_k_colorable_aspect()
                .satisfies_mathematical_dependency(),
            certificate.cnf_digest(),
            certificate.proof_bytes().len(),
            generation_seconds,
            mutated.artifact_digest().stable_token()
        );
    }
}

fn rebuild_without_edge(graph: &GraphVersion, left: &str, right: &str) -> GraphVersion {
    let mut builder = GraphVersion::builder(
        graph.parent_artifacts()[0].clone(),
        format!("certify-delete-edge-{left}-{right}"),
    );
    for vertex in graph.vertices() {
        builder = builder
            .with_vertex(vertex.vertex_label())
            .expect("vertex shape admits");
    }
    for edge in graph.edges() {
        let (edge_left, edge_right) = edge.endpoints();
        let is_target = (edge_left == left && edge_right == right)
            || (edge_left == right && edge_right == left);
        if !is_target {
            builder = builder
                .with_undirected_edge(edge_left, edge_right)
                .expect("edge shape admits");
        }
    }
    builder.finish().expect("mutated graph shape admits")
}
