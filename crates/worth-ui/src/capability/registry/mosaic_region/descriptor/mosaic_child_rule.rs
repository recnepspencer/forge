/// Structural child admission rule for a mosaic region kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicChildRule {
    AcceptsSurfaces,
    AcceptsRegions,
    AcceptsRegionStack,
    LeafOnly,
    MissingForDiagnostics,
}

impl MosaicChildRule {
    pub fn accepts_surfaces() -> Self {
        Self::AcceptsSurfaces
    }

    pub fn accepts_regions() -> Self {
        Self::AcceptsRegions
    }

    pub fn accepts_region_stack() -> Self {
        Self::AcceptsRegionStack
    }

    pub fn leaf_only() -> Self {
        Self::LeafOnly
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn requires_allowed_surface_class(&self) -> bool {
        matches!(self, Self::AcceptsSurfaces)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::AcceptsSurfaces => "accepts_surfaces",
            Self::AcceptsRegions => "accepts_regions",
            Self::AcceptsRegionStack => "accepts_region_stack",
            Self::LeafOnly => "leaf_only",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
