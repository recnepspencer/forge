#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDeclarationRepetitionPosture {
    NotAdmitted,
}

impl UiDeclarationRepetitionPosture {
    pub const fn is_not_admitted(self) -> bool {
        matches!(self, Self::NotAdmitted)
    }
}
