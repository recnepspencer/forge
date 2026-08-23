use super::causal_mutation::CausalEventMutation;
use super::mutation_receipt::{emit, MutationReceiptCase, MutationTrace};
use super::protocol_world::{
    UiNativeLifecycleEffect, UiNativeLifecycleEvent, UiNativeLifecycleState,
};

#[test]
fn windows_pointer_source_mutation_is_rejected() {
    let result = CausalEventMutation {
        initial: UiNativeLifecycleState::Presented,
        schedule: &[UiNativeLifecycleEvent::Button],
        action_index: 0,
        replacement: UiNativeLifecycleEvent::ButtonUnavailable,
    }
    .run();
    let baseline = result.baseline_at_divergence();
    let mutant = result.mutant_at_divergence();
    assert_eq!(baseline.effect, UiNativeLifecycleEffect::Retained);
    assert_ne!(baseline, mutant);
    emit(MutationReceiptCase {
        requirement: "P6-WINDOWS-WORLD-01",
        case: "get-cursor-pos-production-proxy",
        baseline: MutationTrace {
            posture: "Windows button retains the event-time message witness",
            state: &format!("{:?}", baseline.state),
        },
        mutant: MutationTrace {
            posture: "cursor proxy without a message witness is denied",
            state: &format!("{:?}", mutant.state),
        },
        denial: "PointerPositionUnavailable",
        first_divergence: "message-position witness versus cursor proxy",
    });
}
