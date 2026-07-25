use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiHostPresentationCostInput,
    UiHostPresentationCostReport, UiHostProtocolNegotiation, UiHostSurfacePresentationDenial,
    UiHostSurfacePresentationMode, UiHostSurfacePresentationOutcome,
    UiHostSurfaceRegistrationDenial, UiHostSurfaceRegistrationRequest, UiMountedCompletedEffects,
    UiMountedEffectFamily, UiMountedFrameConsumptionView, UiMountedSurfacePresentationCompletion,
    WorthUiHostCapability, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiMeasurementHostAdapter,
};

use super::headless_translation::translate_headless_frame;
use super::{
    UiHeadlessMountedFrameTranscript, UiHeadlessRecorderCapacity, UiHostAdapterSessionAuthority,
    UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt, WorthUiOperationalHostAdapter,
};

#[derive(Clone)]
pub struct WorthUiHeadlessRecorder {
    state: Rc<RefCell<WorthUiHeadlessRecorderState>>,
}

struct WorthUiHeadlessRecorderState {
    capacity: UiHeadlessRecorderCapacity,
    registrations:
        BTreeMap<worth_ui_host_contract::UiHostSurfaceIdentity, UiHostSurfaceRegistrationRequest>,
    transcripts: VecDeque<UiHeadlessMountedFrameTranscript>,
}

impl WorthUiHeadlessRecorder {
    pub fn new(capacity: UiHeadlessRecorderCapacity) -> Self {
        Self {
            state: Rc::new(RefCell::new(WorthUiHeadlessRecorderState {
                capacity,
                registrations: BTreeMap::new(),
                transcripts: VecDeque::new(),
            })),
        }
    }

    pub fn observed_transcripts(&self) -> Box<[UiHeadlessMountedFrameTranscript]> {
        self.state.borrow().transcripts.iter().cloned().collect()
    }

    pub fn drain_transcripts(&self) -> Box<[UiHeadlessMountedFrameTranscript]> {
        self.state.borrow_mut().transcripts.drain(..).collect()
    }

    fn validate_registration(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: UiHostSurfaceRegistrationRequest,
    ) -> Result<(), UiHostSurfaceRegistrationDenial> {
        let agreement = match self.operational_protocol_contract().negotiate() {
            UiHostProtocolNegotiation::Compatible(agreement) => agreement,
            UiHostProtocolNegotiation::Incompatible(_) => {
                return Err(UiHostSurfaceRegistrationDenial::ForeignRegistration);
            }
        };
        let capabilities = self.operational_capability_report();
        if request.host_session_identity() != authority.host_session_identity()
            || request.protocol() != agreement
            || request.capability_generation().as_u64() == 0
            || request.capability_profile_digest() != capabilities.profile_identity_digest()
        {
            return Err(UiHostSurfaceRegistrationDenial::ForeignRegistration);
        }
        let state = self.state.borrow();
        if state.registrations.len() >= state.capacity.surface_bindings() {
            return Err(UiHostSurfaceRegistrationDenial::CapacityExceeded);
        }
        if state
            .registrations
            .contains_key(&request.host_surface_identity())
        {
            return Err(UiHostSurfaceRegistrationDenial::ForeignRegistration);
        }
        Ok(())
    }

    fn validate_presented_view(
        &self,
        view: &UiMountedFrameConsumptionView<'_>,
    ) -> Result<UiHeadlessRecorderCapacity, UiHostSurfacePresentationDenial> {
        if view.requirement().presentation_mode() != UiHostSurfacePresentationMode::RecordOnly {
            return Err(
                UiHostSurfacePresentationDenial::UnsupportedPresentationMode(
                    view.requirement().presentation_mode(),
                ),
            );
        }
        let live_protocol = match self.operational_protocol_contract().negotiate() {
            UiHostProtocolNegotiation::Compatible(agreement) => agreement,
            UiHostProtocolNegotiation::Incompatible(denial) => {
                return Err(UiHostSurfacePresentationDenial::Protocol(denial));
            }
        };
        let live_profile = self
            .operational_capability_report()
            .profile_identity_digest();
        let state = self.state.borrow();
        if state.transcripts.len() >= state.capacity.retained_frames() {
            return Err(UiHostSurfacePresentationDenial::CapacityExceeded);
        }
        let requirement = view.requirement();
        let registration = state
            .registrations
            .get(&requirement.host_surface())
            .ok_or(UiHostSurfacePresentationDenial::SurfaceBindingChanged)?;
        if view.host_session_identity() != registration.host_session_identity() {
            return Err(UiHostSurfacePresentationDenial::SurfaceBindingChanged);
        }
        if view.protocol() != registration.protocol() || live_protocol != registration.protocol() {
            return Err(UiHostSurfacePresentationDenial::ProtocolChanged);
        }
        if view.capability_generation() != registration.capability_generation()
            || requirement.capability_generation() != registration.capability_generation()
        {
            return Err(UiHostSurfacePresentationDenial::CapabilityGenerationChanged);
        }
        if view.capability_profile_digest() != registration.capability_profile_digest()
            || requirement.capability_profile_digest() != registration.capability_profile_digest()
            || live_profile != registration.capability_profile_digest()
        {
            return Err(UiHostSurfacePresentationDenial::CapabilityProfileChanged);
        }
        if requirement.semantic_surface() != registration.semantic_surface_identity()
            || requirement.binding() != registration.binding_generation()
            || requirement.presentation_mode() != registration.presentation_mode()
            || view.projection().surface() != registration.semantic_surface_identity()
            || view.projection().binding() != registration.binding_generation()
        {
            return Err(UiHostSurfacePresentationDenial::SurfaceBindingChanged);
        }
        Ok(state.capacity)
    }
}

impl Default for WorthUiHeadlessRecorder {
    fn default() -> Self {
        Self::new(UiHeadlessRecorderCapacity::production_default())
    }
}

impl WorthUiMeasurementHostAdapter for WorthUiHeadlessRecorder {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("headless recorder advertises no native measurement capabilities")
    }
}

impl WorthUiOperationalHostAdapter for WorthUiHeadlessRecorder {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::MountedFrameRecording])
    }

    fn register_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceRegistrationOutcome {
        if let Err(denial) = self.validate_registration(authority, request) {
            return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                denial,
            );
        }
        self.state
            .borrow_mut()
            .registrations
            .insert(request.host_surface_identity(), request);
        worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::Registered(
            request.confirm_known_empty(),
        )
    }

    fn deregister_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        request: UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome {
        if request.host_session_identity() != authority.host_session_identity() {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
                UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        let mut state = self.state.borrow_mut();
        if state.registrations.get(&request.host_surface_identity()) != Some(&request) {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
                UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        state.registrations.remove(&request.host_surface_identity());
        worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::Deregistered(
            worth_ui_host_contract::UiHostSurfaceDeregistrationReceipt::from_runtime(
                request.host_session_identity(),
                request.host_surface_identity(),
            ),
        )
    }

    fn present_mounted_surface(
        &self,
        authority: &UiHostAdapterSessionAuthority,
        view: &UiMountedFrameConsumptionView<'_>,
    ) -> UiHostSurfacePresentationOutcome {
        if !authority.admits_mounted_presentation(view) {
            return UiHostSurfacePresentationOutcome::RejectedBeforeEffects(
                UiHostSurfacePresentationDenial::SurfaceBindingChanged,
            );
        }
        let capacity = match self.validate_presented_view(view) {
            Ok(capacity) => capacity,
            Err(denial) => {
                return UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial);
            }
        };
        let transcript = match translate_headless_frame(view, capacity) {
            Ok(transcript) => transcript,
            Err(denial) => {
                return UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial);
            }
        };
        let adapter_cost = match projection_cost(view.projection()) {
            Ok(cost) => cost,
            Err(denial) => {
                return UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial);
            }
        };
        self.state.borrow_mut().transcripts.push_back(transcript);
        UiHostSurfacePresentationOutcome::Presented(UiMountedSurfacePresentationCompletion::new(
            UiHostSurfacePresentationMode::RecordOnly,
            UiMountedCompletedEffects::new(vec![UiMountedEffectFamily::RecordedProjection]),
            adapter_cost,
        ))
    }

    fn release_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> UiHostSessionReleaseOutcome {
        let mut state = self.state.borrow_mut();
        let retained_before = state.registrations.len();
        state.registrations.retain(|_, request| {
            request.host_session_identity() != authority.host_session_identity()
        });
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            authority.host_session_identity(),
            retained_before - state.registrations.len(),
        ))
    }
}

fn projection_cost(
    projection: &worth_ui_host_contract::UiMountedProjectionView,
) -> Result<UiHostPresentationCostReport, UiHostSurfacePresentationDenial> {
    let rows = [
        projection.nodes().len(),
        projection.clips().rows().len(),
        projection.layers().rows().len(),
        projection.paint_batches().rows().len(),
        projection.spatial_batches().rows().len(),
        projection.realtime_batches().rows().len(),
        projection.resources().entries().len(),
    ]
    .into_iter()
    .try_fold(0usize, usize::checked_add)
    .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?;
    let bytes = [
        std::mem::size_of_val(projection.nodes()),
        std::mem::size_of_val(projection.clips().rows()),
        std::mem::size_of_val(projection.layers().rows()),
        std::mem::size_of_val(projection.paint_batches().rows()),
        std::mem::size_of_val(projection.spatial_batches().rows()),
        std::mem::size_of_val(projection.realtime_batches().rows()),
        std::mem::size_of_val(projection.resources().entries()),
    ]
    .into_iter()
    .try_fold(0usize, usize::checked_add)
    .ok_or(UiHostSurfacePresentationDenial::CapacityExceeded)?;
    Ok(UiHostPresentationCostReport::from_adapter(
        UiHostPresentationCostInput {
            presented_surfaces: 1,
            translated_rows: u64::try_from(rows)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            translated_bytes: u64::try_from(bytes)
                .map_err(|_| UiHostSurfacePresentationDenial::CapacityExceeded)?,
            native_resource_cache_hits: 0,
            native_resource_cache_misses: 0,
            asynchronous_handoffs: 0,
        },
    ))
}
