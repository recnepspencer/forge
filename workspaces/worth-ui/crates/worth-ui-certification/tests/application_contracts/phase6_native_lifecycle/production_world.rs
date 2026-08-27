pub(super) use super::protocol_events::UiNativeLifecycleEvent;
use super::protocol_events::{protocol, window_event};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, Ime, WindowEvent};
use winit::keyboard::{Key, KeyCode, PhysicalKey};
use worth_ui_host_contract::{
    UiHostApplicationGeneration, UiHostInputDraftSessionIdentity, UiHostInputRecipientBindingInput,
    UiHostInputRecipientBindingReceipt, UiHostInputRecipientFamily, UiHostInputRecipientGeneration,
    UiHostObservationPresentationBasis, UiHostPresentationEpoch, UiMountedFrameIdentity,
    UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration, UiTextProfileGeneration, UI_HOST_OBSERVATION_BATCH_BYTE_LIMIT,
};
pub(super) use worth_ui_host_native::{
    UiNativeLifecycleEffect, UiNativeLifecyclePhase as UiNativeLifecycleState,
    UiNativeLifecycleRequiredAction as UiNativeLifecycleAction,
};
use worth_ui_host_native::{UiNativeLifecycleProtocol, UiNativeLifecycleTransition};

const HOST_SESSION: u64 = 73;
const INITIAL_GENERATION: u64 = 7;
const PENDING_COMPLETION: u64 = 701;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiNativeLifecycleObservation {
    pub(super) state: UiNativeLifecycleState,
    pub(super) effect: UiNativeLifecycleEffect,
    pub(super) retained_delta: u64,
    pub(super) predecessor: Option<u64>,
    pub(super) next_action: Option<UiNativeLifecycleAction>,
}

pub(super) struct UiNativeLifecycleWorld {
    protocol: UiNativeLifecycleProtocol,
    generation: u64,
    binding: UiSurfaceBindingGeneration,
    pending_binding: Option<UiSurfaceBindingGeneration>,
    surface: UiSemanticSurfaceIdentity,
    mounted_instance: UiMountedInstanceIdentity,
    node_receipt: UiMountedNodeReceiptIdentity,
    recipient: Option<UiHostInputRecipientBindingReceipt>,
}

impl UiNativeLifecycleWorld {
    pub(super) fn new(state: UiNativeLifecycleState) -> Self {
        let mut world = Self {
            protocol: UiNativeLifecycleProtocol::new(),
            generation: INITIAL_GENERATION,
            binding: UiSurfaceBindingGeneration::mint_unbound().expect("initial binding"),
            pending_binding: None,
            surface: UiSemanticSurfaceIdentity::mint_unbound().expect("semantic surface"),
            mounted_instance: UiMountedInstanceIdentity::mint_unbound().expect("mounted instance"),
            node_receipt: UiMountedNodeReceiptIdentity::mint_unbound().expect("node receipt"),
            recipient: None,
        };
        world
            .protocol
            .register_session(HOST_SESSION)
            .expect("runtime opens the native host session before lifecycle work");
        world.protocol.install_initial_profile(1.0, [800, 600]);
        world.compile_state(state);
        assert_eq!(world.protocol.phase(), state);
        world
    }

    pub(super) fn apply(&mut self, event: UiNativeLifecycleEvent) -> UiNativeLifecycleObservation {
        let transition = match event {
            UiNativeLifecycleEvent::Keyboard => self.keyboard(),
            UiNativeLifecycleEvent::ExactCapacityText => self.capacity_text(0),
            UiNativeLifecycleEvent::OverCapacityText => self.over_capacity_text(),
            UiNativeLifecycleEvent::ValidImeRange => self.valid_ime_range(),
            UiNativeLifecycleEvent::UnprovableImeRange => self.unprovable_ime_range(),
            UiNativeLifecycleEvent::TextWithoutRecipient => self.text_without_recipient(),
            UiNativeLifecycleEvent::TextWithStaleRecipient => self.text_with_stale_recipient(),
            UiNativeLifecycleEvent::BeginSuccessor => self.begin_successor(),
            UiNativeLifecycleEvent::BeginProfileTransition => {
                self.begin_profile_transition(1.5, [1_200, 800])
            }
            UiNativeLifecycleEvent::BeginZeroSizedProfile => {
                self.begin_profile_transition(1.5, [0, 0])
            }
            UiNativeLifecycleEvent::CompletePresentation => self.complete_presentation(),
            input => self.window_input(input),
        };
        observe(transition)
    }

    pub(super) fn drain_retained(&mut self) -> usize {
        self.protocol.drain(HOST_SESSION).into_batches().len()
    }

    pub(super) fn report(&self) -> worth_ui_host_native::UiNativeInputObservationReport {
        self.protocol.report()
    }

    pub(super) fn request_close(&mut self) -> UiNativeLifecycleObservation {
        observe(self.protocol.request_close())
    }

    fn compile_state(&mut self, state: UiNativeLifecycleState) {
        match state {
            UiNativeLifecycleState::BeforeFirstPresentation => {}
            UiNativeLifecycleState::Presented => self.compile_presented(),
            UiNativeLifecycleState::SuccessorInFlight => {
                self.compile_presented();
                self.window_input(UiNativeLifecycleEvent::Pointer);
                self.begin_successor();
            }
            UiNativeLifecycleState::ProfileTransition => {
                self.compile_presented();
                self.window_input(UiNativeLifecycleEvent::Pointer);
                self.begin_profile_transition(1.5, [1_200, 800]);
            }
            UiNativeLifecycleState::Closing => {
                self.compile_presented();
                self.window_input(UiNativeLifecycleEvent::Pointer);
                self.protocol.request_close();
            }
            UiNativeLifecycleState::Closed => self.protocol.close(),
        }
    }

    fn compile_presented(&mut self) {
        let transition = self.protocol.record_completed_presentation(
            protocol(),
            HOST_SESSION,
            self.basis(self.binding, self.generation),
        );
        assert_eq!(
            transition.effect(),
            UiNativeLifecycleEffect::PresentationCompleted
        );
        self.install_current_recipient();
    }

    fn begin_successor(&mut self) -> UiNativeLifecycleTransition {
        let binding = UiSurfaceBindingGeneration::mint_unbound().expect("successor binding");
        self.pending_binding = Some(binding);
        self.protocol.remember_pending_presentation(
            protocol(),
            HOST_SESSION,
            binding,
            PENDING_COMPLETION,
        )
    }

    fn begin_profile_transition(
        &mut self,
        scale_factor: f64,
        physical_size: [u32; 2],
    ) -> UiNativeLifecycleTransition {
        self.protocol
            .observe_profile_transition_at(scale_factor, physical_size, 41)
    }

    fn complete_presentation(&mut self) -> UiNativeLifecycleTransition {
        if let Some(binding) = self.pending_binding.take() {
            self.generation += 1;
            let transition = self.protocol.complete_pending_presentation(
                UiMountedFrameIdentity::mint_unbound().expect("successor frame"),
                binding,
                UiHostPresentationEpoch::issued_by_host(self.generation),
                PENDING_COMPLETION,
            );
            self.binding = binding;
            self.install_current_recipient();
            return transition;
        }
        if self.protocol.phase() == UiNativeLifecycleState::ProfileTransition {
            self.generation += 1;
            return self.protocol.record_completed_presentation(
                protocol(),
                HOST_SESSION,
                self.basis(self.binding, self.generation),
            );
        }
        self.generation += 1;
        let transition = self.protocol.record_completed_presentation(
            protocol(),
            HOST_SESSION,
            self.basis(self.binding, self.generation),
        );
        if transition.effect() == UiNativeLifecycleEffect::PresentationCompleted {
            self.install_current_recipient();
        }
        transition
    }

    fn keyboard(&mut self) -> UiNativeLifecycleTransition {
        self.protocol.observe_keyboard_components_at(
            &Key::Character("a".into()),
            PhysicalKey::Code(KeyCode::KeyA),
            ElementState::Pressed,
            false,
            Some("a"),
            11,
        )
    }

    fn over_capacity_text(&mut self) -> UiNativeLifecycleTransition {
        self.capacity_text(1)
    }

    fn capacity_text(&mut self, excess: usize) -> UiNativeLifecycleTransition {
        if excess == 0
            && matches!(
                self.protocol.phase(),
                UiNativeLifecycleState::Presented | UiNativeLifecycleState::SuccessorInFlight
            )
        {
            let _ = self.protocol.drain(HOST_SESSION);
        }
        let empty_payload = worth_ui_host_contract::UiHostObservationPayload::ImeComposition {
            revision: 1,
            phase: worth_ui_host_contract::UiHostImeCompositionPhase::Commit(Box::from("")),
        };
        let overhead = worth_ui_host_contract::UiHostObservationReport::input_affine_encoded_len(
            &empty_payload,
        );
        let text_bytes = UI_HOST_OBSERVATION_BATCH_BYTE_LIMIT - overhead + excess;
        self.protocol.observe_window_event_at(
            &WindowEvent::Ime(Ime::Commit("x".repeat(text_bytes))),
            11,
            None,
        )
    }

    fn valid_ime_range(&mut self) -> UiNativeLifecycleTransition {
        self.protocol.observe_window_event_at(
            &WindowEvent::Ime(Ime::Preedit("é".to_owned(), Some((0, 2)))),
            11,
            None,
        )
    }

    fn unprovable_ime_range(&mut self) -> UiNativeLifecycleTransition {
        self.protocol.observe_window_event_at(
            &WindowEvent::Ime(Ime::Preedit("é".to_owned(), Some((1, 2)))),
            11,
            None,
        )
    }

    fn text_without_recipient(&mut self) -> UiNativeLifecycleTransition {
        if let Some(recipient) = self.recipient.take() {
            assert!(self.protocol.clear_input_recipient(recipient));
        }
        self.protocol.observe_window_event_at(
            &WindowEvent::Ime(Ime::Commit("draft".to_owned())),
            11,
            None,
        )
    }

    fn text_with_stale_recipient(&mut self) -> UiNativeLifecycleTransition {
        let stale_binding = UiSurfaceBindingGeneration::mint_unbound().expect("stale binding");
        let stale = self.recipient_for(stale_binding, self.generation + 1);
        if self.protocol.install_input_recipient(stale) {
            self.recipient = Some(stale);
        }
        self.protocol.observe_window_event_at(
            &WindowEvent::Ime(Ime::Commit("draft".to_owned())),
            11,
            None,
        )
    }

    fn window_input(&mut self, event: UiNativeLifecycleEvent) -> UiNativeLifecycleTransition {
        self.protocol.observe_window_event_at(
            &window_event(event),
            11,
            (event == UiNativeLifecycleEvent::Button).then_some(PhysicalPosition::new(12.0, 24.0)),
        )
    }

    fn install_current_recipient(&mut self) {
        let receipt = self.recipient_for(self.binding, self.generation);
        assert!(self.protocol.install_input_recipient(receipt));
        self.recipient = Some(receipt);
    }

    fn recipient_for(
        &self,
        binding: UiSurfaceBindingGeneration,
        generation: u64,
    ) -> UiHostInputRecipientBindingReceipt {
        UiHostInputRecipientBindingReceipt::new(UiHostInputRecipientBindingInput {
            host_session: HOST_SESSION,
            application_generation: UiHostApplicationGeneration::new(1)
                .expect("application generation"),
            recipient_generation: UiHostInputRecipientGeneration::new(generation)
                .expect("recipient generation"),
            family: UiHostInputRecipientFamily::Draft,
            draft_session: UiHostInputDraftSessionIdentity::new(1),
            surface: self.surface,
            binding,
            mounted_instance: self.mounted_instance,
            node_receipt: self.node_receipt,
            text_profile: UiTextProfileGeneration::new(1),
        })
    }

    fn basis(
        &self,
        binding: UiSurfaceBindingGeneration,
        epoch: u64,
    ) -> UiHostObservationPresentationBasis {
        UiHostObservationPresentationBasis::new(
            UiMountedFrameIdentity::mint_unbound().expect("presentation frame"),
            binding,
            UiHostPresentationEpoch::issued_by_host(epoch),
        )
    }
}

fn observe(transition: UiNativeLifecycleTransition) -> UiNativeLifecycleObservation {
    UiNativeLifecycleObservation {
        state: transition.phase(),
        effect: transition.effect(),
        retained_delta: transition.retained_delta(),
        predecessor: transition
            .predecessor()
            .map(|epoch| epoch.diagnostic_value()),
        next_action: transition.required_action(),
    }
}
