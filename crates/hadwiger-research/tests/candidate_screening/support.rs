use hadwiger_research::facade::*;

pub fn handle() -> HadwigerResearchHandle {
    crate::installed_support::installed_hadwiger_research_handle()
        .expect("Hadwiger handle should admit")
}

pub fn complete_graph(vertex_count: usize) -> GraphVersion {
    let handle = handle();
    let graph_id = format!("screening-k{vertex_count}");
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(&graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate declaration should admit");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
    let labels = vertex_labels(vertex_count);
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

pub fn path_graph(vertex_count: usize) -> GraphVersion {
    let handle = handle();
    let graph_id = format!("screening-path-{vertex_count}");
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(&graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate declaration should admit");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
    let labels = vertex_labels(vertex_count);
    let mut builder = GraphVersion::builder(graph.reference(), "v1");
    for label in &labels {
        builder = builder.with_vertex(label).unwrap();
    }
    for pair in labels.windows(2) {
        builder = builder.with_undirected_edge(&pair[0], &pair[1]).unwrap();
    }
    builder.finish().unwrap()
}

pub fn node(
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
) -> &CandidateScreeningInvariantNode {
    catalog
        .nodes()
        .iter()
        .find(|node| node.family() == family)
        .expect("screening node should exist")
}

pub fn transcript(label: &str) -> ScreeningSolverTranscript {
    ScreeningSolverTranscript::new("test-solver", "0.0.0", label, "candidate").unwrap()
}

fn vertex_labels(vertex_count: usize) -> Vec<String> {
    (0..vertex_count).map(|index| format!("v{index}")).collect()
}
