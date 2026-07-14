use hadwiger_research::facade::{
    declare_research_request_checked, hadwiger_research_domain_package, CandidateGraphDeclaration,
    ExactGraphEmbedding, ExactPoint2, GraphIdentity, GraphVersion, HadwigerResearchOperatingContext,
    HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt, HadwigerResearchDomainEntry,
    HadwigerResearchHandle, HadwigerResearchQueryExt,
    verify_unit_distance_embedding_checked,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

fn main() {
    let handle = installed_declarations().expect("real handle should admit");
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

fn installed_declarations() -> Result<HadwigerResearchHandle, String> {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect("identity.id", "identity.id").map_err(|error| error.to_string())?;
    let workspace = in_memory_test_runtime().with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-checker-artifact-dx").map_err(|error| error.to_string())?;
    let installed = workspace.domain(HadwigerResearchDomainEntry).map_err(|error| error.to_string())?;
    installed.research_declarations(&workspace, HadwigerResearchOperatingContext::default())
        .map_err(|error| error.to_string())
}
