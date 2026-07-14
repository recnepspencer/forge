use std::time::Instant;

use hadwiger_research::facade::{
    admit_hadwiger_research_handle, import_frontier_graph_seed_checked,
    verify_k_colorability_checked, FrontierGraphSeedImport, GraphVersion,
    HadwigerCanonicalArtifact, HadwigerResearchOperatingContext,
};

fn main() {
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

    let sat_probe = delete_vertex(graph, "33", "probe-delete-vertex-33");
    time_solve("delete-vertex-33", &handle, &sat_probe);

    let started = Instant::now();
    let checked =
        verify_k_colorability_checked(&handle, graph, 4).expect("full-graph solve returns");
    println!(
        "probe=full-graph-unsat posture={:?} seconds={:.1}",
        checked.colorability_verification().posture(),
        started.elapsed().as_secs_f64()
    );
}

fn time_solve(
    label: &str,
    handle: &hadwiger_research::facade::HadwigerResearchHandle,
    graph: &GraphVersion,
) {
    let started = Instant::now();
    let checked =
        verify_k_colorability_checked(handle, graph, 4).expect("mutated solve returns a posture");
    println!(
        "probe={} vertices={} edges={} posture={:?} seconds={:.1}",
        label,
        graph.vertex_count(),
        graph.edge_count(),
        checked.colorability_verification().posture(),
        started.elapsed().as_secs_f64()
    );
}

fn delete_vertex(graph: &GraphVersion, target: &str, version_id: &str) -> GraphVersion {
    let mut builder = GraphVersion::builder(graph.parent_artifacts()[0].clone(), version_id);
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
