use super::*;

fn interaction_denial(targets: &[u64]) -> String {
    let mut framework = framework_from_artifact(empty_artifact());
    let mut admission = framework.interaction_admission();
    let admitted = [3_u64, 5, 7].map(|target| {
        (
            target,
            admission
                .admit(
                    UiGraphNodeIdentity::new(target),
                    WorthUiTransientInteractionState::DragCapture,
                )
                .expect("interaction source should admit"),
        )
    });
    drop(admission);
    let posture = run_framework_turn(&mut framework, |turn| {
        for target in targets {
            let source_fact = admitted
                .iter()
                .find_map(|(identity, admitted)| (*identity == *target).then_some(*admitted))
                .expect("test source set is fixed");
            assert!(turn
                .submit_interaction(source_fact)
                .submission()
                .is_some_and(|submission| submission.is_queued()));
        }
    });
    assert_eq!(posture, TestFrameworkTurnPosture::Denied);
    format!("{:?}", framework.allocation_frame_dispatcher_counters())
}

#[test]
fn admitted_gateway_inputs_replay_to_the_same_ordered_frame() {
    assert_eq!(
        interaction_denial(&[7, 3, 5]),
        interaction_denial(&[5, 7, 3])
    );
}
