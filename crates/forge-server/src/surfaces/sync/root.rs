use crate::{
    surfaces::{ForgeServerSurfaceFamilyMarker, ForgeServerTypedSurfaceRoot},
    ForgeServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyncSurfaceFamilyMarker;

impl ForgeServerSurfaceFamilyMarker for SyncSurfaceFamilyMarker {
    const FAMILY: ForgeServerSurfaceFamily = ForgeServerSurfaceFamily::Sync;
}

pub type SyncSurfaceRoot = ForgeServerTypedSurfaceRoot<SyncSurfaceFamilyMarker>;
