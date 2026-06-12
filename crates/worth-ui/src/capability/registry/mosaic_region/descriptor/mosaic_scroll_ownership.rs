/// Authority that owns scroll state for a region.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicScrollOwnership {
    RegionOwned,
    SurfaceOwned,
    ViewportOwned,
    NoScrolling,
    MissingForDiagnostics,
}

impl MosaicScrollOwnership {
    pub fn region_owned() -> Self {
        Self::RegionOwned
    }

    pub fn surface_owned() -> Self {
        Self::SurfaceOwned
    }

    pub fn viewport_owned() -> Self {
        Self::ViewportOwned
    }

    pub fn no_scrolling() -> Self {
        Self::NoScrolling
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::RegionOwned => "region_owned",
            Self::SurfaceOwned => "surface_owned",
            Self::ViewportOwned => "viewport_owned",
            Self::NoScrolling => "no_scrolling",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
