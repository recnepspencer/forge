use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("real Hadwiger research handle should admit")
}

fn unit_edge_graph() -> GraphVersion {
    unit_edge_graph_with_id("proof-unit-edge")
}

fn unit_edge_graph_with_id(graph_id: &str) -> GraphVersion {
    let handle = handle();
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration should admit");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into())
        .expect("graph identity should build");
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

fn unit_edge_embedding(version: &GraphVersion, embedding_id: &str) -> ExactGraphEmbedding {
    ExactGraphEmbedding::builder(version.reference(), embedding_id)
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(1, 0))
        .unwrap()
        .finish()
        .unwrap()
}

fn complete_graph_six() -> GraphVersion {
    let handle = handle();
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new("abstract-k6-frontier").with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration should admit");
    let graph = GraphIdentity::from_query_declaration("abstract-k6-frontier", declaration.into())
        .expect("graph identity should build");
    let labels = ["a", "b", "c", "d", "e", "f"];
    let mut builder = GraphVersion::builder(graph.reference(), "v1");
    for label in labels {
        builder = builder.with_vertex(label).unwrap();
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

fn admitted_lower_bound_two(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
) -> HadwigerProofClaimAdmissionChecked {
    let embedding = unit_edge_embedding(version, "embedding-a");
    let unit_checked = verify_unit_distance_embedding_checked(handle, version, embedding).unwrap();
    let color_checked = verify_k_colorability_checked(handle, version, 1).unwrap();

    admit_plane_lower_bound_claim_checked(
        handle,
        PlaneLowerBoundClaimRequest::new("unit-edge-lower-bound", version)
            .with_unit_distance_verification(unit_checked.verification())
            .with_unit_distance_aspect(unit_checked.unit_distance_aspect())
            .with_not_k_colorable_verification(color_checked.colorability_verification())
            .with_not_k_colorable_aspect(color_checked.not_k_colorable_aspect()),
    )
    .expect("unit edge should admit chi(plane) >= 2")
}

#[test]
fn admitted_unit_distance_and_not_one_colorable_admits_lower_bound_claim() {
    let handle = handle();
    let version = unit_edge_graph();

    let admitted = admitted_lower_bound_two(&handle, &version);

    assert_eq!(admitted.proof_claim().claim_statement(), "chi(plane) >= 2");
    assert!(admitted.proof_claim().admits_theorem_authority());
    assert_eq!(
        admitted.authority_chain().weakest_posture(),
        HadwigerAspectPosture::Admitted
    );
    assert_eq!(
        admitted
            .authority_chain()
            .query_declaration_references()
            .first()
            .unwrap()
            .declaration_family_key(),
        "hadwiger.plane_lower_bound_claim"
    );
    assert_eq!(
        admitted
            .authority_chain()
            .checker_artifact_references()
            .len(),
        2
    );
    assert_eq!(admitted.authority_chain().aspect_tokens().len(), 2);
}

#[test]
fn deferred_or_unsupported_aspect_blocks_lower_bound_claim() {
    let handle = handle();
    let version = unit_edge_graph();
    let embedding = unit_edge_embedding(&version, "embedding-a");
    let unit_checked =
        verify_unit_distance_embedding_checked(&handle, &version, embedding).unwrap();
    let color_checked = verify_k_colorability_checked(&handle, &version, 2).unwrap();

    let err = admit_plane_lower_bound_claim_checked(
        &handle,
        PlaneLowerBoundClaimRequest::new("blocked-lower-bound", &version)
            .with_unit_distance_verification(unit_checked.verification())
            .with_unit_distance_aspect(unit_checked.unit_distance_aspect())
            .with_not_k_colorable_verification(color_checked.colorability_verification())
            .with_not_k_colorable_aspect(color_checked.not_k_colorable_aspect()),
    )
    .expect_err("k-colorable evidence cannot admit a not-k-colorable proof claim");

    match err {
        HadwigerProofClaimAdmissionError::Blocked(blocked) => {
            assert!(!blocked.proof_claim().admits_theorem_authority());
            assert!(blocked
                .blockers()
                .iter()
                .any(|blocker| blocker.blocker_kind()
                    == HadwigerProofClaimBlockerKind::AspectNotAdmitted));
        }
        other => panic!("expected blocked proof claim, got {other:?}"),
    }
}

#[test]
fn crossed_checker_artifacts_block_lower_bound_claim() {
    let handle = handle();
    let left = unit_edge_graph_with_id("proof-left-unit-edge");
    let right = unit_edge_graph_with_id("proof-right-unit-edge");
    let unit_left = verify_unit_distance_embedding_checked(
        &handle,
        &left,
        unit_edge_embedding(&left, "u-left"),
    )
    .unwrap();
    let color_left = verify_k_colorability_checked(&handle, &left, 1).unwrap();
    let color_right = verify_k_colorability_checked(&handle, &right, 1).unwrap();

    let err = admit_plane_lower_bound_claim_checked(
        &handle,
        PlaneLowerBoundClaimRequest::new("crossed-lower-bound", &left)
            .with_unit_distance_verification(unit_left.verification())
            .with_unit_distance_aspect(unit_left.unit_distance_aspect())
            .with_not_k_colorable_verification(color_right.colorability_verification())
            .with_not_k_colorable_aspect(color_left.not_k_colorable_aspect()),
    )
    .expect_err("checker evidence from another graph cannot admit theorem authority");

    match err {
        HadwigerProofClaimAdmissionError::Blocked(blocked) => {
            assert!(!blocked.proof_claim().admits_theorem_authority());
            assert!(blocked
                .blockers()
                .iter()
                .any(|blocker| blocker.blocker_kind()
                    == HadwigerProofClaimBlockerKind::ArtifactMismatch));
        }
        other => panic!("expected blocked crossed proof claim, got {other:?}"),
    }
}

#[test]
fn chi_plane_at_least_six_frontier_lane_blocks_without_real_witness_authority() {
    let handle = handle();
    let version = complete_graph_six();
    let embedding = ExactGraphEmbedding::builder(version.reference(), "line-not-unit-embedding")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(1, 0))
        .unwrap()
        .with_vertex("c", ExactPoint2::integer(2, 0))
        .unwrap()
        .with_vertex("d", ExactPoint2::integer(3, 0))
        .unwrap()
        .with_vertex("e", ExactPoint2::integer(4, 0))
        .unwrap()
        .with_vertex("f", ExactPoint2::integer(5, 0))
        .unwrap()
        .finish()
        .unwrap();
    let unit_checked =
        verify_unit_distance_embedding_checked(&handle, &version, embedding).unwrap();
    let color_checked = verify_k_colorability_checked(&handle, &version, 5).unwrap();

    assert_eq!(
        unit_checked.verification().posture(),
        HadwigerCheckerPosture::Rejected
    );
    assert_ne!(
        color_checked.colorability_verification().posture(),
        ColorabilityVerificationPosture::UnsatVerified
    );

    let err = admit_plane_lower_bound_claim_checked(
        &handle,
        PlaneLowerBoundClaimRequest::new("blocked-frontier-six", &version)
            .with_unit_distance_verification(unit_checked.verification())
            .with_unit_distance_aspect(unit_checked.unit_distance_aspect())
            .with_not_k_colorable_verification(color_checked.colorability_verification())
            .with_not_k_colorable_aspect(color_checked.not_k_colorable_aspect()),
    )
    .expect_err("chi(plane) >= 6 cannot be admitted without real witness authority");

    match err {
        HadwigerProofClaimAdmissionError::Blocked(blocked) => {
            assert_eq!(blocked.proof_claim().claim_statement(), "chi(plane) >= 6");
            assert!(!blocked.proof_claim().admits_theorem_authority());
            assert!(blocked
                .blockers()
                .iter()
                .any(|blocker| blocker.blocker_kind()
                    == HadwigerProofClaimBlockerKind::CheckerArtifactNotAdmitted));
        }
        other => panic!("expected blocked frontier lower-bound claim, got {other:?}"),
    }
}

#[test]
fn checked_hexagonal_upper_bound_admits_upper_bound_claim() {
    let handle = handle();
    let construction = HexagonalSevenColoringConstruction::with_side_length_fraction(2, 5).unwrap();
    let checked = verify_hexagonal_seven_coloring_checked(&handle, construction).unwrap();

    let admitted = admit_plane_upper_bound_claim_checked(
        &handle,
        PlaneUpperBoundClaimRequest::from_checked_upper_bound(
            "plane-upper-seven",
            checked.verification(),
        ),
    )
    .expect("checked seven-coloring should admit upper-bound claim");

    assert_eq!(admitted.proof_claim().claim_statement(), "chi(plane) <= 7");
    assert!(admitted.proof_claim().admits_theorem_authority());
    assert!(admitted.authority_chain().uses_checked_upper_bound());
    assert!(!admitted.authority_chain().uses_background_upper_bound());
}

#[test]
fn sealed_background_plane_seven_upper_bound_retains_query_reference() {
    let handle = handle();

    let retained = retain_background_plane_seven_upper_bound_checked(
        &handle,
        "classical-seven-coloring-theorem",
        "source:classical-hexagonal-tiling",
        "provenance:digest-or-citation-bundle",
    )
    .expect("background upper theorem should retain");

    assert_eq!(retained.theorem_statement(), "chi(plane) <= 7");
    assert_eq!(
        retained
            .query_declaration_reference()
            .declaration_family_key(),
        "hadwiger.background_theorem"
    );
    assert_eq!(
        retained.authority_owner(),
        HadwigerArtifactAuthorityOwner::TheoremAuthority
    );
}

#[test]
fn exact_value_requires_matching_lower_and_upper_bounds() {
    let handle = handle();
    let version = unit_edge_graph();
    let lower = admitted_lower_bound_two(&handle, &version);
    let construction = HexagonalSevenColoringConstruction::with_side_length_fraction(2, 5).unwrap();
    let upper = verify_hexagonal_seven_coloring_checked(&handle, construction).unwrap();

    let err = admit_plane_exact_value_claim_checked(
        &handle,
        PlaneExactValueClaimRequest::from_checked_upper_bound(
            "mismatched-exact",
            lower.proof_claim(),
            upper.verification(),
        ),
    )
    .expect_err(">=2 plus <=7 is not exact equality");

    match err {
        HadwigerProofClaimAdmissionError::Blocked(blocked) => {
            assert_eq!(blocked.proof_claim().claim_statement(), "chi(plane) = 2");
            assert!(!blocked.proof_claim().admits_theorem_authority());
        }
        other => panic!("expected blocked exact claim, got {other:?}"),
    }
}

#[test]
fn changed_background_provenance_changes_retained_theorem_digest() {
    let handle = handle();
    let left = retain_background_plane_seven_upper_bound_checked(
        &handle,
        "classical-seven-coloring-theorem",
        "source:classical-hexagonal-tiling",
        "provenance:a",
    )
    .unwrap();
    let right = retain_background_plane_seven_upper_bound_checked(
        &handle,
        "classical-seven-coloring-theorem",
        "source:classical-hexagonal-tiling",
        "provenance:b",
    )
    .unwrap();

    assert_ne!(left.artifact_digest(), right.artifact_digest());
}
