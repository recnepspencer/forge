/// Runtime-mediated action a mosaic placement policy may authorize.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicPlacementAction {
    Dock,
    Tab,
    Split,
    Pin,
    Collapse,
    Overlay,
    Float,
    Modal,
    StatusProjection,
    ToolbarProjection,
    ImperativeMutationForDiagnostics,
}

impl MosaicPlacementAction {
    pub fn dock() -> Self {
        Self::Dock
    }

    pub fn tab() -> Self {
        Self::Tab
    }

    pub fn split() -> Self {
        Self::Split
    }

    pub fn pin() -> Self {
        Self::Pin
    }

    pub fn collapse() -> Self {
        Self::Collapse
    }

    pub fn overlay() -> Self {
        Self::Overlay
    }

    pub fn float() -> Self {
        Self::Float
    }

    pub fn modal() -> Self {
        Self::Modal
    }

    pub fn status_projection() -> Self {
        Self::StatusProjection
    }

    pub fn toolbar_projection() -> Self {
        Self::ToolbarProjection
    }

    pub fn imperative_mutation_for_diagnostics() -> Self {
        Self::ImperativeMutationForDiagnostics
    }

    pub(crate) fn is_imperative_mutation(&self) -> bool {
        matches!(self, Self::ImperativeMutationForDiagnostics)
    }

    pub(crate) fn requires_float_or_overlay_support(&self) -> bool {
        matches!(self, Self::Overlay | Self::Float | Self::Modal)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Dock => "dock",
            Self::Tab => "tab",
            Self::Split => "split",
            Self::Pin => "pin",
            Self::Collapse => "collapse",
            Self::Overlay => "overlay",
            Self::Float => "float",
            Self::Modal => "modal",
            Self::StatusProjection => "status_projection",
            Self::ToolbarProjection => "toolbar_projection",
            Self::ImperativeMutationForDiagnostics => "imperative_mutation",
        }
    }
}

/// Runtime support posture for placement modes that need explicit platform mediation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicPlacementSupport {
    Supported,
    UnsupportedFloatOrOverlayForDiagnostics,
}

impl MosaicPlacementSupport {
    pub fn supported() -> Self {
        Self::Supported
    }

    pub fn unsupported_float_or_overlay_for_diagnostics() -> Self {
        Self::UnsupportedFloatOrOverlayForDiagnostics
    }

    pub(crate) fn supports_float_or_overlay(&self) -> bool {
        matches!(self, Self::Supported)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::UnsupportedFloatOrOverlayForDiagnostics => "unsupported_float_or_overlay",
        }
    }
}
