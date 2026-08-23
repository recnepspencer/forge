use super::causal_mutation::CausalEventMutation;
use super::mutation_receipt::{emit, MutationReceiptCase, MutationTrace};
use super::protocol_world::{
    UiNativeLifecycleEffect, UiNativeLifecycleEvent, UiNativeLifecycleState,
};

#[test]
fn pointer_time_mutation_is_rejected() {
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
    assert_eq!(
        mutant.effect,
        UiNativeLifecycleEffect::Denied(
            worth_ui_host_native::UiNativeInputObservationStop::PointerPositionUnavailable
        )
    );
    assert_ne!(baseline, mutant);
    emit(MutationReceiptCase {
        requirement: "P6-POINTER-TIME-01",
        case: "post-delivery-cursor-proxy",
        baseline: MutationTrace {
            posture: "button carries its event-time pointer witness",
            state: &format!("{:?}", baseline.state),
        },
        mutant: MutationTrace {
            posture: "post-delivery cursor proxy has no admissible witness",
            state: &format!("{:?}", mutant.state),
        },
        denial: "PointerPositionUnavailable",
        first_divergence: "button witness replaced by unavailable cursor source",
    });
}
