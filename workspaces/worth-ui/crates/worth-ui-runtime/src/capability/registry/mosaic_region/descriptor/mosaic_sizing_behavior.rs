/// Structural sizing behavior a mosaic region requests from later layout.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicSizingBehavior {
    FillsAvailableSpace,
    ContentDriven,
    ViewportBounded,
    OverlayAnchored,
    MissingForDiagnostics,
}

impl MosaicSizingBehavior {
    pub fn fills_available_space() -> Self {
        Self::FillsAvailableSpace
    }

    pub fn content_driven() -> Self {
        Self::ContentDriven
    }

    pub fn viewport_bounded() -> Self {
        Self::ViewportBounded
    }

    pub fn overlay_anchored() -> Self {
        Self::OverlayAnchored
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::FillsAvailableSpace => "fills_available_space",
            Self::ContentDriven => "content_driven",
            Self::ViewportBounded => "viewport_bounded",
            Self::OverlayAnchored => "overlay_anchored",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
