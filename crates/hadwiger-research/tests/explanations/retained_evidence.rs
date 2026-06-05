use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryContributionComposedOrchestrationInput,
};
use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("Hadwiger handle should admit")
}

fn graph_version(graph_id: &str, labels: &[&str], edges: &[(&str, &str)]) -> GraphVersion {
    let handle = handle();
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration should admit");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
    let mut builder = GraphVersion::builder(graph.reference(), "v1");
    for label in labels {
        builder = builder.with_vertex(*label).unwrap();
    }
    for (left, right) in edges {
        builder = builder.with_undirected_edge(*left, *right).unwrap();
    }
    builder.finish().unwrap()
}

fn wrong_unit_distance_check(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
) -> UnitDistanceVerificationChecked {
    let embedding = ExactGraphEmbedding::builder(version.reference(), "bad-embedding")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(2, 0))
        .unwrap()
        .finish()
        .unwrap();
    verify_unit_distance_embedding_checked(handle, version, embedding).unwrap()
}

fn admitted_unit_distance_check(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
) -> UnitDistanceVerificationChecked {
    let embedding = ExactGraphEmbedding::builder(version.reference(), "good-embedding")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(1, 0))
        .unwrap()
        .finish()
        .unwrap();
    verify_unit_distance_embedding_checked(handle, version, embedding).unwrap()
}

#[test]
fn rejected_unit_distance_checker_result_gets_retained_explanation() {
    let handle = handle();
    let version = graph_version("explain-bad-edge", &["a", "b"], &[("a", "b")]);
    let unit_checked = wrong_unit_distance_check(&handle, &version);

    let explanation = explain_rejection(
        &handle,
        ExplainRejectionRequest::for_checker_rejection(
            "bad-unit-distance-edge",
            &version,
            unit_checked.verification(),
        )
        .with_rejected_aspect(unit_checked.unit_distance_aspect())
        .with_repair_obligation(
            "provide exact coordinates with every graph edge at squared distance 1",
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        explanation.stop_family(),
        HadwigerExplanationStopFamily::CheckerRejection
    );
    assert_eq!(
        explanation.authority_surface(),
        HadwigerExplanationAuthoritySurface::CheckerArtifact
    );
    assert!(!explanation.repair_obligations().is_empty());
    assert!(explanation.reusable_negative_evidence().is_some());
    assert!(!explanation.admits_theorem_authority());
    assert_eq!(
        explanation.checker_artifact_reference(),
        &unit_checked.verification().reference()
    );
}

#[test]
fn admitted_checker_artifact_cannot_be_explained_as_rejection() {
    let handle = handle();
    let version = graph_version("explain-good-edge", &["a", "b"], &[("a", "b")]);
    let unit_checked = admitted_unit_distance_check(&handle, &version);

    let error = explain_rejection(
        &handle,
        ExplainRejectionRequest::for_checker_rejection(
            "not-a-rejection",
            &version,
            unit_checked.verification(),
        ),
    )
    .expect_err("admitted checker evidence must not be retained as rejection");

    assert_eq!(
        error,
        HadwigerExplanationError::CheckerVerificationNotRejected
    );
}

#[test]
fn empty_repair_obligations_are_rejected_at_request_boundary() {
    let handle = handle();
    let version = graph_version("empty-obligation", &["a", "b"], &[("a", "b")]);
    let unit_checked = wrong_unit_distance_check(&handle, &version);

    let error = ExplainRejectionRequest::for_checker_rejection(
        "empty-obligation",
        &version,
        unit_checked.verification(),
    )
    .with_repair_obligation("  ")
    .expect_err("empty obligation should fail before explanation construction");

    assert_eq!(
        error,
        HadwigerArtifactShapeError::EmptyField {
            field: "repair_obligation"
        }
    );
}

#[test]
fn blocked_frontier_lower_bound_gets_partial_admission_explanation() {
    let handle = handle();
    let version = complete_graph_six();
    let embedding = line_embedding_for_k6(&version);
    let unit_checked =
        verify_unit_distance_embedding_checked(&handle, &version, embedding).unwrap();
    let color_checked = verify_k_colorability_checked(&handle, &version, 5).unwrap();

    let blocked = admit_plane_lower_bound_claim_checked(
        &handle,
        PlaneLowerBoundClaimRequest::new("frontier-six-blocked", &version)
            .with_unit_distance_verification(unit_checked.verification())
            .with_unit_distance_aspect(unit_checked.unit_distance_aspect())
            .with_not_k_colorable_verification(color_checked.colorability_verification())
            .with_not_k_colorable_aspect(color_checked.not_k_colorable_aspect()),
    )
    .expect_err("frontier six cannot admit without real witness authority");

    let HadwigerProofClaimAdmissionError::Blocked(blocked) = blocked else {
        panic!("expected blocked proof claim");
    };
    let partial = explain_partial_admission(
        &handle,
        ExplainPartialAdmissionRequest::from_blocked_proof_claim(
            "frontier-six-partial",
            &version,
            &blocked,
        )
        .with_surviving_artifact(version.reference())
        .with_repair_obligation("supply admitted unit-distance and non-5-colorability evidence")
        .unwrap(),
    )
    .unwrap();

    assert!(!partial.surviving_evidence().is_empty());
    assert_eq!(
        partial.blocked_claim().proof_claim().claim_statement(),
        "chi(plane) >= 6"
    );
    assert!(!partial.repair_obligations().is_empty());
    assert!(!partial.admits_theorem_authority());
}

#[test]
fn missing_evidence_blocker_gets_conservative_escalation_explanation() {
    let handle = handle();
    let version = graph_version("phase6-missing-evidence", &["a", "b"], &[("a", "b")]);

    let blocked = admit_plane_lower_bound_claim_checked(
        &handle,
        PlaneLowerBoundClaimRequest::new("missing-lower-bound-inputs", &version),
    )
    .expect_err("missing proof inputs should block");

    let HadwigerProofClaimAdmissionError::Blocked(blocked) = blocked else {
        panic!("expected blocked proof claim");
    };
    let partial = explain_partial_admission(
        &handle,
        ExplainPartialAdmissionRequest::from_blocked_proof_claim(
            "missing-partial",
            &version,
            &blocked,
        )
        .with_repair_obligation("supply checker evidence before retrying proof admission")
        .unwrap(),
    )
    .unwrap();

    let escalation = partial
        .conservative_escalation()
        .expect("missing authority should conservatively escalate");
    assert_eq!(
        escalation.observed_posture(),
        Some(HadwigerAspectPosture::Missing)
    );
    assert!(!escalation.admits_theorem_authority());
}

#[test]
fn explanation_digest_changes_when_repair_obligation_changes() {
    let handle = handle();
    let version = graph_version("explain-digest", &["a", "b"], &[("a", "b")]);
    let unit_checked = wrong_unit_distance_check(&handle, &version);

    let left = explain_rejection(
        &handle,
        ExplainRejectionRequest::for_checker_rejection(
            "digest-left",
            &version,
            unit_checked.verification(),
        )
        .with_repair_obligation("repair one")
        .unwrap(),
    )
    .unwrap();
    let right = explain_rejection(
        &handle,
        ExplainRejectionRequest::for_checker_rejection(
            "digest-left",
            &version,
            unit_checked.verification(),
        )
        .with_repair_obligation("repair two")
        .unwrap(),
    )
    .unwrap();

    assert_ne!(left.artifact_digest(), right.artifact_digest());
}

#[test]
fn query_owned_recovery_brief_is_retained_without_local_translation() {
    let hadwiger_handle = handle();
    let query_handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(HadwigerResearchDomainEntry)
        .with_operating_context(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let checked = query_handle.orchestrate_declaration_with_contributions_checked(
        ForgeQueryContributionComposedOrchestrationInput::new(
            RejectionExplanationDeclaration::new("candidate-a", "bad-edge"),
        ),
    );
    let recovery = query_handle
        .recover_from_contribution_composed_checked(checked)
        .expect("empty contribution composed request should recover through Query");

    let explanation = explain_query_recovery_brief(
        &hadwiger_handle,
        HadwigerQueryRecoveryExplanationRequest::new("query-recovery-a", recovery.clone()),
    )
    .unwrap();

    assert!(explanation.is_query_owned());
    assert_eq!(
        explanation.stop_family(),
        HadwigerExplanationStopFamily::QueryContributionComposition
    );
    assert_eq!(
        explanation.authority_surface(),
        HadwigerExplanationAuthoritySurface::QueryDeclarationProgression
    );
    assert_eq!(explanation.query_recovery_brief(), Some(&recovery));
    assert!(!explanation.admits_theorem_authority());
}

fn complete_graph_six() -> GraphVersion {
    let labels = ["a", "b", "c", "d", "e", "f"];
    let mut edges = Vec::new();
    for left in 0..labels.len() {
        for right in (left + 1)..labels.len() {
            edges.push((labels[left], labels[right]));
        }
    }
    graph_version("phase6-abstract-k6", &labels, &edges)
}

fn line_embedding_for_k6(version: &GraphVersion) -> ExactGraphEmbedding {
    let labels = ["a", "b", "c", "d", "e", "f"];
    let mut builder = ExactGraphEmbedding::builder(version.reference(), "line-not-unit");
    for (index, label) in labels.iter().enumerate() {
        builder = builder
            .with_vertex(*label, ExactPoint2::integer(index as i128, 0))
            .unwrap();
    }
    builder.finish().unwrap()
}
