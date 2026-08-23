//! Typed denials shared by alpha and intrinsic-color raster production.

use super::batch::UiGlyphRasterAdmissionDenial;
use super::demand::UiGlyphRasterDemandDenial;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGlyphRasterizationDenial {
    Demand(UiGlyphRasterDemandDenial),
    ForeignLayout,
    ForeignScale,
    ForeignCollectionLineage,
    ForeignProfile,
    MissingProvenance,
    ForeignDemandRecord,
    UnsupportedColorSource,
    InvalidFaceResource,
    InvalidGlyphId,
    InvalidColorPalette,
    OutlineUnavailable,
    BitmapUnavailable,
    UnsupportedBitmapFormat,
    InvalidColorPixels,
    EmptyRaster,
    ExtentExceeded,
    BatchCapacityExceeded,
    StagedByteCapacityExceeded,
    TransactionOutputMismatch,
    Record(UiGlyphRasterAdmissionDenial),
}

impl From<UiGlyphRasterDemandDenial> for UiGlyphRasterizationDenial {
    fn from(denial: UiGlyphRasterDemandDenial) -> Self {
        Self::Demand(denial)
    }
}
