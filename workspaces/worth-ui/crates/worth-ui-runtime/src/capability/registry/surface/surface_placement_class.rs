/// Structural placement class a surface may request from later mosaic lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SurfacePlacementClass {
    PrimaryRegion,
    AuxiliaryRegion,
    TransientLayer,
    ModalLayer,
    OverlayLayer,
    StatusRegion,
    UnsupportedForDiagnostics(String),
}

impl SurfacePlacementClass {
    pub fn primary_region() -> Self {
        Self::PrimaryRegion
    }

    pub fn auxiliary_region() -> Self {
        Self::AuxiliaryRegion
    }

    pub fn transient_layer() -> Self {
        Self::TransientLayer
    }

    pub fn modal_layer() -> Self {
        Self::ModalLayer
    }

    pub fn overlay_layer() -> Self {
        Self::OverlayLayer
    }

    pub fn status_region() -> Self {
        Self::StatusRegion
    }

    pub fn unsupported_for_diagnostics(name: impl Into<String>) -> Self {
        Self::UnsupportedForDiagnostics(name.into())
    }

    pub(crate) fn is_unsupported(&self) -> bool {
        matches!(self, Self::UnsupportedForDiagnostics(_))
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::PrimaryRegion => "primary_region".to_owned(),
            Self::AuxiliaryRegion => "auxiliary_region".to_owned(),
            Self::TransientLayer => "transient_layer".to_owned(),
            Self::ModalLayer => "modal_layer".to_owned(),
            Self::OverlayLayer => "overlay_layer".to_owned(),
            Self::StatusRegion => "status_region".to_owned(),
            Self::UnsupportedForDiagnostics(name) => {
                format!("unsupported:{name}")
            }
        }
    }
}
