use worth_ui::facade::intent::UiIntentExecutionTransitionPosture;

use super::provider::{AttemptStep, ExecutionScript, ScriptedProvider};
use super::{advance, dispatch, execution_census};
use crate::intent::admission::phase3::world::AdmissionWorld;

#[test]
fn one_pending_provider_does_not_head_of_line_block_a_ready_peer() {
    let (provider, observation) = ScriptedProvider::new([
        ExecutionScript::running([
            AttemptStep::PendingBeforeEffect,
            AttemptStep::FailedBeforeEffect,
        ]),
        ExecutionScript::running([AttemptStep::Completed]),
    ]);
    let mut world = AdmissionWorld::launch_with_provider(2, provider);
    dispatch(&mut world, 0, 20);
    dispatch(&mut world, 1, 20);
    let starts = advance(&mut world, 1);
    assert_eq!(starts.transitions().len(), 2);

    let peer_turn = advance(&mut world, 2);
    assert_eq!(peer_turn.transitions().len(), 2);
    assert_eq!(
        peer_turn.transitions()[0].posture(),
        UiIntentExecutionTransitionPosture::PendingBeforeEffect
    );
    assert!(matches!(
        peer_turn.transitions()[1].posture(),
        UiIntentExecutionTransitionPosture::Completed { .. }
    ));
    assert_eq!(execution_census(&world), [1, 1, 0, 1]);

    let settled = advance(&mut world, 3);
    assert!(matches!(
        settled.transitions()[0].posture(),
        UiIntentExecutionTransitionPosture::FailedBeforeEffect { .. }
    ));
    assert_eq!(observation.counts()[1], 3);
    assert_eq!(execution_census(&world), [0, 0, 0, 1]);
    let _ = world.session.shutdown();
}
