use hadwiger_research::facade::{
    admit_hadwiger_research_handle, declare_research_request_checked, CandidateGraphDeclaration,
    GraphIdentity, GraphVersion, HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
    HadwigerResearchAdmissionError, HadwigerResearchOperatingContext,
};

fn graph_artifact_dx() -> Result<(), HadwigerResearchAdmissionError> {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())?;

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
    let _ = graph_artifact_dx as fn() -> Result<(), HadwigerResearchAdmissionError>;
}
