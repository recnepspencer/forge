use hadwiger_research::facade::*;
use worth_query::facade::foundation::WorthQueryOrdinaryOutcome;

fn query_source(graph_id: &str, graph_version: &str) -> HadwigerQueryDeclarationReference {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("default Hadwiger research handle should admit");
    let checked = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(graph_id).with_graph_version(graph_version),
    );
    checked
        .admitted()
        .expect("candidate graph declaration should admit")
        .into()
}

fn graph(source: HadwigerQueryDeclarationReference) -> GraphIdentity {
    GraphIdentity::from_query_declaration("candidate-a", source)
        .expect("query declaration should build graph identity")
}

fn version(graph: &GraphIdentity) -> GraphVersion {
    GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("a")
        .expect("vertex a shape should be valid")
        .with_vertex("b")
        .expect("vertex b shape should be valid")
        .with_undirected_edge("b", "a")
        .expect("edge should normalize")
        .finish()
        .expect("graph version shape should be valid")
}

#[test]
fn graph_identity_and_version_keep_query_source_and_parent_links() {
    let source = query_source("candidate-a", "v1");
    let graph = graph(source.clone());
    let version = version(&graph);

    assert_eq!(graph.authority_owner().as_str(), "query_declaration");
    assert_eq!(graph.artifact_kind(), HadwigerArtifactKind::GraphIdentity);
    assert_eq!(version.artifact_kind(), HadwigerArtifactKind::GraphVersion);
    assert_ne!(graph.artifact_digest(), version.artifact_digest());
    assert_eq!(version.parent_artifacts(), &[graph.reference()]);

    match graph.source_reference() {
        HadwigerArtifactSourceReference::QueryDeclaration(reference) => {
            assert_eq!(reference, &source);
            assert_eq!(reference.domain_key(), "WORTH.hadwiger.research");
            assert_eq!(
                reference.declaration_family_key(),
                "hadwiger.candidate_graph"
            );
            assert!(!reference.handle_identity_digest().is_empty());
            assert!(!reference.declaration_digest().is_empty());
            assert_eq!(
                reference.canonicalization_version(),
                "WORTH.query.declaration.v1"
            );
        }
        other => panic!("expected query declaration source, got {other:?}"),
    }
}

#[test]
fn graph_versions_converge_for_equivalent_undirected_edges() {
    let graph = graph(query_source("candidate-a", "v1"));
    let left = GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("a")
        .unwrap()
        .with_vertex("b")
        .unwrap()
        .with_undirected_edge("a", "b")
        .unwrap()
        .finish()
        .unwrap();
    let right = GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("b")
        .unwrap()
        .with_vertex("a")
        .unwrap()
        .with_undirected_edge("b", "a")
        .unwrap()
        .finish()
        .unwrap();

    assert_eq!(left.artifact_digest(), right.artifact_digest());
    assert_eq!(left.edges()[0].endpoints(), ("a", "b"));
    assert_eq!(right.edges()[0].endpoints(), ("a", "b"));
}

#[test]
fn digest_changes_when_source_payload_parent_or_authority_changes() {
    let source = query_source("candidate-a", "v1");
    let alternate_source = query_source("candidate-a", "v2");
    let graph = graph(source);
    let alternate_graph =
        GraphIdentity::from_query_declaration("candidate-a", alternate_source).unwrap();

    let baseline = version(&graph);
    let changed_version = GraphVersion::builder(graph.reference(), "v2")
        .with_vertex("a")
        .unwrap()
        .with_vertex("b")
        .unwrap()
        .with_undirected_edge("a", "b")
        .unwrap()
        .finish()
        .unwrap();
    let changed_vertex = GraphVersion::builder(graph.reference(), "v1")
        .with_vertex("a")
        .unwrap()
        .with_vertex("c")
        .unwrap()
        .with_undirected_edge("a", "c")
        .unwrap()
        .finish()
        .unwrap();
    let changed_parent = version(&alternate_graph);

    assert_ne!(
        baseline.artifact_digest(),
        changed_version.artifact_digest()
    );
    assert_ne!(baseline.artifact_digest(), changed_vertex.artifact_digest());
    assert_ne!(baseline.artifact_digest(), changed_parent.artifact_digest());
    assert_ne!(graph.artifact_digest(), alternate_graph.artifact_digest());
    assert_ne!(
        graph.reference().stable_token(),
        baseline.reference().stable_token()
    );
}

#[test]
fn graph_version_builder_rejects_shape_errors_without_math_claims() {
    let graph = graph(query_source("candidate-a", "v1"));

    assert_eq!(
        GraphVersion::builder(graph.reference(), "v1")
            .with_vertex("a")
            .unwrap()
            .with_vertex("a"),
        Err(HadwigerArtifactShapeError::DuplicateVertex {
            vertex_label: "a".to_string()
        })
    );
    assert_eq!(
        GraphVersion::builder(graph.reference(), "v1")
            .with_vertex("a")
            .unwrap()
            .with_undirected_edge("a", "b"),
        Err(HadwigerArtifactShapeError::MissingEdgeEndpoint {
            vertex_label: "b".to_string()
        })
    );
    assert_eq!(
        GraphVersion::builder(graph.reference(), "v1")
            .with_vertex("a")
            .unwrap()
            .with_undirected_edge("a", "a"),
        Err(HadwigerArtifactShapeError::SelfEdge {
            vertex_label: "a".to_string()
        })
    );
}

#[test]
fn query_envelope_references_are_self_describing() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("default Hadwiger research handle should admit");
    let envelope = match orchestrate_research_request_entry(
        &handle,
        CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1"),
    ) {
        WorthQueryOrdinaryOutcome::Bound(envelope) => envelope,
        _ => panic!("expected bound declaration envelope"),
    };
    let reference: HadwigerQueryEnvelopeReference = envelope.into();

    assert_eq!(reference.domain_key(), "WORTH.hadwiger.research");
    assert_eq!(
        reference.declaration_family_key(),
        "hadwiger.candidate_graph"
    );
    assert!(!reference.handle_identity_digest().is_empty());
    assert!(!reference.operating_context_identity_digest().is_empty());
    assert!(!reference.declaration_digest().is_empty());
    assert!(reference.progression_digest().is_some());
    assert!(reference.route_plan_digest().is_some());
    assert!(!reference.receipt_digest().is_empty());
    assert!(!reference.envelope_digest().is_empty());
}
