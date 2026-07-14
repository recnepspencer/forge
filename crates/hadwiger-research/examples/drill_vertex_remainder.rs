//! Targeted vertex-deletion drill for vertices named in `DRILL_TARGETS`
//! (comma-separated labels). Completes criticality coverage left open by the
//! WL-class representative sweep in `drill_edge_criticality`.

use std::time::Instant;

use hadwiger_research::facade::{
    admit_hadwiger_research_handle, import_frontier_graph_seed_checked,
    verify_k_colorability_checked, ColorabilityVerificationPosture, FrontierGraphSeedImport,
    GraphVersion, HadwigerCanonicalArtifact, HadwigerResearchOperatingContext,
};

fn main() {
    let targets = std::env::var("DRILL_TARGETS").expect("DRILL_TARGETS lists vertex labels");
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
    for target in targets.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        let mutated = rebuild_without_vertex(graph, target);
        let started = Instant::now();
        let checked = verify_k_colorability_checked(&handle, &mutated, 4)
            .expect("colorability checker returns a posture");
        let posture = checked.colorability_verification().posture();
        let finding = match posture {
            ColorabilityVerificationPosture::SatModelVerified => "CRITICAL",
            ColorabilityVerificationPosture::UnsatVerified
            | ColorabilityVerificationPosture::UnsupportedCertificateBudget => {
                "REMOVABLE-CANDIDATE"
            }
            ColorabilityVerificationPosture::Rejected => "UNDETERMINED",
        };
        println!(
            "mutation=vertex-deletion:{} class=0000000000000000 class_size=1 pressure=0 posture={:?} finding={} vertices={} edges={} seconds={:.1} digest={}",
            target,
            posture,
            finding,
            mutated.vertex_count(),
            mutated.edge_count(),
            started.elapsed().as_secs_f64(),
            mutated.artifact_digest().stable_token()
        );
    }
}

fn rebuild_without_vertex(graph: &GraphVersion, target: &str) -> GraphVersion {
    let mut builder = GraphVersion::builder(
        graph.parent_artifacts()[0].clone(),
        format!("drill-delete-vertex-{target}"),
    );
    for vertex in graph.vertices() {
        if vertex.vertex_label() != target {
            builder = builder
                .with_vertex(vertex.vertex_label())
                .expect("vertex shape admits");
        }
    }
    for edge in graph.edges() {
        let (left, right) = edge.endpoints();
        if left != target && right != target {
            builder = builder
                .with_undirected_edge(left, right)
                .expect("edge shape admits");
        }
    }
    builder.finish().expect("mutated graph shape admits")
}
