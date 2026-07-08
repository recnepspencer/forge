use crate::declaration::UiDeclarationStructuralRole;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiLayoutOperatorFamily {
    Page,
    PageSet,
    Region,
    Mosaic,
    LocalComposition,
    Control,
    DiagnosticSurface,
}

impl UiLayoutOperatorFamily {
    pub(crate) const fn from_structural_role(
        role: UiDeclarationStructuralRole,
    ) -> UiLayoutOperatorFamily {
        match role {
            UiDeclarationStructuralRole::Page => Self::Page,
            UiDeclarationStructuralRole::PageSet => Self::PageSet,
            UiDeclarationStructuralRole::Region => Self::Region,
            UiDeclarationStructuralRole::Mosaic => Self::Mosaic,
            UiDeclarationStructuralRole::LocalComposition => Self::LocalComposition,
            UiDeclarationStructuralRole::Control => Self::Control,
            UiDeclarationStructuralRole::DiagnosticSurface => Self::DiagnosticSurface,
        }
    }
}
