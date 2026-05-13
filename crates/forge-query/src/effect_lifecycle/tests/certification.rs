use crate::effect_lifecycle::{
    certify_effect_lifecycle_phase4, certify_effect_lifecycle_seeded,
    EffectLifecyclePhase4LaneKind, EffectLifecyclePhase4LaneOutcome,
    EffectLifecycleSeededOutcomeClass,
};

#[test]
fn seeded_certification_replays_the_same_seed_identically() {
    let left = certify_effect_lifecycle_seeded(17, 12);
    let right = certify_effect_lifecycle_seeded(17, 12);

    assert_eq!(
        left.seeded_sequence_digest(),
        right.seeded_sequence_digest()
    );
    assert_eq!(left.seed_replay_digest(), right.seed_replay_digest());
    assert_eq!(
        left.certification_bundle_digest(),
        right.certification_bundle_digest()
    );
    assert!(left.replay_is_deterministic());
    assert!(right.replay_is_deterministic());
}

#[test]
fn seeded_certification_changes_when_the_seed_changes() {
    let left = certify_effect_lifecycle_seeded(17, 12);
    let right = certify_effect_lifecycle_seeded(18, 12);

    assert_ne!(
        left.seeded_sequence_digest(),
        right.seeded_sequence_digest()
    );
    assert_ne!(
        left.certification_bundle_digest(),
        right.certification_bundle_digest()
    );
}

#[test]
fn seeded_certification_covers_distinct_postures_and_batch_widths() {
    let bundle = certify_effect_lifecycle_seeded(17, 12);
    let rows = bundle.rows();

    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::ScalarExecuted));
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::BatchExecuted));
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::Advisory));
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::RebindRequired));
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::Deferred));
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::Denied));
    assert!(rows.iter().any(|row| row.batch_width() > 1));
    assert!(rows.iter().all(|row| !row.row_digest().is_empty()));
    assert!(rows
        .iter()
        .all(|row| !row.counter_snapshot_digest().is_empty()));
    assert!(rows
        .iter()
        .any(|row| row.counters().effect_executor_rediscovery_count() == 0));
    assert!(rows
        .iter()
        .any(|row| row.counters().batch_lowering_count() == 1));
}

#[test]
fn seeded_certification_never_certifies_less_than_full_template_coverage() {
    let bundle = certify_effect_lifecycle_seeded(17, 1);
    let rows = bundle.rows();

    assert!(rows.len() >= 9);
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::ScalarExecuted));
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::BatchExecuted));
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::Advisory));
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::RebindRequired));
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::Deferred));
    assert!(rows
        .iter()
        .any(|row| row.outcome_class() == EffectLifecycleSeededOutcomeClass::Denied));
}

#[test]
fn phase4_certification_covers_named_execution_and_boundary_lanes() {
    let bundle = certify_effect_lifecycle_phase4();
    let rows = bundle.rows();

    assert!(rows.iter().any(|row| row.lane_kind()
        == EffectLifecyclePhase4LaneKind::BranchMutationExecution
        && row.outcome() == EffectLifecyclePhase4LaneOutcome::Executed));
    assert!(rows.iter().any(|row| row.lane_kind()
        == EffectLifecyclePhase4LaneKind::RelationalMergeExecution
        && row.outcome() == EffectLifecyclePhase4LaneOutcome::Executed));
    assert!(rows.iter().any(|row| row.lane_kind()
        == EffectLifecyclePhase4LaneKind::BridgeWritebackExecution
        && row.outcome() == EffectLifecyclePhase4LaneOutcome::Executed));
    assert!(rows.iter().any(|row| row.lane_kind()
        == EffectLifecyclePhase4LaneKind::BatchExecution
        && row.outcome() == EffectLifecyclePhase4LaneOutcome::Executed));
    assert!(rows.iter().any(|row| row.lane_kind()
        == EffectLifecyclePhase4LaneKind::BatchLaneDenial
        && row.outcome() == EffectLifecyclePhase4LaneOutcome::Denied));
    assert!(rows.iter().any(
        |row| row.lane_kind() == EffectLifecyclePhase4LaneKind::PreviewRebind
            && row.outcome() == EffectLifecyclePhase4LaneOutcome::RebindRequired
    ));
    assert!(rows.iter().any(|row| row.lane_kind()
        == EffectLifecyclePhase4LaneKind::DeferredReplay
        && row.outcome() == EffectLifecyclePhase4LaneOutcome::Deferred));
    assert!(rows.iter().any(|row| row.lane_kind()
        == EffectLifecyclePhase4LaneKind::HostOverrideDenial
        && row.outcome() == EffectLifecyclePhase4LaneOutcome::Denied));
    assert!(rows.iter().any(|row| row.lane_kind()
        == EffectLifecyclePhase4LaneKind::StaleAfterAdmission
        && row.outcome() == EffectLifecyclePhase4LaneOutcome::Denied));
    assert!(rows.iter().any(|row| row.lane_kind()
        == EffectLifecyclePhase4LaneKind::StaleAfterLowering
        && row.outcome() == EffectLifecyclePhase4LaneOutcome::Denied));
    assert!(rows.iter().any(|row| row.lane_kind()
        == EffectLifecyclePhase4LaneKind::RelationalOracle
        && row.outcome() == EffectLifecyclePhase4LaneOutcome::Verified));
    assert!(rows.iter().any(
        |row| row.lane_kind() == EffectLifecyclePhase4LaneKind::BridgeOracle
            && row.outcome() == EffectLifecyclePhase4LaneOutcome::Verified
    ));
    assert!(rows.iter().any(
        |row| row.lane_kind() == EffectLifecyclePhase4LaneKind::SeededReplay
            && row.outcome() == EffectLifecyclePhase4LaneOutcome::Certified
    ));
}

#[test]
fn phase4_certification_retains_batch_native_and_zero_rediscovery_evidence() {
    let bundle = certify_effect_lifecycle_phase4();
    let batch = bundle
        .rows()
        .iter()
        .find(|row| row.lane_kind() == EffectLifecyclePhase4LaneKind::BatchExecution)
        .expect("batch lane should be certified");
    let override_denial = bundle
        .rows()
        .iter()
        .find(|row| row.lane_kind() == EffectLifecyclePhase4LaneKind::HostOverrideDenial)
        .expect("host override lane should be certified");

    assert_eq!(batch.counters().batch_lowering_count(), 1);
    assert_eq!(batch.counters().effect_execution_width(), 1);
    assert_eq!(batch.counters().effect_executor_rediscovery_count(), 0);
    assert_eq!(override_denial.counters().effect_execution_width(), 0);
    assert!(!bundle.phase4_bundle_digest().is_empty());
    assert!(!bundle.seeded_bundle_digest().is_empty());
}
