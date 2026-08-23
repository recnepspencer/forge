use super::mutation_receipt::{emit, MutationReceiptCase, MutationTrace};
use super::protocol_world::{
    UiNativeLifecycleEffect, UiNativeLifecycleEvent, UiNativeLifecycleState, UiNativeLifecycleWorld,
};

#[test]
fn ime_phase_mutation_is_rejected() {
    let mut baseline = UiNativeLifecycleWorld::new(UiNativeLifecycleState::Presented);
    let preedit = baseline.apply(UiNativeLifecycleEvent::Preedit);
    let commit = baseline.apply(UiNativeLifecycleEvent::ImeCommit);
    let cancel = baseline.apply(UiNativeLifecycleEvent::ImeCancel);
    assert_eq!(preedit.effect, UiNativeLifecycleEffect::Retained);
    assert_eq!(commit.effect, UiNativeLifecycleEffect::Retained);
    assert_eq!(cancel.effect, UiNativeLifecycleEffect::Retained);
    let baseline_state = format!(
        "{:?}:{:?}:{:?}",
        preedit.effect, commit.effect, cancel.effect
    );

    let mut mutant = UiNativeLifecycleWorld::new(UiNativeLifecycleState::BeforeFirstPresentation);
    let mutant = mutant.apply(UiNativeLifecycleEvent::Preedit);
    assert_eq!(
        mutant.effect,
        UiNativeLifecycleEffect::Denied(
            worth_ui_host_native::UiNativeInputObservationStop::NoPresentationBasis
        )
    );
    assert_ne!(preedit, mutant);
    emit(MutationReceiptCase {
        requirement: "P6-IME-01",
        case: "preedit-as-text-input",
        baseline: MutationTrace {
            posture: "preedit, commit, and cancel remain distinct retained phases",
            state: &baseline_state,
        },
        mutant: MutationTrace {
            posture: "preedit is admitted without a presentation basis",
            state: &format!("{:?}", mutant.state),
        },
        denial: "NoPresentationBasis",
        first_divergence: "preedit admission before first presentation",
    });
}
