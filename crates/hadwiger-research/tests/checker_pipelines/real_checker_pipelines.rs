use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("real Hadwiger research handle should admit")
}

fn graph_version(graph_id: &str, edges: &[(&str, &str)]) -> GraphVersion {
    let handle = handle();
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration should admit");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
    let mut builder = GraphVersion::builder(graph.reference(), "v1");
    for label in sorted_vertices(edges) {
        builder = builder.with_vertex(label).unwrap();
    }
    for (left, right) in edges {
        builder = builder.with_undirected_edge(*left, *right).unwrap();
    }
    builder.finish().unwrap()
}

#[test]
fn exact_unit_edge_admits_unit_distance_verification_and_aspect() {
    let handle = handle();
    let version = graph_version("unit-edge", &[("a", "b")]);
    let embedding = ExactGraphEmbedding::builder(version.reference(), "embedding-a")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(1, 0))
        .unwrap()
        .finish()
        .unwrap();

    let checked = verify_unit_distance_embedding_checked(&handle, &version, embedding).unwrap();

    assert!(checked.verification().is_admitted());
    assert_eq!(
        checked.verification().posture(),
        HadwigerCheckerPosture::Admitted
    );
    assert_eq!(
        checked
            .verification()
            .query_declaration_reference()
            .domain_key(),
        "forge.hadwiger.research"
    );
    assert_eq!(
        checked
            .verification()
            .query_declaration_reference()
            .declaration_family_key(),
        "hadwiger.unit_distance_verification"
    );
    assert!(checked
        .unit_distance_aspect()
        .satisfies_mathematical_dependency());
}

#[test]
fn wrong_edge_length_rejects_only_unit_distance_posture() {
    let handle = handle();
    let version = graph_version("wrong-edge", &[("a", "b")]);
    let embedding = ExactGraphEmbedding::builder(version.reference(), "embedding-a")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(2, 0))
        .unwrap()
        .finish()
        .unwrap();

    let checked = verify_unit_distance_embedding_checked(&handle, &version, embedding).unwrap();

    assert_eq!(
        checked.verification().posture(),
        HadwigerCheckerPosture::Rejected
    );
    assert!(!checked
        .unit_distance_aspect()
        .satisfies_mathematical_dependency());
    assert_eq!(version.vertices().len(), 2);
    assert_eq!(version.edges().len(), 1);
}

#[test]
fn missing_coordinate_returns_typed_error() {
    let handle = handle();
    let version = graph_version("missing-coordinate", &[("a", "b")]);
    let embedding = ExactGraphEmbedding::builder(version.reference(), "embedding-a")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .finish()
        .unwrap();

    assert_eq!(
        verify_unit_distance_embedding_checked(&handle, &version, embedding),
        Err(HadwigerExactGeometryError::MissingCoordinate {
            vertex_label: "b".to_string()
        })
    );
}

#[test]
fn k_colorability_encoding_is_deterministic_across_insertion_order() {
    let handle = handle();
    let left = graph_version("path-left", &[("a", "b"), ("b", "c")]);
    let right = graph_version("path-right", &[("c", "b"), ("b", "a")]);

    let left_checked = verify_k_colorability_checked(&handle, &left, 2).unwrap();
    let right_checked = verify_k_colorability_checked(&handle, &right, 2).unwrap();

    assert_eq!(
        left_checked.encoding().variable_map(),
        right_checked.encoding().variable_map()
    );
    assert_eq!(
        left_checked.encoding().clauses(),
        right_checked.encoding().clauses()
    );
}

#[test]
fn satisfiable_coloring_is_admitted_only_after_model_replay() {
    let handle = handle();
    let version = graph_version("path-sat", &[("a", "b"), ("b", "c")]);

    let checked = verify_k_colorability_checked(&handle, &version, 2).unwrap();

    assert_eq!(checked.solver_run().posture(), SolverRunPosture::Sat);
    assert_eq!(
        checked.colorability_verification().posture(),
        ColorabilityVerificationPosture::SatModelVerified
    );
    assert_eq!(
        checked.solver_run().query_declaration_reference(),
        checked
            .colorability_verification()
            .query_declaration_reference()
    );
    assert_eq!(
        checked
            .colorability_verification()
            .query_declaration_reference()
            .declaration_family_key(),
        "hadwiger.colorability"
    );
    assert!(!checked.solver_run().model().is_empty());
    assert!(!checked
        .not_k_colorable_aspect()
        .satisfies_mathematical_dependency());
}

#[test]
fn non_k_colorability_requires_checked_refutation_certificate() {
    let handle = handle();
    let version = graph_version("triangle-unsat", &[("a", "b"), ("b", "c"), ("a", "c")]);

    let checked = verify_k_colorability_checked(&handle, &version, 2).unwrap();

    assert_eq!(checked.solver_run().posture(), SolverRunPosture::Unsat);
    assert_eq!(
        checked.colorability_verification().posture(),
        ColorabilityVerificationPosture::UnsatVerified
    );
    assert!(checked
        .not_k_colorable_aspect()
        .satisfies_mathematical_dependency());
}

#[test]
fn large_unsat_without_certificate_budget_is_not_admitted() {
    let handle = handle();
    let edges = complete_graph_edges(13);
    let version = graph_version(
        "large-budget-boundary",
        &edges
            .iter()
            .map(|(left, right)| (left.as_str(), right.as_str()))
            .collect::<Vec<_>>(),
    );

    let checked = verify_k_colorability_checked(&handle, &version, 1).unwrap();

    assert_eq!(
        checked.colorability_verification().posture(),
        ColorabilityVerificationPosture::UnsupportedCertificateBudget
    );
    assert!(!checked
        .not_k_colorable_aspect()
        .satisfies_mathematical_dependency());
}

#[test]
fn repeated_varisat_helper_path_converges_on_artifact_digests() {
    let handle = handle();
    let version = graph_version("digest-parity", &[("a", "b"), ("b", "c")]);

    let left = verify_k_colorability_checked(&handle, &version, 2).unwrap();
    let right = verify_k_colorability_checked(&handle, &version, 2).unwrap();

    assert_eq!(
        left.encoding().artifact_digest(),
        right.encoding().artifact_digest()
    );
    assert_eq!(
        left.colorability_verification().artifact_digest(),
        right.colorability_verification().artifact_digest()
    );
}

#[test]
fn hexagonal_seven_coloring_side_two_fifths_admits_upper_bound_evidence() {
    let handle = handle();
    let construction = HexagonalSevenColoringConstruction::with_side_length_fraction(2, 5).unwrap();

    let checked = verify_hexagonal_seven_coloring_checked(&handle, construction).unwrap();

    assert!(checked.verification().admits_upper_bound_evidence());
    assert_eq!(checked.verified_color_count(), 7);
    assert_eq!(
        checked
            .verification()
            .query_declaration_reference()
            .declaration_family_key(),
        "hadwiger.whole_plane_coloring_construction"
    );
    assert!(checked
        .verification()
        .causal_evidence()
        .replay_digest()
        .contains(handle.handle_identity_digest()));
}

#[test]
fn invalid_hexagonal_side_lengths_and_color_rules_do_not_admit_evidence() {
    let handle = handle();
    assert_eq!(
        verify_hexagonal_seven_coloring_checked(
            &handle,
            HexagonalSevenColoringConstruction::with_side_length_fraction(1, 2).unwrap()
        ),
        Err(HadwigerPlaneColoringError::InvalidSideLength)
    );
    assert_eq!(
        verify_hexagonal_seven_coloring_checked(
            &handle,
            HexagonalSevenColoringConstruction::with_side_length_fraction(2, 5)
                .unwrap()
                .with_color_rule(1, 1)
        ),
        Err(HadwigerPlaneColoringError::InvalidColorRule)
    );
}

fn sorted_vertices<'a>(edges: &'a [(&'a str, &'a str)]) -> Vec<&'a str> {
    let mut vertices = edges
        .iter()
        .flat_map(|(left, right)| [*left, *right])
        .collect::<Vec<_>>();
    vertices.sort_unstable();
    vertices.dedup();
    vertices
}

fn complete_graph_edges(vertex_count: usize) -> Vec<(String, String)> {
    let labels = (0..vertex_count)
        .map(|index| format!("v{index}"))
        .collect::<Vec<_>>();
    let mut edges = Vec::new();
    for left in 0..labels.len() {
        for right in (left + 1)..labels.len() {
            edges.push((labels[left].clone(), labels[right].clone()));
        }
    }
    edges
}
