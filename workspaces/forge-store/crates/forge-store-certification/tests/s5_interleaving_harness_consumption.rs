#[path = "s4_closeout/fixture.rs"]
mod closeout_fixture;
mod s5_interleaving_harness_support;

use forge_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, CounterContractKind, HarnessCoverageStage,
    OracleFamilyKind, PhysicalScenarioActor, PhysicalScenarioExpectation, PhysicalScenarioFault,
    PhysicalScenarioIntent, PhysicalSimulationProfile, PhysicalSimulationScenarioFamily,
    SimulationPlanDenial, SupportedOracleFamilySet,
};
use forge_store_test_support::NativeStoreAspectFixture;
use s5_interleaving_harness_support::{
    complete_context, context_without_s5_lane_registration, replay_bundle,
};

#[test]
fn s5_scenario_families_lower_replay_cover_and_emit_evidence_through_s4_5_pipeline() {
    for lane in forge_store_certification::physical_isolation_lanes() {
        let plan = lower_physical_simulation_plan(lane.scenario().clone(), complete_context())
            .unwrap_or_else(|err| panic!("{} failed to lower: {err:?}", lane.name()));
        let replay = replay_bundle(&plan, lane.expected_fault());
        assert_ci_closeout_profile(&plan, &replay);
        let mutation = forge_store_certification::PhysicalIsolationMutationEvidence::from_replay(
            plan.scenario_family(),
            &replay,
        );
        let matrix =
            forge_store_certification::physical_isolation_coverage_matrix(
                lane.scenario(),
                &plan,
                &replay,
                &mutation,
            );
        assert_family_counter_topology(plan.scenario_family(), &plan);
        assert_family_mutation_topology(plan.scenario_family(), &mutation, &matrix);
        let evidence =
            forge_store_physical_certification::PhysicalCertificationEvidenceBundle::from_replay_bundle(
                replay,
            )
            .unwrap();
        let primary = evidence.primary();

        assert_eq!(
            plan.scenario_family(),
            lane.scenario().definition().family()
        );
        assert!(plan
            .oracle_families()
            .contains(OracleFamilyKind::PhysicalIsolationInterleaving));
        assert!(plan
            .oracle_families()
            .contains(OracleFamilyKind::TranscriptReplayEvidence));
        assert_eq!(
            primary.counter_row_count(),
            plan.counter_contracts().iter().count()
        );
        assert_eq!(
            primary.oracle_verdict_count(),
            plan.oracle_families().iter().count()
        );
        assert_eq!(matrix.sequence(), HarnessCoverageStage::SimulationAdmission);
        assert!(matrix.rows().iter().any(|row| {
            row.surface() == forge_store_physical_certification::CoverageSurfaceKind::Transcript
        }));
        assert!(matrix.rows().iter().any(|row| {
            row.surface() == forge_store_physical_certification::CoverageSurfaceKind::MutationResult
        }));
    }
}

#[test]
fn s5_family_lowering_is_readiness_gated_and_rejects_missing_required_s4_5_surfaces() {
    let lane = forge_store_certification::physical_isolation_lanes()
        .into_iter()
        .find(|lane| {
            lane.scenario().definition().family()
                == PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock
        })
        .unwrap();

    let no_registration = lower_physical_simulation_plan(
        lane.scenario().clone(),
        context_without_s5_lane_registration(),
    )
    .unwrap_err();
    assert_eq!(
        no_registration,
        SimulationPlanDenial::MissingPhysicalIsolationLaneRegistration
    );

    let no_s5_oracle = lower_physical_simulation_plan(
        lane.scenario().clone(),
        complete_context().with_supported_oracle_families(
            SupportedOracleFamilySet::all_for_ci_certification()
                .without(OracleFamilyKind::PhysicalIsolationInterleaving),
        ),
    )
    .unwrap_err();
    assert_eq!(
        no_s5_oracle,
        SimulationPlanDenial::MissingOracleFamily(OracleFamilyKind::PhysicalIsolationInterleaving)
    );
}

#[test]
fn s5_interleaving_oracle_is_bound_to_family_specific_observations() {
    for lane in forge_store_certification::physical_isolation_lanes() {
        let plan =
            lower_physical_simulation_plan(lane.scenario().clone(), complete_context()).unwrap();
        let replay = replay_bundle(&plan, lane.expected_fault());
        let verdict = replay
            .oracle_verdicts()
            .iter()
            .find(|verdict| verdict.family() == OracleFamilyKind::PhysicalIsolationInterleaving)
            .unwrap();

        assert_eq!(
            verdict.basis().scenario_family(),
            plan.scenario_family(),
            "{} did not bind oracle basis to scenario family",
            lane.name()
        );
    }
}

#[test]
fn readiness_shape_probe_remains_non_claiming_and_not_s5_closeout_authority() {
    let probe = physical_scenario("store.physical.s5.readiness-shape.non-claim")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("shape-probe", 4)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .actor(PhysicalScenarioActor::compaction_driver("compactor"))
        .fault(PhysicalScenarioFault::no_fault())
        .schedule(forge_store_test_support::physical_isolation_boundary_yieldpoint())
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap();
    let plan =
        lower_physical_simulation_plan(probe, context_without_s5_lane_registration()).unwrap();

    assert!(plan
        .oracle_families()
        .contains(OracleFamilyKind::PhysicalIsolationReadinessShape));
    assert!(!plan
        .oracle_families()
        .contains(OracleFamilyKind::PhysicalIsolationInterleaving));

    let mismatched = physical_scenario("store.physical.s5.compaction-interlock.non-claim")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock)
        .intent(PhysicalScenarioIntent::PhysicalIsolationCompactionEarlyReclaimMutant)
        .fixture(
            NativeStoreAspectFixture::segment_header("bad-shape", 5)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .actor(PhysicalScenarioActor::compaction_driver("compactor"))
        .fault(PhysicalScenarioFault::early_reclaim())
        .schedule(forge_store_test_support::physical_isolation_boundary_yieldpoint())
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap();
    assert!(matches!(
        lower_physical_simulation_plan(mismatched, complete_context()).unwrap_err(),
        SimulationPlanDenial::UnsupportedScenarioShape { .. }
    ));
}

fn assert_ci_closeout_profile(
    plan: &forge_store_physical_certification::PhysicalSimulationPlan,
    replay: &forge_store_physical_certification::SimulationReplayBundle,
) {
    assert_eq!(plan.profile(), PhysicalSimulationProfile::CiCertification);
    assert_eq!(
        replay.schedule().profile(),
        PhysicalSimulationProfile::CiCertification
    );
    assert_eq!(
        replay.schedule().exploration_cost().budget().max_steps(),
        forge_store_test_support::ci_certification_state_space_budget().max_steps()
    );
}

fn assert_family_counter_topology(
    family: PhysicalSimulationScenarioFamily,
    plan: &forge_store_physical_certification::PhysicalSimulationPlan,
) {
    match family {
        PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock => {
            assert_counter(plan, CounterContractKind::CompactionCandidateRanges);
            assert_counter(plan, CounterContractKind::CopiedPages);
            assert_counter(plan, CounterContractKind::BlockedReclaimAttempts);
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock
        | PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover => {
            assert_counter(plan, CounterContractKind::PublicationSwaps);
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability => {
            assert_counter(plan, CounterContractKind::BlockedReclaimAttempts);
            assert_counter(plan, CounterContractKind::CompactionCandidateRanges);
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability
        | PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability => {
            assert_counter(plan, CounterContractKind::FutureS5SpecificCounters);
            assert!(!plan
                .counter_contracts()
                .contains(CounterContractKind::PublicationSwaps));
        }
        _ => {}
    }
}

fn assert_family_mutation_topology(
    family: PhysicalSimulationScenarioFamily,
    mutation: &forge_store_certification::PhysicalIsolationMutationEvidence,
    matrix: &forge_store_physical_certification::GeneratedCoverageMatrix,
) {
    let mutation_result_row = matrix
        .rows()
        .iter()
        .find(|row| {
            row.surface() == forge_store_physical_certification::CoverageSurfaceKind::MutationResult
        })
        .unwrap();
    for required in forge_store_certification::physical_isolation_required_mutation_rows(family) {
        assert!(
            mutation.required_rows().contains(required),
            "missing S5 mutation row {required:?} for {family:?}"
        );
        assert!(
            row_for_matrix_contains_s5_mutation(mutation_result_row, *required),
            "missing coverage matrix S5 mutation row {required:?} for {family:?}"
        );
    }
    if family == PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability {
        assert!(mutation.physical().compaction_mutations().is_empty());
    }
}

fn assert_counter(
    plan: &forge_store_physical_certification::PhysicalSimulationPlan,
    kind: CounterContractKind,
) {
    assert!(plan.counter_contracts().contains(kind), "missing {kind:?}");
}

fn row_for_matrix_contains_s5_mutation(
    row: &forge_store_physical_certification::PhysicalCoverageMatrixRow,
    mutation: forge_store_physical_certification::PhysicalIsolationMutationKind,
) -> bool {
    row.has_dimension(
        &forge_store_physical_certification::CoverageRowDimension::PhysicalIsolationMutation(
            mutation,
        ),
    )
}
