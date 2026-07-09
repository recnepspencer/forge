use worth_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario, ForbiddenShortcutSet,
    PartialOrderReductionPosture, PhysicalActorStep, PhysicalInterleavingSchedule,
    PhysicalScenarioActor, PhysicalScenarioActorRole, PhysicalScenarioExpectation,
    PhysicalScenarioIntent, PhysicalScenarioSchedule, PhysicalSimulationCapabilitySet,
    PhysicalSimulationProfile, PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily,
    ReplaySeed, ScheduleOrderingAuthorityKind, SimulationEvidencePolicy, SimulationPlanningContext,
    StateSpaceBudget, SupportedObserverSet, SupportedOracleFamilySet,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, deterministic_developer_smoke_schedule,
    developer_smoke_state_space_budget, NativeStoreAspectFixture,
};

const ROOT_PUBLICATION_YIELDPOINT: &str = "root-publication-before-observe";

#[test]
fn same_plan_seed_profile_actors_and_budget_reproduce_schedule_identity() {
    let plan = lower_physical_simulation_plan(
        s5_scenario(
            "store.physical.s5.deterministic.schedule",
            "schedule",
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
            PhysicalScenarioActor::foreground_reader("reader"),
        ),
        complete_context(PhysicalSimulationProfile::DeveloperSmoke),
    )
    .unwrap();
    let first = deterministic_developer_smoke_schedule(&plan).unwrap();
    let second = deterministic_developer_smoke_schedule(&plan).unwrap();

    assert_eq!(first.actor_steps(), second.actor_steps());
    assert_eq!(
        first.actor_step_sequence().canonical_steps(),
        first.actor_steps()
    );
    assert_eq!(
        first.identity().digest_bytes(),
        second.identity().digest_bytes()
    );
    assert_eq!(first.seed(), ReplaySeed::from_u64(0x5eed_45));
    assert_eq!(first.profile(), PhysicalSimulationProfile::DeveloperSmoke);
    assert_eq!(
        first.ordering_authority().kind(),
        ScheduleOrderingAuthorityKind::DeterministicActorSteps
    );
    assert_actor_step(
        &first.actor_steps()[0],
        0,
        "reader",
        PhysicalScenarioActorRole::ForegroundReader,
    );
    assert_actor_step(
        &first.actor_steps()[1],
        1,
        "reclaimer",
        PhysicalScenarioActorRole::MaintenanceReclaimer,
    );
    assert_eq!(first.actor_step_sequence().unique_actor_ids().len(), 2);
    assert_exploration_cost(&first, 32, 2, 30);
}

#[test]
fn replay_identity_changes_when_seed_budget_scenario_profile_or_actors_change() {
    let baseline = schedule_for(
        "store.physical.s5.identity.baseline",
        "identity-baseline",
        PhysicalSimulationProfile::DeveloperSmoke,
        PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        PhysicalScenarioActor::foreground_reader("reader"),
        ReplaySeed::from_u64(0x5eed_45),
        developer_smoke_state_space_budget(),
    );
    let different_seed = schedule_for(
        "store.physical.s5.identity.baseline",
        "identity-baseline",
        PhysicalSimulationProfile::DeveloperSmoke,
        PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        PhysicalScenarioActor::foreground_reader("reader"),
        ReplaySeed::from_u64(0x5eed_46),
        developer_smoke_state_space_budget(),
    );
    let different_budget = schedule_for(
        "store.physical.s5.identity.baseline",
        "identity-baseline",
        PhysicalSimulationProfile::DeveloperSmoke,
        PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        PhysicalScenarioActor::foreground_reader("reader"),
        ReplaySeed::from_u64(0x5eed_45),
        StateSpaceBudget::bounded_steps(64).unwrap(),
    );
    let different_scenario = schedule_for(
        "store.physical.s5.identity.other-scenario",
        "identity-other-scenario",
        PhysicalSimulationProfile::DeveloperSmoke,
        PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        PhysicalScenarioActor::foreground_reader("reader"),
        ReplaySeed::from_u64(0x5eed_45),
        developer_smoke_state_space_budget(),
    );
    let different_profile = schedule_for(
        "store.physical.s5.identity.baseline",
        "identity-baseline",
        PhysicalSimulationProfile::CiCertification,
        PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        PhysicalScenarioActor::foreground_reader("reader"),
        ReplaySeed::from_u64(0x5eed_45),
        developer_smoke_state_space_budget(),
    );
    let different_actor_id = schedule_for(
        "store.physical.s5.identity.baseline",
        "identity-baseline",
        PhysicalSimulationProfile::DeveloperSmoke,
        PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        PhysicalScenarioActor::foreground_reader("reader-2"),
        ReplaySeed::from_u64(0x5eed_45),
        developer_smoke_state_space_budget(),
    );
    let different_actor_role = schedule_for(
        "store.physical.s5.identity.baseline",
        "identity-baseline",
        PhysicalSimulationProfile::DeveloperSmoke,
        PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        PhysicalScenarioActor::foreground_writer("reader"),
        ReplaySeed::from_u64(0x5eed_45),
        developer_smoke_state_space_budget(),
    );

    assert_digest_differs(&baseline, &different_seed);
    assert_digest_differs(&baseline, &different_budget);
    assert_digest_differs(&baseline, &different_scenario);
    assert_digest_differs(&baseline, &different_profile);
    assert_digest_differs(&baseline, &different_actor_id);
    assert_digest_differs(&baseline, &different_actor_role);
    assert_exploration_cost(&different_budget, 64, 2, 62);
}

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
        s5_scenario(scenario_name, fixture_label, first_actor, second_actor),
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
        .with_capabilities(PhysicalSimulationCapabilitySet::s5_readiness_shape_probe())
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::roadmap2_baseline())
}

fn s5_scenario(
    scenario_name: &'static str,
    fixture_label: &'static str,
    first_actor: PhysicalScenarioActor,
    second_actor: PhysicalScenarioActor,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario(scenario_name)
        .family(PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe)
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
        .expectation(PhysicalScenarioExpectation::non_claiming_s5_readiness_shape())
        .certify_definition()
        .unwrap()
}
