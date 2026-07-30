use worth_ui_host_contract::{
    UiHostKey, UiHostKeyboardModifiers, UiHostObservationPresentationBasis,
    UiHostObservationSequence,
};

#[derive(Debug)]
pub struct UiActivateInteraction {
    source: UiActivateInteractionSource,
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
    sequence: UiHostObservationSequence,
    key: UiHostKey,
    modifiers: UiHostKeyboardModifiers,
}

impl UiActivateInteraction {
    pub(crate) const fn from_pointer(gesture: super::super::UiTargetedPointerGesture) -> Self {
        Self {
            source: UiActivateInteractionSource::Pointer(gesture),
        }
    }

    pub(crate) const fn from_keyboard(evidence: UiKeyboardActivationEvidence) -> Self {
        Self {
            source: UiActivateInteractionSource::Keyboard(evidence),
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
}

impl UiKeyboardActivationEvidence {
    pub(crate) const fn new(input: super::UiKeyboardSemanticInput) -> Self {
        Self {
            target: input.target,
            presentation: input.presentation,
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
