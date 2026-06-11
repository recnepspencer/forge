use hadwiger_research::facade::*;

pub fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("Hadwiger handle admits")
}

pub fn graph_version(graph_id: &str) -> GraphVersion {
    let handle = handle();
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration admits");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
    GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("a")
        .unwrap()
        .with_vertex("b")
        .unwrap()
        .with_undirected_edge("a", "b")
        .unwrap()
        .finish()
        .unwrap()
}

pub fn complete_graph(
    handle: &HadwigerResearchHandle,
    graph_id: &str,
    labels: &[&str],
) -> GraphVersion {
    let declaration = declare_research_request_checked(
        handle,
        CandidateGraphDeclaration::new(graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration admits");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
    let mut builder = GraphVersion::builder(graph.reference(), "v1");
    for label in labels {
        builder = builder.with_vertex(*label).unwrap();
    }
    for left in 0..labels.len() {
        for right in (left + 1)..labels.len() {
            builder = builder
                .with_undirected_edge(labels[left], labels[right])
                .unwrap();
        }
    }
    builder.finish().unwrap()
}
