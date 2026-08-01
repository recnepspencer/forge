#[derive(Debug)]
pub enum UiSemanticInteraction {
    Activate(super::UiActivateInteraction),
    EditCommit(super::UiEditCommitInteraction),
    SelectionCommit(super::UiSelectionCommitInteraction),
    Submit(super::UiSubmitInteraction),
}

impl UiSemanticInteraction {
    pub const fn family(&self) -> crate::capability::UiSemanticInteractionFamily {
        match self {
            Self::Activate(_) => crate::capability::UiSemanticInteractionFamily::Activate,
            Self::EditCommit(_) => crate::capability::UiSemanticInteractionFamily::EditCommit,
            Self::SelectionCommit(_) => {
                crate::capability::UiSemanticInteractionFamily::SelectionCommit
            }
            Self::Submit(_) => crate::capability::UiSemanticInteractionFamily::Submit,
        }
    }

    pub const fn target(&self) -> super::super::UiPresentedInteractionTargetView {
        match self {
            Self::Activate(interaction) => interaction.target(),
            Self::EditCommit(interaction) => interaction.target(),
            Self::SelectionCommit(interaction) => interaction.target(),
            Self::Submit(interaction) => interaction.target(),
        }
    }

    pub const fn generation(&self) -> &crate::runtime::WorthUiActiveApplicationGenerationIdentity {
        match self {
            Self::Activate(interaction) => interaction.generation(),
            Self::EditCommit(interaction) => interaction.generation(),
            Self::SelectionCommit(interaction) => interaction.generation(),
            Self::Submit(interaction) => interaction.generation(),
        }
    }

    pub const fn time_basis(&self) -> worth_ui_host_contract::UiHostObservationTimeBasis {
        match self {
            Self::Activate(interaction) => interaction.time_basis(),
            Self::EditCommit(interaction) => interaction.time_basis(),
            Self::SelectionCommit(interaction) => interaction.activation().time_basis(),
            Self::Submit(interaction) => interaction.time_basis(),
        }
    }
}
