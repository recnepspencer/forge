use worth_ui_inspection::{
    UiIntentInteractionEvidenceFamily, UiIntentInteractionEvidenceInput,
    UiIntentInteractionEvidenceTargetInput,
};

use super::{UiSelectionCommitInteraction, UiSemanticInteraction};
use crate::runtime::interaction::UiPresentedInteractionTargetView;

pub(crate) fn semantic_evidence_input(
    interaction: &UiSemanticInteraction,
) -> UiIntentInteractionEvidenceInput {
    let (family, sequence) = match interaction {
        UiSemanticInteraction::Activate(interaction) => (
            UiIntentInteractionEvidenceFamily::Activate,
            interaction.source_sequence().value(),
        ),
        UiSemanticInteraction::EditCommit(interaction) => (
            UiIntentInteractionEvidenceFamily::EditCommit,
            interaction.source_sequence().value(),
        ),
        UiSemanticInteraction::SelectionCommit(interaction) => (
            UiIntentInteractionEvidenceFamily::SelectionCommit,
            interaction.activation().source_sequence().value(),
        ),
        UiSemanticInteraction::Submit(interaction) => (
            UiIntentInteractionEvidenceFamily::Submit,
            interaction.sequence().value(),
        ),
    };
    evidence_input(family, sequence, interaction.target())
}

pub(crate) fn selection_evidence_input(
    interaction: &UiSelectionCommitInteraction,
) -> UiIntentInteractionEvidenceInput {
    evidence_input(
        UiIntentInteractionEvidenceFamily::SelectionCommit,
        interaction.activation().source_sequence().value(),
        interaction.target(),
    )
}

fn evidence_input(
    family: UiIntentInteractionEvidenceFamily,
    source_sequence: u64,
    target: UiPresentedInteractionTargetView,
) -> UiIntentInteractionEvidenceInput {
    let presentation = target.presentation();
    let target = UiIntentInteractionEvidenceTargetInput::from_diagnostic_parts(
        presentation.frame().diagnostic_value(),
        presentation.epoch().diagnostic_value(),
        target.mounted_instance().diagnostic_value(),
        target.semantic_digest(),
    );
    UiIntentInteractionEvidenceInput::from_diagnostic_parts(source_sequence, target, family)
}
