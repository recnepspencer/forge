#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiDeclarationPlanningOperatorKind {
    PageRoot,
    PageSet,
    Region,
    Mosaic,
    LocalComposition,
    Control,
    DiagnosticSurface,
    Stack,
    Row,
    Grid,
    Split,
    Overlay,
    Scroll,
    PortalAnchor,
}

impl UiDeclarationPlanningOperatorKind {
    pub(crate) fn admit_explicit_claim(claim_name: &str) -> Option<Self> {
        Some(match claim_name.trim().to_ascii_lowercase().as_str() {
            "stack" => Self::Stack,
            "row" => Self::Row,
            "grid" => Self::Grid,
            "split" => Self::Split,
            "overlay" => Self::Overlay,
            "scroll" => Self::Scroll,
            "portal-anchor" | "portal_anchor" => Self::PortalAnchor,
            _ => return None,
        })
    }
}
