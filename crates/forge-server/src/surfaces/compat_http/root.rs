use crate::{
    surfaces::{ForgeServerSurfaceFamilyMarker, ForgeServerTypedSurfaceRoot},
    ForgeServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompatHttpSurfaceFamilyMarker;

impl ForgeServerSurfaceFamilyMarker for CompatHttpSurfaceFamilyMarker {
    const FAMILY: ForgeServerSurfaceFamily = ForgeServerSurfaceFamily::CompatHttp;
}

pub type CompatHttpSurfaceRoot = ForgeServerTypedSurfaceRoot<CompatHttpSurfaceFamilyMarker>;
