use super::causal_mutation::CausalEventMutation;
use super::mutation_receipt::{emit, MutationReceiptCase, MutationTrace};
use super::protocol_world::{
    UiNativeLifecycleEffect, UiNativeLifecycleEvent, UiNativeLifecycleState,
};
use worth_ui_host_native::UiNativeInputObservationStop;

#[test]
fn settlement_mutation_is_rejected() {
    let result = CausalEventMutation {
        initial: UiNativeLifecycleState::Presented,
        schedule: &[UiNativeLifecycleEvent::ButtonUnavailable],
        action_index: 0,
        replacement: UiNativeLifecycleEvent::Button,
    }
    .run();
    let baseline = result.baseline_at_divergence();
    assert_eq!(
        baseline.effect,
        UiNativeLifecycleEffect::Denied(UiNativeInputObservationStop::PointerPositionUnavailable)
    );

    let mutant = result.mutant_at_divergence();
    assert_eq!(mutant.effect, UiNativeLifecycleEffect::Retained);
    assert_ne!(baseline, mutant);
    emit(MutationReceiptCase {
        requirement: "P6-SETTLEMENT-01",
        case: "generic-error-for-typed-settlement",
        baseline: MutationTrace {
            posture: "typed denial remains a typed settlement outcome",
            state: &format!("{:?}", baseline.state),
        },
        mutant: MutationTrace {
            posture: "generic settlement erases the pointer witness denial",
            state: &format!("{:?}", mutant.state),
        },
        denial: "PointerPositionUnavailable",
        first_divergence: "Denied outcome versus admitted button outcome",
    });
}
