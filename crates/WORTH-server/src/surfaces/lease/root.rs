use crate::{
    surfaces::{WorthServerSurfaceFamilyMarker, WorthServerTypedSurfaceRoot},
    WorthServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeaseSurfaceFamilyMarker;

impl WorthServerSurfaceFamilyMarker for LeaseSurfaceFamilyMarker {
    const FAMILY: WorthServerSurfaceFamily = WorthServerSurfaceFamily::Lease;
}

pub type LeaseSurfaceRoot = WorthServerTypedSurfaceRoot<LeaseSurfaceFamilyMarker>;
