use worth_store_certification::courtroom::operational_recovery::{
    S10ScenarioExecutionMatrix, S10ScenarioExecutionMatrixDenial, ScenarioScaleEvidence,
    ScenarioScaleProfile,
};
use worth_store_physical_certification::{
    CounterContractOracle, DetachedSimulationReplayParts, ExecutedTranscriptParts,
    FixtureCapabilityDeclaration, FixtureMutationBoundary, LargeStoreFixtureProfile,
    MaterializedFixtureScaleEvidence, OperationalRecoveryProductionDriver,
    OperationalRecoveryTraceJoinDenial, PhysicalCertificationEvidenceBundle,
    PhysicalFixtureBuilder, PhysicalInterleavingSchedule, PhysicalSimulationPlan,
    ReusablePhysicalOracleFamily, StateSpaceBudget,
};
use worth_store_test_support::harness::recovery::counter_evidence as counter_support;
use worth_store_test_support::{
    developer_smoke_replay_seed, production_backed_physical_fixture_materialization,
};

#[test]
fn joined_matrix_counts_distinct_real_replays_and_binds_fixture_scale() {
    let plan = counter_support::lower_physical_isolation_plan();
    let first_seed = developer_smoke_replay_seed();
    let second_seed = worth_store_physical_certification::ReplaySeed::from_u64(
        first_seed.value().saturating_add(1),
    );
    let driver = OperationalRecoveryProductionDriver::uninterrupted();
    let _ = driver.derive_audit(&[]);
    let matrix = S10ScenarioExecutionMatrix::join(
        [run(&plan, first_seed), run(&plan, second_seed)],
        [driver.trace()],
    )
    .unwrap();
    assert_eq!(matrix.schedules_executed(), 2);

    let manifest = matrix.primary().replay().fixture_manifest();
    let scale =
        ScenarioScaleEvidence::from_execution(ScenarioScaleProfile::Smoke, &matrix).unwrap();
    assert_eq!(scale.schedules_executed(), 2);
    assert_eq!(scale.store_bytes(), manifest.scale().declared_store_bytes());
}

#[test]
fn joined_matrix_rejects_a_fixture_claim_substituted_under_the_same_plan() {
    let plan = counter_support::lower_physical_isolation_plan();
    let seed = developer_smoke_replay_seed();
    let driver = OperationalRecoveryProductionDriver::uninterrupted();
    let _ = driver.derive_audit(&[]);
    assert_eq!(
        S10ScenarioExecutionMatrix::join(
            [
                run_with_root(&plan, seed, 10),
                run_with_root(&plan, seed, 11)
            ],
            [driver.trace()],
        )
        .unwrap_err(),
        S10ScenarioExecutionMatrixDenial::FixtureMismatch
    );
}

#[test]
fn joined_matrix_rejects_duplicate_physical_schedule_evidence_and_empty_driver_evidence() {
    let plan = counter_support::lower_physical_isolation_plan();
    let seed = developer_smoke_replay_seed();
    let driver = OperationalRecoveryProductionDriver::uninterrupted();
    let _ = driver.derive_audit(&[]);
    assert_eq!(
        S10ScenarioExecutionMatrix::join([run(&plan, seed), run(&plan, seed)], [driver.trace()])
            .unwrap_err(),
        S10ScenarioExecutionMatrixDenial::DuplicateScheduleTranscript
    );
    assert_eq!(
        S10ScenarioExecutionMatrix::join([run(&plan, seed)], []).unwrap_err(),
        S10ScenarioExecutionMatrixDenial::DriverTrace(OperationalRecoveryTraceJoinDenial::Empty)
    );
}

fn run(
    plan: &PhysicalSimulationPlan,
    seed: worth_store_physical_certification::ReplaySeed,
) -> PhysicalCertificationEvidenceBundle {
    run_with_root(plan, seed, 10)
}

fn run_with_root(
    plan: &PhysicalSimulationPlan,
    seed: worth_store_physical_certification::ReplaySeed,
    root_reference: u64,
) -> PhysicalCertificationEvidenceBundle {
    let scale_workspace = tempfile::tempdir().unwrap();
    let materialization = production_backed_physical_fixture_materialization(
        LargeStoreFixtureProfile::StoreLargerThanMemory,
        root_reference,
    )
    .unwrap();
    let materialized_scale = MaterializedFixtureScaleEvidence::materialize(
        scale_workspace.path(),
        materialization.scale(),
    )
    .unwrap();
    let fixture = PhysicalFixtureBuilder::production_backed("s10-execution-matrix")
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
    let trace = counter_support::observed_trace(plan);
    let counters = counter_support::counter_receipt(plan, trace.clone());
    let verdict = ReusablePhysicalOracleFamily::physical_isolation_readiness_shape()
        .oracle(CounterContractOracle)
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
