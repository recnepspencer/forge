use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, WindowEvent};
use winit::keyboard::{Key, PhysicalKey};
use worth_ui_host_contract::{
    UiHostObservationDrain, UiHostObservationPresentationBasis, UiHostPresentationEpoch,
    UiHostProtocolAgreement, UiMountedFrameIdentity, UiSurfaceBindingGeneration,
};

use crate::native::{
    UiNativeLifecycleEffect, UiNativeLifecycleProtocol, UiNativeLifecycleRequiredAction,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeInputObservationContractDisposition {
    Ignored,
    Retained,
    Stopped,
}

pub struct UiNativeInputObservationContract {
    protocol: UiNativeLifecycleProtocol,
}

impl UiNativeInputObservationContract {
    pub fn new() -> Self {
        Self {
            protocol: UiNativeLifecycleProtocol::new(),
        }
    }

    pub fn install_initial_profile(&mut self, scale_factor: f64, physical_size: [u32; 2]) {
        self.protocol
            .install_initial_profile(scale_factor, physical_size);
    }

    pub fn observe_window_event_at(
        &mut self,
        event: &WindowEvent,
        event_tick: u64,
        pointer_position: Option<PhysicalPosition<f64>>,
    ) -> UiNativeInputObservationContractDisposition {
        map_transition(
            self.protocol
                .observe_window_event_at(event, event_tick, pointer_position),
        )
    }

    pub fn observe_keyboard_components_at(
        &mut self,
        logical_key: &Key,
        physical_key: PhysicalKey,
        key_state: ElementState,
        repeat: bool,
        text: Option<&str>,
        event_tick: u64,
    ) -> UiNativeInputObservationContractDisposition {
        map_transition(self.protocol.observe_keyboard_components_at(
            logical_key,
            physical_key,
            key_state,
            repeat,
            text,
            event_tick,
        ))
    }

    pub fn observe_profile_transition_at(
        &mut self,
        scale_factor: f64,
        physical_size: [u32; 2],
        event_tick: u64,
    ) {
        self.protocol
            .observe_profile_transition_at(scale_factor, physical_size, event_tick);
    }

    pub fn record_completed_presentation(
        &mut self,
        protocol: UiHostProtocolAgreement,
        host_session: u64,
        presentation: UiHostObservationPresentationBasis,
    ) -> bool {
        if self.protocol.register_session(host_session).is_err() {
            return false;
        }
        self.protocol
            .record_completed_presentation(protocol, host_session, presentation)
            .effect()
            == UiNativeLifecycleEffect::PresentationCompleted
    }

    pub fn remember_pending_presentation(
        &mut self,
        protocol: UiHostProtocolAgreement,
        host_session: u64,
        host_surface: worth_ui_host_contract::UiHostSurfaceIdentity,
        binding: UiSurfaceBindingGeneration,
        completion_identity: u64,
    ) -> bool {
        if self.protocol.register_session(host_session).is_err() {
            return false;
        }
        self.protocol
            .remember_pending_presentation(
                protocol,
                host_session,
                host_surface,
                binding,
                completion_identity,
            )
            .required_action()
            == Some(UiNativeLifecycleRequiredAction::CompletePresentation)
    }

    pub fn complete_pending_presentation(
        &mut self,
        frame: UiMountedFrameIdentity,
        binding: UiSurfaceBindingGeneration,
        epoch: UiHostPresentationEpoch,
        completion_identity: u64,
    ) -> bool {
        self.protocol
            .complete_pending_presentation(frame, binding, epoch, completion_identity)
            .effect()
            == UiNativeLifecycleEffect::PresentationCompleted
    }

    pub fn drain(&mut self, host_session: u64) -> UiHostObservationDrain {
        self.protocol.drain(host_session)
    }

    pub fn release_session(&mut self, host_session: u64) {
        self.protocol.release_session(host_session);
    }

    pub fn install_input_recipient(
        &mut self,
        binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        self.protocol.install_input_recipient(binding)
    }

    pub fn close(&mut self) {
        self.protocol.close();
    }

    pub fn has_retained_observations(&self) -> bool {
        self.protocol.has_retained_observations()
    }

    pub fn report(&self) -> super::UiNativeInputObservationReport {
        self.protocol.report()
    }
}

impl Default for UiNativeInputObservationContract {
    fn default() -> Self {
        Self::new()
    }
}

fn map_transition(
    transition: crate::native::UiNativeLifecycleTransition,
) -> UiNativeInputObservationContractDisposition {
    match transition.effect() {
        UiNativeLifecycleEffect::Ignored | UiNativeLifecycleEffect::NoOp => {
            UiNativeInputObservationContractDisposition::Ignored
        }
        UiNativeLifecycleEffect::Retained => UiNativeInputObservationContractDisposition::Retained,
        UiNativeLifecycleEffect::Denied(_) => UiNativeInputObservationContractDisposition::Stopped,
        UiNativeLifecycleEffect::PresentationCompleted
        | UiNativeLifecycleEffect::CloseDeferred
        | UiNativeLifecycleEffect::Closed => UiNativeInputObservationContractDisposition::Ignored,
    }
}
