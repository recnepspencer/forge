use super::UiIntentEvidenceReference;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UiIntentInteractionEvidenceFamily {
    Activate,
    EditCommit,
    SelectionCommit,
    Submit,
}

/// Authority-free source facts retained for one semantic interaction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiIntentInteractionEvidenceInput {
    source_sequence: u64,
    target: UiIntentInteractionEvidenceTargetInput,
    family: UiIntentInteractionEvidenceFamily,
}

/// Authority-free mounted target facts retained for one semantic interaction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiIntentInteractionEvidenceTargetInput {
    presented_frame: u64,
    presentation_epoch: u64,
    mounted_instance: u64,
    semantic_target_digest: u64,
}

/// Immutable projection of a semantic interaction retained for inspection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentInteractionEvidence {
    reference: UiIntentEvidenceReference,
    input: UiIntentInteractionEvidenceInput,
}

impl UiIntentInteractionEvidenceInput {
    #[doc(hidden)]
    pub const fn from_diagnostic_parts(
        source_sequence: u64,
        target: UiIntentInteractionEvidenceTargetInput,
        family: UiIntentInteractionEvidenceFamily,
    ) -> Self {
        Self {
            source_sequence,
            target,
            family,
        }
    }

    pub const fn source_sequence(self) -> u64 {
        self.source_sequence
    }

    pub const fn presented_frame(self) -> u64 {
        self.target.presented_frame()
    }

    pub const fn presentation_epoch(self) -> u64 {
        self.target.presentation_epoch()
    }

    pub const fn mounted_instance(self) -> u64 {
        self.target.mounted_instance()
    }

    pub const fn semantic_target_digest(self) -> u64 {
        self.target.semantic_target_digest()
    }

    pub const fn target(self) -> UiIntentInteractionEvidenceTargetInput {
        self.target
    }

    pub const fn family(self) -> UiIntentInteractionEvidenceFamily {
        self.family
    }
}

impl UiIntentInteractionEvidenceTargetInput {
    #[doc(hidden)]
    pub const fn from_diagnostic_parts(
        presented_frame: u64,
        presentation_epoch: u64,
        mounted_instance: u64,
        semantic_target_digest: u64,
    ) -> Self {
        Self {
            presented_frame,
            presentation_epoch,
            mounted_instance,
            semantic_target_digest,
        }
    }

    pub const fn presented_frame(self) -> u64 {
        self.presented_frame
    }

    pub const fn presentation_epoch(self) -> u64 {
        self.presentation_epoch
    }

    pub const fn mounted_instance(self) -> u64 {
        self.mounted_instance
    }

    pub const fn semantic_target_digest(self) -> u64 {
        self.semantic_target_digest
    }
}

impl UiIntentInteractionEvidence {
    #[doc(hidden)]
    pub const fn from_retained_input(
        reference: UiIntentEvidenceReference,
        input: UiIntentInteractionEvidenceInput,
    ) -> Self {
        Self { reference, input }
    }

    pub const fn reference(self) -> UiIntentEvidenceReference {
        self.reference
    }

    pub const fn input(self) -> UiIntentInteractionEvidenceInput {
        self.input
    }
}
