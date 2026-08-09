use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiHostSessionReleaseOutcome,
    UiHostSessionReleaseReceipt, UiHostSurfaceRegistrationRequest, WorthUiHostCapabilityReport,
    WorthUiHostContract, WorthUiHostMechanicsAdapter, WorthUiMeasurementHostAdapter,
};

/// Effect-free Phase 1 native mechanics binding.
///
/// It owns only registration bookkeeping and the qualified profile identity.
/// Native resource acquisition and presentation remain unavailable until the
/// Phase 2 vertical slice activates their consuming states.
pub struct WorthUiPreparedNativeHost {
    registrations: Rc<RefCell<BTreeMap<u64, UiHostSurfaceRegistrationRequest>>>,
    profile: super::UiNativePlatformProfileIdentity,
}

impl WorthUiMeasurementHostAdapter for WorthUiPreparedNativeHost {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("Phase 1 native mechanics expose no measurement effects")
    }
}

impl WorthUiHostMechanicsAdapter for WorthUiPreparedNativeHost {
    fn mechanical_host_contract(&self) -> WorthUiHostContract {
        debug_assert_eq!(
            self.profile,
            super::UiNativePlatformProfileIdentity::WORTH_UI_WINDOWS_DX12_V1
        );
        WorthUiHostContract::native()
    }

    fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(Vec::new())
    }

    fn perform_surface_registration(
        &self,
        request: UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceRegistrationOutcome {
        let key = request.binding_generation().diagnostic_value();
        let mut registrations = self.registrations.borrow_mut();
        if registrations
            .get(&key)
            .is_some_and(|current| *current != request)
        {
            return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        registrations.insert(key, request);
        worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RegisteredKnownEmpty
    }

    fn perform_surface_deregistration(
        &self,
        request: UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome {
        let key = request.binding_generation().diagnostic_value();
        if self.registrations.borrow_mut().remove(&key) != Some(request) {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
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
        let mut registrations = self.registrations.borrow_mut();
        let before = registrations.len();
        registrations.retain(|_, request| request.host_session_identity() != host_session_identity);
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            host_session_identity,
            before - registrations.len(),
        ))
    }
}
