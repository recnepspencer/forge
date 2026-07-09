use std::marker::PhantomData;

use crate::{
    registration::WorthServerSurfaceRegistry, surfaces::WorthServerSurfaceCapabilities,
    WorthServerSurfaceFamily,
};

pub trait WorthServerSurfaceRoot {
    fn family(&self) -> WorthServerSurfaceFamily;

    fn capabilities(&self) -> &WorthServerSurfaceCapabilities;
}

pub trait WorthServerSurfaceFamilyMarker {
    const FAMILY: WorthServerSurfaceFamily;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthServerTypedSurfaceRoot<Marker>
where
    Marker: WorthServerSurfaceFamilyMarker,
{
    capabilities: WorthServerSurfaceCapabilities,
    marker: PhantomData<Marker>,
}

impl<Marker> WorthServerTypedSurfaceRoot<Marker>
where
    Marker: WorthServerSurfaceFamilyMarker,
{
    pub(crate) fn new(surface_registry: &WorthServerSurfaceRegistry) -> Self {
        Self {
            capabilities: surface_registry.capabilities_for(Marker::FAMILY),
            marker: PhantomData,
        }
    }

    pub fn capabilities(&self) -> &WorthServerSurfaceCapabilities {
        &self.capabilities
    }
}

impl<Marker> WorthServerSurfaceRoot for WorthServerTypedSurfaceRoot<Marker>
where
    Marker: WorthServerSurfaceFamilyMarker,
{
    fn family(&self) -> WorthServerSurfaceFamily {
        Marker::FAMILY
    }

    fn capabilities(&self) -> &WorthServerSurfaceCapabilities {
        &self.capabilities
    }
}
