use worth_ui::facade::intent::{
    UiIntentAdmissionMetrics, UiIntentExecutionAdvanceOutcome, UiIntentExecutionAdvanceReport,
};

use crate::intent::admission::phase3::world::AdmissionWorld;
use crate::intent::execution::execution_reading;

#[test]
fn unchanged_turns_and_motion_storms_do_no_semantic_intent_work() {
    let (mut world, provider) = AdmissionWorld::launch_with_provider_observation(1);
    assert_zero_advance(advance(&mut world, 1));
    assert_eq!(
        world.session.intent_admission_metrics(),
        UiIntentAdmissionMetrics::default()
    );

    world.motion_storm(0, 16);
    assert_eq!(
        world.session.intent_admission_metrics(),
        UiIntentAdmissionMetrics::default()
    );
    assert_eq!(provider.begin_calls(), 0);
    assert_zero_advance(advance(&mut world, 2));
    assert_eq!(provider.begin_calls(), 0);
    let _ = world.session.shutdown();
}

fn advance(world: &mut AdmissionWorld, tick: u64) -> UiIntentExecutionAdvanceReport {
    match world
        .session
        .advance_intent_executions(execution_reading(tick))
    {
        UiIntentExecutionAdvanceOutcome::Advanced(report) => report,
        UiIntentExecutionAdvanceOutcome::Stopped(stop) => {
            panic!("monotonic unchanged turn stopped: {stop:?}")
        }
    }
}

fn assert_zero_advance(report: UiIntentExecutionAdvanceReport) {
    assert!(report.transitions().is_empty());
    assert_eq!(report.active_slots_visited(), 0);
    assert_eq!(report.provider_calls(), 0);
    assert_eq!(report.provider_polls(), 0);
    assert_eq!(report.cancellation_calls(), 0);
    assert_eq!(report.settlements(), 0);
}
