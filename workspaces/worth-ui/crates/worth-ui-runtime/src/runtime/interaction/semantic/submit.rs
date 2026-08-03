use worth_ui_host_contract::{
    UiHostKey, UiHostKeyboardModifiers, UiHostObservationPresentationBasis,
    UiHostObservationSequence,
};

#[derive(Debug)]
pub struct UiSubmitInteraction {
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    presentation: UiHostObservationPresentationBasis,
    generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
    sequence: UiHostObservationSequence,
    key: UiHostKey,
    modifiers: UiHostKeyboardModifiers,
}

impl UiSubmitInteraction {
    pub(crate) fn seal(input: super::UiKeyboardSemanticInput) -> Self {
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
