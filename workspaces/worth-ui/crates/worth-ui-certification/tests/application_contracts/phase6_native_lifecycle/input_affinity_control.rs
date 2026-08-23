use super::causal_mutation::CausalEventMutation;
use super::mutation_receipt::{emit, MutationReceiptCase, MutationTrace};
use super::protocol_world::{
    UiNativeLifecycleEffect, UiNativeLifecycleEvent, UiNativeLifecycleState,
};

#[test]
fn input_affinity_mutation_is_rejected() {
    let result = CausalEventMutation {
        initial: UiNativeLifecycleState::Presented,
        schedule: &[
            UiNativeLifecycleEvent::BeginSuccessor,
            UiNativeLifecycleEvent::Pointer,
        ],
        action_index: 0,
        replacement: UiNativeLifecycleEvent::BeginProfileTransition,
    }
    .run();
    let baseline = result.baseline_final();
    let mutant = result.mutant_final();
    assert_eq!(baseline.effect, UiNativeLifecycleEffect::Retained);
    assert_eq!(
        mutant.effect,
        UiNativeLifecycleEffect::Denied(
            worth_ui_host_native::UiNativeInputObservationStop::StalePresentationAffinity
        )
    );
    assert_ne!(baseline, mutant);
    emit(MutationReceiptCase {
        requirement: "P6-INPUT-AFFINITY-01",
        case: "current-coordinate-retargeting",
        baseline: MutationTrace {
            posture: "input remains affine to the last completed presentation",
            state: &format!("{:?}", baseline.state),
        },
        mutant: MutationTrace {
            posture: "retargeted input is denied during profile transition",
            state: &format!("{:?}", mutant.state),
        },
        denial: "StalePresentationAffinity",
        first_divergence: "Pointer after successor admission versus profile-transition input",
    });
}
