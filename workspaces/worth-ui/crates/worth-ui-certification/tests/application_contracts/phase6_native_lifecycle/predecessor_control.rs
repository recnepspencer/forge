use super::mutation_receipt::{emit, MutationReceiptCase, MutationTrace};
use super::protocol_world::{
    UiNativeLifecycleEffect, UiNativeLifecycleEvent, UiNativeLifecycleState, UiNativeLifecycleWorld,
};

#[test]
fn predecessor_handoff_mutation_is_rejected() {
    let mut baseline = UiNativeLifecycleWorld::new(UiNativeLifecycleState::Presented);
    assert_eq!(
        baseline
            .apply(UiNativeLifecycleEvent::BeginSuccessor)
            .effect,
        UiNativeLifecycleEffect::NoOp
    );
    let baseline = baseline.apply(UiNativeLifecycleEvent::CompletePresentation);

    let mut mutant = UiNativeLifecycleWorld::new(UiNativeLifecycleState::Presented);
    let mutant = mutant.complete_unissued_successor();
    assert_eq!(
        baseline.effect,
        UiNativeLifecycleEffect::PresentationCompleted
    );
    assert_ne!(baseline, mutant);
    emit(MutationReceiptCase {
        requirement: "P6-PREDECESSOR-01",
        case: "stale-phase-five-source",
        baseline: MutationTrace {
            posture: "current successor handoff completes",
            state: &format!("{:?}", baseline.state),
        },
        mutant: MutationTrace {
            posture: "stale handoff cannot complete a successor",
            state: &format!("{:?}", mutant.state),
        },
        denial: "PresentationCompleted requires the pending successor binding",
        first_divergence: "CompletePresentation without successor admission",
    });
}
