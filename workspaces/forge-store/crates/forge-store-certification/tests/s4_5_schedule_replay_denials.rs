use forge_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, ForbiddenShortcutSet,
    PhysicalInterleavingSchedule, PhysicalScenarioActor, PhysicalScenarioExpectation,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    ReplaySeed, ScheduleOrderingAuthorityAttempt, ScheduleReplayDenial, SimulationEvidencePolicy,
    SimulationPlanningContext, StateSpaceBudget, SupportedObserverSet, SupportedOracleFamilySet,
};
use forge_store_test_support::{
    admitted_developer_smoke_driver_contracts, developer_smoke_state_space_budget,
    NativeStoreAspectFixture,
};

#[test]
fn missing_seed_denies_before_schedule_construction() {
    let plan = lower_physical_simulation_plan(s5_scenario(), complete_context()).unwrap();
    let denial = PhysicalInterleavingSchedule::from_optional_seed(
        &plan,
        None,
        developer_smoke_state_space_budget(),
    )
    .unwrap_err();

    assert_eq!(denial, ScheduleReplayDenial::MissingSeed);
}

#[test]
fn unbounded_or_empty_budget_denies_before_schedule_construction() {
    assert_eq!(
        StateSpaceBudget::unbounded_exploration().unwrap_err(),
        ScheduleReplayDenial::UnboundedExplorationDenied
    );
    assert_eq!(
        StateSpaceBudget::bounded_steps(0).unwrap_err(),
        ScheduleReplayDenial::EmptyStateSpaceBudget
    );
}

#[test]
fn budget_too_small_for_plan_actor_steps_denies() {
    let plan = lower_physical_simulation_plan(s5_scenario(), complete_context()).unwrap();
    let denial = PhysicalInterleavingSchedule::from_lowered_plan(
        &plan,
        ReplaySeed::from_u64(7),
        StateSpaceBudget::bounded_steps(1).unwrap(),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        ScheduleReplayDenial::StateSpaceBudgetExceeded {
            required_steps: 2,
            max_steps: 1,
        }
    );
}

#[test]
fn ambient_ordering_sources_deny_as_schedule_authority() {
    assert_eq!(
        ScheduleOrderingAuthorityAttempt::wall_clock()
            .admit()
            .unwrap_err(),
        ScheduleReplayDenial::WallClockScheduleDenied
    );
    assert_eq!(
        ScheduleOrderingAuthorityAttempt::unordered_map_iteration()
            .admit()
            .unwrap_err(),
        ScheduleReplayDenial::UnorderedMapScheduleDenied
    );
    assert_eq!(
        ScheduleOrderingAuthorityAttempt::ambient_thread_order()
            .admit()
            .unwrap_err(),
        ScheduleReplayDenial::AmbientThreadScheduleDenied
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

fn s5_scenario() -> forge_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario("store.physical.s5.schedule.denial")
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("schedule-denial", 5)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap()
}
