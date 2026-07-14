use hadwiger_research::facade::*;
use worth_query::facade::foundation::{
    WorthQueryApplicationFacade, WorthQueryContributionComposedOrchestrationInput,
};

fn handle() -> HadwigerResearchHandle {
    crate::installed_support::installed_hadwiger_research_handle()
        .expect("Hadwiger handle should admit")
}

fn graph_version(graph_id: &str) -> GraphVersion {
    let handle = handle();
    let declaration = declare_research_request_checked(
        &handle,
        CandidateGraphDeclaration::new(graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration should admit");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
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

fn rejected_unit_distance(
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

fn rejection_explanation(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
) -> HadwigerRejectionExplanation {
    let unit_checked = rejected_unit_distance(handle, version);
    explain_rejection(
        handle,
        ExplainRejectionRequest::for_checker_rejection(
            "bad-unit-distance-edge",
            version,
            unit_checked.verification(),
        )
        .with_rejected_aspect(unit_checked.unit_distance_aspect())
        .with_repair_obligation("repair exact coordinates for every unit edge")
        .unwrap(),
    )
    .unwrap()
}

fn partial_explanation(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
) -> HadwigerPartialAdmissionExplanation {
    let blocked = admit_plane_lower_bound_claim_checked(
        handle,
        PlaneLowerBoundClaimRequest::new("missing-lower-bound-inputs", version),
    )
    .expect_err("missing proof inputs should block");
    let HadwigerProofClaimAdmissionError::Blocked(blocked) = blocked else {
        panic!("expected blocked proof claim");
    };
    explain_partial_admission(
        handle,
        ExplainPartialAdmissionRequest::from_blocked_proof_claim("partial", version, &blocked)
            .with_surviving_artifact(version.reference())
            .with_repair_obligation("supply admitted checker evidence")
            .unwrap(),
    )
    .unwrap()
}

fn query_recovery_explanation(handle: &HadwigerResearchHandle) -> HadwigerQueryRecoveryExplanation {
    let checked = handle.orchestrate_declaration_with_contributions_checked(
        WorthQueryContributionComposedOrchestrationInput::new(
            RejectionExplanationDeclaration::new("candidate-a", "bad-edge"),
        ),
    );
    let recovery = handle
        .recover_from_contribution_composed_checked(checked)
        .expect("empty contribution composition should recover");
    explain_query_recovery_brief(
        handle,
        HadwigerQueryRecoveryExplanationRequest::new("query-recovery", recovery),
    )
    .unwrap()
}

fn corpus_with_retained_evidence() -> (
    HadwigerResearchHandle,
    GraphVersion,
    HadwigerRejectionExplanation,
    ResearchEvidenceCorpus,
) {
    let handle = handle();
    let version = graph_version("phase7-corpus");
    let rejection = rejection_explanation(&handle, &version);
    let partial = partial_explanation(&handle, &version);
    let recovery = query_recovery_explanation(&handle);
    let corpus = ResearchEvidenceCorpus::builder("frontier-a")
        .with_graph_version(version.reference())
        .with_checker_rejection(rejection.clone())
        .unwrap()
        .with_partial_admission(partial)
        .unwrap()
        .with_query_recovery(recovery)
        .finish()
        .unwrap();
    (handle, version, rejection, corpus)
}

#[test]
fn corpus_retains_failed_partial_and_query_owned_evidence() {
    let (_handle, version, rejection, corpus) = corpus_with_retained_evidence();

    assert!(corpus.has_reference(&version.reference()));
    assert!(corpus.has_reference(&rejection.reference()));
    assert!(!corpus.reusable_negative_evidence().is_empty());
    assert!(corpus.has_query_recovery_evidence());
    assert!(corpus.rejected_evidence_available());
    assert!(!corpus.admits_theorem_authority());
}

#[test]
fn equivalent_corpora_converge_despite_insertion_order() {
    let handle = handle();
    let version = graph_version("phase7-converge");
    let rejection = rejection_explanation(&handle, &version);
    let partial = partial_explanation(&handle, &version);

    let left = ResearchEvidenceCorpus::builder("same")
        .with_graph_version(version.reference())
        .with_checker_rejection(rejection.clone())
        .unwrap()
        .with_partial_admission(partial.clone())
        .unwrap()
        .finish()
        .unwrap();
    let right = ResearchEvidenceCorpus::builder("same")
        .with_partial_admission(partial)
        .unwrap()
        .with_checker_rejection(rejection)
        .unwrap()
        .with_graph_version(version.reference())
        .finish()
        .unwrap();

    assert_eq!(left.corpus_digest(), right.corpus_digest());
}

#[test]
fn graph_resident_failure_creates_dead_end_suppression_proof() {
    let (handle, version, rejection, corpus) = corpus_with_retained_evidence();
    let negative = rejection
        .reusable_negative_evidence()
        .expect("rejection explanation should retain negative evidence");
    let graph_failure = attach_failure_to_research_graph(
        &handle,
        &corpus,
        negative,
        FailureScope::edge_local(version.reference(), "b", "a").unwrap(),
    )
    .unwrap();

    let signature = DeadEndSignature::from_graph_resident_failure(&graph_failure).unwrap();
    let suppression = ExperimentSuppressionProof::from_dead_end_signature(
        signature,
        graph_failure.failure_basis_fingerprint(),
    )
    .unwrap();

    assert!(suppression.blocks_equivalent_experiment());
    assert!(!suppression.admits_theorem_authority());
    assert!(!graph_failure.admits_theorem_authority());
}

#[test]
fn discovery_pipeline_suppresses_retained_dead_end_work() {
    let (handle, _version, rejection, corpus) = corpus_with_retained_evidence();
    let negative = rejection.reusable_negative_evidence().unwrap();

    let observations = mine_research_patterns(&handle, &corpus).unwrap();
    let hypotheses = propose_invariant_hypotheses(&handle, &corpus, &observations).unwrap();
    let plans = plan_next_experiments(&handle, &corpus, &hypotheses).unwrap();
    let retained_negative_sources_suppression = plans.suppression_proofs().iter().any(|proof| {
        proof
            .parent_artifacts()
            .iter()
            .any(|parent| parent == &negative.reference())
    });
    let frontier =
        update_discovery_frontier(&handle, &corpus, observations, hypotheses, plans).unwrap();

    assert!(frontier.scorecard().suppression_hits() > 0);
    assert_eq!(frontier.scorecard().counters().query_readiness_checks(), 1);
    assert!(retained_negative_sources_suppression);
    assert!(!frontier.admits_theorem_authority());
    assert!(!frontier.registers_query_invariant_authority());
    assert!(frontier
        .experiment_plans()
        .iter()
        .any(ExperimentPlan::is_suppressed));
}

#[test]
fn derived_frontier_recomputes_from_same_corpus() {
    let (handle, _version, _rejection, corpus) = corpus_with_retained_evidence();

    let left = recompute_derived_discovery_frontier(&handle, &corpus).unwrap();
    let right = recompute_derived_discovery_frontier(&handle, &corpus).unwrap();

    assert_eq!(left.artifact_digest(), right.artifact_digest());
    assert_eq!(left.source_corpus_digest(), corpus.corpus_digest());
    assert!(left.counters().candidate_breadth() >= left.experiment_plans().len());
    assert_eq!(left.counters().query_readiness_checks(), 1);
    assert!(left.rejected_evidence_available());
}

#[test]
fn discovery_without_query_recovery_reports_zero_readiness_checks() {
    let handle = handle();
    let version = graph_version("phase7-no-query-recovery");
    let rejection = rejection_explanation(&handle, &version);
    let corpus = ResearchEvidenceCorpus::builder("frontier-no-query")
        .with_graph_version(version.reference())
        .with_checker_rejection(rejection)
        .unwrap()
        .finish()
        .unwrap();

    let observations = mine_research_patterns(&handle, &corpus).unwrap();
    let hypotheses = propose_invariant_hypotheses(&handle, &corpus, &observations).unwrap();
    let plans = plan_next_experiments(&handle, &corpus, &hypotheses).unwrap();
    let frontier =
        update_discovery_frontier(&handle, &corpus, observations, hypotheses, plans).unwrap();

    assert!(!corpus.has_query_recovery_evidence());
    assert_eq!(frontier.scorecard().counters().query_readiness_checks(), 0);
}

#[test]
fn reactivation_changes_suppression_relation_without_theorem_authority() {
    let (handle, version, rejection, corpus) = corpus_with_retained_evidence();
    let negative = rejection.reusable_negative_evidence().unwrap();
    let failure = attach_failure_to_research_graph(
        &handle,
        &corpus,
        negative,
        FailureScope::edge_local(version.reference(), "a", "b").unwrap(),
    )
    .unwrap();
    let signature = DeadEndSignature::from_graph_resident_failure(&failure).unwrap();
    let suppression = ExperimentSuppressionProof::from_dead_end_signature(
        signature,
        failure.failure_basis_fingerprint(),
    )
    .unwrap();
    let reactivation =
        ReactivationCondition::from_new_evidence(suppression.reference(), version.reference())
            .unwrap();
    let reactivated = suppression.reactivated_by(&reactivation).unwrap();

    assert_eq!(
        reactivated.relation(),
        SuppressionRelation::ReactivatedByNewEvidence
    );
    assert!(!reactivated.admits_theorem_authority());
}
