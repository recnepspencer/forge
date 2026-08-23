use worth_store_test_support::harness::recovery::counter_evidence as counter_support;

use worth_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, CrashRecoversOldOrNewNeverMixedOracle,
    DetachedSimulationReplayParts, ExecutedTranscriptParts, FixtureCapabilityDeclaration,
    FixtureMutationBoundary, ForbiddenShortcutSet, LargeStoreFixtureProfile,
    NoPrivateMutationOracle, ObservedPhysicalTrace, PhysicalCertificationEvidenceBundle,
    PhysicalFixtureBuilder, PhysicalInterleavingSchedule, PhysicalProofOracleKind,
    PhysicalProofOracleVerdictKind, PhysicalScenarioActor, PhysicalScenarioExpectation,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet,
    PhysicalSimulationObserver, PhysicalSimulationPlan, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    ProductionBackedPhysicalFixture, RecoveryOutcomeObservation, ReusablePhysicalOracleFamily,
    ShortcutRejectionObservation, SimulationEvidencePolicy, SimulationPlanningContext,
    StateSpaceBudget, SupportedObserverSet, SupportedOracleFamilySet,
};
use worth_store_physical_certification::{OracleFamilyKind, SimulationReplayBundle};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, developer_smoke_replay_seed,
    production_backed_physical_fixture_materialization, NativeStoreAspectFixture,
};

#[test]
fn shortcut_plan_gets_generic_replay_evidence_without_physical_isolation_authority() {
    let plan = counter_support::lower_shortcut_plan();
    let trace = shortcut_trace(&plan);
    let shortcut_verdict = ReusablePhysicalOracleFamily::forbidden_shortcut_rejection()
        .oracle(NoPrivateMutationOracle)
        .judge(&plan, &trace)
        .unwrap();
    let counter_receipt = counter_support::counter_receipt(&plan, trace.clone());
    let parts = ExecutedTranscriptParts::new(
        &plan,
        schedule(&plan),
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap()
    .with_oracle_verdict(shortcut_verdict);

    let replay = detached_replay_bundle_from_parts(parts);
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();

    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::TranscriptReplayEvidence));
    assert!(!plan
        .oracle_families()
        .contains(OracleFamilyKind::PhysicalIsolationReadinessShape));
    assert_eq!(
        evidence.replay().oracle_verdicts()[0].family(),
        OracleFamilyKind::ForbiddenShortcutRejection
    );
    assert!(evidence.replay().oracle_verdicts().iter().any(|verdict| {
        verdict.family() == OracleFamilyKind::TranscriptReplayEvidence
            && verdict.oracle() == PhysicalProofOracleKind::TranscriptReplay
    }));
}

#[test]
fn recovery_plan_produces_replay_evidence_without_physical_isolation_authority() {
    let plan = lower_recovery_plan();
    let trace = recovery_trace(&plan);
    let recovery_verdict = ReusablePhysicalOracleFamily::recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(&plan, &trace)
        .unwrap();
    let counter_receipt = counter_support::counter_receipt(&plan, trace.clone());
    let parts = ExecutedTranscriptParts::new(
        &plan,
        schedule(&plan),
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap()
    .with_oracle_verdict(recovery_verdict);

    let replay = detached_replay_bundle_from_parts(parts);
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();

    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::TranscriptReplayEvidence));
    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::RecoveryDogfood));
    assert!(!plan
        .oracle_families()
        .contains(OracleFamilyKind::PhysicalIsolationReadinessShape));
    assert!(evidence.replay().oracle_verdicts().iter().any(|verdict| {
        verdict.family() == OracleFamilyKind::TranscriptReplayEvidence
            && verdict.oracle() == PhysicalProofOracleKind::TranscriptReplay
    }));
    assert!(evidence
        .replay()
        .oracle_verdicts()
        .iter()
        .any(|verdict| verdict.family() == OracleFamilyKind::RecoveryDogfood));
}

#[test]
fn failed_recovery_oracle_materializes_failure_digest() {
    let plan = lower_recovery_plan();
    let trace = recovery_trace_with_outcome(&plan, RecoveryOutcomeObservation::mixed_root());
    let recovery_verdict = ReusablePhysicalOracleFamily::recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(&plan, &trace)
        .unwrap();
    assert_eq!(
        recovery_verdict.kind(),
        PhysicalProofOracleVerdictKind::Failed
    );
    let counter_receipt = counter_support::counter_receipt(&plan, trace.clone());
    let parts = ExecutedTranscriptParts::new(
        &plan,
        schedule(&plan),
        &production_fixture(),
        trace,
        counter_receipt,
    )
    .unwrap()
    .with_oracle_verdict(recovery_verdict);

    let replay = detached_replay_bundle_from_parts(parts);
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    let failure_digest = evidence.failure_digest().unwrap();

    assert_eq!(
        failure_digest.transcript_digest(),
        evidence.replay().transcript_identity().digest_bytes()
    );
    assert_eq!(failure_digest.failed_oracle_count(), 1);
}

fn detached_replay_bundle_from_parts(parts: ExecutedTranscriptParts) -> SimulationReplayBundle {
    let transcript =
        worth_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            parts.with_transcript_replay_verdict().unwrap(),
        )
        .unwrap();
    let detached = DetachedSimulationReplayParts::from_transcript(&transcript);
    drop(transcript);
    detached.admit_replay_bundle().unwrap()
}

fn lower_recovery_plan() -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(recovery_scenario(), complete_context()).unwrap()
}

fn recovery_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase10.recovery.transcript")
        .family(PhysicalSimulationScenarioFamily::RecoveryDogfood)
        .intent(PhysicalScenarioIntent::RecoveryReplayDogfood)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase10-recovery-transcript", 10)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::recovery_driver("recovery"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "fresh-runtime-replay-open",
        ))
        .expectation(PhysicalScenarioExpectation::recovery_dogfood())
        .certify_definition()
        .unwrap()
}

fn complete_context() -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(
            PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
        )
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
}

fn schedule(plan: &PhysicalSimulationPlan) -> PhysicalInterleavingSchedule {
    PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        developer_smoke_replay_seed(),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap()
}

fn production_fixture() -> ProductionBackedPhysicalFixture {
    PhysicalFixtureBuilder::production_backed("phase10-transcript-oracle-authority")
        .materialize_with(
            production_backed_physical_fixture_materialization(
                LargeStoreFixtureProfile::StoreLargerThanMemory,
                10,
            )
            .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap()
}

fn recovery_trace(plan: &PhysicalSimulationPlan) -> ObservedPhysicalTrace {
    recovery_trace_with_outcome(plan, RecoveryOutcomeObservation::recovered_old_root())
}

fn recovery_trace_with_outcome(
    plan: &PhysicalSimulationPlan,
    outcome: RecoveryOutcomeObservation,
) -> ObservedPhysicalTrace {
    PhysicalSimulationObserver::recovery_outcome()
        .observe_plan(plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_recovery_outcome_observation(outcome)
        .complete()
        .unwrap()
}

fn shortcut_trace(plan: &PhysicalSimulationPlan) -> ObservedPhysicalTrace {
    PhysicalSimulationObserver::shortcut_rejection()
        .observe_plan(plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_shortcut_rejection_observation(ShortcutRejectionObservation::private_mutation_denied())
        .with_compaction_interlock_observation(
            worth_store_physical_certification::CompactionInterlockObservation::from_store_interlock_evidence(
                worth_store_test_support::harness::physical_isolation::compaction::
                    compaction_interlock_foundational_evidence_for_seed(17),
            )
            .expect("executed compaction publication provides interlock evidence"),
        )
        .complete()
        .unwrap()
}

fn developer_smoke_production_trace(
) -> worth_store_physical_certification::ProductionBoundaryDriverTrace {
    admitted_developer_smoke_driver_contracts()
        .unwrap()
        .iter()
        .find_map(|driver| driver.production_boundary_trace())
        .unwrap()
}
