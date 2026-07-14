use hadwiger_research::facade::{
    admit_hadwiger_research_handle, import_frontier_graph_seed_checked,
    verify_k_colorability_checked, EdgeIdentity, FrontierGraphSeedImport, GraphVersion,
    HadwigerCanonicalArtifact, HadwigerResearchOperatingContext,
};

const SPOKE_PAIRS: [(&str, &str, [&str; 2]); 6] = [
    ("1", "100", ["29", "33"]),
    ("1", "85", ["30", "32"]),
    ("1", "88", ["31", "37"]),
    ("1", "91", ["26", "36"]),
    ("1", "94", ["27", "35"]),
    ("1", "97", ["28", "34"]),
];

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
    for (hub, satellite, spokes) in SPOKE_PAIRS {
        for spoke in spokes {
            run_mutation(
                &handle,
                graph,
                format!("delete-hub-spoke:{hub}-{spoke}:satellite-{satellite}"),
                |edge| *edge != normalized(hub, spoke),
            );
            run_mutation(
                &handle,
                graph,
                format!("delete-satellite-spoke:{satellite}-{spoke}:hub-{hub}"),
                |edge| *edge != normalized(satellite, spoke),
            );
            run_vertex_deletion(&handle, graph, hub, satellite, spoke);
        }
    }
}

fn run_mutation(
    handle: &hadwiger_research::facade::HadwigerResearchHandle,
    graph: &GraphVersion,
    label: String,
    keep_edge: impl Fn(&(String, String)) -> bool,
) {
    let mutated = rebuild_graph(graph, &label, |vertex| Some(vertex.to_string()), keep_edge);
    report_colorability(handle, label, mutated);
}

fn run_vertex_deletion(
    handle: &hadwiger_research::facade::HadwigerResearchHandle,
    graph: &GraphVersion,
    hub: &str,
    satellite: &str,
    spoke: &str,
) {
    let label = format!("delete-spoke-vertex:{spoke}:hub-{hub}:satellite-{satellite}");
    let mutated = rebuild_graph(
        graph,
        &label,
        |vertex| (vertex != spoke).then(|| vertex.to_string()),
        |_| true,
    );
    report_colorability(handle, label, mutated);
}

fn report_colorability(
    handle: &hadwiger_research::facade::HadwigerResearchHandle,
    label: String,
    graph: GraphVersion,
) {
    let checked = verify_k_colorability_checked(handle, &graph, 4)
        .expect("colorability checker returns a posture");
    println!(
        "mutation={} vertices={} edges={} posture={:?} aspect_dependency={} digest={}",
        label,
        graph.vertex_count(),
        graph.edge_count(),
        checked.colorability_verification().posture(),
        checked
            .not_k_colorable_aspect()
            .satisfies_mathematical_dependency(),
        graph.artifact_digest().stable_token()
    );
}

fn rebuild_graph(
    graph: &GraphVersion,
    version_id: &str,
    map_vertex: impl Fn(&str) -> Option<String>,
    keep_edge: impl Fn(&(String, String)) -> bool,
) -> GraphVersion {
    let mut builder = GraphVersion::builder(graph.parent_artifacts()[0].clone(), version_id);
    for vertex in graph.vertices() {
        if let Some(label) = map_vertex(vertex.vertex_label()) {
            builder = builder.with_vertex(label).expect("vertex shape admits");
        }
    }
    for edge in graph.edges() {
        let normalized = normalized_edge(edge);
        if !keep_edge(&normalized) {
            continue;
        }
        if let (Some(left), Some(right)) = (map_vertex(&normalized.0), map_vertex(&normalized.1)) {
            builder = builder
                .with_undirected_edge(left, right)
                .expect("edge shape admits");
        }
    }
    builder.finish().expect("mutated graph shape admits")
}

fn normalized_edge(edge: &EdgeIdentity) -> (String, String) {
    let (left, right) = edge.endpoints();
    normalized(left, right)
}

fn normalized(left: &str, right: &str) -> (String, String) {
    if left <= right {
        (left.to_string(), right.to_string())
    } else {
        (right.to_string(), left.to_string())
    }
}
