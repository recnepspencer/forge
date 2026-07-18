use crate::NativeStoreAspectFixture;
use worth_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, ObservedPhysicalTrace,
    PhysicalScenarioActor, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationObserver, PhysicalSimulationPlan,
    PhysicalSimulationProfile, PhysicalSimulationScenarioFamily, RecoveryOutcomeObservation,
};

use super::{
    compaction_interlock_trace, complete_context_for_profile, developer_smoke_production_trace,
};

pub fn lower_recovery_plan() -> PhysicalSimulationPlan {
    lower_recovery_plan_for_profile(PhysicalSimulationProfile::DeveloperSmoke)
}

pub fn lower_recovery_plan_for_profile(
    profile: PhysicalSimulationProfile,
) -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(recovery_scenario(), complete_context_for_profile(profile))
        .unwrap()
}

pub fn recovery_trace(plan: &PhysicalSimulationPlan) -> ObservedPhysicalTrace {
    recovery_trace_with_outcome(plan, RecoveryOutcomeObservation::recovered_new_root())
}

pub fn recovery_trace_with_outcome(
    plan: &PhysicalSimulationPlan,
    outcome: RecoveryOutcomeObservation,
) -> ObservedPhysicalTrace {
    PhysicalSimulationObserver::recovery_outcome()
        .observe_plan(plan)
        .unwrap()
        .with_runtime_trace(developer_smoke_production_trace())
        .with_recovery_outcome_observation(outcome)
        .with_compaction_interlock_observation(
            compaction_interlock_trace::store_compaction_observation(),
        )
        .complete()
        .unwrap()
}

fn recovery_scenario() -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s45.phase9.checkpoint-recovery-replay")
        .family(PhysicalSimulationScenarioFamily::RecoveryDogfood)
        .intent(PhysicalScenarioIntent::RecoveryReplayDogfood)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase9-checkpoint-recovery", 9)
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
