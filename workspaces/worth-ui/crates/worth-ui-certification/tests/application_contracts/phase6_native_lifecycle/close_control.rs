use super::mutation_receipt::{emit, MutationReceiptCase, MutationTrace};
use super::protocol_world::{
    UiNativeLifecycleEffect, UiNativeLifecycleEvent, UiNativeLifecycleState, UiNativeLifecycleWorld,
};

#[test]
fn close_requirement_mutation_is_rejected() {
    let mut baseline = UiNativeLifecycleWorld::new(UiNativeLifecycleState::Presented);
    let baseline = baseline.apply(UiNativeLifecycleEvent::Close);
    assert_eq!(baseline.effect, UiNativeLifecycleEffect::CloseDeferred);

    let mut mutant = UiNativeLifecycleWorld::new(UiNativeLifecycleState::BeforeFirstPresentation);
    let mutant = mutant.apply(UiNativeLifecycleEvent::Close);
    assert_eq!(mutant.effect, UiNativeLifecycleEffect::Closed);
    assert_ne!(baseline, mutant);
    emit(MutationReceiptCase {
        requirement: "P6-CLOSE-01",
        case: "open-requirement",
        baseline: MutationTrace {
            posture: "close drains retained observations before terminal closure",
            state: &format!("{:?}", baseline.state),
        },
        mutant: MutationTrace {
            posture: "close skips the retained-observation drain",
            state: &format!("{:?}", mutant.state),
        },
        denial: "CloseDeferred",
        first_divergence: "presented close versus close before first presentation",
    });
}
