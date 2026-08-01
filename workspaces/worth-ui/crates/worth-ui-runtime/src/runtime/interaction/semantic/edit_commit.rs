use worth_ui_host_contract::{
    UiHostObservationPresentationBasis, UiHostObservationSequence, UiHostObservationTimeBasis,
};

use crate::runtime::WorthUiActiveApplicationGenerationIdentity;

use super::super::{UiDraftFieldIdentity, UiDraftSessionIdentity};

#[derive(Debug)]
pub struct UiEditCommitInteraction {
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    presentation: UiHostObservationPresentationBasis,
    generation: WorthUiActiveApplicationGenerationIdentity,
    session: UiDraftSessionIdentity,
    field: UiDraftFieldIdentity,
    source_sequence: UiHostObservationSequence,
    time_basis: UiHostObservationTimeBasis,
    input_revision: Option<u64>,
    draft_revision: u64,
    committed_text: std::sync::Arc<str>,
}

pub(crate) struct UiEditCommitInput {
    pub(crate) target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    pub(crate) presentation: UiHostObservationPresentationBasis,
    pub(crate) generation: WorthUiActiveApplicationGenerationIdentity,
    pub(crate) session: UiDraftSessionIdentity,
    pub(crate) field: UiDraftFieldIdentity,
    pub(crate) source_sequence: UiHostObservationSequence,
    pub(crate) time_basis: UiHostObservationTimeBasis,
    pub(crate) input_revision: Option<u64>,
    pub(crate) draft_revision: u64,
    pub(crate) committed_text: std::sync::Arc<str>,
}

impl UiEditCommitInteraction {
    pub(crate) fn seal(input: UiEditCommitInput) -> Self {
        Self {
            target: input.target,
            presentation: input.presentation,
            generation: input.generation,
            session: input.session,
            field: input.field,
            source_sequence: input.source_sequence,
            time_basis: input.time_basis,
            input_revision: input.input_revision,
            draft_revision: input.draft_revision,
            committed_text: input.committed_text,
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

    pub const fn session(&self) -> UiDraftSessionIdentity {
        self.session
    }

    pub const fn field(&self) -> UiDraftFieldIdentity {
        self.field
    }

    pub const fn source_sequence(&self) -> UiHostObservationSequence {
        self.source_sequence
    }

    pub const fn time_basis(&self) -> UiHostObservationTimeBasis {
        self.time_basis
    }

    pub const fn input_revision(&self) -> Option<u64> {
        self.input_revision
    }

    pub const fn draft_revision(&self) -> u64 {
        self.draft_revision
    }

    pub fn committed_text(&self) -> &str {
        &self.committed_text
    }

    pub(crate) fn committed_text_reference(&self) -> std::sync::Arc<str> {
        std::sync::Arc::clone(&self.committed_text)
    }
}
