/// Clipping posture for a mosaic region kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicClippingPosture {
    ClipToRegion,
    AllowOverlayEscape,
    ViewportClipped,
    MissingForDiagnostics,
}

impl MosaicClippingPosture {
    pub fn clip_to_region() -> Self {
        Self::ClipToRegion
    }

    pub fn allow_overlay_escape() -> Self {
        Self::AllowOverlayEscape
    }

    pub fn viewport_clipped() -> Self {
        Self::ViewportClipped
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::ClipToRegion => "clip_to_region",
            Self::AllowOverlayEscape => "allow_overlay_escape",
            Self::ViewportClipped => "viewport_clipped",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
