use std::cell::RefCell;
use std::rc::Rc;

use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiHostMeasurementObservationValue, UiHostMeasurementRequest,
    UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt, UiHostSurfacePresentationMode,
    UiHostSurfacePresentationOutcome, UiHostSurfaceRegistrationDenial,
    UiHostSurfaceRegistrationRequest, UiMountedFrameConsumptionView, UiViewportExtentObservation,
    WorthUiHostCapability, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiHostMechanicsAdapter, WorthUiMeasurementHostAdapter,
};

use super::UiNativeHostState;

#[path = "mechanics_adapter/presentation.rs"]
mod presentation;

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
        presentation::perform_native_presentation(&mut self.state.borrow_mut(), view)
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
        state.retained_draw_lists.remove(&key);
        state.reconstruction_required.remove(&key);
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
            state.retained_draw_lists.remove(&key);
            state.reconstruction_required.remove(&key);
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
