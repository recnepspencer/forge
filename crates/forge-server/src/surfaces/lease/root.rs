use crate::{
    surfaces::{ForgeServerSurfaceFamilyMarker, ForgeServerTypedSurfaceRoot},
    ForgeServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeaseSurfaceFamilyMarker;

impl ForgeServerSurfaceFamilyMarker for LeaseSurfaceFamilyMarker {
    const FAMILY: ForgeServerSurfaceFamily = ForgeServerSurfaceFamily::Lease;
}

pub type LeaseSurfaceRoot = ForgeServerTypedSurfaceRoot<LeaseSurfaceFamilyMarker>;
