use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    crate::installed_support::installed_hadwiger_research_handle()
        .expect("real Hadwiger research handle should admit")
}

fn source(handle: &HadwigerResearchHandle) -> HadwigerQueryDeclarationReference {
    declare_research_request_checked(
        handle,
        CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration should admit")
    .into()
}

fn graph_version(handle: &HadwigerResearchHandle) -> GraphVersion {
    let graph = GraphIdentity::from_query_declaration("candidate-a", source(handle)).unwrap();
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

fn assert_artifact<A: HadwigerCanonicalArtifact>(artifact: &A) {
    assert!(!artifact.artifact_digest().stable_token().is_empty());
    assert!(!artifact.reference().stable_token().is_empty());
    assert!(!artifact.source_reference().stable_token().is_empty());
}

#[test]
fn real_unit_distance_verification_is_checker_authority_not_theorem_authority() {
    let handle = handle();
    let version = graph_version(&handle);
    let embedding = ExactGraphEmbedding::builder(version.reference(), "embedding-a")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(1, 0))
        .unwrap()
        .finish()
        .unwrap();

    let checked = verify_unit_distance_embedding_checked(&handle, &version, embedding).unwrap();
    let verification = checked.verification();

    assert!(verification.is_admitted());
    assert!(checked
        .unit_distance_aspect()
        .satisfies_mathematical_dependency());
    assert!(!verification.admits_theorem_authority());
    assert_eq!(
        verification.authority_owner(),
        HadwigerArtifactAuthorityOwner::Checker
    );
    assert_eq!(
        verification.checker_identity(),
        "hadwiger.exact_unit_distance"
    );
    assert_eq!(verification.checker_version(), "0.1.0");
    assert_eq!(
        verification.boundary_kind(),
        HadwigerCheckerBoundaryKind::InProcess
    );
    assert_eq!(verification.posture(), HadwigerCheckerPosture::Admitted);
    assert!(verification
        .causal_evidence()
        .replay_digest()
        .contains(handle.handle_identity_digest()));
    assert_artifact(verification);
}

#[test]
fn colorability_artifacts_preserve_parents_and_real_checker_authority() {
    let handle = handle();
    let version = GraphVersion::builder(
        GraphIdentity::from_query_declaration("triangle", source(&handle))
            .unwrap()
            .reference(),
        "triangle-v1",
    )
    .with_vertex("a")
    .unwrap()
    .with_vertex("b")
    .unwrap()
    .with_vertex("c")
    .unwrap()
    .with_undirected_edge("a", "b")
    .unwrap()
    .with_undirected_edge("b", "c")
    .unwrap()
    .with_undirected_edge("a", "c")
    .unwrap()
    .finish()
    .unwrap();

    let checked = verify_k_colorability_checked(&handle, &version, 2).unwrap();

    assert_eq!(
        checked.colorability_verification().posture(),
        ColorabilityVerificationPosture::UnsatVerified
    );
    assert!(checked
        .not_k_colorable_aspect()
        .satisfies_mathematical_dependency());
    assert_eq!(checked.solver_run().posture(), SolverRunPosture::Unsat);
    assert_eq!(
        checked.solver_run().authority_owner(),
        HadwigerArtifactAuthorityOwner::Checker
    );
    assert_eq!(
        checked.colorability_verification().authority_owner(),
        HadwigerArtifactAuthorityOwner::Checker
    );
    assert_eq!(
        checked.solver_run().parent_artifacts(),
        &[checked.encoding().reference()]
    );
    assert_eq!(
        checked.colorability_verification().parent_artifacts(),
        &[checked.solver_run().reference()]
    );
}

#[test]
fn gadget_reduction_composition_advisory_and_proof_candidates_remain_non_theorem() {
    let handle = handle();
    let version = graph_version(&handle);
    let gadget = GadgetDefinition::new(version.reference(), "gadget-a").unwrap();
    let contract = GadgetContract::new(gadget.reference(), "contract-a").unwrap();
    let reduction = ReductionTrace::new(contract.reference(), "reduction-a").unwrap();
    let composition = GraphComposition::new(
        "composition-a",
        vec![
            version.reference(),
            gadget.reference(),
            reduction.reference(),
        ],
    )
    .unwrap();
    let proof = ProofClaim::candidate_lower_bound(version.reference(), "claim-a", 5).unwrap();
    let advisory =
        AIAdvisoryArtifact::new(version.reference(), "advisory-a", "advisory-source-digest")
            .unwrap();

    assert_eq!(contract.parent_artifacts(), &[gadget.reference()]);
    assert_eq!(reduction.parent_artifacts(), &[contract.reference()]);
    assert_eq!(composition.parent_artifacts().len(), 3);
    assert!(!proof.admits_theorem_authority());
    assert!(!advisory.admits_theorem_authority());
    assert_eq!(
        advisory.authority_owner(),
        HadwigerArtifactAuthorityOwner::AIAdvisory
    );
}

#[test]
fn checker_causal_evidence_rejects_empty_boundary_fields() {
    assert_eq!(
        HadwigerCheckerCausalEvidence::new("", "route", "evaluation", "diagnostics", "replay"),
        Err(HadwigerArtifactShapeError::EmptyField {
            field: "truth_view_basis_digest"
        })
    );
}
