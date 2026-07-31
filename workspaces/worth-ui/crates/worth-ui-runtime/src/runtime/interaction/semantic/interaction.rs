#[derive(Debug)]
pub enum UiSemanticInteraction {
    Activate(super::UiActivateInteraction),
    EditCommit(super::UiEditCommitInteraction),
    Submit(super::UiSubmitInteraction),
}

impl UiSemanticInteraction {
    pub const fn family(&self) -> crate::capability::UiSemanticInteractionFamily {
        match self {
            Self::Activate(_) => crate::capability::UiSemanticInteractionFamily::Activate,
            Self::EditCommit(_) => crate::capability::UiSemanticInteractionFamily::EditCommit,
            Self::Submit(_) => crate::capability::UiSemanticInteractionFamily::Submit,
        }
    }

    pub const fn target(&self) -> super::super::UiPresentedInteractionTargetView {
        match self {
            Self::Activate(interaction) => interaction.target(),
            Self::EditCommit(interaction) => interaction.target(),
            Self::Submit(interaction) => interaction.target(),
        }
    }
}
