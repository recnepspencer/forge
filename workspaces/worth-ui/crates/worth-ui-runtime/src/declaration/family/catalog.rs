use crate::declaration::family::UiDeclarationFamilyKind;

pub struct UiDeclarationFamilyCatalog {
    _sealed: (),
}

impl UiDeclarationFamilyCatalog {
    pub const fn closed_initial_set() -> &'static [UiDeclarationFamilyKind] {
        &[
            UiDeclarationFamilyKind::Page,
            UiDeclarationFamilyKind::PageSet,
            UiDeclarationFamilyKind::Region,
            UiDeclarationFamilyKind::Mosaic,
            UiDeclarationFamilyKind::LocalComposition,
            UiDeclarationFamilyKind::Control,
            UiDeclarationFamilyKind::QueryBinding,
            UiDeclarationFamilyKind::Intent,
            UiDeclarationFamilyKind::DiagnosticSurface,
        ]
    }
}
