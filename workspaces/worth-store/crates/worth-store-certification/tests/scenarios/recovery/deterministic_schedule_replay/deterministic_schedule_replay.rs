mod exploration;
mod identity;
mod owner_execution;

use worth_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, ForbiddenShortcutSet,
    PartialOrderReductionPosture, PhysicalActorStep, PhysicalInterleavingSchedule,
    PhysicalScenarioActor, PhysicalScenarioActorRole, PhysicalScenarioExpectation,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    ReplaySeed, SimulationEvidencePolicy, SimulationPlanningContext, StateSpaceBudget,
    SupportedObserverSet, SupportedOracleFamilySet,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, developer_smoke_state_space_budget,
    NativeStoreAspectFixture,
};

const ROOT_PUBLICATION_YIELDPOINT: &str = "root-publication-before-observe";

fn schedule_for(
    scenario_name: &'static str,
    fixture_label: &'static str,
    profile: PhysicalSimulationProfile,
    first_actor: PhysicalScenarioActor,
    second_actor: PhysicalScenarioActor,
    seed: ReplaySeed,
    budget: StateSpaceBudget,
) -> PhysicalInterleavingSchedule {
    let plan = lower_physical_simulation_plan(
        physical_isolation_scenario(scenario_name, fixture_label, first_actor, second_actor),
        complete_context(profile),
    )
    .unwrap();
    PhysicalInterleavingSchedule::from_lowered_plan(&plan, seed, budget).unwrap()
}

fn assert_digest_differs(
    baseline: &PhysicalInterleavingSchedule,
    variant: &PhysicalInterleavingSchedule,
) {
    assert_ne!(
        baseline.identity().digest_bytes(),
        variant.identity().digest_bytes()
    );
}

fn assert_actor_step(
    step: &PhysicalActorStep,
    expected_index: u32,
    expected_actor_id: &str,
    expected_role: PhysicalScenarioActorRole,
) {
    assert_eq!(step.step_index(), expected_index);
    assert_eq!(step.actor_id(), expected_actor_id);
    assert_eq!(step.actor_role(), expected_role);
    assert_eq!(step.yieldpoint(), ROOT_PUBLICATION_YIELDPOINT);
}

fn assert_exploration_cost(
    schedule: &PhysicalInterleavingSchedule,
    max_steps: u32,
    explored_steps: u32,
    pruned_steps: u32,
) {
    assert_eq!(schedule.exploration_cost().budget().max_steps(), max_steps);
    assert_eq!(schedule.exploration_cost().explored_steps(), explored_steps);
    assert_eq!(schedule.exploration_cost().pruned_steps(), pruned_steps);
    assert_eq!(
        schedule.exploration_cost().partial_order_reduction(),
        PartialOrderReductionPosture::NotApplied
    );
}

fn complete_context(profile: PhysicalSimulationProfile) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(profile)
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

fn physical_isolation_scenario(
    scenario_name: &'static str,
    fixture_label: &'static str,
    first_actor: PhysicalScenarioActor,
    second_actor: PhysicalScenarioActor,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario(scenario_name)
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header(fixture_label, 5)
                .boundary_fact()
                .clone(),
        )
        .actor(first_actor)
        .actor(second_actor)
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            ROOT_PUBLICATION_YIELDPOINT,
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap()
}

fn physical_isolation_scenario_with_three_actors(
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.interleaving.exploration")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("exploration", 9)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_writer("writer"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            ROOT_PUBLICATION_YIELDPOINT,
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap()
}
