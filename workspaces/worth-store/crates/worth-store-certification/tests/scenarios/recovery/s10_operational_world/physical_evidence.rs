use worth_foundational::{BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator};
use worth_store_certification::courtroom::operational_recovery::S10ScenarioExecutionMatrix;
use worth_store_certification::courtroom::operational_recovery::{
    S10OperationalScenarioKind, ScenarioScaleProfile,
};
use worth_store_physical_backend::ProductionStorageBoundarySeam;
use worth_store_physical_certification::{
    CrashRecoversOldOrNewNeverMixedOracle, DetachedSimulationReplayParts, ExecutedTranscriptParts,
    ExpectedFaultLocalization, FixtureCapabilityDeclaration, FixtureMutationBoundary,
    LargeStoreFixtureProfile, MaterializedFixtureScaleEvidence, OperationalRecoveryDriverTrace,
    PhysicalArtifactFaultLocus, PhysicalCertificationEvidenceBundle, PhysicalFaultEvent,
    PhysicalFixtureBuilder, PhysicalInterleavingSchedule, PhysicalSimulationPlan,
    PhysicalSimulationProfile, RecoveryOutcomeObservation, ReplaySeed,
    ReusablePhysicalOracleFamily, StateSpaceBudget,
};
use worth_store_test_support::harness::recovery::{
    checkpoint_publication as recovery_support, counter_evidence as counter_support,
};
use worth_store_test_support::{
    developer_smoke_replay_seed, production_backed_physical_fixture_materialization,
};

pub fn execution_matrix(
    profile: ScenarioScaleProfile,
    kind: S10OperationalScenarioKind,
    trace: OperationalRecoveryDriverTrace,
    crash_coverage: impl IntoIterator<
        Item = worth_store_physical_certification::OperationalRecoveryCrashCutEvidence,
    >,
) -> S10ScenarioExecutionMatrix {
    let (physical_profile, fixture_profile, schedules) = match profile {
        ScenarioScaleProfile::Smoke => (
            PhysicalSimulationProfile::DeveloperSmoke,
            LargeStoreFixtureProfile::StoreLargerThanMemory,
            2,
        ),
        ScenarioScaleProfile::Ci => (
            PhysicalSimulationProfile::CiCertification,
            LargeStoreFixtureProfile::StoreLargerThanMemory,
            2,
        ),
        ScenarioScaleProfile::Release => (
            PhysicalSimulationProfile::ReleaseCertification,
            LargeStoreFixtureProfile::OperationalRecoveryRelease,
            3,
        ),
    };
    let plan = recovery_support::lower_recovery_plan_for_profile(physical_profile);
    let scale_workspace = tempfile::tempdir().expect("materialized scale workspace");
    let materialization =
        production_backed_physical_fixture_materialization(fixture_profile, 10).unwrap();
    let materialized_scale = MaterializedFixtureScaleEvidence::materialize(
        scale_workspace.path(),
        materialization.scale(),
    )
    .expect("OS-materialized fixture scale");
    let first = developer_smoke_replay_seed();
    let runs = (0..schedules)
        .map(|offset| {
            run(
                &plan,
                ReplaySeed::from_u64(first.value().saturating_add(offset)),
                fixture_profile,
                materialized_scale,
            )
        })
        .collect::<Vec<_>>();
    let mutant = run_controlled_mixed_root_defect(
        &plan,
        ReplaySeed::from_u64(first.value().saturating_add(10_000)),
        fixture_profile,
        materialized_scale,
    );
    S10ScenarioExecutionMatrix::join_for_scenario_with_crash_coverage(
        kind,
        runs,
        [trace],
        [mutant],
        crash_coverage,
    )
    .unwrap()
}

fn run(
    plan: &PhysicalSimulationPlan,
    seed: ReplaySeed,
    fixture_profile: LargeStoreFixtureProfile,
    materialized_scale: MaterializedFixtureScaleEvidence,
) -> PhysicalCertificationEvidenceBundle {
    let materialization =
        production_backed_physical_fixture_materialization(fixture_profile, 10).unwrap();
    let fixture = PhysicalFixtureBuilder::production_backed("s10-operational-world")
        .materialize_with(
            materialization
                .bind_materialized_scale(materialized_scale)
                .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap();
    let trace = recovery_support::recovery_trace(plan);
    let counters = counter_support::counter_receipt(plan, trace.clone());
    let verdict = ReusablePhysicalOracleFamily::recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(plan, &trace)
        .unwrap();
    let schedule = PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        seed,
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap();
    let parts = ExecutedTranscriptParts::new(plan, schedule, &fixture, trace, counters)
        .unwrap()
        .with_oracle_verdict(verdict)
        .with_transcript_replay_verdict()
        .unwrap();
    let transcript =
        worth_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            parts,
        )
        .unwrap();
    let replay = DetachedSimulationReplayParts::from_transcript(&transcript)
        .admit_replay_bundle()
        .unwrap();
    PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap()
}

fn run_controlled_mixed_root_defect(
    plan: &PhysicalSimulationPlan,
    seed: ReplaySeed,
    fixture_profile: LargeStoreFixtureProfile,
    materialized_scale: MaterializedFixtureScaleEvidence,
) -> PhysicalCertificationEvidenceBundle {
    let materialization =
        production_backed_physical_fixture_materialization(fixture_profile, 10).unwrap();
    let fixture = PhysicalFixtureBuilder::production_backed("s10-operational-world")
        .materialize_with(
            materialization
                .bind_materialized_scale(materialized_scale)
                .unwrap(),
        )
        .capability(FixtureCapabilityDeclaration::for_mutation_boundary(
            FixtureMutationBoundary::Manifest,
        ))
        .and_reopen_through_physical_authority()
        .unwrap();
    let trace = recovery_support::recovery_trace_with_outcome(
        plan,
        RecoveryOutcomeObservation::mixed_root(),
    );
    let counters = counter_support::counter_receipt(plan, trace.clone());
    let verdict = ReusablePhysicalOracleFamily::recovery_dogfood()
        .oracle(CrashRecoversOldOrNewNeverMixedOracle)
        .judge(plan, &trace)
        .unwrap();
    let schedule = PhysicalInterleavingSchedule::from_lowered_plan(
        plan,
        seed,
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap();
    let fault = PhysicalFaultEvent::byte_corruption(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        PhysicalArtifactFaultLocus::root_pointer(
            BoundaryArtifactLocator::new(
                BoundaryArtifactId::new(seed.value()),
                BoundaryArtifactField::Basis,
            ),
            ExpectedFaultLocalization::ProductionDriverBoundary,
        ),
    )
    .unwrap();
    let parts = ExecutedTranscriptParts::new(plan, schedule, &fixture, trace, counters)
        .unwrap()
        .with_faults([fault])
        .with_oracle_verdict(verdict)
        .with_transcript_replay_verdict()
        .unwrap();
    let transcript =
        worth_store_physical_certification::PhysicalSimulationTranscript::from_executed_parts(
            parts,
        )
        .unwrap();
    let replay = DetachedSimulationReplayParts::from_transcript(&transcript)
        .admit_replay_bundle()
        .unwrap();
    PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap()
}
