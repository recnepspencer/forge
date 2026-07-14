use crate::{
    surfaces::{WorthServerSurfaceFamilyMarker, WorthServerTypedSurfaceRoot},
    WorthServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyncSurfaceFamilyMarker;

impl WorthServerSurfaceFamilyMarker for SyncSurfaceFamilyMarker {
    const FAMILY: WorthServerSurfaceFamily = WorthServerSurfaceFamily::Sync;
}

pub type SyncSurfaceRoot = WorthServerTypedSurfaceRoot<SyncSurfaceFamilyMarker>;
