#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDeclarationOrderingGuarantee {
    NotSemanticallyClaimed,
}

impl UiDeclarationOrderingGuarantee {
    pub const fn is_not_semantically_claimed(self) -> bool {
        matches!(self, Self::NotSemanticallyClaimed)
    }
}
