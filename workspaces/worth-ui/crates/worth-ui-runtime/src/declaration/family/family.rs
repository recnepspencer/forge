use crate::declaration::family::contracts::{
    UiControlDeclarationFamily, UiDiagnosticSurfaceDeclarationFamily, UiIntentDeclarationFamily,
    UiLocalCompositionDeclarationFamily, UiMosaicDeclarationFamily, UiPageDeclarationFamily,
    UiPageSetDeclarationFamily, UiQueryBindingDeclarationFamily, UiRegionDeclarationFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UiDeclarationFamilyKind {
    Page,
    PageSet,
    Region,
    Mosaic,
    LocalComposition,
    Control,
    QueryBinding,
    Intent,
    DiagnosticSurface,
}

impl UiDeclarationFamilyKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Page => "page",
            Self::PageSet => "page-set",
            Self::Region => "region",
            Self::Mosaic => "mosaic",
            Self::LocalComposition => "local-composition",
            Self::Control => "control",
            Self::QueryBinding => "query-binding",
            Self::Intent => "intent",
            Self::DiagnosticSurface => "diagnostic-surface",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclarationFamily {
    Page(UiPageDeclarationFamily),
    PageSet(UiPageSetDeclarationFamily),
    Region(UiRegionDeclarationFamily),
    Mosaic(UiMosaicDeclarationFamily),
    LocalComposition(UiLocalCompositionDeclarationFamily),
    Control(UiControlDeclarationFamily),
    QueryBinding(UiQueryBindingDeclarationFamily),
    Intent(UiIntentDeclarationFamily),
    DiagnosticSurface(UiDiagnosticSurfaceDeclarationFamily),
}

impl UiDeclarationFamily {
    pub const fn kind(&self) -> UiDeclarationFamilyKind {
        match self {
            Self::Page(_) => UiDeclarationFamilyKind::Page,
            Self::PageSet(_) => UiDeclarationFamilyKind::PageSet,
            Self::Region(_) => UiDeclarationFamilyKind::Region,
            Self::Mosaic(_) => UiDeclarationFamilyKind::Mosaic,
            Self::LocalComposition(_) => UiDeclarationFamilyKind::LocalComposition,
            Self::Control(_) => UiDeclarationFamilyKind::Control,
            Self::QueryBinding(_) => UiDeclarationFamilyKind::QueryBinding,
            Self::Intent(_) => UiDeclarationFamilyKind::Intent,
            Self::DiagnosticSurface(_) => UiDeclarationFamilyKind::DiagnosticSurface,
        }
    }
}
