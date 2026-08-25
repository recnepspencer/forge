mod branches;
mod causal_trace;
mod independence;
mod interruption;
mod ordinary;
mod provider;
mod recovery;
mod replacement;
mod shutdown;

pub(in crate::intent) use provider::{
    AttemptStep, ExecutionScript, RecoveryStep, ScriptedProvider,
};

use worth_ui::facade::intent::{
    UiIntentExecutionAdvanceOutcome, UiIntentExecutionAdvanceReport,
    UiIntentExecutionDispatchOutcome, UiIntentExecutionTransition,
};
use worth_ui_runtime::certification_support::WorthUiIntentExecutionReservationCertificationExt;

use crate::intent::admission::phase3::world::AdmissionWorld;

pub(in crate::intent) fn dispatch(world: &mut AdmissionWorld, target: usize, deadline: u64) {
    let admitted = world.admit_exact(target);
    let UiIntentExecutionDispatchOutcome::AttemptPrepared(_) = world
        .session
        .dispatch_admitted_intent(admitted, super::execution_deadline(deadline))
    else {
        panic!("a current admitted intent must prepare exactly one attempt")
    };
}

pub(in crate::intent) fn advance(
    world: &mut AdmissionWorld,
    tick: u64,
) -> UiIntentExecutionAdvanceReport {
    match world
        .session
        .advance_intent_executions(super::execution_reading(tick))
    {
        UiIntentExecutionAdvanceOutcome::Advanced(report) => report,
        UiIntentExecutionAdvanceOutcome::Stopped(stop) => {
            panic!("monotonic lifecycle advance unexpectedly stopped: {stop:?}")
        }
    }
}

pub(in crate::intent) fn only_transition(
    report: UiIntentExecutionAdvanceReport,
) -> UiIntentExecutionTransition {
    let mut transitions = report.into_transitions().into_vec();
    assert_eq!(
        transitions.len(),
        1,
        "the step must emit one exact transition"
    );
    transitions.pop().expect("one transition was measured")
}

pub(in crate::intent) fn execution_census(world: &AdmissionWorld) -> [usize; 4] {
    let metrics = world
        .session
        .intent_execution_reservation_metrics_for_certification();
    [
        metrics.active_attempts(),
        metrics.active_occupancy(),
        metrics.recovering_attempts(),
        metrics.consequence_pending_attempts(),
    ]
}
