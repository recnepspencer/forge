/// Domain-agnostic structural role for a mosaic-owned region kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicRegionRole {
    Primary,
    Auxiliary,
    Side,
    Bottom,
    Status,
    Toolbar,
    Stack,
    Split,
    Overlay,
    Modal,
    Floating,
    Viewport,
    ProductDomainNameForDiagnostics(String),
}

impl MosaicRegionRole {
    pub fn primary() -> Self {
        Self::Primary
    }

    pub fn auxiliary() -> Self {
        Self::Auxiliary
    }

    pub fn side() -> Self {
        Self::Side
    }

    pub fn bottom() -> Self {
        Self::Bottom
    }

    pub fn status() -> Self {
        Self::Status
    }

    pub fn toolbar() -> Self {
        Self::Toolbar
    }

    pub fn stack() -> Self {
        Self::Stack
    }

    pub fn split() -> Self {
        Self::Split
    }

    pub fn overlay() -> Self {
        Self::Overlay
    }

    pub fn modal() -> Self {
        Self::Modal
    }

    pub fn floating() -> Self {
        Self::Floating
    }

    pub fn viewport() -> Self {
        Self::Viewport
    }

    pub fn product_domain_name_for_diagnostics(name: impl Into<String>) -> Self {
        Self::ProductDomainNameForDiagnostics(name.into())
    }

    pub(crate) fn is_product_domain_name(&self) -> bool {
        matches!(self, Self::ProductDomainNameForDiagnostics(_))
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Primary => "primary".to_owned(),
            Self::Auxiliary => "auxiliary".to_owned(),
            Self::Side => "side".to_owned(),
            Self::Bottom => "bottom".to_owned(),
            Self::Status => "status".to_owned(),
            Self::Toolbar => "toolbar".to_owned(),
            Self::Stack => "stack".to_owned(),
            Self::Split => "split".to_owned(),
            Self::Overlay => "overlay".to_owned(),
            Self::Modal => "modal".to_owned(),
            Self::Floating => "floating".to_owned(),
            Self::Viewport => "viewport".to_owned(),
            Self::ProductDomainNameForDiagnostics(name) => {
                format!("product_domain_name:{name}")
            }
        }
    }
}
