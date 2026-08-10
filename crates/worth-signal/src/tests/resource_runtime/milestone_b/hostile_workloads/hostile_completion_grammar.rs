use super::super::super::declaration_and_visibility::raw_completion;
use super::super::super::diagnostics::resource_malformed_completion_report;
use super::super::super::support::resource_declaration;
use super::super::super::{
    assert_hostile_evidence_shape, assert_performance_closeout_claim_shape,
    required_hostile_evidence_row, required_performance_claim_row, required_scenario_row,
    resource_milestone_b_certification_run, resource_milestone_b_hostile_scenario_evidence,
    resource_milestone_b_performance_closeout, resource_milestone_b_scenario_matrix,
    CompletionDenialClass, RawCompletionEnvelope, ResourceAttemptId, ResourceBranchEpoch,
    ResourceGeneration, ResourceMilestoneBPerformanceClaimId,
    ResourceMilestoneBScenarioEvidenceKind, ResourceMilestoneBScenarioId, ResourceNodeId,
    ResourcePayloadContractDigest, ResourceRequestId, ResourceRequestIntent, SignalGraph,
    TestRuntime, REQUIRED_RESOURCE_CERTIFICATION_FAMILIES,
    REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS,
};
use super::super::support::{
    resource_certification_fixture_artifacts, resource_late_cancelled_completion_report,
    resource_late_superseded_completion_report, resource_late_timed_out_completion_report,
};

#[test]
fn resource_async_nightmare_grammar_preserves_canonical_truth_across_restore_and_replay() {
    // Phase 9 async nightmare grammar coverage:
    // - 15: async resource lifecycle parity
    // - 16: out-of-order completion supersession
    // - 17: async rollback and observation equivalence
    // - 18: async branch restore and replay equivalence
    // - 19A / 19B: mixed completion-ordering, completion-integrity,
    //   request-identity, liveness, and async-pressure failures in one lane
    let (bundle, hostile_evidence, summary_read, diagnostics_summary, diagnostics_denial) =
        resource_certification_fixture_artifacts(ResourceRequestId::new(9_999));

    assert!(bundle.passed());
    assert_eq!(
        bundle.summary().passed_family_count(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(bundle.summary().failed_family_count(), 0);

    let scenario_matrix = resource_milestone_b_scenario_matrix(&bundle, &hostile_evidence)
        .expect("nightmare grammar fixture should satisfy milestone B scenario matrix");
    let performance_closeout = resource_milestone_b_performance_closeout(
        &scenario_matrix,
        summary_read,
        diagnostics_summary,
        diagnostics_denial.clone(),
    )
    .expect("nightmare grammar fixture should satisfy performance closeout");
    let run = resource_milestone_b_certification_run(
        bundle.clone(),
        scenario_matrix.clone(),
        performance_closeout.clone(),
    )
    .expect("nightmare grammar fixture should satisfy milestone B certification run");

    assert!(scenario_matrix.passed());
    assert!(performance_closeout.passed());
    assert!(run.passed());
    assert_eq!(
        hostile_evidence.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS.len()
    );

    let superseded_row = required_hostile_evidence_row(
        &hostile_evidence,
        ResourceMilestoneBScenarioId::LateCompletionAfterSupersessionRejected,
    );
    let cancelled_row = required_hostile_evidence_row(
        &hostile_evidence,
        ResourceMilestoneBScenarioId::LateCompletionAfterCancellationRejected,
    );
    let timed_out_row = required_hostile_evidence_row(
        &hostile_evidence,
        ResourceMilestoneBScenarioId::LateCompletionAfterTimeoutRejected,
    );
    let malformed_row = required_hostile_evidence_row(
        &hostile_evidence,
        ResourceMilestoneBScenarioId::MalformedCompletionRejected,
    );
    let duplicate_row = required_hostile_evidence_row(
        &hostile_evidence,
        ResourceMilestoneBScenarioId::DuplicateCompletionRejected,
    );
    let contradictory_row = required_hostile_evidence_row(
        &hostile_evidence,
        ResourceMilestoneBScenarioId::ContradictoryCompletionRejected,
    );
    let unknown_row = required_hostile_evidence_row(
        &hostile_evidence,
        ResourceMilestoneBScenarioId::UnknownRequestCompletionRejected,
    );
    assert_hostile_evidence_shape(superseded_row);
    assert_hostile_evidence_shape(cancelled_row);
    assert_hostile_evidence_shape(timed_out_row);
    assert_hostile_evidence_shape(malformed_row);
    assert_hostile_evidence_shape(duplicate_row);
    assert_hostile_evidence_shape(contradictory_row);
    assert_hostile_evidence_shape(unknown_row);
    assert_eq!(
        superseded_row.expected_denial_class(),
        CompletionDenialClass::Superseded
    );
    assert_eq!(
        cancelled_row.expected_denial_class(),
        CompletionDenialClass::Cancelled
    );
    assert_eq!(
        timed_out_row.expected_denial_class(),
        CompletionDenialClass::TimedOut
    );
    assert_eq!(
        malformed_row.expected_denial_class(),
        CompletionDenialClass::Malformed
    );
    assert_eq!(
        duplicate_row.expected_denial_class(),
        CompletionDenialClass::Duplicate
    );
    assert_eq!(
        contradictory_row.expected_denial_class(),
        CompletionDenialClass::Contradictory
    );
    assert_eq!(
        unknown_row.expected_denial_class(),
        CompletionDenialClass::UnknownRequest
    );
    assert_ne!(
        superseded_row.evidence_digest(),
        cancelled_row.evidence_digest(),
        "mixed async denial families must stay provenance-distinct"
    );
    assert_ne!(
        superseded_row.evidence_digest(),
        timed_out_row.evidence_digest(),
        "timeout truth must not collapse into supersession truth"
    );
    assert_ne!(
        cancelled_row.evidence_digest(),
        malformed_row.evidence_digest(),
        "completion-integrity failures must stay distinct from lifecycle denial truth"
    );
    assert_ne!(
        duplicate_row.evidence_digest(),
        contradictory_row.evidence_digest(),
        "duplicate delivery and contradictory delivery must remain distinct nightmare grammar evidence"
    );
    assert_ne!(
        contradictory_row.evidence_digest(),
        unknown_row.evidence_digest(),
        "request-identity failures must not collapse into contradictory payload drift"
    );

    let rollback_row = required_scenario_row(
        &scenario_matrix,
        ResourceMilestoneBScenarioId::RollbackObservationEquivalence,
    );
    let replay_row = required_scenario_row(
        &scenario_matrix,
        ResourceMilestoneBScenarioId::LifecycleReplayParity,
    );
    let branch_row = required_scenario_row(
        &scenario_matrix,
        ResourceMilestoneBScenarioId::BranchRestoreReplayEquivalence,
    );
    let inflight_row = required_scenario_row(
        &scenario_matrix,
        ResourceMilestoneBScenarioId::InflightBoundedness,
    );
    assert_eq!(
        rollback_row.evidence_kind(),
        ResourceMilestoneBScenarioEvidenceKind::CertificationFamily
    );
    assert_eq!(
        replay_row.evidence_kind(),
        ResourceMilestoneBScenarioEvidenceKind::CertificationFamily
    );
    assert_eq!(
        branch_row.evidence_kind(),
        ResourceMilestoneBScenarioEvidenceKind::CertificationFamily
    );
    assert_eq!(
        inflight_row.evidence_kind(),
        ResourceMilestoneBScenarioEvidenceKind::CertificationFamily
    );

    let rollback_claim = required_performance_claim_row(
        &performance_closeout,
        ResourceMilestoneBPerformanceClaimId::RollbackObservationRollbackBounded,
    );
    let branch_claim = required_performance_claim_row(
        &performance_closeout,
        ResourceMilestoneBPerformanceClaimId::BranchRestoreReplayRestoreBounded,
    );
    let inflight_claim = required_performance_claim_row(
        &performance_closeout,
        ResourceMilestoneBPerformanceClaimId::InflightBoundednessAdmissionBounded,
    );
    let hostile_claim = required_performance_claim_row(
        &performance_closeout,
        ResourceMilestoneBPerformanceClaimId::HostileCompletionDenialsScalarBounded,
    );
    assert_performance_closeout_claim_shape(rollback_claim);
    assert_performance_closeout_claim_shape(branch_claim);
    assert_performance_closeout_claim_shape(inflight_claim);
    assert_performance_closeout_claim_shape(hostile_claim);
    assert_eq!(hostile_claim.performance().input_width(), 4);
    assert_eq!(hostile_claim.performance().denied_count(), 4);
    assert_eq!(
        diagnostics_denial
            .performance()
            .diagnostics_allocation_count(),
        0,
        "strict diagnostics denial must stay zero-cold inside the nightmare grammar workload"
    );

    assert_eq!(run.bundle().bundle_digest(), bundle.bundle_digest());
    assert_eq!(
        run.scenario_matrix().matrix_digest(),
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        run.performance_closeout().closeout_digest(),
        performance_closeout.closeout_digest()
    );
}

#[test]
fn resource_milestone_b_hostile_scenario_evidence_rejects_non_hostile_batch_denials() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let accepted = raw_completion(
        &runtime,
        node,
        admitted_request.handle(),
        admitted_request.attempt(),
        64,
    );
    let contradictory = raw_completion(
        &runtime,
        node,
        admitted_request.handle(),
        admitted_request.attempt(),
        96,
    );
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();
    let unknown = RawCompletionEnvelope::new(
        ResourceRequestId::new(77_001),
        ResourceGeneration::new(1),
        ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
        ResourceAttemptId::ZERO,
        digest,
        32,
    );
    let malformed = RawCompletionEnvelope::new(
        admitted_request.handle().request_id(),
        admitted_request.handle().generation(),
        admitted_request.handle().branch_epoch(),
        admitted_request.attempt(),
        ResourcePayloadContractDigest::new("payload-contract:999:1024"),
        64,
    );
    let oversized_batch = runtime.admit_resource_completion_batch([
        contradictory,
        accepted.clone(),
        accepted,
        unknown,
        malformed,
    ]);

    let err = resource_milestone_b_hostile_scenario_evidence(
        resource_late_superseded_completion_report(),
        resource_late_cancelled_completion_report(),
        resource_late_timed_out_completion_report(),
        resource_malformed_completion_report(),
        &oversized_batch,
    )
    .expect_err("nightmare hostile rows must reject arbitrary completion batches");

    assert!(err
        .to_string()
        .contains("requires hostile mixed batch denial evidence"));
}
