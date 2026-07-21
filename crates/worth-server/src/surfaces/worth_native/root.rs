use crate::{
    surfaces::{WorthServerSurfaceFamilyMarker, WorthServerTypedSurfaceRoot},
    WorthServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthNativeSurfaceFamilyMarker;

impl WorthServerSurfaceFamilyMarker for WorthNativeSurfaceFamilyMarker {
    const FAMILY: WorthServerSurfaceFamily = WorthServerSurfaceFamily::WorthNative;
}

pub type WorthNativeSurfaceRoot = WorthServerTypedSurfaceRoot<WorthNativeSurfaceFamilyMarker>;
