use crate::{
    surfaces::{WorthServerSurfaceFamilyMarker, WorthServerTypedSurfaceRoot},
    WorthServerSurfaceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BinarySurfaceFamilyMarker;

impl WorthServerSurfaceFamilyMarker for BinarySurfaceFamilyMarker {
    const FAMILY: WorthServerSurfaceFamily = WorthServerSurfaceFamily::Binary;
}

pub type BinarySurfaceRoot = WorthServerTypedSurfaceRoot<BinarySurfaceFamilyMarker>;
