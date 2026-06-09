use crate::{
    surfaces::{ForgeServerSurfaceFamilyMarker, ForgeServerTypedSurfaceRoot},
    ForgeServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeNativeSurfaceFamilyMarker;

impl ForgeServerSurfaceFamilyMarker for ForgeNativeSurfaceFamilyMarker {
    const FAMILY: ForgeServerSurfaceFamily = ForgeServerSurfaceFamily::ForgeNative;
}

pub type ForgeNativeSurfaceRoot = ForgeServerTypedSurfaceRoot<ForgeNativeSurfaceFamilyMarker>;
