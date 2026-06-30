pub(crate) mod subject;

use subject::{
    certify_retained_cancellation_chain, certify_retained_cancellation_chain_with_checkpoints,
    duplicate_checkpoint_denial_outcome, foreign_checkpoint_stage_denial_outcome,
    live_extraction_denial_outcome, missing_trigger_local_replay_outcome,
    projection_consumed_forgery_denial_outcome, projection_consumed_mismatch_outcome,
    retained_cancellation_outcome_matrix, retained_replay_mismatch_outcome,
};
use worth_spatial::facade::user_response::{
    WorthPolicyDecision, WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};

#[test]
fn mb_m6_4_retained_planar_history_cancellation_chain() {
    let subject = certify_retained_cancellation_chain("contract");
    let counters = subject.receipt.counters();

    assert!(!subject.receipt.compiled_product_family_digest().is_empty());
    assert!(!subject
        .receipt
        .compiled_product_identity_digest()
        .is_empty());
    assert!(!subject
        .receipt
        .equivalence_policy_identity_digest()
        .is_empty());
    assert_eq!(counters.checkpoint_count(), 32);
    assert_eq!(counters.replayed_checkpoint_count(), 8);
    assert_eq!(counters.trigger_local_replay_count(), 0);
    assert_eq!(counters.transform_step_count(), 32);
    assert_eq!(counters.retained_artifact_count(), 64);
    assert!(counters.projection_consumed_fact_count() >= 32);
    assert!(counters.diagnostic_trigger_count() > 0);
    assert_eq!(counters.user_outcome_count(), 1);
    assert!(subject.catalog_retained_artifact_count > 0);
    assert!(subject.catalog_replay_checkpoint_count > 0);
    assert_eq!(subject.receipt.checkpoints().len(), 32);
    assert_eq!(subject.user_outcome.kind(), WorthUserOutcomeKind::Admitted);
    assert_human_readable(subject.user_outcome.human_response().summary());

    let repeated = certify_retained_cancellation_chain("contract");
    assert_eq!(
        subject.receipt.compiled_product_family_digest(),
        repeated.receipt.compiled_product_family_digest()
    );
    assert_eq!(
        subject.receipt.compiled_product_identity_digest(),
        repeated.receipt.compiled_product_identity_digest()
    );
    assert_eq!(
        subject.receipt.equivalence_policy_identity_digest(),
        repeated.receipt.equivalence_policy_identity_digest()
    );

    for checkpoint in subject.receipt.checkpoints() {
        assert_eq!(
            checkpoint.transform_stage_receipt_identity(),
            subject.catalog_transform_stage_identity
        );
        assert_eq!(
            checkpoint.retained_replay_stage_identity(),
            subject.catalog_retained_replay_stage_identity
        );
    }
}

#[test]
fn mb_m6_4_hostile_retained_chain_uses_128_checkpoint_profile() {
    let subject = certify_retained_cancellation_chain_with_checkpoints("hostile-128", 128);
    let counters = subject.receipt.counters();

    assert_eq!(counters.checkpoint_count(), 128);
    assert_eq!(counters.replayed_checkpoint_count(), 32);
    assert_eq!(counters.transform_step_count(), 128);
    assert_eq!(counters.retained_artifact_count(), 256);
    assert_eq!(subject.user_outcome.kind(), WorthUserOutcomeKind::Admitted);
}

#[test]
fn mb_m6_4_retained_outcome_matrix_branches_each_history_stop() {
    let outcomes = retained_cancellation_outcome_matrix("matrix");

    assert_one_kind(&outcomes, WorthUserOutcomeKind::PolicyRequired);
    assert_one_kind(&outcomes, WorthUserOutcomeKind::PredicateUncertain);
    assert_one_kind(&outcomes, WorthUserOutcomeKind::Denied);
    assert_kind_count(&outcomes, WorthUserOutcomeKind::IntegrityMismatch, 2);

    assert_branch(
        &outcomes[0],
        WorthUserOutcomeKind::PolicyRequired,
        WorthUserOutcomeCauseKind::PolicyRequired,
    );
    assert!(outcomes[0]
        .choices()
        .contains(&WorthPolicyDecision::pause_for_manual_inspection()));
    assert_branch(
        &outcomes[1],
        WorthUserOutcomeKind::PredicateUncertain,
        WorthUserOutcomeCauseKind::PredicateUncertain,
    );
    assert_branch(
        &outcomes[2],
        WorthUserOutcomeKind::IntegrityMismatch,
        WorthUserOutcomeCauseKind::IntegrityMismatch,
    );
    assert_branch(
        &outcomes[3],
        WorthUserOutcomeKind::Denied,
        WorthUserOutcomeCauseKind::DeniedMovementOrRotation,
    );
    assert_branch(
        &outcomes[4],
        WorthUserOutcomeKind::IntegrityMismatch,
        WorthUserOutcomeCauseKind::IntegrityMismatch,
    );

    assert_message_contains(&outcomes, "checkpoint 9");
    assert_message_contains(&outcomes, "checkpoint 13");
    assert_message_contains(&outcomes, "checkpoint 17");
    assert_message_contains(&outcomes, "checkpoint 21");
    assert_message_contains(&outcomes, "checkpoint 25");

    for outcome in &outcomes {
        assert_human_readable(outcome.human_response().summary());
        assert!(!outcome.evidence().digest().is_empty());
        assert!(!outcome.evidence().source_identity().is_empty());
        if outcome.kind() != WorthUserOutcomeKind::PolicyRequired {
            assert!(outcome.choices().is_empty());
        }
    }
}

#[test]
fn mb_m6_4_projection_consumed_facts_match_retained_checkpoints() {
    let subject = certify_retained_cancellation_chain("projection-match");

    for checkpoint in subject.receipt.checkpoints() {
        assert_eq!(
            checkpoint.retained_basis_identity(),
            subject.receipt.retained_basis_identity()
        );
        assert!(checkpoint
            .projection_consumed_identity()
            .contains(checkpoint.retained_basis_identity()));
        assert!(checkpoint
            .projection_consumed_identity()
            .contains("replay-evidence:"));
    }
    assert_all_checkpoint_identities_are_distinct(subject.receipt.checkpoints());

    let mismatch = projection_consumed_mismatch_outcome("projection-mismatch");
    assert_branch(
        &mismatch,
        WorthUserOutcomeKind::IntegrityMismatch,
        WorthUserOutcomeCauseKind::IntegrityMismatch,
    );
    assert!(mismatch
        .human_response()
        .summary()
        .contains("checkpoint 25"));
}

fn assert_all_checkpoint_identities_are_distinct(
    checkpoints: &[worth_spatial::facade::retained_cancellation_chain::RetainedCancellationCheckpoint],
) {
    let mut capture_identities = std::collections::BTreeSet::new();
    let mut replay_identities = std::collections::BTreeSet::new();
    let mut projection_identities = std::collections::BTreeSet::new();
    for checkpoint in checkpoints {
        assert!(capture_identities.insert(checkpoint.retained_artifact_capture_identity()));
        assert!(replay_identities.insert(checkpoint.replay_checkpoint_identity()));
        assert!(projection_identities.insert(checkpoint.projection_consumed_identity()));
    }
}

#[test]
fn mb_m6_4_retained_replay_mismatch_cannot_be_reported_as_missing_evidence() {
    let outcome = retained_replay_mismatch_outcome("replay-mismatch");

    assert_branch(
        &outcome,
        WorthUserOutcomeKind::IntegrityMismatch,
        WorthUserOutcomeCauseKind::IntegrityMismatch,
    );
    assert!(outcome.human_response().summary().contains("checkpoint 17"));
}

#[test]
fn mb_m6_4_replay_denies_live_extraction_and_duplicate_checkpoint_evidence() {
    let live_extraction = live_extraction_denial_outcome("live-extraction");
    assert_branch(
        &live_extraction,
        WorthUserOutcomeKind::Unsupported,
        WorthUserOutcomeCauseKind::UnsupportedInput,
    );
    assert!(live_extraction
        .human_response()
        .summary()
        .contains("forbids live extraction"));

    let duplicate = duplicate_checkpoint_denial_outcome("duplicate-checkpoint");
    assert_branch(
        &duplicate,
        WorthUserOutcomeKind::NoOptions,
        WorthUserOutcomeCauseKind::MissingEvidence,
    );
    assert!(duplicate
        .human_response()
        .summary()
        .contains("distinct retained evidence"));
}

#[test]
fn mb_m6_4_trigger_local_replay_and_projection_forgery_are_denied() {
    let unsampled_trigger = missing_trigger_local_replay_outcome("unsampled-trigger");
    assert_branch(
        &unsampled_trigger,
        WorthUserOutcomeKind::NoOptions,
        WorthUserOutcomeCauseKind::MissingEvidence,
    );
    assert!(unsampled_trigger
        .human_response()
        .summary()
        .contains("checkpoint 9"));
    assert_human_readable(unsampled_trigger.human_response().summary());

    let forged_projection = projection_consumed_forgery_denial_outcome("forged-projection");
    assert_branch(
        &forged_projection,
        WorthUserOutcomeKind::IntegrityMismatch,
        WorthUserOutcomeCauseKind::IntegrityMismatch,
    );
    assert!(forged_projection
        .human_response()
        .summary()
        .contains("checkpoint 25"));
    assert_human_readable(forged_projection.human_response().summary());

    let foreign_stage = foreign_checkpoint_stage_denial_outcome("foreign-stage");
    assert_branch(
        &foreign_stage,
        WorthUserOutcomeKind::IntegrityMismatch,
        WorthUserOutcomeCauseKind::IntegrityMismatch,
    );
    assert!(foreign_stage
        .human_response()
        .summary()
        .contains("checkpoint 11"));
    assert!(foreign_stage
        .human_response()
        .summary()
        .contains("workload catalog transform evidence receipt"));
    assert_human_readable(foreign_stage.human_response().summary());
}

fn assert_branch(
    outcome: &WorthUserOutcome,
    kind: WorthUserOutcomeKind,
    cause_kind: WorthUserOutcomeCauseKind,
) {
    assert_eq!(outcome.kind(), kind);
    assert_eq!(outcome.cause().map(|cause| cause.kind()), Some(cause_kind));
}

fn assert_one_kind(outcomes: &[WorthUserOutcome], kind: WorthUserOutcomeKind) {
    assert_kind_count(outcomes, kind, 1);
}

fn assert_kind_count(outcomes: &[WorthUserOutcome], kind: WorthUserOutcomeKind, count: usize) {
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome.kind() == kind)
            .count(),
        count
    );
}

fn assert_message_contains(outcomes: &[WorthUserOutcome], expected: &str) {
    assert!(
        outcomes
            .iter()
            .any(|outcome| outcome.human_response().summary().contains(expected)),
        "missing retained cancellation outcome message containing {expected:?}"
    );
}

fn assert_human_readable(message: &str) {
    assert!(!message.trim().is_empty());
    assert!(
        !message.contains('_'),
        "user-facing retained cancellation message must not leak machine tokens: {message}"
    );
    assert!(
        !message
            .split_whitespace()
            .any(|word| word.matches('-').count() >= 3),
        "user-facing retained cancellation message must explain causes in prose: {message}"
    );
}
