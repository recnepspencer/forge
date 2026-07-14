use hadwiger_research::facade::{
    declare_research_request_checked, hadwiger_research_domain_package, CandidateGraphDeclaration,
    GraphIdentity, GraphVersion, HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
    HadwigerResearchDomainEntry, HadwigerResearchHandle, HadwigerResearchOperatingContext,
    HadwigerResearchQueryExt,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

fn graph_artifact_dx() -> Result<(), String> {
    let handle = installed_declarations()?;

    let checked = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new("moser-spindle-seed").with_graph_version("v1"),
    );

    let declaration = checked.admitted().expect("phase-2 smoke declaration admits");
    let source = declaration.into();

    let graph = GraphIdentity::from_query_declaration("moser-spindle-seed", source)
        .expect("graph identity shape is valid");
    let version = GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("a")
        .expect("vertex a is valid")
        .with_vertex("b")
        .expect("vertex b is valid")
        .with_undirected_edge("a", "b")
        .expect("edge is valid")
        .finish()
        .expect("graph version is valid");

    assert_eq!(graph.authority_owner().as_str(), "query_declaration");
    assert_ne!(graph.artifact_digest(), version.artifact_digest());
    assert_eq!(version.parent_artifacts(), &[graph.reference()]);

    Ok(())
}

fn main() {
    let _ = graph_artifact_dx as fn() -> Result<(), String>;
}

fn installed_declarations() -> Result<HadwigerResearchHandle, String> {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect("identity.id", "identity.id").map_err(|error| error.to_string())?;
    let workspace = in_memory_test_runtime().with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-graph-artifact-dx").map_err(|error| error.to_string())?;
    let installed = workspace.domain(HadwigerResearchDomainEntry).map_err(|error| error.to_string())?;
    installed.research_declarations(&workspace, HadwigerResearchOperatingContext::default())
        .map_err(|error| error.to_string())
}
