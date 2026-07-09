use worth_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, ForbiddenShortcutSet, ForegroundReadActor,
    PhysicalScenarioActorRole, PhysicalScenarioExpectation, PhysicalScenarioIntent,
    PhysicalScenarioSchedule, PhysicalSimulationActor, PhysicalSimulationActorAdmissionDenial,
    PhysicalSimulationCapabilitySet, PhysicalSimulationProfile, PhysicalSimulationProfileSet,
    PhysicalSimulationScenarioFamily, ReclaimActor, SimulationEvidencePolicy,
    SimulationPlanningContext, SupportedObserverSet, SupportedOracleFamilySet,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture,
};

#[test]
fn admitted_actor_contracts_flow_through_scenario_lowering() {
    let reclaimer = ReclaimActor::admit("reclaimer")
        .unwrap()
        .actor()
        .scenario_actor();
    let reader = ForegroundReadActor::admit("reader")
        .unwrap()
        .actor()
        .scenario_actor();

    let scenario = physical_scenario("store.physical.s5.actor.contract.lowering")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("actor", 4)
                .boundary_fact()
                .clone(),
        )
        .actor(reclaimer)
        .actor(reader)
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap();

    let plan = lower_physical_simulation_plan(scenario, complete_context()).unwrap();

    assert!(plan.actors().contains_actor_id("reclaimer"));
    assert!(plan.actors().contains_actor_id("reader"));
    assert!(plan
        .actors()
        .contains_role(PhysicalScenarioActorRole::MaintenanceReclaimer));
    assert!(plan
        .actors()
        .contains_role(PhysicalScenarioActorRole::ForegroundReader));
}

#[test]
fn future_actor_contract_still_denies_before_scenario_authoring() {
    assert_eq!(
        PhysicalSimulationActor::future_extension_slot("future").unwrap_err(),
        PhysicalSimulationActorAdmissionDenial::FutureExtensionActorCannotExecute
    );
}

fn complete_context() -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(PhysicalSimulationProfile::DeveloperSmoke)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(PhysicalSimulationCapabilitySet::s5_readiness_shape_probe())
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
}
