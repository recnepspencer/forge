use crate::{
    surfaces::{ForgeServerSurfaceFamilyMarker, ForgeServerTypedSurfaceRoot},
    ForgeServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegrationSurfaceFamilyMarker;

impl ForgeServerSurfaceFamilyMarker for IntegrationSurfaceFamilyMarker {
    const FAMILY: ForgeServerSurfaceFamily = ForgeServerSurfaceFamily::Integration;
}

pub type IntegrationSurfaceRoot = ForgeServerTypedSurfaceRoot<IntegrationSurfaceFamilyMarker>;
