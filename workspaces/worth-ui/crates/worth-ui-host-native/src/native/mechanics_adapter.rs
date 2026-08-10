use std::cell::RefCell;
use std::rc::Rc;

use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiHostMeasurementObservationValue, UiHostMeasurementRequest,
    UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt, UiHostSurfacePresentationMode,
    UiHostSurfacePresentationOutcome, UiHostSurfaceRegistrationDenial,
    UiHostSurfaceRegistrationRequest, UiMountedCompletedEffects, UiMountedEffectFamily,
    UiMountedFrameConsumptionView, UiMountedSurfacePresentationCompletion,
    UiViewportExtentObservation, WorthUiHostCapability, WorthUiHostCapabilityReport,
    WorthUiHostContract, WorthUiHostMechanicsAdapter, WorthUiMeasurementHostAdapter,
};

use super::{
    presentation::{present_initial, UiNativePresentationFailure, UiWgpuNativePresentationPort},
    UiNativeHostState,
};

pub struct WorthUiNativeMechanicsAdapter {
    state: Rc<RefCell<UiNativeHostState>>,
    profile: crate::UiNativePlatformProfileIdentity,
}

impl WorthUiNativeMechanicsAdapter {
    pub(crate) fn from_preparation(
        state: Rc<RefCell<UiNativeHostState>>,
        profile: crate::UiNativePlatformProfileIdentity,
    ) -> Self {
        Self { state, profile }
    }
}

impl WorthUiMeasurementHostAdapter for WorthUiNativeMechanicsAdapter {
    fn observe_measurement(
        &self,
        request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        let state = self.state.borrow();
        let graphics = state
            .graphics
            .as_ref()
            .expect("native measurement requires a live qualified surface");
        match request.family() {
            worth_ui_host_contract::UiMeasurementRequestFamily::ViewportExtent => {
                let [width, height] = graphics.extent();
                UiHostMeasurementObservationValue::ViewportExtent(UiViewportExtentObservation {
                    width: width as f32 / graphics.scale_factor as f32,
                    height: height as f32 / graphics.scale_factor as f32,
                })
            }
            worth_ui_host_contract::UiMeasurementRequestFamily::DpiScaleFactor => {
                UiHostMeasurementObservationValue::DpiScaleFactor(UiDpiScaleFactorObservation {
                    scale_factor: graphics.scale_factor as f32,
                })
            }
            _ => unreachable!("the Phase 2 seed admits only viewport and DPI measurement"),
        }
    }
}

impl WorthUiHostMechanicsAdapter for WorthUiNativeMechanicsAdapter {
    fn mechanical_host_contract(&self) -> WorthUiHostContract {
        debug_assert_eq!(
            self.profile,
            crate::UiNativePlatformProfileIdentity::WORTH_UI_WINDOWS_DX12_V1
        );
        WorthUiHostContract::native()
    }

    fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(vec![
            WorthUiHostCapability::ViewportObservation,
            WorthUiHostCapability::DpiObservation,
            WorthUiHostCapability::NativePaint,
        ])
    }

    fn perform_surface_registration(
        &self,
        request: UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceRegistrationOutcome {
        let key = request.binding_generation().diagnostic_value();
        let mut state = self.state.borrow_mut();
        if state
            .registrations
            .get(&key)
            .is_some_and(|current| *current != request)
        {
            return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        if !state.registrations.contains_key(&key) {
            let owner = match state
                .resources
                .register(super::UiNativeResourceClass::HostRegistration)
            {
                Ok(owner) => owner,
                Err(()) => {
                    return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                        UiHostSurfaceRegistrationDenial::CapacityExceeded,
                    );
                }
            };
            state.registrations.insert(key, request);
            state.registration_resources.insert(key, owner);
        }
        worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RegisteredKnownEmpty
    }

    fn perform_mounted_surface_presentation(
        &self,
        view: &UiMountedFrameConsumptionView<'_>,
    ) -> UiHostSurfacePresentationOutcome {
        if view.requirement().presentation_mode() != UiHostSurfacePresentationMode::NativeDisplay {
            return UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfacePresentationDenial::UnsupportedPresentationMode(
                    view.requirement().presentation_mode(),
                ),
            );
        }
        let mut state = self.state.borrow_mut();
        let super::UiNativeHostState {
            graphics,
            resources,
            ..
        } = &mut *state;
        let Some(graphics) = graphics.as_mut() else {
            return UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfacePresentationDenial::AdapterDeclined,
            );
        };
        let result = present_initial::<UiWgpuNativePresentationPort>(graphics, resources, view);
        let (observation, cost) = match result {
            Ok(presented) => presented.into_parts(),
            Err(failure) => return settle_presentation_failure(&mut state, failure),
        };
        state.effect_posture = super::UiNativeEffectPosture::Presented;
        state.last_presentation = Some(observation);
        UiHostSurfacePresentationOutcome::Presented(UiMountedSurfacePresentationCompletion::new(
            UiHostSurfacePresentationMode::NativeDisplay,
            worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(
                view.attempt().diagnostic_value(),
            ),
            UiMountedCompletedEffects::new(vec![UiMountedEffectFamily::NativePaint]),
            cost,
        ))
    }

    fn perform_surface_deregistration(
        &self,
        request: UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome {
        let key = request.binding_generation().diagnostic_value();
        let mut state = self.state.borrow_mut();
        if state.registrations.remove(&key) != Some(request) {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
                UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        let Some(owner) = state.registration_resources.remove(&key) else {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::DeregistrationIndeterminate(
                worth_ui_host_contract::UiHostSurfaceDeregistrationIndeterminate::after_effects_may_have_begun(request),
            );
        };
        state
            .resources
            .release(owner)
            .expect("registration owner must remain exact");
        worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::Deregistered(
            worth_ui_host_contract::UiHostSurfaceDeregistrationReceipt::from_runtime(
                request.host_session_identity(),
                request.host_surface_identity(),
            ),
        )
    }

    fn release_mechanical_host_session(
        &self,
        host_session_identity: u64,
    ) -> UiHostSessionReleaseOutcome {
        let mut state = self.state.borrow_mut();
        let before = state.registrations.len();
        let released = state
            .registrations
            .iter()
            .filter_map(|(key, request)| {
                (request.host_session_identity() == host_session_identity).then_some(*key)
            })
            .collect::<Vec<_>>();
        for key in released {
            state.registrations.remove(&key);
            let owner = state
                .registration_resources
                .remove(&key)
                .expect("registration resource must exist");
            state
                .resources
                .release(owner)
                .expect("registration owner must remain exact");
        }
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            host_session_identity,
            before - state.registrations.len(),
        ))
    }
}

fn settle_presentation_failure(
    state: &mut UiNativeHostState,
    failure: UiNativePresentationFailure,
) -> UiHostSurfacePresentationOutcome {
    match failure {
        UiNativePresentationFailure::BeforeEffects(denial) => {
            UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial)
        }
        UiNativePresentationFailure::Indeterminate(pending) => {
            state.pending_presentations.push(pending);
            mark_presentation_indeterminate(state)
        }
    }
}

fn mark_presentation_indeterminate(
    state: &mut UiNativeHostState,
) -> UiHostSurfacePresentationOutcome {
    state.effect_posture = super::UiNativeEffectPosture::PresentationIndeterminate;
    UiHostSurfacePresentationOutcome::PresentationIndeterminate
}

#[cfg(test)]
mod tests {
    use super::{settle_presentation_failure, UiNativePresentationFailure};
    use crate::native::{UiNativeEffectPosture, UiNativeHostState, UiNativePendingPresentation};

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
        super::super::presentation::prove_nonuniform_readback_port();
        let mut state = UiNativeHostState::new();
        let external_dropped = std::rc::Rc::new(std::cell::Cell::new(false));
        let pending = UiNativePendingPresentation::scripted(
            &mut state.resources,
            std::rc::Rc::clone(&external_dropped),
        );
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
}
