use std::marker::PhantomData;

use crate::{
    registration::ForgeServerSurfaceRegistry, surfaces::ForgeServerSurfaceCapabilities,
    ForgeServerSurfaceFamily,
};

pub trait ForgeServerSurfaceRoot {
    fn family(&self) -> ForgeServerSurfaceFamily;

    fn capabilities(&self) -> &ForgeServerSurfaceCapabilities;
}

pub trait ForgeServerSurfaceFamilyMarker {
    const FAMILY: ForgeServerSurfaceFamily;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeServerTypedSurfaceRoot<Marker>
where
    Marker: ForgeServerSurfaceFamilyMarker,
{
    capabilities: ForgeServerSurfaceCapabilities,
    marker: PhantomData<Marker>,
}

impl<Marker> ForgeServerTypedSurfaceRoot<Marker>
where
    Marker: ForgeServerSurfaceFamilyMarker,
{
    pub(crate) fn new(surface_registry: &ForgeServerSurfaceRegistry) -> Self {
        Self {
            capabilities: surface_registry.capabilities_for(Marker::FAMILY),
            marker: PhantomData,
        }
    }

    pub fn capabilities(&self) -> &ForgeServerSurfaceCapabilities {
        &self.capabilities
    }
}

impl<Marker> ForgeServerSurfaceRoot for ForgeServerTypedSurfaceRoot<Marker>
where
    Marker: ForgeServerSurfaceFamilyMarker,
{
    fn family(&self) -> ForgeServerSurfaceFamily {
        Marker::FAMILY
    }

    fn capabilities(&self) -> &ForgeServerSurfaceCapabilities {
        &self.capabilities
    }
}
