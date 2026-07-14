#![allow(dead_code)]

use worth_store_test_support::harness::physical_isolation::interleaving_resources as resources;
use worth_store_test_support::harness::recovery::checkpoint_publication as checkpoint_support;
use worth_store_test_support::harness::recovery::closeout as closeout_fixture;
use worth_store_test_support::harness::recovery::compaction_mutation as compaction_mutation_support;
use worth_store_test_support::harness::recovery::compaction_observation as compaction_interlock_trace;
use worth_store_test_support::harness::recovery::coverage as coverage_support;
#[path = "../../../scenarios/physical_isolation/readiness/shortcut_report.rs"]
mod shortcut_report;

use worth_store_physical_certification::{
    lower_physical_simulation_plan, register_physical_isolation_certification_lane,
    CheckpointInterlockObservation, HarnessCoverageStage, IndependentVerifierObservation,
    OfflineVerifierBoundarySeam, PhysicalCertificationEvidenceBundle, PhysicalInterleavingSchedule,
    PhysicalIsolationCertificationLaneRegistration,
    PhysicalIsolationCompactionMutationObservationSet,
    PhysicalIsolationCompactionMutationReplayBinding, PhysicalIsolationCorrectnessNonClaimEvidence,
    PhysicalIsolationHarnessReadinessReceipt, PhysicalSimulationPlan, SimulationPlanningContext,
};
use worth_store_physical_isolation::{
    admit_physical_isolation_entry, PhysicalIsolationEntryRequest,
};

pub(crate) fn complete_context() -> SimulationPlanningContext {
    worth_store_certification::physical_isolation_ci_certification_planning_context(
        physical_isolation_lane_registration(),
        compaction_mutation_support::compaction_mutation_origin(),
    )
}

pub(crate) fn context_without_physical_isolation_lane_registration() -> SimulationPlanningContext {
    worth_store_certification::physical_isolation_ci_certification_context_without_lane_registration(
        compaction_mutation_support::compaction_mutation_origin(),
    )
}

pub(crate) fn developer_smoke_context() -> SimulationPlanningContext {
    worth_store_certification::physical_isolation_planning_context(
        physical_isolation_lane_registration(),
        compaction_mutation_support::compaction_mutation_origin(),
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
        expected_fault,
    )
}

pub(crate) fn trace_fixtures(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> worth_store_certification::PhysicalIsolationTraceFixtures {
    worth_store_certification::PhysicalIsolationTraceFixtures::complete(
        compaction_interlock_observation(),
        compaction_mutations(plan, schedule).ok(),
        checkpoint_interlock_observation(),
        independent_verifier_observation(),
    )
}

pub(crate) fn schedule(plan: &PhysicalSimulationPlan) -> PhysicalInterleavingSchedule {
    worth_store_test_support::deterministic_ci_certification_schedule(plan).unwrap()
}

pub(crate) fn compaction_mutations(
    plan: &PhysicalSimulationPlan,
    schedule: &PhysicalInterleavingSchedule,
) -> Result<
    PhysicalIsolationCompactionMutationObservationSet,
    worth_store_physical_certification::CoverageGapDenial,
> {
    let binding =
        PhysicalIsolationCompactionMutationReplayBinding::from_plan_and_schedule(plan, schedule)?;
    let lanes =
        compaction_mutation_support::complete_scheduled_compaction_mutation_lanes(plan, schedule)?;
    PhysicalIsolationCompactionMutationObservationSet::from_scheduled_lanes(binding, lanes)
}

pub(crate) fn compaction_interlock_observation(
) -> worth_store_physical_certification::CompactionInterlockObservation {
    compaction_interlock_trace::store_compaction_observation()
}

pub(crate) fn checkpoint_interlock_observation() -> CheckpointInterlockObservation {
    CheckpointInterlockObservation::from_store_interlock_evidence(
        checkpoint_support::checkpoint_evidence(),
    )
    .unwrap()
}

pub(crate) fn independent_verifier_observation() -> IndependentVerifierObservation {
    IndependentVerifierObservation::agreement(
        OfflineVerifierBoundarySeam::RuntimeVerifierComparison,
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
