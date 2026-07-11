#[path = "../../../support/recovery/closeout/fixture.rs"]
mod closeout_fixture;
#[path = "../../../support/physical_isolation/interleaving_harness_support/interleaving_harness_support.rs"]
mod s5_interleaving_harness_support;

use forge_store_physical_certification::{
    lower_physical_simulation_plan, CoverageGapDenial, ExecutedPhysicalSimulationObservation,
    OracleDenial, OracleFamilyKind, PhysicalIsolationInterleavingOracle,
    PhysicalSimulationObserver, PhysicalSimulationScenarioFamily, ReusablePhysicalOracleFamily,
    ShortcutRejectionObservation,
};
use s5_interleaving_harness_support::{
    compaction_interlock_observation, compaction_mutations, complete_context,
    independent_verifier_observation, replay_bundle_from_trace, schedule, trace_fixtures,
};

#[test]
fn physical_isolation_interleaving_oracle_rejects_wrong_family_observation_topology() {
    for lane in forge_store_certification::physical_isolation_lanes() {
        let plan =
            lower_physical_simulation_plan(lane.scenario().clone(), complete_context()).unwrap();
        let schedule = schedule(&plan);
        let fixtures = match plan.scenario_family() {
            PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock
            | PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability => {
                trace_fixtures(&plan, &schedule).without_compaction_interlock()
            }
            PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock
            | PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover => {
                trace_fixtures(&plan, &schedule).without_checkpoint_interlock()
            }
            PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability
            | PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability => {
                trace_fixtures(&plan, &schedule).without_independent_verifier()
            }
            other => panic!("unexpected S5 interleaving family {other:?}"),
        };
        let trace = forge_store_certification::observe_physical_isolation_trace(
            &plan, &schedule, fixtures,
        )
        .unwrap();

        assert_eq!(
            oracle_denial(&plan, &trace),
            expected_oracle_denial(plan.scenario_family()),
            "{} accepted the wrong observation topology",
            lane.name()
        );
    }
}

#[test]
fn physical_isolation_observer_requires_scheduled_mutation_rows_for_mutation_bound_families() {
    for lane in forge_store_certification::physical_isolation_lanes() {
        let plan =
            lower_physical_simulation_plan(lane.scenario().clone(), complete_context()).unwrap();
        let schedule = schedule(&plan);
        let result = forge_store_certification::observe_physical_isolation_trace(
            &plan,
            &schedule,
            trace_fixtures(&plan, &schedule).without_compaction_mutations(),
        );
        if plan.scenario_family()
            == PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability
        {
            let trace = result.unwrap();
            let replay = replay_bundle_from_trace(&plan, schedule, trace, lane.expected_fault());
            let mutation =
                forge_store_certification::PhysicalIsolationMutationEvidence::try_from_replay(
                    plan.scenario_family(),
                    &replay,
                )
                .unwrap();
            assert_eq!(
                mutation.required_rows(),
                forge_store_certification::physical_isolation_required_mutation_rows(
                    PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability,
                )
            );
        } else {
            assert_eq!(
                result.unwrap_err(),
                CoverageGapDenial::MissingMutationResult
            );
        }
    }
}

#[test]
fn physical_isolation_mutation_evidence_rejects_missing_or_wrong_family_rows() {
    for lane in forge_store_certification::physical_isolation_lanes() {
        let plan =
            lower_physical_simulation_plan(lane.scenario().clone(), complete_context()).unwrap();
        let schedule = schedule(&plan);
        let trace = if plan.scenario_family()
            == PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability
        {
            future_chunk_trace_polluted_with_compaction_mutations(&plan)
        } else {
            trace_without_compaction_mutation_rows(&plan)
        };
        let replay = replay_bundle_from_trace(&plan, schedule, trace, lane.expected_fault());

        assert_eq!(
            forge_store_certification::PhysicalIsolationMutationEvidence::try_from_replay(
                plan.scenario_family(),
                &replay,
            )
            .unwrap_err(),
            CoverageGapDenial::MissingMutationResult,
            "{} accepted missing or wrong S5 mutation evidence",
            lane.name()
        );
    }
}

fn oracle_denial(
    plan: &forge_store_physical_certification::PhysicalSimulationPlan,
    trace: &forge_store_physical_certification::ObservedPhysicalTrace,
) -> OracleDenial {
    ReusablePhysicalOracleFamily::physical_isolation_interleaving()
        .oracle(PhysicalIsolationInterleavingOracle)
        .judge(plan, trace)
        .unwrap_err()
}

const fn expected_oracle_denial(family: PhysicalSimulationScenarioFamily) -> OracleDenial {
    match family {
        PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock
        | PhysicalSimulationScenarioFamily::PhysicalIsolationReclaimReachability => {
            OracleDenial::MissingCompactionInterlockObservation
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock
        | PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover => {
            OracleDenial::MissingCheckpointInterlockObservation
        }
        PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability
        | PhysicalSimulationScenarioFamily::PhysicalIsolationFutureChunkStability => {
            OracleDenial::MissingIndependentVerifierObservation
        }
        _ => OracleDenial::OracleFamilyNotRequired {
            family: OracleFamilyKind::PhysicalIsolationInterleaving,
        },
    }
}

fn trace_without_compaction_mutation_rows(
    plan: &forge_store_physical_certification::PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    let builder = base_raw_trace_builder(plan);
    match plan.scenario_family() {
        PhysicalSimulationScenarioFamily::PhysicalIsolationCheckpointPublicationInterlock
        | PhysicalSimulationScenarioFamily::PhysicalIsolationRestartDuringCutover => builder
            .with_checkpoint_interlock_observation(
                s5_interleaving_harness_support::checkpoint_interlock_observation(),
            )
            .complete()
            .unwrap(),
        PhysicalSimulationScenarioFamily::PhysicalIsolationTierMovementStability => builder
            .with_independent_verifier_observation(independent_verifier_observation())
            .complete()
            .unwrap(),
        _ => builder.complete().unwrap(),
    }
}

fn future_chunk_trace_polluted_with_compaction_mutations(
    plan: &forge_store_physical_certification::PhysicalSimulationPlan,
) -> forge_store_physical_certification::ObservedPhysicalTrace {
    let compaction_lane = forge_store_certification::physical_isolation_lanes()
        .into_iter()
        .find(|lane| {
            lane.scenario().definition().family()
                == PhysicalSimulationScenarioFamily::PhysicalIsolationCompactionInterlock
        })
        .unwrap();
    let compaction_plan =
        lower_physical_simulation_plan(compaction_lane.scenario().clone(), complete_context())
            .unwrap();
    let compaction_schedule = schedule(&compaction_plan);
    base_raw_trace_builder(plan)
        .with_scheduled_compaction_mutation_lanes(
            compaction_mutations(&compaction_plan, &compaction_schedule).unwrap(),
        )
        .with_independent_verifier_observation(independent_verifier_observation())
        .complete()
        .unwrap()
}

fn base_raw_trace_builder(
    plan: &forge_store_physical_certification::PhysicalSimulationPlan,
) -> forge_store_physical_certification::PhysicalObservationBuilder<'_> {
    let execution = ExecutedPhysicalSimulationObservation::from_executed_plan(plan).unwrap();
    PhysicalSimulationObserver::independent_physical_trace()
        .observe_executed_plan(plan, &execution)
        .unwrap()
        .with_compaction_interlock_observation(compaction_interlock_observation())
        .with_shortcut_rejection_observation(ShortcutRejectionObservation::private_mutation_denied())
}
