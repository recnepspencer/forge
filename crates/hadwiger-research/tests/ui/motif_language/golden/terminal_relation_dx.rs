use hadwiger_research::facade::{
    certify_terminal_forcing_relation_checked, declare_research_request_checked,
    hadwiger_research_domain_package, verify_k_colorability_checked, CandidateGraphDeclaration,
    GraphIdentity, GraphVersion, HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
    HadwigerResearchDomainEntry, HadwigerResearchHandle, HadwigerResearchOperatingContext,
    HadwigerResearchQueryExt, MotifArtifact, MotifSeedDeclaration, MotifTerminal,
    TerminalForcingRelationCertificate, TerminalForcingRelationKind,
    TerminalForcingStudyDeclaration,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

fn main() {
    let handle = installed_declarations().expect("handle admits");
    let motif_declaration =
        declare_research_request_checked(&handle, MotifSeedDeclaration::new("terminal-motif"))
            .admitted()
            .expect("motif declaration admits");
    let motif = MotifArtifact::builder("terminal-motif", motif_declaration.into())
        .with_terminal(MotifTerminal::new("a").expect("terminal admits"))
        .expect("terminal admits")
        .with_terminal(MotifTerminal::new("b").expect("terminal admits"))
        .expect("terminal admits")
        .finish()
        .expect("motif builds");

    let graph_declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new("terminal-k2").with_graph_version("v1"),
    )
    .admitted()
    .expect("graph declaration admits");
    let graph = GraphIdentity::from_query_declaration("terminal-k2", graph_declaration.into())
        .expect("graph identity builds");
    let graph_version = GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("a")
        .expect("vertex admits")
        .with_vertex("b")
        .expect("vertex admits")
        .with_undirected_edge("a", "b")
        .expect("edge admits")
        .finish()
        .expect("graph version builds");
    let color_checked =
        verify_k_colorability_checked(&handle, &graph_version, 1).expect("colorability checks");
    let certificate = TerminalForcingRelationCertificate::from_checked_colorability(
        "a-b-must-differ",
        motif.reference(),
        TerminalForcingRelationKind::MustDiffer,
        ["a", "b"],
        color_checked.colorability_verification().clone(),
        color_checked.not_k_colorable_aspect().clone(),
    )
    .expect("terminal certificate admits");
    let relation = certify_terminal_forcing_relation_checked(
        &handle,
        TerminalForcingStudyDeclaration::new("terminal-study", motif.reference().stable_token())
            .with_terminal("a")
            .expect("terminal admits")
            .with_terminal("b")
            .expect("terminal admits"),
        &motif,
        certificate,
    )
    .expect("terminal relation checks");

    assert!(relation.is_checked());
    assert!(relation.satisfies_terminal_relation_dependency());
    assert!(!relation.admits_theorem_authority());
}

fn installed_declarations() -> Result<HadwigerResearchHandle, String> {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect("identity.id", "identity.id").map_err(|error| error.to_string())?;
    let workspace = in_memory_test_runtime().with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-terminal-relation-dx").map_err(|error| error.to_string())?;
    let installed = workspace.domain(HadwigerResearchDomainEntry).map_err(|error| error.to_string())?;
    installed.research_declarations(&workspace, HadwigerResearchOperatingContext::default())
        .map_err(|error| error.to_string())
}
