use super::*;
use worth_store_physical_certification::{ScheduleOrderingAuthorityKind, SchedulePerturbationSeed};
use worth_store_test_support::deterministic_developer_smoke_schedule;

#[test]
fn same_plan_seed_profile_actors_and_budget_reproduce_schedule_identity() {
    let plan = lower_physical_simulation_plan(
        physical_isolation_scenario(
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
    assert_eq!(
        first.seed(),
        SchedulePerturbationSeed::from_u64(0x005e_ed45)
    );
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
    assert_exploration_cost(&first, 32, 2, 0);
}

#[test]
fn replay_identity_changes_when_seed_budget_scenario_profile_or_actors_change() {
    let schedule = |scenario, fixture, profile, first, second, seed, budget| {
        schedule_for(scenario, fixture, profile, first, second, seed, budget)
    };
    let baseline = schedule(
        "store.physical.s5.identity.baseline",
        "identity-baseline",
        PhysicalSimulationProfile::DeveloperSmoke,
        PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
        PhysicalScenarioActor::foreground_reader("reader"),
        SchedulePerturbationSeed::from_u64(0x005e_ed45),
        developer_smoke_state_space_budget(),
    );
    let variants = [
        schedule(
            "store.physical.s5.identity.baseline",
            "identity-baseline",
            PhysicalSimulationProfile::DeveloperSmoke,
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
            PhysicalScenarioActor::foreground_reader("reader"),
            SchedulePerturbationSeed::from_u64(0x005e_ed46),
            developer_smoke_state_space_budget(),
        ),
        schedule(
            "store.physical.s5.identity.baseline",
            "identity-baseline",
            PhysicalSimulationProfile::DeveloperSmoke,
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
            PhysicalScenarioActor::foreground_reader("reader"),
            SchedulePerturbationSeed::from_u64(0x005e_ed45),
            StateSpaceBudget::bounded_steps(64).unwrap(),
        ),
        schedule(
            "store.physical.s5.identity.other-scenario",
            "identity-other-scenario",
            PhysicalSimulationProfile::DeveloperSmoke,
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
            PhysicalScenarioActor::foreground_reader("reader"),
            SchedulePerturbationSeed::from_u64(0x005e_ed45),
            developer_smoke_state_space_budget(),
        ),
        schedule(
            "store.physical.s5.identity.baseline",
            "identity-baseline",
            PhysicalSimulationProfile::CiCertification,
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
            PhysicalScenarioActor::foreground_reader("reader"),
            SchedulePerturbationSeed::from_u64(0x005e_ed45),
            developer_smoke_state_space_budget(),
        ),
        schedule(
            "store.physical.s5.identity.baseline",
            "identity-baseline",
            PhysicalSimulationProfile::DeveloperSmoke,
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
            PhysicalScenarioActor::foreground_reader("reader-2"),
            SchedulePerturbationSeed::from_u64(0x005e_ed45),
            developer_smoke_state_space_budget(),
        ),
        schedule(
            "store.physical.s5.identity.baseline",
            "identity-baseline",
            PhysicalSimulationProfile::DeveloperSmoke,
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
            PhysicalScenarioActor::foreground_writer("reader"),
            SchedulePerturbationSeed::from_u64(0x005e_ed45),
            developer_smoke_state_space_budget(),
        ),
    ];
    for variant in &variants {
        assert_digest_differs(&baseline, variant);
    }
    assert_exploration_cost(&variants[1], 64, 2, 0);
}
