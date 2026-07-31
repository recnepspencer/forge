#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthUiIntentInteractionFamily {
    Activate,
    EditCommit,
    SelectionCommit,
    Submit,
}

impl WorthUiIntentInteractionFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Activate => "activate",
            Self::EditCommit => "edit-commit",
            Self::SelectionCommit => "selection-commit",
            Self::Submit => "submit",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "activate" => Some(Self::Activate),
            "edit-commit" => Some(Self::EditCommit),
            "selection-commit" => Some(Self::SelectionCommit),
            "submit" => Some(Self::Submit),
            _ => None,
        }
    }
}
