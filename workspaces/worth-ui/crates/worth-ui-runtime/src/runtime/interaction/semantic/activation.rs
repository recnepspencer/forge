use crate::runtime::WorthUiActiveApplicationGenerationIdentity;
use worth_ui_host_contract::{
    UiHostKey, UiHostKeyboardModifiers, UiHostObservationPresentationBasis,
    UiHostObservationSequence, UiHostObservationTimeBasis,
};

#[derive(Debug)]
pub struct UiActivateInteraction {
    source: UiActivateInteractionSource,
    generation: WorthUiActiveApplicationGenerationIdentity,
}

#[derive(Debug)]
pub enum UiActivateInteractionSource {
    Pointer(super::super::UiTargetedPointerGesture),
    Keyboard(UiKeyboardActivationEvidence),
}

#[derive(Debug)]
pub struct UiKeyboardActivationEvidence {
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    presentation: UiHostObservationPresentationBasis,
    generation: WorthUiActiveApplicationGenerationIdentity,
    sequence: UiHostObservationSequence,
    time_basis: UiHostObservationTimeBasis,
    key: UiHostKey,
    modifiers: UiHostKeyboardModifiers,
}

impl UiActivateInteraction {
    pub(crate) fn from_pointer(
        gesture: super::super::UiTargetedPointerGesture,
        generation: WorthUiActiveApplicationGenerationIdentity,
    ) -> Self {
        Self {
            source: UiActivateInteractionSource::Pointer(gesture),
            generation,
        }
    }

    pub(crate) fn from_keyboard(evidence: UiKeyboardActivationEvidence) -> Self {
        let generation = evidence.generation.clone();
        Self {
            source: UiActivateInteractionSource::Keyboard(evidence),
            generation,
        }
    }

    pub const fn source(&self) -> &UiActivateInteractionSource {
        &self.source
    }

    pub const fn target(&self) -> crate::runtime::interaction::UiPresentedInteractionTargetView {
        match &self.source {
            UiActivateInteractionSource::Pointer(gesture) => gesture.released_target(),
            UiActivateInteractionSource::Keyboard(evidence) => evidence.target,
        }
    }

    pub const fn generation(&self) -> &WorthUiActiveApplicationGenerationIdentity {
        &self.generation
    }

    pub const fn time_basis(&self) -> UiHostObservationTimeBasis {
        match &self.source {
            UiActivateInteractionSource::Pointer(gesture) => gesture.release_time_basis(),
            UiActivateInteractionSource::Keyboard(evidence) => evidence.time_basis(),
        }
    }

    pub const fn source_sequence(&self) -> UiHostObservationSequence {
        match &self.source {
            UiActivateInteractionSource::Pointer(gesture) => gesture.release_sequence(),
            UiActivateInteractionSource::Keyboard(evidence) => evidence.sequence(),
        }
    }
}

impl UiKeyboardActivationEvidence {
    pub(crate) fn new(input: super::UiKeyboardSemanticInput) -> Self {
        Self {
            target: input.target,
            presentation: input.presentation,
            generation: input.generation,
            sequence: input.sequence,
            time_basis: input.time_basis,
            key: input.key,
            modifiers: input.modifiers,
        }
    }

    pub const fn target(&self) -> crate::runtime::interaction::UiPresentedInteractionTargetView {
        self.target
    }

    pub const fn presentation(&self) -> UiHostObservationPresentationBasis {
        self.presentation
    }

    pub const fn generation(&self) -> &WorthUiActiveApplicationGenerationIdentity {
        &self.generation
    }

    pub const fn sequence(&self) -> UiHostObservationSequence {
        self.sequence
    }

    pub const fn time_basis(&self) -> UiHostObservationTimeBasis {
        self.time_basis
    }

    pub const fn key(&self) -> UiHostKey {
        self.key
    }

    pub const fn modifiers(&self) -> UiHostKeyboardModifiers {
        self.modifiers
    }
}
