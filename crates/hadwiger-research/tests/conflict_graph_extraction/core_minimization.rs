use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    crate::installed_support::installed_hadwiger_research_handle()
        .expect("real Hadwiger handle admits")
}

fn tile(tile_id: &str, color_id: &str, x_min: i128, x_max: i128) -> RectangularTileRegion {
    RectangularTileRegion::new(
        tile_id,
        TilingColorId::new(color_id).unwrap(),
        ExactRational::integer(x_min),
        ExactRational::integer(x_max),
        ExactRational::integer(0),
        ExactRational::integer(1),
    )
    .unwrap()
    .with_boundary_ownership(BoundaryOwnershipPolicy::owned_half_open("left,bottom").unwrap())
}

fn conflict_graph(handle: &HadwigerResearchHandle) -> TilingConflictGraph {
    let cell = TilingCell::builder("core-cell")
        .with_rectangular_tile(tile("tile-a", "red", 0, 1))
        .unwrap()
        .with_rectangular_tile(tile("tile-b", "red", 1, 2))
        .unwrap()
        .finish()
        .unwrap();
    let contact =
        evaluate_tiling_same_color_contact_checked(handle, &cell, "tile-a", "tile-b").unwrap();
    extract_conflict_graph_checked(
        handle,
        TilingConflictGraphExtractionRequest::from_tiling_contact_report("core-graph", contact),
    )
    .unwrap()
}

fn deletion_version(
    graph: &TilingConflictGraph,
    version_id: &str,
    vertices: &[&str],
    edges: &[(&str, &str)],
) -> GraphVersion {
    let mut builder = GraphVersion::builder(graph.graph_identity().reference(), version_id);
    for vertex in vertices {
        builder = builder.with_vertex(*vertex).unwrap();
    }
    for (left, right) in edges {
        builder = builder.with_undirected_edge(*left, *right).unwrap();
    }
    builder.finish().unwrap()
}

#[test]
fn non_colorable_conflict_graph_without_minimality_certificate_is_unsupported() {
    let handle = handle();
    let graph = conflict_graph(&handle);
    let report = extract_conflict_core_checked(
        &handle,
        ConflictCoreExtractionRequest::new("core-a", &graph, 1),
    )
    .unwrap();

    assert_eq!(
        report.posture(),
        ConflictCoreExtractionPosture::UnsupportedMinimality
    );
    assert_eq!(
        report.colorability_verification().unwrap().posture(),
        ColorabilityVerificationPosture::UnsatVerified
    );
    assert_eq!(report.counters().vertices_inspected(), 2);
    assert_eq!(report.counters().edges_inspected(), 1);
    assert_eq!(report.counters().deletion_candidates(), 3);
    assert_eq!(report.counters().deletion_checks_attempted(), 0);
    assert_eq!(report.counters().unsupported_checks(), 1);
    assert!(!report.admits_theorem_authority());
}

#[test]
fn one_edge_conflict_core_becomes_vertex_and_edge_minimal_with_real_deletion_checks() {
    let handle = handle();
    let graph = conflict_graph(&handle);
    let without_a = deletion_version(&graph, "without-a", &["tile-b"], &[]);
    let without_b = deletion_version(&graph, "without-b", &["tile-a"], &[]);
    let without_edge = deletion_version(&graph, "without-edge", &["tile-a", "tile-b"], &[]);
    let delete_a = verify_k_colorability_checked(&handle, &without_a, 1)
        .unwrap()
        .colorability_verification()
        .clone();
    let delete_b = verify_k_colorability_checked(&handle, &without_b, 1)
        .unwrap()
        .colorability_verification()
        .clone();
    let delete_edge = verify_k_colorability_checked(&handle, &without_edge, 1)
        .unwrap()
        .colorability_verification()
        .clone();
    let certificate = ConflictCoreMinimalityCertificate::new(
        "one-edge-minimality",
        vec![
            ConflictCoreDeletionCheck::vertex_colorable_after_deletion(
                "tile-a", without_a, delete_a,
            )
            .unwrap(),
            ConflictCoreDeletionCheck::vertex_colorable_after_deletion(
                "tile-b", without_b, delete_b,
            )
            .unwrap(),
            ConflictCoreDeletionCheck::edge_colorable_after_deletion(
                "tile-b",
                "tile-a",
                without_edge,
                delete_edge,
            )
            .unwrap(),
        ],
    )
    .unwrap();

    let report = extract_conflict_core_checked(
        &handle,
        ConflictCoreExtractionRequest::new("core-minimal", &graph, 1)
            .with_minimality_certificate(certificate)
            .unwrap(),
    )
    .unwrap();

    assert_eq!(
        report.posture(),
        ConflictCoreExtractionPosture::VertexAndEdgeMinimal
    );
    assert_eq!(report.counters().deletion_checks_attempted(), 3);
    assert_eq!(report.counters().deletion_checks_admitted(), 3);
    assert_eq!(report.counters().unsupported_checks(), 0);
    assert_eq!(report.query_declaration_digest().is_empty(), false);
}

#[test]
fn weak_or_duplicate_minimality_certificate_is_rejected_or_non_authoritative() {
    let handle = handle();
    let graph = conflict_graph(&handle);
    let unsupported = ConflictCoreDeletionCheck::unsupported(
        ConflictCoreDeletionCheckKind::VertexRemoval,
        "tile-a",
    )
    .unwrap();

    assert_eq!(
        ConflictCoreMinimalityCertificate::new(
            "duplicate-checks",
            vec![unsupported.clone(), unsupported]
        ),
        Err(ConflictGraphError::DuplicateDeletionCheck {
            target: "vertex_removal:tile-a".to_string()
        })
    );

    let certificate = ConflictCoreMinimalityCertificate::new(
        "weak-checks",
        vec![ConflictCoreDeletionCheck::unsupported(
            ConflictCoreDeletionCheckKind::VertexRemoval,
            "tile-a",
        )
        .unwrap()],
    )
    .unwrap();
    let report = extract_conflict_core_checked(
        &handle,
        ConflictCoreExtractionRequest::new("core-weak", &graph, 1)
            .with_minimality_certificate(certificate)
            .unwrap(),
    )
    .unwrap();

    assert_eq!(
        report.posture(),
        ConflictCoreExtractionPosture::UnsupportedMinimality
    );
    assert_eq!(report.counters().deletion_checks_admitted(), 0);
}

#[test]
fn minimality_certificate_rejects_deletion_targets_outside_conflict_graph() {
    let handle = handle();
    let graph = conflict_graph(&handle);
    let without_a = deletion_version(&graph, "without-a-invalid-target", &["tile-b"], &[]);
    let delete_a = verify_k_colorability_checked(&handle, &without_a, 1)
        .unwrap()
        .colorability_verification()
        .clone();
    let certificate = ConflictCoreMinimalityCertificate::new(
        "invalid-target",
        vec![ConflictCoreDeletionCheck::vertex_colorable_after_deletion(
            "ghost-tile",
            without_a,
            delete_a,
        )
        .unwrap()],
    )
    .unwrap();

    assert_eq!(
        extract_conflict_core_checked(
            &handle,
            ConflictCoreExtractionRequest::new("core-invalid-target", &graph, 1)
                .with_minimality_certificate(certificate)
                .unwrap()
        ),
        Err(ConflictGraphError::DeletionCheckGraphMismatch {
            target: "ghost-tile".to_string()
        })
    );
}

#[test]
fn minimality_certificate_rejects_verification_for_wrong_deletion_graph() {
    let handle = handle();
    let graph = conflict_graph(&handle);
    let correct_without_a = deletion_version(&graph, "without-a-correct-target", &["tile-b"], &[]);
    let wrong_without_edge = deletion_version(
        &graph,
        "wrong-without-edge-for-target",
        &["tile-a", "tile-b"],
        &[],
    );
    let wrong_verification = verify_k_colorability_checked(&handle, &wrong_without_edge, 1)
        .unwrap()
        .colorability_verification()
        .clone();
    let certificate = ConflictCoreMinimalityCertificate::new(
        "wrong-verification-graph",
        vec![ConflictCoreDeletionCheck::vertex_colorable_after_deletion(
            "tile-a",
            correct_without_a,
            wrong_verification,
        )
        .unwrap()],
    )
    .unwrap();

    assert_eq!(
        extract_conflict_core_checked(
            &handle,
            ConflictCoreExtractionRequest::new("core-wrong-verification", &graph, 1)
                .with_minimality_certificate(certificate)
                .unwrap()
        ),
        Err(ConflictGraphError::DeletionCheckVerificationMismatch {
            target: "tile-a".to_string()
        })
    );
}
