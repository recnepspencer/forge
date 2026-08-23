use super::causal_mutation::CausalEventMutation;
use super::mutation_receipt::{emit, MutationReceiptCase, MutationTrace};
use super::protocol_world::{
    UiNativeLifecycleEffect, UiNativeLifecycleEvent, UiNativeLifecycleState,
};

#[test]
fn profile_order_mutation_is_rejected() {
    let result = CausalEventMutation {
        initial: UiNativeLifecycleState::Presented,
        schedule: &[
            UiNativeLifecycleEvent::BeginProfileTransition,
            UiNativeLifecycleEvent::Pointer,
        ],
        action_index: 0,
        replacement: UiNativeLifecycleEvent::Pointer,
    }
    .run();
    let baseline = result.baseline_final();
    let mutant = result.mutant_final();
    assert_eq!(
        baseline.effect,
        UiNativeLifecycleEffect::Denied(
            worth_ui_host_native::UiNativeInputObservationStop::StalePresentationAffinity
        )
    );
    assert_eq!(mutant.effect, UiNativeLifecycleEffect::Retained);
    assert_ne!(baseline, mutant);
    emit(MutationReceiptCase {
        requirement: "P6-PROFILE-ORDER-01",
        case: "synthetic-event-time",
        baseline: MutationTrace {
            posture: "profile transition invalidates stale input before admission",
            state: &format!("{:?}", baseline.state),
        },
        mutant: MutationTrace {
            posture: "input is admitted against the previous profile",
            state: &format!("{:?}", mutant.state),
        },
        denial: "StalePresentationAffinity",
        first_divergence: "profile transition precedes pointer admission",
    });
}
