use super::*;

#[test]
fn replay_contract_reports_lineage_event_drift_at_digest_layer_when_artifacts_are_tampered() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let first_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&second)[0])
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "lineage-drift",
    );
    let promotion = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    let promoted_commit_id = promotion
        .promoted_commit_id()
        .expect("promotion should publish a metadata-only commit");

    assert!(runtime.history_authority().tamper_commit_envelope_for_test(
        promoted_commit_id,
        |envelope| {
            if let Some(event) = envelope
                .published_lineage_mut_for_test()
                .lineage_events_mut()
                .first_mut()
            {
                event.kind = crate::facade::lineage::LineageEventKind::Retire;
            }
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: promoted_commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert!(
        replay.mismatches.iter().any(|mismatch| {
            mismatch.class == ReplayMismatchClass::LineageDrift
                && mismatch.surface == ReplayObservableSurface::Lineage
                && mismatch.verification_layer
                    == crate::facade::replay::ReplayVerificationLayer::DigestParity
        }),
        "{:?}",
        replay.mismatches
    );
}

#[test]
fn replay_contract_reports_lineage_decision_log_drift_at_digest_layer_when_artifacts_are_tampered()
{
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "source");
    let second = create_entity_outcome(&mut runtime, "target");
    let first_entity = changed_entities(&first)[0];
    let second_entity = changed_entities(&second)[0];
    let first_lineage = runtime
        .lineage_access()
        .for_record(first_entity)
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_access()
        .for_record(second_entity)
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "lineage-decision-drift",
    );
    let promotion = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    let promoted_commit_id = promotion
        .promoted_commit_id()
        .expect("promotion should publish a metadata-only commit");

    assert!(runtime.history_authority().tamper_commit_envelope_for_test(
        promoted_commit_id,
        |envelope| {
            if let Some(decision) = envelope
                .published_lineage_mut_for_test()
                .lineage_decision_log_mut()
                .first_mut()
            {
                decision.kind = crate::facade::lineage::LineageDecisionKind::RetireAccepted;
            }
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: promoted_commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert!(
        replay.mismatches.iter().any(|mismatch| {
            mismatch.class == ReplayMismatchClass::LineageDrift
                && mismatch.surface == ReplayObservableSurface::Lineage
                && mismatch.verification_layer
                    == crate::facade::replay::ReplayVerificationLayer::DigestParity
        }),
        "{:?}",
        replay.mismatches
    );
}

#[test]
fn replay_contract_uses_retained_envelope_basis_only_in_normal_mode() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "source");
    let second = create_entity_outcome(&mut runtime, "target");
    let first_entity = changed_entities(&first)[0];
    let second_entity = changed_entities(&second)[0];
    let first_lineage = runtime
        .lineage_access()
        .for_record(first_entity)
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_access()
        .for_record(second_entity)
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "lineage-retained-envelope-basis",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    assert!(runtime
        .durability_authority()
        .remove_durable_envelope_for_test(second.commit.commit_id));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: second.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert!(runtime.replay().compare_outcome(&replay));
    assert_eq!(
        replay
            .lineage_authority_basis
            .as_ref()
            .map(|basis| basis.kind()),
        Some(crate::facade::replay::ReplayAuthorityBasisKind::RetainedEnvelopeCanonical)
    );
}

#[test]
fn replay_contract_rejects_retained_envelope_basis_in_audit_mode() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "source");
    let second = create_entity_outcome(&mut runtime, "target");
    let first_entity = changed_entities(&first)[0];
    let second_entity = changed_entities(&second)[0];
    let first_lineage = runtime
        .lineage_access()
        .for_record(first_entity)
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_access()
        .for_record(second_entity)
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "lineage-audit-basis",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    assert!(runtime
        .durability_authority()
        .remove_durable_envelope_for_test(second.commit.commit_id));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: second.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
        });

    assert_eq!(
        replay.failure,
        Some(ReplayFailureClass::AuthoritativeBasisUnavailable)
    );
}

#[test]
fn replay_contract_uses_checkpoint_canonical_basis_in_audit_mode_when_durable_log_tail_is_absent() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "source");
    let second = create_entity_outcome(&mut runtime, "target");
    let first_entity = changed_entities(&first)[0];
    let second_entity = changed_entities(&second)[0];
    let first_lineage = runtime
        .lineage_access()
        .for_record(first_entity)
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_access()
        .for_record(second_entity)
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "lineage-checkpoint-basis",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    runtime.durability_authority().checkpoint().unwrap();
    assert!(runtime
        .durability_authority()
        .remove_durable_envelope_for_test(second.commit.commit_id));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: second.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::AuditRecoveryVerification,
        });

    assert!(runtime.replay().compare_outcome(&replay));
    assert_eq!(
        replay
            .lineage_authority_basis
            .as_ref()
            .map(|basis| basis.kind()),
        Some(crate::facade::replay::ReplayAuthorityBasisKind::DurableLogCanonical)
    );
}

#[test]
fn replay_contract_preserves_metadata_only_promotion_commit_truth_and_recovery() {
    let mut runtime = persisted_runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "source");
    let second = create_entity_outcome(&mut runtime, "target");
    let first_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&second)[0])
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "metadata-only-promotion",
    );
    let promoted = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    let promoted_commit_id = promoted.promoted_commit_id().expect("promotion commit id");
    let promoted_commit = runtime
        .history()
        .branch_head(&BranchId("main".to_string()))
        .cloned()
        .expect("promoted branch head");

    assert_eq!(promoted_commit.commit_id, promoted_commit_id);
    assert_eq!(promoted_commit.version_id, second.commit.version_id);

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: promoted_commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert!(runtime.replay().compare_outcome(&replay));

    let recovery_plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered
        .durability_authority()
        .recover(recovery_plan)
        .unwrap();
    let recovered_head = recovered
        .history()
        .branch_head(&BranchId("main".to_string()))
        .cloned()
        .expect("recovered promoted branch head");
    let recovered_replay = recovered
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: promoted_commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    assert_eq!(recovered_head.commit_id, promoted_commit_id);
    assert_eq!(recovered_head.version_id, second.commit.version_id);
    assert!(recovered.replay().compare_outcome(&recovered_replay));
}
