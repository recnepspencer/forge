use worth_ui_host_contract::{
    UiHostKey, UiHostKeyboardModifiers, UiHostObservationPresentationBasis,
    UiHostObservationSequence,
};

#[derive(Debug)]
pub struct UiActivateInteraction {
    source: UiActivateInteractionSource,
    generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
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
    generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    sequence: UiHostObservationSequence,
    key: UiHostKey,
    modifiers: UiHostKeyboardModifiers,
}

impl UiActivateInteraction {
    pub(crate) fn from_pointer(
        gesture: super::super::UiTargetedPointerGesture,
        generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
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

    pub const fn generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.generation
    }
}

impl UiKeyboardActivationEvidence {
    pub(crate) fn new(input: super::UiKeyboardSemanticInput) -> Self {
        Self {
            target: input.target,
            presentation: input.presentation,
            generation: input.generation,
            sequence: input.sequence,
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

    pub const fn generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.generation
    }

    pub const fn sequence(&self) -> UiHostObservationSequence {
        self.sequence
    }

    pub const fn key(&self) -> UiHostKey {
        self.key
    }

    pub const fn modifiers(&self) -> UiHostKeyboardModifiers {
        self.modifiers
    }
}
