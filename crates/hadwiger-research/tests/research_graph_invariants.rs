use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryAspectMutationBuilder, ForgeQueryCommitIdentity,
    ForgeQueryContributionComposedOrchestrationInput, ForgeQueryEntityIdentity,
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryRuntimeWriteAuthorityAdapter, ForgeQuerySnapshotIdentity,
    WriteAuthorityExecutionReceipt,
};
use forge_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, RelationalBridgeSnapshotIdentityParts,
};
use hadwiger_research::facade::*;
#[path = "research_graph_invariants/registration.rs"]
mod registration;
fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
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
fn rejection_explanation(
    handle: &HadwigerResearchHandle,
    version: &GraphVersion,
) -> HadwigerRejectionExplanation {
    let embedding = ExactGraphEmbedding::builder(version.reference(), "bad-embedding")
        .with_vertex("a", ExactPoint2::integer(0, 0))
        .unwrap()
        .with_vertex("b", ExactPoint2::integer(2, 0))
        .unwrap()
        .finish()
        .unwrap();
    let unit_checked = verify_unit_distance_embedding_checked(handle, version, embedding).unwrap();
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
        .expect("empty contribution composition should recover");
    explain_query_recovery_brief(
        handle,
        HadwigerQueryRecoveryExplanationRequest::new("query-recovery", recovery),
    )
    .unwrap()
}

fn corpus_frontier_and_suppressed_plans() -> (
    HadwigerResearchHandle,
    ResearchEvidenceCorpus,
    DiscoveryFrontier,
    ExperimentBatch,
) {
    let handle = handle();
    let version = graph_version("phase8-frontier");
    let rejection = rejection_explanation(&handle, &version);
    let partial = partial_explanation(&handle, &version);
    let recovery = query_recovery_explanation(&handle);
    let corpus = ResearchEvidenceCorpus::builder("phase8-corpus")
        .with_graph_version(version.reference())
        .with_checker_rejection(rejection)
        .unwrap()
        .with_partial_admission(partial)
        .unwrap()
        .with_query_recovery(recovery)
        .finish()
        .unwrap();
    let observations = mine_research_patterns(&handle, &corpus).unwrap();
    let hypotheses = propose_invariant_hypotheses(&handle, &corpus, &observations).unwrap();
    let plans = plan_next_experiments(&handle, &corpus, &hypotheses).unwrap();
    let frontier =
        update_discovery_frontier(&handle, &corpus, observations, hypotheses, plans.clone())
            .unwrap();
    (handle, corpus, frontier, plans)
}

fn unsuppressed_plans(handle: &HadwigerResearchHandle) -> ExperimentBatch {
    let version = graph_version("phase8-unsuppressed");
    let partial = partial_explanation(handle, &version);
    let corpus = ResearchEvidenceCorpus::builder("phase8-clean")
        .with_graph_version(version.reference())
        .with_partial_admission(partial)
        .unwrap()
        .finish()
        .unwrap();
    let observations = mine_research_patterns(handle, &corpus).unwrap();
    let hypotheses = propose_invariant_hypotheses(handle, &corpus, &observations).unwrap();
    plan_next_experiments(handle, &corpus, &hypotheses).unwrap()
}

fn suppression_violation(
    handle: &HadwigerResearchHandle,
    catalog: &HadwigerResearchInvariantCatalog,
    corpus: &ResearchEvidenceCorpus,
) -> ResearchGraphInvariantViolation {
    certify_research_graph_invariant_violation(
        handle,
        ResearchGraphInvariantCheckRequest::for_experiment_batch(
            catalog,
            unsuppressed_plans(handle),
        )
        .with_corpus(corpus),
    )
    .unwrap()
}

#[derive(Clone, Copy, Debug)]
struct Phase8BoundaryReceiptBuilder;

impl ForgeQueryRuntimeWriteAuthorityAdapter for Phase8BoundaryReceiptBuilder {
    fn write(
        &mut self,
        _bridge: &forge_runtime_bridge::facade::RuntimeBridge,
        _relational_runtime: Option<&mut forge_relational::facade::runtime::RelationalRuntime>,
        command: forge_query::facade::ForgeQueryWriteCommand,
    ) -> Result<WriteAuthorityExecutionReceipt, forge_query::facade::ForgeQueryWorkspaceError> {
        Ok(self.build_write_authority_execution_receipt(
            &command,
            phase8_mutation_receipt("phase8-runtime-commit"),
        ))
    }
}

fn phase8_boundary_source(commit_identity: &str) -> WriteAuthorityExecutionReceipt {
    let command = ForgeQueryAspectMutationBuilder::new()
        .aspect("hadwiger.research_graph.invariant", commit_identity)
        .build_insert("hadwiger_research_graph")
        .expect("phase8 boundary command should build");
    Phase8BoundaryReceiptBuilder
        .build_write_authority_execution_receipt(&command, phase8_mutation_receipt(commit_identity))
}

fn phase8_mutation_receipt(commit_identity: &str) -> ForgeQueryMutationReceipt {
    let commit_position = phase8_commit_position(commit_identity);
    ForgeQueryMutationReceipt::from_authoritative_parts(
        ForgeQueryCommitIdentity::from_relational_commit_id(commit_position),
        ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(commit_position, commit_position),
        ),
        vec![ForgeQueryMutationDelta::new(
            "hadwiger_research_graph",
            ForgeQueryEntityIdentity::from_relational_record(
                RelationalBridgeRecordIdentityParts::entity(8, commit_position, 0),
            ),
            ForgeQueryMutationKind::Updated,
            vec!["hadwiger.research_graph.invariant".to_string()],
        )],
    )
}

fn phase8_commit_position(label: &str) -> u64 {
    label
        .bytes()
        .fold(8_000_u64, |acc, byte| {
            acc.wrapping_mul(131).wrapping_add(u64::from(byte))
        })
        .max(1)
}

#[test]
fn catalog_drafting_emits_required_rule_families() {
    let (handle, corpus, frontier, _plans) = corpus_frontier_and_suppressed_plans();

    let left = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    let right = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();

    for family in [
        ResearchGraphInvariantFamily::FailureResidency,
        ResearchGraphInvariantFamily::SuppressionRelation,
        ResearchGraphInvariantFamily::HypothesisLifecycle,
        ResearchGraphInvariantFamily::BranchPromotion,
        ResearchGraphInvariantFamily::ExecutableExperimentAdmission,
    ] {
        assert!(left.has_rule_family(family));
    }
    assert_eq!(left.artifact_digest(), right.artifact_digest());
    assert_eq!(left.counters().checked_rules(), 5);
    assert_eq!(left.counters().registration_ready_rules(), 5);
    assert!(!left.registers_query_invariant_authority());
}

#[test]
fn unsuppressed_experiment_against_rejected_evidence_certifies_violation() {
    let (handle, corpus, frontier, _suppressed) = corpus_frontier_and_suppressed_plans();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    let violation = suppression_violation(&handle, &catalog, &corpus);

    assert_eq!(
        violation.violation_kind(),
        ResearchGraphInvariantViolationKind::SuppressedExperimentMissingProof
    );
    assert_eq!(
        violation.rule_family(),
        ResearchGraphInvariantFamily::SuppressionRelation
    );
    assert!(!violation.admits_theorem_authority());
}

#[test]
fn query_recovery_without_readiness_counter_certifies_drift() {
    let handle = handle();
    let version = graph_version("phase8-readiness-drift");
    let partial = partial_explanation(&handle, &version);
    let recovery = query_recovery_explanation(&handle);
    let corpus = ResearchEvidenceCorpus::builder("phase8-query-drift")
        .with_graph_version(version.reference())
        .with_partial_admission(partial)
        .unwrap()
        .with_query_recovery(recovery)
        .finish()
        .unwrap();
    let clean_batch = unsuppressed_plans(&handle);
    assert_eq!(clean_batch.query_readiness_checks(), 0);
    let observations = mine_research_patterns(&handle, &corpus).unwrap();
    let hypotheses = propose_invariant_hypotheses(&handle, &corpus, &observations).unwrap();
    let plans = plan_next_experiments(&handle, &corpus, &hypotheses).unwrap();
    let frontier =
        update_discovery_frontier(&handle, &corpus, observations, hypotheses, plans).unwrap();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();

    let violation = certify_research_graph_invariant_violation(
        &handle,
        ResearchGraphInvariantCheckRequest::for_experiment_batch(&catalog, clean_batch)
            .with_corpus(&corpus),
    )
    .unwrap();

    assert_eq!(
        violation.violation_kind(),
        ResearchGraphInvariantViolationKind::ExecutableExperimentReadinessDrift
    );
}

#[test]
fn denial_materialization_requires_real_lower_runtime_envelope() {
    let (handle, corpus, frontier, _suppressed) = corpus_frontier_and_suppressed_plans();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    let violation = suppression_violation(&handle, &catalog, &corpus);

    let error = materialize_research_graph_invariant_denial(
        &handle,
        ResearchGraphInvariantDenialRequest::from_violation(&catalog, &violation),
    )
    .expect_err("Phase 8 must not fabricate lower-runtime envelopes");

    assert_eq!(
        error,
        ResearchGraphInvariantError::MissingLowerRuntimeBoundaryEnvelope
    );
}

#[test]
fn denial_materializes_from_query_boundary_source_and_retains_source_basis() {
    let (handle, corpus, frontier, _suppressed) = corpus_frontier_and_suppressed_plans();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    let violation = suppression_violation(&handle, &catalog, &corpus);
    let source = phase8_boundary_source("phase8-boundary-source-a");

    let denial = materialize_research_graph_invariant_denial(
        &handle,
        ResearchGraphInvariantDenialRequest::from_violation(&catalog, &violation)
            .for_lower_runtime_boundary_source(&source),
    )
    .unwrap();

    assert_eq!(
        denial.lower_runtime_source_kind(),
        "write-authority-execution-receipt"
    );
    assert_eq!(
        denial.lower_runtime_source_digest(),
        source
            .boundary_envelope()
            .envelope_identity()
            .terminal_projection_for_reporting()
    );
    assert_eq!(
        denial.lower_runtime_envelope_digest(),
        source
            .boundary_envelope()
            .envelope_identity()
            .terminal_projection_for_reporting()
    );
    assert!(!denial.query_denial().unwrap().denial_digest().is_empty());
    assert!(!denial.admits_theorem_authority());
}

#[test]
fn boundary_source_and_envelope_compatibility_paths_converge() {
    let (handle, corpus, frontier, _suppressed) = corpus_frontier_and_suppressed_plans();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    let violation = suppression_violation(&handle, &catalog, &corpus);
    let source = phase8_boundary_source("phase8-boundary-source-b");

    let from_source = materialize_research_graph_invariant_denial(
        &handle,
        ResearchGraphInvariantDenialRequest::from_violation(&catalog, &violation)
            .for_lower_runtime_boundary_source(source.boundary_envelope()),
    )
    .unwrap();
    let from_envelope = materialize_research_graph_invariant_denial(
        &handle,
        ResearchGraphInvariantDenialRequest::from_violation(&catalog, &violation)
            .for_lower_runtime_boundary_envelope(source.boundary_envelope()),
    )
    .unwrap();

    assert_eq!(
        from_source.query_denial().unwrap().denial_digest(),
        from_envelope.query_denial().unwrap().denial_digest()
    );
    assert_eq!(
        from_source.artifact_digest(),
        from_envelope.artifact_digest()
    );
}

#[test]
fn changed_boundary_source_digest_changes_hadwiger_denial_digest() {
    let (handle, corpus, frontier, _suppressed) = corpus_frontier_and_suppressed_plans();
    let catalog = draft_research_graph_invariant_catalog(&handle, &corpus, &frontier).unwrap();
    let violation = suppression_violation(&handle, &catalog, &corpus);
    let first_source = phase8_boundary_source("phase8-boundary-source-c1");
    let second_source = phase8_boundary_source("phase8-boundary-source-c2");

    let first = materialize_research_graph_invariant_denial(
        &handle,
        ResearchGraphInvariantDenialRequest::from_violation(&catalog, &violation)
            .for_lower_runtime_boundary_source(&first_source),
    )
    .unwrap();
    let second = materialize_research_graph_invariant_denial(
        &handle,
        ResearchGraphInvariantDenialRequest::from_violation(&catalog, &violation)
            .for_lower_runtime_boundary_source(&second_source),
    )
    .unwrap();

    assert_ne!(
        first.lower_runtime_source_digest(),
        second.lower_runtime_source_digest()
    );
    assert_ne!(first.artifact_digest(), second.artifact_digest());
}
