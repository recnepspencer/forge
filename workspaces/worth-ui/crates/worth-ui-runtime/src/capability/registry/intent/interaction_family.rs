/// Initial semantic interaction families accepted by 3.14 intent definitions.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiSemanticInteractionFamily {
    Activate,
    EditCommit,
    SelectionCommit,
    Submit,
}

impl UiSemanticInteractionFamily {
    pub(super) const fn canonical_order(self) -> u8 {
        match self {
            Self::Activate => 0,
            Self::EditCommit => 1,
            Self::SelectionCommit => 2,
            Self::Submit => 3,
        }
    }

    pub(super) const fn digest_basis(self) -> &'static str {
        match self {
            Self::Activate => "activate",
            Self::EditCommit => "edit_commit",
            Self::SelectionCommit => "selection_commit",
            Self::Submit => "submit",
        }
    }
}
