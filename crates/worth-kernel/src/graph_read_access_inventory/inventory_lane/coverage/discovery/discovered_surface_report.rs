use super::WorthGraphReadAccessDiscoveredSurface;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadAccessDiscoveredSurfaceReport {
    surfaces: Vec<WorthGraphReadAccessDiscoveredSurface>,
}

impl WorthGraphReadAccessDiscoveredSurfaceReport {
    pub(crate) fn new(surfaces: Vec<WorthGraphReadAccessDiscoveredSurface>) -> Self {
        Self { surfaces }
    }

    pub(crate) fn surfaces(&self) -> &[WorthGraphReadAccessDiscoveredSurface] {
        &self.surfaces
    }
}
