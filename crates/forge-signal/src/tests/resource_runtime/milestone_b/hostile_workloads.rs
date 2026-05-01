use super::support::*;
use super::*;

#[test]
fn resource_async_branch_restore_replay_equivalence_converges_for_equivalent_hostile_suffixes() {
    // Phase 9 branch-local async restore/replay torture coverage:
    // - 18: async branch restore and replay equivalence
    // - reinforces 15 and 17 under branch-local hostile async suffixes
    let outcome = resource_branch_replay_workload(ResourceRequestId::new(50_001));
    let feature = &outcome.feature;
    let sibling = &outcome.sibling;

    for (name, branch) in [("feature", feature), ("sibling", sibling)] {
        assert_ne!(
            branch.replay_after_snapshot_drift.replay_digest(),
            branch.replay_before_restore.replay_digest(),
            "{name} branch drift must perturb replay truth before restore"
        );
        assert_eq!(
            branch.head_snapshot_after_restore, branch.head_snapshot_before_restore,
            "{name} restore must preserve the branch head snapshot checkpoint"
        );
        assert!(
            branch.replay_history_after_restore.frames.len()
                >= branch.replay_history_before_restore.frames.len(),
            "{name} restore may append restore evidence, but it must not erase prior branch replay history"
        );
        assert!(
            branch
                .replay_history_after_restore
                .frames
                .iter()
                .all(|frame| frame.branch_id == branch.branch_id),
            "{name} replay history must stay branch-local after restore"
        );
        assert_eq!(
            branch
                .replay_history_after_restore
                .frames
                .iter()
                .filter(|frame| frame.kind == ReplayEventKind::TransactionCommitted)
                .count(),
            branch
                .replay_history_before_restore
                .frames
                .iter()
                .filter(|frame| frame.kind == ReplayEventKind::TransactionCommitted)
                .count(),
            "{name} restore must not invent or erase committed async replay history"
        );
        assert_eq!(
            branch.replay_after_restore.descriptor_digest(),
            branch.replay_before_restore.descriptor_digest(),
            "{name} restore must preserve descriptor truth"
        );
        assert_eq!(
            branch.replay_after_restore.lifecycle_digest(),
            branch.replay_before_restore.lifecycle_digest(),
            "{name} restore must preserve lifecycle truth"
        );
        assert_eq!(
            branch.replay_after_restore.denied_completion_digest(),
            branch.replay_before_restore.denied_completion_digest(),
            "{name} restore must preserve denial history"
        );
        assert_eq!(
            branch.replay_after_restore.in_flight_digest(),
            branch.replay_before_restore.in_flight_digest(),
            "{name} restore must reconstruct the same in-flight story"
        );
        assert_eq!(
            branch.replay_after_restore.retry_lineage_digest(),
            branch.replay_before_restore.retry_lineage_digest(),
            "{name} restore must preserve retry lineage truth"
        );
        assert_eq!(
            branch.replay_after_restore.replay_digest(),
            branch.replay_before_restore.replay_digest(),
            "{name} equivalent restored suffix must converge exactly"
        );
        assert_eq!(
            branch.restore_report.performance().boundary(),
            ResourceBoundaryKind::BranchRestore,
            "{name} restore must report branch-restore boundary truth"
        );
        assert_eq!(
            branch.restore_report.restored_in_flight_width(),
            branch.replay_after_restore.in_flight_width(),
            "{name} restore report must match replayed in-flight width"
        );
        assert_eq!(
            branch
                .diagnostics_after_restore
                .replay_reconstruction()
                .replay_digest(),
            branch.replay_after_restore.replay_digest(),
            "{name} diagnostics replay provenance must agree with replay reconstruction"
        );
    }

    assert_eq!(
        feature.replay_after_restore.descriptor_digest(),
        sibling.replay_after_restore.descriptor_digest(),
        "equivalent branch restores must converge on identical descriptor truth"
    );
    assert_eq!(
        feature.replay_after_restore.lifecycle_digest(),
        sibling.replay_after_restore.lifecycle_digest(),
        "equivalent branch restores must converge on identical lifecycle truth"
    );
    assert_eq!(
        feature.replay_after_restore.denied_completion_digest(),
        sibling.replay_after_restore.denied_completion_digest(),
        "equivalent branch restores must converge on identical denial truth"
    );
    assert_eq!(
        feature.replay_after_restore.in_flight_digest(),
        sibling.replay_after_restore.in_flight_digest(),
        "equivalent branch restores must converge on identical inflight truth"
    );
    assert_eq!(
        feature.replay_after_restore.replay_digest(),
        sibling.replay_after_restore.replay_digest(),
        "equivalent branch restores must converge on identical replay truth"
    );
    assert_eq!(
        feature.diagnostics_after_restore.provenance_digest(),
        sibling.diagnostics_after_restore.provenance_digest(),
        "equivalent restored suffixes must preserve branch-local diagnostics explanations"
    );
    assert_eq!(
        feature
            .replay_history_after_restore
            .frames
            .iter()
            .filter(|frame| frame.kind == ReplayEventKind::SnapshotRestored)
            .count(),
        sibling
            .replay_history_after_restore
            .frames
            .iter()
            .filter(|frame| frame.kind == ReplayEventKind::SnapshotRestored)
            .count(),
        "equivalent restored suffixes must preserve identical restore replay causality"
    );
}

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

#[test]
fn resource_async_lifecycle_and_rollback_workload_preserves_committed_truth_and_suppresses_observation(
) {
    let outcome = resource_async_lifecycle_rollback_workload();

    assert_eq!(
        outcome.pre_rollback_replay.descriptor_digest(),
        outcome.post_rollback_replay.descriptor_digest(),
        "rollback lane must preserve descriptor truth exactly"
    );
    assert_eq!(
        outcome.pre_rollback_replay.lifecycle_digest(),
        outcome.post_rollback_replay.lifecycle_digest(),
        "rollback lane must preserve lifecycle truth exactly"
    );
    assert_eq!(
        outcome.pre_rollback_replay.output_continuity_digest(),
        outcome.post_rollback_replay.output_continuity_digest(),
        "rollback lane must preserve output continuity truth exactly"
    );
    assert_eq!(
        outcome.pre_rollback_replay.in_flight_digest(),
        outcome.post_rollback_replay.in_flight_digest(),
        "rollback lane must restore the same in-flight story"
    );
    assert_eq!(
        outcome.pre_rollback_replay.retry_lineage_digest(),
        outcome.post_rollback_replay.retry_lineage_digest(),
        "rollback lane must not leak retry-lineage drift"
    );
    assert_eq!(
        outcome.pre_rollback_replay.replay_digest(),
        outcome.post_rollback_replay.replay_digest(),
        "rollback lane must be indistinguishable from the control path where the failed completion never committed"
    );
    assert!(
        outcome.delivered_observations_after_rollback.is_empty(),
        "rollback-suppressed completion must not deliver observer packets"
    );
    assert_eq!(outcome.rollback_observation.events().len(), 1);
    assert_eq!(
        outcome.rollback_observation.events()[0].outcome(),
        ObservationBoundaryOutcome::RollbackSuppressed
    );
    assert_eq!(outcome.control_commit_observation.events().len(), 1);
    assert_eq!(
        outcome.control_commit_observation.events()[0].outcome(),
        ObservationBoundaryOutcome::Delivered
    );
    assert_eq!(
        outcome.rollback_observation.events()[0].observer_id(),
        outcome.control_commit_observation.events()[0].observer_id(),
        "rollback suppression must preserve observer identity exactly"
    );
    assert_eq!(
        outcome.rollback_observation.events()[0].handle_id(),
        outcome.control_commit_observation.events()[0].handle_id(),
        "rollback suppression must preserve observation handle identity exactly"
    );
    assert_eq!(
        outcome.rollback_observation.events()[0].policy(),
        outcome.control_commit_observation.events()[0].policy(),
        "rollback suppression must preserve observation policy exactly"
    );
    assert_eq!(
        outcome.rollback_observation.events()[0].touched(),
        outcome.control_commit_observation.events()[0].touched(),
        "rollback suppression must preserve touched classification exactly"
    );
    assert_eq!(
        outcome.rollback_observation.events()[0].recomputed(),
        outcome.control_commit_observation.events()[0].recomputed(),
        "rollback suppression must preserve recomputed classification exactly"
    );
    assert_eq!(
        outcome.rollback_observation.events()[0].meaningful_change(),
        outcome.control_commit_observation.events()[0].meaningful_change(),
        "rollback suppression must preserve meaningful-change classification exactly"
    );
    assert_eq!(
        outcome.rollback_observation.events()[0].trigger_matched(),
        outcome.control_commit_observation.events()[0].trigger_matched(),
        "rollback suppression must preserve trigger-match classification exactly"
    );
    assert_eq!(
        outcome.rollback_observation.events()[0]
            .matched_resource_nodes()
            .iter()
            .map(|node: &ObservedResourceNodeState| node.node())
            .collect::<Vec<_>>(),
        outcome.control_commit_observation.events()[0]
            .matched_resource_nodes()
            .iter()
            .map(|node: &ObservedResourceNodeState| node.node())
            .collect::<Vec<_>>(),
        "rollback suppression must preserve the same matched resource scope the no-failure control path would deliver"
    );
    assert_eq!(
        outcome.rollback_observation.events()[0].matched_resource_nodes()[0].lifecycle(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        outcome
            .delivered_observations_after_control_commit
        .len(),
        1,
        "the same completion should still deliver one observer packet on the no-failure control path"
    );
    assert_eq!(
        outcome.delivered_observations_after_control_commit[0].observer_id,
        outcome.control_commit_observation.events()[0]
            .observer_id()
            .get()
    );
    assert_eq!(
        outcome.delivered_observations_after_control_commit[0].handle_id,
        outcome.control_commit_observation.events()[0]
            .handle_id()
            .get()
    );
    assert_eq!(
        outcome.delivered_observations_after_control_commit[0].matched_node_count,
        outcome.control_commit_observation.events()[0]
            .matched_resource_nodes()
            .len()
    );
    assert_eq!(
        outcome.delivered_observations_after_control_commit[0].touched,
        outcome.control_commit_observation.events()[0].touched()
    );
    assert_eq!(
        outcome.delivered_observations_after_control_commit[0].recomputed,
        outcome.control_commit_observation.events()[0].recomputed()
    );
    assert_eq!(
        outcome.delivered_observations_after_control_commit[0].meaningful_change,
        outcome.control_commit_observation.events()[0].meaningful_change()
    );
    assert_eq!(
        outcome.delivered_observations_after_control_commit[0].trigger_matched,
        outcome.control_commit_observation.events()[0].trigger_matched()
    );
    assert_ne!(
        outcome.post_rollback_replay.lifecycle_digest(),
        outcome.control_path_replay.lifecycle_digest(),
        "control-path commit should move lifecycle truth beyond the rollback-preserved state"
    );
    assert_ne!(
        outcome.post_rollback_replay.replay_digest(),
        outcome.control_path_replay.replay_digest(),
        "control-path commit should append committed replay truth beyond the rollback-preserved lane"
    );
    assert!(!outcome
        .diagnostics_after_rollback
        .provenance_digest()
        .is_empty());
}

#[test]
fn resource_lifecycle_certification_rejects_non_equivalent_replay_truth() {
    let outcome = resource_branch_replay_workload(ResourceRequestId::new(9_991));

    let err = resource_certification_builder()
        .with_async_resource_lifecycle_parity(
            &outcome.feature.replay_after_restore,
            &outcome.feature.replay_after_snapshot_drift,
            &outcome.feature.diagnostics_after_restore,
            &outcome.feature.diagnostics_after_restore,
        )
        .expect_err("non-equivalent replay truth must not certify lifecycle parity");

    assert!(err
        .to_string()
        .contains("equivalent replay and diagnostics truth"));
}

#[test]
fn resource_rollback_certification_rejects_control_observation_mismatch() {
    let outcome = resource_async_lifecycle_rollback_workload();

    let err = resource_certification_builder()
        .with_async_rollback_observation_equivalence(
            outcome.rollback_report,
            outcome.rollback_observation.clone(),
            outcome.rollback_observation,
            &outcome.pre_rollback_replay,
            &outcome.post_rollback_replay,
            &outcome.diagnostics_after_rollback,
        )
        .expect_err(
            "rollback certification must reject a control path that is not a delivered packet",
        );

    assert!(err
        .to_string()
        .contains("requires only delivered events on the no-failure control path"));
}

#[test]
fn resource_async_inflight_pressure_workload_keeps_matching_local_and_bounded() {
    let outcome = resource_async_inflight_pressure_workload();

    assert_eq!(
        outcome.pressure_performance.boundary(),
        ResourceBoundaryKind::CompletionBatchAdmission
    );
    assert_eq!(outcome.pressure_performance.input_width(), 4);
    assert_eq!(outcome.pressure_performance.admitted_count(), 1);
    assert_eq!(outcome.pressure_performance.denied_count(), 3);
    assert_eq!(outcome.pressure_performance.lifecycle_transition_count(), 1);
    assert_eq!(
        outcome.pressure_performance.operational_allocation_count(),
        3
    );
    assert_eq!(
        outcome
            .pressure_performance
            .retained_history_allocation_count(),
        0
    );
    assert_eq!(
        outcome.pressure_performance.diagnostics_allocation_count(),
        4
    );
    assert_eq!(
        outcome
            .pressure_performance
            .facade_report_allocation_count(),
        1
    );
    assert_eq!(
        outcome.pressure_performance.density_strategy(),
        ResourceDensityStrategy::BurstySortedDeduplicated
    );
    assert_eq!(outcome.pressure_batch.denied_completions().len(), 3);
    assert!(outcome.pressure_batch.denied_completions().iter().any(
        |denied: &DeniedResourceCompletion| denied.class() == CompletionDenialClass::Duplicate
    ));
    assert!(outcome
        .pressure_batch
        .denied_completions()
        .iter()
        .any(|denied: &DeniedResourceCompletion| denied.class()
            == CompletionDenialClass::Contradictory));
    assert!(outcome
        .pressure_batch
        .denied_completions()
        .iter()
        .any(|denied: &DeniedResourceCompletion| denied.class()
            == CompletionDenialClass::UnknownRequest));
    assert_eq!(outcome.telemetry.resource_retry_admission_count, 1);
    assert_eq!(outcome.telemetry.resource_retry_schedule_count, 1);
    assert_eq!(
        outcome
            .telemetry
            .resource_retry_already_scheduled_denial_count,
        1
    );
    assert_eq!(
        outcome
            .telemetry
            .resource_superseded_completion_denial_count,
        1
    );
    assert_eq!(
        outcome.telemetry.resource_duplicate_completion_denial_count,
        1
    );
    assert_eq!(
        outcome
            .telemetry
            .resource_contradictory_completion_denial_count,
        1
    );
    assert_eq!(
        outcome
            .telemetry
            .resource_unknown_request_completion_denial_count,
        2
    );
    assert_eq!(outcome.telemetry.resource_stale_completion_denial_count, 1);
    assert_eq!(outcome.telemetry.resource_branch_restore_count, 1);
    assert!(
        outcome.branch_restore_report.broad_rebuild_denial_count() > 0,
        "branch restore under async pressure must report bounded broad-rebuild denial evidence"
    );
    assert!(
        outcome.branch_restore_report.restored_in_flight_width() > 0,
        "branch restore should carry live inflight width under pressure"
    );
    assert_eq!(
        outcome.runtime_summary.in_flight_request_count(),
        outcome.replay_after_restore.in_flight_width() as u64,
        "runtime summary and replay reconstruction must agree on retained inflight width after pressure churn"
    );
    assert!(
        !outcome.drifted_branch_handle_live_after_restore,
        "restore must not leave post-snapshot drift as ghost inflight state"
    );
    assert_eq!(
        outcome
            .zombie_completion_after_restore
            .denied_completion()
            .expect("restored-away zombie completion should deny explicitly")
            .class(),
        CompletionDenialClass::UnknownRequest
    );
    assert_eq!(
        outcome
            .pre_restore_completion_after_restore
            .denied_completion()
            .expect("pre-restore completion should deny under the restored branch epoch")
            .class(),
        CompletionDenialClass::Stale
    );
    assert!(
        outcome
            .pre_restore_completion_after_restore
            .admitted_completion()
            .is_none(),
        "restore must preserve inflight truth without letting pre-restore completion authority survive branch-epoch rotation"
    );
    assert!(
        outcome.telemetry.resource_hot_in_flight_lookup_count >= 4,
        "completion matching and churn should remain attributable through hot inflight lookups"
    );
}

#[test]
fn resource_async_liveness_failures_preserve_inflight_truth_and_reject_zombie_completion() {
    let outcome = resource_async_inflight_pressure_workload();

    assert!(
        !outcome.drifted_branch_handle_live_after_restore,
        "restored-away drift must not survive as ghost inflight state"
    );
    assert_eq!(
        outcome
            .zombie_completion_after_restore
            .denied_completion()
            .expect("zombie completion after restore should deny explicitly")
            .class(),
        CompletionDenialClass::UnknownRequest
    );
    assert_eq!(
        outcome
            .pre_restore_completion_after_restore
            .denied_completion()
            .expect("pre-restore completion should be stale after restore rekeys the branch epoch")
            .class(),
        CompletionDenialClass::Stale
    );
    assert!(
        outcome
            .pre_restore_completion_after_restore
            .admitted_completion()
            .is_none(),
        "restore must not let pre-restore completion authority survive even while it preserves live inflight truth"
    );
    assert_eq!(
        outcome.runtime_summary.in_flight_request_count(),
        outcome.replay_after_restore.in_flight_width() as u64,
        "runtime summary and replay reconstruction must stay aligned after zombie denial"
    );
}

#[test]
fn resource_inflight_certification_rejects_non_hostile_pressure_evidence() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit");

    let err = resource_certification_builder()
        .with_async_inflight_boundedness(
            runtime.resource_runtime_summary(),
            &runtime.reconstruct_resource_replay_summary(),
            runtime.telemetry().resource,
            admitted.performance(),
        )
        .expect_err("trivial one-request evidence must not certify hostile inflight boundedness");

    assert!(err
        .to_string()
        .contains("requires hostile async pressure evidence"));
}
