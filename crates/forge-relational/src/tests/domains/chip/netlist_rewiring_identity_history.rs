use crate::facade::history::BranchId;
use crate::facade::lineage::{LineageDecisionKind, LineageResolutionStatus};
use crate::facade::replay::{
    RelationalReplayRequest, ReplayAuthorityBasisKind, ReplayExecutionMode,
    ReplayLineageDigestMode, ReplayVerificationMode,
};
use crate::tests::support::*;

#[test]
fn netlist_rewiring_identity_history_preserves_exact_lineage_truth() {
    let mut runtime = persisted_runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "net-a");
    let second = create_entity_outcome(&mut runtime, "net-b");
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

    let candidate = record_lineage_candidate(
        &mut runtime,
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "rewire",
    );
    let resolution = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();

    assert_eq!(resolution.status(), LineageResolutionStatus::Promoted);
    assert!(resolution.promoted_event_id().is_some());
    let promotion_commit_id = resolution
        .promoted_commit_id()
        .expect("promotion commit id");
    let envelope = runtime
        .replay_access()
        .canonical_commit_envelope(promotion_commit_id)
        .cloned()
        .expect("promotion envelope");
    assert!(envelope.lineage_decision_log().iter().any(|decision| {
        decision.kind == LineageDecisionKind::CorrespondencePromotionAccepted
            && decision.candidate_id == Some(candidate.candidate_id)
    }));
    assert_eq!(
        envelope.event_batch_digest_basis().canonical_event_ids(),
        envelope.lineage_event_ids()
    );

    let replay = runtime.replay_authority().replay_commit(RelationalReplayRequest {
        commit_id: promotion_commit_id,
        branch_id: BranchId("main".to_string()),
        execution_mode: ReplayExecutionMode::SerialDeterministic,
        verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
    });
    assert!(replay.failure.is_none());
    assert_eq!(
        replay
            .lineage_authority_basis
            .as_ref()
            .map(|basis| basis.kind()),
        Some(ReplayAuthorityBasisKind::DurableLogCanonical)
    );
    assert_eq!(
        replay
            .lineage_authority_basis
            .as_ref()
            .map(|basis| basis.digest_mode()),
        Some(ReplayLineageDigestMode::ExactCanonicalArtifactDigest)
    );

    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime
        .durability_access()
        .recovery_plan(crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification);
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let recovered_envelope = recovered
        .replay_access()
        .canonical_commit_envelope(promotion_commit_id)
        .cloned()
        .expect("recovered promotion envelope");
    assert_eq!(recovered_envelope, envelope);
}
