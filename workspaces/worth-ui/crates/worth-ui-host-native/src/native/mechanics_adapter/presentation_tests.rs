use super::{settle_presentation_failure, UiNativePresentationFailure};
use crate::native::presentation::{
    reserve_presentation_owners, settle_port_result, UiNativePendingExternalObligation,
    UiNativePresentationPortFailure,
};
use crate::native::{UiNativeEffectPosture, UiNativeHostState};

struct PendingProbe(std::rc::Rc<std::cell::Cell<bool>>);

impl UiNativePendingExternalObligation for PendingProbe {
    fn try_settle(&mut self, _device: Option<&wgpu::Device>) -> bool {
        false
    }
}

impl Drop for PendingProbe {
    fn drop(&mut self) {
        self.0.set(true);
    }
}

#[test]
fn scripted_before_effect_failure_keeps_before_effect_posture() {
    let mut state = UiNativeHostState::new();
    let outcome = settle_presentation_failure(
        &mut state,
        UiNativePresentationFailure::BeforeEffects(
            worth_ui_host_contract::UiHostSurfacePresentationDenial::AdapterDeclined,
        ),
    );
    assert!(matches!(
        outcome,
        worth_ui_host_contract::UiHostSurfacePresentationOutcome::RejectedBeforeEffects(_)
    ));
    assert_eq!(state.effect_posture, UiNativeEffectPosture::BeforeEffects);
}

#[test]
fn external_port_orchestration_and_effect_postures_are_exact() {
    crate::native::presentation::prove_nonuniform_readback_port();
    let mut state = UiNativeHostState::new();
    let external_dropped = std::rc::Rc::new(std::cell::Cell::new(false));
    let owners = reserve_presentation_owners(&mut state.resources)
        .unwrap_or_else(|_| panic!("empty registry must reserve presentation owners"));
    let pending = settle_port_result(
        &mut state.resources,
        owners,
        Err(UiNativePresentationPortFailure::ReadbackUnsettled(
            Box::new(PendingProbe(std::rc::Rc::clone(&external_dropped))),
        )),
    );
    let Err(UiNativePresentationFailure::Indeterminate(pending)) = pending else {
        panic!("unsettled port work must remain indeterminate");
    };
    let outcome = settle_presentation_failure(
        &mut state,
        UiNativePresentationFailure::Indeterminate(pending),
    );
    assert!(matches!(
        outcome,
        worth_ui_host_contract::UiHostSurfacePresentationOutcome::PresentationIndeterminate
    ));
    assert_eq!(
        state.effect_posture,
        UiNativeEffectPosture::PresentationIndeterminate
    );
    assert_eq!(state.pending_presentations.len(), 1);
    assert!(!external_dropped.get());
    assert_eq!(state.resources.current().readback_buffers, 1);
    assert_eq!(state.resources.current().pending_submissions, 1);
    state.pending_presentations.clear();
    assert!(external_dropped.get());
}
