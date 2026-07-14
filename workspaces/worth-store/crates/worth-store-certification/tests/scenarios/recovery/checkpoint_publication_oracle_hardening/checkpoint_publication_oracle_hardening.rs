use worth_store_test_support::harness::recovery::checkpoint_publication as checkpoint_oracle_support;
use worth_store_test_support::harness::recovery::counter_evidence as support;

use checkpoint_oracle_support::{
    checkpoint_crash_replay_trace, checkpoint_crash_replay_trace_without_crash_lane,
    checkpoint_origin, checkpoint_trace, detached_replay_bundle_from_parts,
    lower_checkpoint_crash_replay_plan, lower_checkpoint_plan, production_fixture, schedule,
};
use worth_store_physical_certification::{
    CrashRecoversOldOrNewNeverMixedOracle, ExecutedPhysicalSimulationObservation,
    ExecutedTranscriptParts, NoMixedRootOracle, OldReaderSeesOldRootOracle, OracleDenial,
    PhysicalCertificationEvidenceBundle, PhysicalProofOracleKind, PhysicalProofOracleVerdictKind,
    PhysicalSimulationObserver, PostSwapReaderSeesNewRootOracle, ReusablePhysicalOracleFamily,
};

#[test]
fn checkpoint_publication_observation_satisfies_physical_isolation_reader_oracles() {
    let plan = lower_checkpoint_plan();
    let trace = checkpoint_trace(&plan);
    let family = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape();

    for verdict in [
        family
            .oracle(NoMixedRootOracle)
            .judge(&plan, &trace)
            .unwrap(),
        family
            .oracle(OldReaderSeesOldRootOracle)
            .judge(&plan, &trace)
            .unwrap(),
        family
            .oracle(PostSwapReaderSeesNewRootOracle)
            .judge(&plan, &trace)
            .unwrap(),
    ] {
        assert_eq!(verdict.kind(), PhysicalProofOracleVerdictKind::Satisfied);
    }
}

#[test]
fn checkpoint_publication_observation_is_carried_by_replay_transcript() {
    let plan = lower_checkpoint_plan();
    let trace = checkpoint_trace(&plan);
    let no_mixed_root_verdict = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(NoMixedRootOracle)
        .judge(&plan, &trace)
        .unwrap();
    let replay = detached_replay_bundle_from_parts(
        ExecutedTranscriptParts::new(
            &plan,
            schedule(&plan),
            &production_fixture(),
            trace.clone(),
            support::counter_receipt(&plan, trace.clone()),
        )
        .unwrap()
        .with_oracle_verdict(no_mixed_root_verdict),
    );
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();

    assert!(evidence.replay().trace().checkpoint_interlock().is_some());
    assert!(evidence.replay().oracle_verdicts().iter().any(|verdict| {
        verdict.family()
            == worth_store_physical_certification::OracleFamilyKind::TranscriptReplayEvidence
            && verdict.oracle() == PhysicalProofOracleKind::TranscriptReplay
    }));
    assert!(evidence
        .replay()
        .oracle_verdicts()
        .iter()
        .any(|verdict| verdict.oracle() == PhysicalProofOracleKind::NoMixedRoot));
}

#[test]
fn checkpoint_interlock_observation_changes_replay_basis_identity() {
    let plan = lower_checkpoint_plan();
    let checkpoint_trace = checkpoint_trace(&plan);
    let trace_without_checkpoint = support::observed_trace(&plan);
    let family = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape();
    let checkpoint_verdict = family
        .oracle(NoMixedRootOracle)
        .judge(&plan, &checkpoint_trace)
        .unwrap();
    let without_checkpoint_verdict = family
        .oracle(NoMixedRootOracle)
        .judge(&plan, &trace_without_checkpoint)
        .unwrap();
    let replay_with_checkpoint = detached_replay_bundle_from_parts(
        ExecutedTranscriptParts::new(
            &plan,
            schedule(&plan),
            &production_fixture(),
            checkpoint_trace.clone(),
            support::counter_receipt(&plan, checkpoint_trace),
        )
        .unwrap()
        .with_oracle_verdict(checkpoint_verdict),
    );
    let replay_without_checkpoint = detached_replay_bundle_from_parts(
        ExecutedTranscriptParts::new(
            &plan,
            schedule(&plan),
            &production_fixture(),
            trace_without_checkpoint.clone(),
            support::counter_receipt(&plan, trace_without_checkpoint),
        )
        .unwrap()
        .with_oracle_verdict(without_checkpoint_verdict),
    );

    assert_ne!(
        replay_with_checkpoint
            .replay_basis_identity()
            .digest_bytes(),
        replay_without_checkpoint
            .replay_basis_identity()
            .digest_bytes()
    );
}

#[test]
fn checkpoint_publication_lane_is_paired_with_recovery_crash_replay_proof() {
    let plan = lower_checkpoint_crash_replay_plan();
    let trace = checkpoint_crash_replay_trace(&plan);
    let no_mixed_root_verdict = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(NoMixedRootOracle)
        .judge(&plan, &trace)
        .unwrap();
    let recovery_verdict = ReusablePhysicalOracleFamily::recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(&plan, &trace)
        .unwrap();
    let replay = detached_replay_bundle_from_parts(
        ExecutedTranscriptParts::new(
            &plan,
            schedule(&plan),
            &production_fixture(),
            trace.clone(),
            support::counter_receipt(&plan, trace),
        )
        .unwrap()
        .with_oracle_verdict(no_mixed_root_verdict)
        .with_oracle_verdict(recovery_verdict),
    );
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();

    assert!(evidence.replay().trace().checkpoint_interlock().is_some());
    assert!(evidence.replay().trace().recovery_outcome().is_some());
    assert!(evidence
        .replay()
        .trace()
        .checkpoint_crash_replay()
        .is_some());
    assert_eq!(
        evidence
            .replay()
            .trace()
            .checkpoint_crash_replay()
            .unwrap()
            .checkpoint_origin(),
        &checkpoint_origin()
    );
    assert!(evidence.replay().oracle_verdicts().iter().any(|verdict| {
        verdict.oracle() == PhysicalProofOracleKind::CrashRecoversOldOrNewNeverMixed
            && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
    }));
    assert!(evidence.replay().oracle_verdicts().iter().any(|verdict| {
        verdict.oracle() == PhysicalProofOracleKind::TranscriptReplay
            && verdict.kind() == PhysicalProofOracleVerdictKind::Satisfied
    }));
}

#[test]
fn support_attached_recovery_outcome_does_not_carry_checkpoint_crash_lane() {
    let plan = lower_checkpoint_crash_replay_plan();
    let scheduled_trace = checkpoint_crash_replay_trace(&plan);
    let support_attached_trace = checkpoint_crash_replay_trace_without_crash_lane(&plan);
    let family = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape();
    let scheduled_verdict = family
        .oracle(NoMixedRootOracle)
        .judge(&plan, &scheduled_trace)
        .unwrap();
    let scheduled_recovery_verdict = ReusablePhysicalOracleFamily::recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(&plan, &scheduled_trace)
        .unwrap();
    let support_attached_verdict = family
        .oracle(NoMixedRootOracle)
        .judge(&plan, &support_attached_trace)
        .unwrap();
    let support_attached_recovery_verdict = ReusablePhysicalOracleFamily::recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(&plan, &support_attached_trace)
        .unwrap();
    let scheduled_replay = detached_replay_bundle_from_parts(
        ExecutedTranscriptParts::new(
            &plan,
            schedule(&plan),
            &production_fixture(),
            scheduled_trace.clone(),
            support::counter_receipt(&plan, scheduled_trace),
        )
        .unwrap()
        .with_oracle_verdict(scheduled_verdict)
        .with_oracle_verdict(scheduled_recovery_verdict),
    );
    let support_attached_replay = detached_replay_bundle_from_parts(
        ExecutedTranscriptParts::new(
            &plan,
            schedule(&plan),
            &production_fixture(),
            support_attached_trace.clone(),
            support::counter_receipt(&plan, support_attached_trace),
        )
        .unwrap()
        .with_oracle_verdict(support_attached_verdict)
        .with_oracle_verdict(support_attached_recovery_verdict),
    );

    assert!(scheduled_replay.trace().checkpoint_crash_replay().is_some());
    assert!(support_attached_replay
        .trace()
        .checkpoint_crash_replay()
        .is_none());
    assert_ne!(
        scheduled_replay.replay_basis_identity().digest_bytes(),
        support_attached_replay
            .replay_basis_identity()
            .digest_bytes()
    );
}

#[test]
fn missing_checkpoint_or_compaction_observation_cannot_satisfy_reader_oracle() {
    let plan = lower_checkpoint_plan();
    let trace = PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(
            &plan,
            &ExecutedPhysicalSimulationObservation::from_executed_plan(&plan).unwrap(),
        )
        .unwrap()
        .complete()
        .unwrap();

    let denial = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(NoMixedRootOracle)
        .judge(&plan, &trace)
        .unwrap_err();

    assert_eq!(denial, OracleDenial::MissingCompactionInterlockObservation);
}
