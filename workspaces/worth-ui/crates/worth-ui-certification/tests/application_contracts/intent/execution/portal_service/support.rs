use worth_ui::facade::intent::{
    UiIntentExecutionAdvanceOutcome, UiIntentExecutionTransition,
    UiIntentExecutionTransitionPosture,
};

use crate::intent::admission::phase3::world::AdmissionWorld;

pub(super) fn only_transition(world: &mut AdmissionWorld) -> UiIntentExecutionTransition {
    let report = match world
        .session
        .advance_intent_executions(super::super::execution_reading(1))
    {
        UiIntentExecutionAdvanceOutcome::Advanced(report) => report,
        UiIntentExecutionAdvanceOutcome::Stopped(stop) => {
            panic!("portal runtime-service advance stopped: {stop:?}")
        }
    };
    let mut transitions = report.into_transitions().into_vec();
    assert_eq!(transitions.len(), 1);
    let transition = transitions.pop().unwrap();
    assert!(matches!(
        transition.posture(),
        UiIntentExecutionTransitionPosture::Completed { .. }
    ));
    transition
}
