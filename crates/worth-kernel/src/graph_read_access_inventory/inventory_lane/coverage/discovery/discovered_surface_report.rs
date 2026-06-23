use super::WorthGraphReadAccessDiscoveredSurface;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthGraphReadAccessDiscoveredSurfaceReport {
    surfaces: Vec<WorthGraphReadAccessDiscoveredSurface>,
}

impl WorthGraphReadAccessDiscoveredSurfaceReport {
    pub(super) fn new(surfaces: Vec<WorthGraphReadAccessDiscoveredSurface>) -> Self {
        Self { surfaces }
    }

    pub(super) fn surfaces(&self) -> &[WorthGraphReadAccessDiscoveredSurface] {
        &self.surfaces
    }

    pub(super) const fn discovered_surface_count(&self) -> usize {
        self.surfaces.len()
    }
}
