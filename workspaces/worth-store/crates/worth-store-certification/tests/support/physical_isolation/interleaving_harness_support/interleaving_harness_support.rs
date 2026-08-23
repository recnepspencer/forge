#![allow(dead_code)]

use std::sync::OnceLock;

use crate::independent_verifier_observation;
use worth_store_physical_certification::{
    lower_physical_simulation_plan, CheckpointInterlockObservation, CompactionInterlockObservation,
    IndependentVerifierObservation, PhysicalSimulationPlan, SimulationPlanningContext,
};
use worth_store_physical_isolation::{
    read_during_checkpoint_verdict_for_certification_test, CheckpointInterlockFoundationalEvidence,
};
use worth_store_test_support::harness::physical_isolation::interleaving_resources as resources;

fn complete_context() -> SimulationPlanningContext {
    static CONTEXT: OnceLock<SimulationPlanningContext> = OnceLock::new();
    CONTEXT
        .get_or_init(
            worth_store_physical_certification::physical_isolation_ci_certification_planning_context,
        )
        .clone()
}

pub(crate) fn replay_bundle(
    plan: &PhysicalSimulationPlan,
    expected_fault: worth_store_physical_certification::PhysicalScenarioFaultKind,
) -> worth_store_physical_certification::SimulationReplayBundle {
    let schedule = worth_store_test_support::deterministic_ci_certification_schedule(plan).unwrap();
    let trace = worth_store_physical_certification::observe_physical_isolation_trace(
        plan,
        &schedule,
        worth_store_physical_certification::PhysicalIsolationTraceFixtures::complete(
            checkpoint_interlock_observation(),
            independent_verifier_observation(),
        )
        .with_compaction_interlock_observation(compaction_interlock_observation()),
    )
    .unwrap();
    worth_store_physical_certification::assemble_physical_isolation_replay_bundle(
        plan,
        schedule,
        &resources::production_fixture(),
        trace,
        resources::store_residency_observation(plan),
        expected_fault,
    )
}

fn checkpoint_interlock_observation() -> CheckpointInterlockObservation {
    CheckpointInterlockObservation::from_store_interlock_evidence(
        CheckpointInterlockFoundationalEvidence::after_executed_interlock(
            &read_during_checkpoint_verdict_for_certification_test(),
        ),
    )
    .unwrap()
}

fn compaction_interlock_observation() -> CompactionInterlockObservation {
    CompactionInterlockObservation::from_store_interlock_evidence(
        worth_store_test_support::harness::physical_isolation::compaction::
            compaction_interlock_foundational_evidence_for_seed(17),
    )
    .expect("executed compaction publication provides interlock evidence")
}

fn independent_verifier_observation() -> IndependentVerifierObservation {
    independent_verifier_observation::observed_runtime_comparison(
        independent_verifier_observation::RuntimeComparisonFixture::Equivalent,
    )
}

pub(crate) fn lower_lane(
    lane: &worth_store_physical_certification::PhysicalIsolationHarnessLane,
) -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(lane.scenario().clone(), complete_context()).unwrap()
}
