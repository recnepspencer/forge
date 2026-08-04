#![allow(dead_code)]

use std::sync::OnceLock;

use crate::independent_verifier_observation;
use crate::physical_isolation_shortcut_report as shortcut_report;
use worth_store_test_support::harness::physical_isolation::interleaving_resources as resources;
use worth_store_test_support::harness::recovery::checkpoint_publication as checkpoint_support;
use worth_store_test_support::harness::recovery::closeout as closeout_fixture;
use worth_store_test_support::harness::recovery::coverage as coverage_support;

use worth_store_physical_certification::{
    lower_physical_simulation_plan, register_physical_isolation_certification_lane,
    CheckpointInterlockObservation, HarnessCoverageStage, IndependentVerifierObservation,
    PhysicalCertificationEvidenceBundle, PhysicalInterleavingSchedule,
    PhysicalIsolationCertificationLaneRegistration, PhysicalIsolationCorrectnessNonClaimEvidence,
    PhysicalIsolationHarnessReadinessReceipt, PhysicalSimulationPlan, SimulationPlanningContext,
};
use worth_store_physical_isolation::{
    admit_physical_isolation_entry, PhysicalIsolationEntryRequest,
};

pub(crate) fn complete_context() -> SimulationPlanningContext {
    static CONTEXT: OnceLock<SimulationPlanningContext> = OnceLock::new();
    CONTEXT
        .get_or_init(|| {
            worth_store_certification::physical_isolation_ci_certification_planning_context(
                physical_isolation_lane_registration(),
            )
        })
        .clone()
}

pub(crate) fn context_without_physical_isolation_lane_registration() -> SimulationPlanningContext {
    worth_store_certification::physical_isolation_ci_certification_context_without_lane_registration(
    )
}

pub(crate) fn developer_smoke_context() -> SimulationPlanningContext {
    worth_store_certification::physical_isolation_planning_context(
        physical_isolation_lane_registration(),
    )
}

pub(crate) fn replay_bundle(
    plan: &PhysicalSimulationPlan,
    expected_fault: worth_store_physical_certification::PhysicalScenarioFaultKind,
) -> worth_store_physical_certification::SimulationReplayBundle {
    let schedule = schedule(plan);
    let trace = worth_store_certification::observe_physical_isolation_trace(
        plan,
        &schedule,
        trace_fixtures(plan, &schedule),
    )
    .unwrap();
    replay_bundle_from_trace(plan, schedule, trace, expected_fault)
}

pub(crate) fn replay_bundle_from_trace(
    plan: &PhysicalSimulationPlan,
    schedule: PhysicalInterleavingSchedule,
    trace: worth_store_physical_certification::ObservedPhysicalTrace,
    expected_fault: worth_store_physical_certification::PhysicalScenarioFaultKind,
) -> worth_store_physical_certification::SimulationReplayBundle {
    worth_store_certification::assemble_physical_isolation_replay_bundle(
        plan,
        schedule,
        &resources::production_fixture(),
        trace,
        resources::store_residency_observation(plan),
        expected_fault,
    )
}

pub(crate) fn trace_fixtures(
    _plan: &PhysicalSimulationPlan,
    _schedule: &PhysicalInterleavingSchedule,
) -> worth_store_certification::PhysicalIsolationTraceFixtures {
    worth_store_certification::PhysicalIsolationTraceFixtures::complete(
        checkpoint_interlock_observation(),
        independent_verifier_observation(),
    )
}

pub(crate) fn schedule(plan: &PhysicalSimulationPlan) -> PhysicalInterleavingSchedule {
    worth_store_test_support::deterministic_ci_certification_schedule(plan).unwrap()
}

pub(crate) fn checkpoint_interlock_observation() -> CheckpointInterlockObservation {
    CheckpointInterlockObservation::from_store_interlock_evidence(
        checkpoint_support::checkpoint_evidence(),
    )
    .unwrap()
}

pub(crate) fn independent_verifier_observation() -> IndependentVerifierObservation {
    independent_verifier_observation::observed_runtime_comparison(
        independent_verifier_observation::RuntimeComparisonFixture::Equivalent,
    )
}

fn physical_isolation_lane_registration() -> PhysicalIsolationCertificationLaneRegistration {
    let completion = closeout_fixture::recovery_completion();
    let entry = admit_physical_isolation_entry(
        PhysicalIsolationEntryRequest::from_recovery_completion(&completion),
    )
    .unwrap();
    register_physical_isolation_certification_lane(&entry, simulation_harness_readiness_receipt())
}

pub(crate) fn simulation_harness_readiness_receipt() -> PhysicalIsolationHarnessReadinessReceipt {
    static RECEIPT: OnceLock<PhysicalIsolationHarnessReadinessReceipt> = OnceLock::new();
    RECEIPT.get_or_init(build_harness_readiness_receipt).clone()
}

fn build_harness_readiness_receipt() -> PhysicalIsolationHarnessReadinessReceipt {
    let plan = coverage_support::lowered_ci_plan();
    let replay = coverage_support::replay_bundle(&plan);
    let matrix = coverage_support::complete_registry(&plan, &replay)
        .generate_matrix()
        .unwrap();
    let evidence = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay).unwrap();
    PhysicalIsolationHarnessReadinessReceipt::from_store_harness_evidence(
        &matrix,
        &evidence,
        &shortcut_report::complete_shortcut_report(),
        PhysicalIsolationCorrectnessNonClaimEvidence::shape_probe_only(),
    )
    .unwrap()
}

pub(crate) fn lower_lane(
    lane: &worth_store_certification::PhysicalIsolationHarnessLane,
) -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(lane.scenario().clone(), complete_context()).unwrap()
}

pub(crate) const fn roadmap_sequence() -> HarnessCoverageStage {
    HarnessCoverageStage::SimulationAdmission
}
