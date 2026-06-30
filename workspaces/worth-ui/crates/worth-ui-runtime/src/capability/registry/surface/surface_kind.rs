/// Domain-agnostic shell meaning for a registered surface.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SurfaceKind {
    PrimaryContent,
    AuxiliaryContent,
    TransientContent,
    ModalContent,
    OverlayContent,
    StatusContent,
    SettingsContent,
    DiagnosticsContent,
    ProductDomainNameForDiagnostics(String),
}

impl SurfaceKind {
    pub fn primary_content() -> Self {
        Self::PrimaryContent
    }

    pub fn auxiliary_content() -> Self {
        Self::AuxiliaryContent
    }

    pub fn transient_content() -> Self {
        Self::TransientContent
    }

    pub fn modal_content() -> Self {
        Self::ModalContent
    }

    pub fn overlay_content() -> Self {
        Self::OverlayContent
    }

    pub fn status_content() -> Self {
        Self::StatusContent
    }

    pub fn settings_content() -> Self {
        Self::SettingsContent
    }

    pub fn diagnostics_content() -> Self {
        Self::DiagnosticsContent
    }

    pub fn product_domain_name_for_diagnostics(name: impl Into<String>) -> Self {
        Self::ProductDomainNameForDiagnostics(name.into())
    }

    pub(crate) fn is_product_domain_name(&self) -> bool {
        matches!(self, Self::ProductDomainNameForDiagnostics(_))
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::PrimaryContent => "primary_content".to_owned(),
            Self::AuxiliaryContent => "auxiliary_content".to_owned(),
            Self::TransientContent => "transient_content".to_owned(),
            Self::ModalContent => "modal_content".to_owned(),
            Self::OverlayContent => "overlay_content".to_owned(),
            Self::StatusContent => "status_content".to_owned(),
            Self::SettingsContent => "settings_content".to_owned(),
            Self::DiagnosticsContent => "diagnostics_content".to_owned(),
            Self::ProductDomainNameForDiagnostics(name) => {
                format!("product_domain_name:{name}")
            }
        }
    }
}
