use crate::{
    surfaces::{WorthServerSurfaceFamilyMarker, WorthServerTypedSurfaceRoot},
    WorthServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegrationSurfaceFamilyMarker;

impl WorthServerSurfaceFamilyMarker for IntegrationSurfaceFamilyMarker {
    const FAMILY: WorthServerSurfaceFamily = WorthServerSurfaceFamily::Integration;
}

pub type IntegrationSurfaceRoot = WorthServerTypedSurfaceRoot<IntegrationSurfaceFamilyMarker>;
