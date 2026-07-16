use super::*;
use worth_store_physical_certification::{
    explore_physical_interleavings, ScheduleExplorationCompletion,
};

#[test]
fn exploration_enumerates_real_actor_orders_and_reports_bound_exhaustion() {
    let plan = lower_physical_simulation_plan(
        physical_isolation_scenario_with_three_actors(),
        complete_context(PhysicalSimulationProfile::DeveloperSmoke),
    )
    .unwrap();
    let complete = explore_physical_interleavings(
        &plan,
        ReplaySeed::from_u64(11),
        StateSpaceBudget::bounded_steps(18).unwrap(),
    )
    .unwrap();
    assert_eq!(
        complete.completion(),
        ScheduleExplorationCompletion::Complete
    );
    assert_eq!(complete.total_schedules(), 6);
    assert_eq!(complete.schedules().len(), 6);
    assert_eq!(complete.explored_transitions(), 18);
    let orders = complete
        .schedules()
        .iter()
        .map(|schedule| {
            schedule
                .actor_steps()
                .iter()
                .map(PhysicalActorStep::actor_id)
                .collect::<Vec<_>>()
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(orders.len(), 6);

    let bounded = explore_physical_interleavings(
        &plan,
        ReplaySeed::from_u64(11),
        StateSpaceBudget::bounded_steps(9).unwrap(),
    )
    .unwrap();
    assert_eq!(
        bounded.completion(),
        ScheduleExplorationCompletion::BoundExhausted
    );
    assert_eq!(bounded.schedules().len(), 3);
    assert_eq!(bounded.explored_transitions(), 9);
}
