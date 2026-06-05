use hadwiger_research::facade::{
    admit_hadwiger_research_handle, declare_research_request_checked, CandidateGraphDeclaration,
    ExactGraphEmbedding, ExactPoint2, GraphIdentity, GraphVersion, HadwigerResearchOperatingContext,
    HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
    verify_unit_distance_embedding_checked,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("real handle should admit");
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration should admit");
    let graph = GraphIdentity::from_query_declaration("candidate-a", declaration.into())
        .expect("graph identity should build");
    let graph_version = GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("a")
        .expect("vertex shape should be valid")
        .with_vertex("b")
        .expect("vertex shape should be valid")
        .with_undirected_edge("a", "b")
        .expect("edge shape should be valid")
        .finish()
        .expect("graph version should build");
    let embedding = ExactGraphEmbedding::builder(graph_version.reference(), "embedding-a")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .expect("coordinate shape should be valid")
        .with_vertex("b", ExactPoint2::integer(1, 0))
        .expect("coordinate shape should be valid")
        .finish()
        .expect("embedding shape should be valid");

    let checked = verify_unit_distance_embedding_checked(
        &handle,
        &graph_version,
        embedding,
    )
    .expect("real unit-distance verification should run");

    assert!(checked.verification().is_admitted());
    assert!(checked
        .unit_distance_aspect()
        .satisfies_mathematical_dependency());
}
