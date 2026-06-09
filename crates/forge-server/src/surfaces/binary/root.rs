use crate::{
    surfaces::{ForgeServerSurfaceFamilyMarker, ForgeServerTypedSurfaceRoot},
    ForgeServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BinarySurfaceFamilyMarker;

impl ForgeServerSurfaceFamilyMarker for BinarySurfaceFamilyMarker {
    const FAMILY: ForgeServerSurfaceFamily = ForgeServerSurfaceFamily::Binary;
}

pub type BinarySurfaceRoot = ForgeServerTypedSurfaceRoot<BinarySurfaceFamilyMarker>;
