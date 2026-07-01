#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclarationSlotParticipationIntent {
    None,
    DeclaredSlotParticipant { slot_name: Box<str> },
}

impl UiDeclarationSlotParticipationIntent {
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn slot_name(&self) -> Option<&str> {
        match self {
            Self::None => None,
            Self::DeclaredSlotParticipant { slot_name } => Some(slot_name),
        }
    }
}
