use super::{
    presentation_epoch, require_owner_reconstruction, settle_presentation_failure,
    UiNativePresentationFailure,
};
use crate::native::presentation::{
    reserve_presentation_owners, settle_port_result, UiNativePendingExternalObligation,
    UiNativePresentationPortFailure,
};
use crate::native::{UiNativeEffectPosture, UiNativeHostState};

struct PendingProbe(std::rc::Rc<std::cell::Cell<bool>>);

impl UiNativePendingExternalObligation for PendingProbe {
    fn poll_observation(
        &mut self,
        basis: crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis,
        _device: Option<&wgpu::Device>,
    ) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation {
        basis.observe(crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Pending)
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
    let owners = reserve_presentation_owners(
        &mut state.resources,
        &mut state.physical_signal,
        crate::native::physical_work_signal::UiNativePhysicalPresentationBasis::test(),
    )
    .unwrap_or_else(|_| panic!("empty registry must reserve presentation owners"));
    let pending = settle_port_result(
        &mut state.resources,
        &mut state.physical_signal,
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

#[test]
fn derived_state_loss_rejects_without_effects_until_owner_reconstruction_arrives() {
    let mut state = UiNativeHostState::new();
    let binding = 71;
    state.reconstruction_required.insert(binding);

    let first = require_owner_reconstruction(&mut state, binding);
    let second = require_owner_reconstruction(&mut state, binding);

    for outcome in [first, second] {
        assert!(matches!(
            outcome,
            worth_ui_host_contract::UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfacePresentationDenial::ReconstructionRequired
            )
        ));
    }
    assert!(state.reconstruction_required.contains(&binding));
    assert!(state.pending_presentations.is_empty());
    assert_eq!(state.effect_posture, UiNativeEffectPosture::BeforeEffects);
    assert!(state.resources.current().is_zero());
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P3-RECONSTRUCTION-01\":\"stale-derived-state\"}}"
    );
}

#[test]
fn unchanged_reuses_the_last_physical_presentation_epoch() {
    let mut state = UiNativeHostState::new();
    let binding = 73;
    let physical = presentation_epoch(&mut state, binding, 101, true).unwrap();
    let unchanged = presentation_epoch(&mut state, binding, 102, false).unwrap();
    assert_eq!(unchanged, physical);
    assert_eq!(unchanged.diagnostic_value(), 101);
    assert!(presentation_epoch(&mut state, binding + 1, 103, false).is_none());
    println!("WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P3-UNCHANGED-01\":\"fresh-unchanged-epoch\"}}");
}
