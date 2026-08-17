//! Borrowed alpha and intrinsic-color raster-batch transport.
//!
//! Pixel slices are borrowed. Alpha and color records are distinct types so
//! one format cannot be substituted for the other.

use super::{
    UiGlyphRasterAttribution, UiGlyphRasterBatchIdentity, UiGlyphRasterDemandIdentity,
    UiGlyphRasterKey, UiGlyphRasterLane, UiQualifiedTextLayoutIdentity,
};

const MAX_RASTER_EDGE: u32 = 16_384;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGlyphRasterExtent {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGlyphRasterBearing {
    x_over_64: i32,
    y_over_64: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGlyphRasterContentDigest([u8; 32]);

#[derive(Clone, Copy, Debug)]
pub struct UiAlphaRasterRecordView<'pixels> {
    key: UiGlyphRasterKey,
    attribution: UiGlyphRasterAttribution,
    bearing: UiGlyphRasterBearing,
    extent: UiGlyphRasterExtent,
    stride: u32,
    pixels: &'pixels [u8],
    digest: UiGlyphRasterContentDigest,
}

#[derive(Clone, Copy, Debug)]
pub struct UiColorRasterRecordView<'pixels> {
    key: UiGlyphRasterKey,
    attribution: UiGlyphRasterAttribution,
    bearing: UiGlyphRasterBearing,
    extent: UiGlyphRasterExtent,
    stride: u32,
    pixels: &'pixels [u8],
    digest: UiGlyphRasterContentDigest,
}

#[derive(Clone, Copy, Debug)]
pub struct UiAlphaRasterBatchView<'batch, 'pixels> {
    demand: UiGlyphRasterDemandIdentity,
    miss: UiGlyphRasterDemandIdentity,
    batch: UiGlyphRasterBatchIdentity,
    layout: UiQualifiedTextLayoutIdentity,
    lane: UiGlyphRasterLane,
    records: &'batch [UiAlphaRasterRecordView<'pixels>],
}

#[derive(Clone, Copy, Debug)]
pub struct UiColorRasterBatchView<'batch, 'pixels> {
    demand: UiGlyphRasterDemandIdentity,
    miss: UiGlyphRasterDemandIdentity,
    batch: UiGlyphRasterBatchIdentity,
    layout: UiQualifiedTextLayoutIdentity,
    lane: UiGlyphRasterLane,
    records: &'batch [UiColorRasterRecordView<'pixels>],
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct UiGlyphRasterRecordViewInput<'pixels> {
    pub key: UiGlyphRasterKey,
    pub attribution: UiGlyphRasterAttribution,
    pub bearing: UiGlyphRasterBearing,
    pub extent: UiGlyphRasterExtent,
    pub stride: u32,
    pub pixels: &'pixels [u8],
    pub digest: UiGlyphRasterContentDigest,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGlyphRasterViewDenial {
    ByteLengthOverflow,
    ByteLengthMismatch { expected: usize, actual: usize },
    StrideMismatch,
}

impl UiGlyphRasterExtent {
    pub const fn new(width: u32, height: u32) -> Option<Self> {
        if width == 0 || height == 0 || width > MAX_RASTER_EDGE || height > MAX_RASTER_EDGE {
            None
        } else {
            Some(Self { width, height })
        }
    }

    pub const fn width(self) -> u32 {
        self.width
    }

    pub const fn height(self) -> u32 {
        self.height
    }
}

impl UiGlyphRasterBearing {
    pub const fn from_sixty_fourths(x_over_64: i32, y_over_64: i32) -> Self {
        Self {
            x_over_64,
            y_over_64,
        }
    }

    pub const fn x_over_64(self) -> i32 {
        self.x_over_64
    }

    pub const fn y_over_64(self) -> i32 {
        self.y_over_64
    }
}

impl UiGlyphRasterContentDigest {
    #[doc(hidden)]
    pub const fn from_text_mechanics(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

fn expected_bytes(extent: UiGlyphRasterExtent, channels: usize) -> Option<usize> {
    usize::try_from(extent.width)
        .ok()
        .and_then(|width| {
            usize::try_from(extent.height)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .and_then(|pixels| pixels.checked_mul(channels))
}

fn admit_record<'pixels>(
    input: UiGlyphRasterRecordViewInput<'pixels>,
    channels: usize,
) -> Result<(), UiGlyphRasterViewDenial> {
    let expected = expected_bytes(input.extent, channels)
        .ok_or(UiGlyphRasterViewDenial::ByteLengthOverflow)?;
    if input.pixels.len() != expected {
        return Err(UiGlyphRasterViewDenial::ByteLengthMismatch {
            expected,
            actual: input.pixels.len(),
        });
    }
    let expected_stride = usize::try_from(input.extent.width)
        .ok()
        .and_then(|width| width.checked_mul(channels))
        .ok_or(UiGlyphRasterViewDenial::ByteLengthOverflow)?;
    if usize::try_from(input.stride).ok() != Some(expected_stride) {
        return Err(UiGlyphRasterViewDenial::StrideMismatch);
    }
    Ok(())
}

impl<'pixels> UiAlphaRasterRecordView<'pixels> {
    #[doc(hidden)]
    pub fn from_text_mechanics(
        input: UiGlyphRasterRecordViewInput<'pixels>,
    ) -> Result<Self, UiGlyphRasterViewDenial> {
        admit_record(input, 1)?;
        Ok(Self {
            key: input.key,
            attribution: input.attribution,
            bearing: input.bearing,
            extent: input.extent,
            stride: input.stride,
            pixels: input.pixels,
            digest: input.digest,
        })
    }

    pub const fn key(self) -> UiGlyphRasterKey {
        self.key
    }
    pub const fn attribution(self) -> UiGlyphRasterAttribution {
        self.attribution
    }
    pub const fn bearing(self) -> UiGlyphRasterBearing {
        self.bearing
    }
    pub const fn extent(self) -> UiGlyphRasterExtent {
        self.extent
    }
    pub const fn stride(self) -> u32 {
        self.stride
    }
    pub const fn pixels(self) -> &'pixels [u8] {
        self.pixels
    }
    pub const fn digest(self) -> UiGlyphRasterContentDigest {
        self.digest
    }
}

impl<'pixels> UiColorRasterRecordView<'pixels> {
    #[doc(hidden)]
    pub fn from_text_mechanics(
        input: UiGlyphRasterRecordViewInput<'pixels>,
    ) -> Result<Self, UiGlyphRasterViewDenial> {
        admit_record(input, 4)?;
        Ok(Self {
            key: input.key,
            attribution: input.attribution,
            bearing: input.bearing,
            extent: input.extent,
            stride: input.stride,
            pixels: input.pixels,
            digest: input.digest,
        })
    }

    pub const fn key(self) -> UiGlyphRasterKey {
        self.key
    }
    pub const fn attribution(self) -> UiGlyphRasterAttribution {
        self.attribution
    }
    pub const fn bearing(self) -> UiGlyphRasterBearing {
        self.bearing
    }
    pub const fn extent(self) -> UiGlyphRasterExtent {
        self.extent
    }
    pub const fn stride(self) -> u32 {
        self.stride
    }
    pub const fn pixels(self) -> &'pixels [u8] {
        self.pixels
    }
    pub const fn digest(self) -> UiGlyphRasterContentDigest {
        self.digest
    }
}

impl<'batch, 'pixels> UiAlphaRasterBatchView<'batch, 'pixels> {
    #[doc(hidden)]
    pub const fn from_text_mechanics(
        demand: UiGlyphRasterDemandIdentity,
        miss: UiGlyphRasterDemandIdentity,
        batch: UiGlyphRasterBatchIdentity,
        layout: UiQualifiedTextLayoutIdentity,
        lane: UiGlyphRasterLane,
        records: &'batch [UiAlphaRasterRecordView<'pixels>],
    ) -> Self {
        Self {
            demand,
            miss,
            batch,
            layout,
            lane,
            records,
        }
    }

    pub const fn demand_identity(self) -> UiGlyphRasterDemandIdentity {
        self.demand
    }
    pub const fn miss_identity(self) -> UiGlyphRasterDemandIdentity {
        self.miss
    }
    pub const fn batch_identity(self) -> UiGlyphRasterBatchIdentity {
        self.batch
    }

    pub const fn layout_identity(self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }
    pub const fn lane(self) -> UiGlyphRasterLane {
        self.lane
    }
    pub const fn records(self) -> &'batch [UiAlphaRasterRecordView<'pixels>] {
        self.records
    }
}

impl<'batch, 'pixels> UiColorRasterBatchView<'batch, 'pixels> {
    #[doc(hidden)]
    pub const fn from_text_mechanics(
        demand: UiGlyphRasterDemandIdentity,
        miss: UiGlyphRasterDemandIdentity,
        batch: UiGlyphRasterBatchIdentity,
        layout: UiQualifiedTextLayoutIdentity,
        lane: UiGlyphRasterLane,
        records: &'batch [UiColorRasterRecordView<'pixels>],
    ) -> Self {
        Self {
            demand,
            miss,
            batch,
            layout,
            lane,
            records,
        }
    }

    pub const fn demand_identity(self) -> UiGlyphRasterDemandIdentity {
        self.demand
    }
    pub const fn miss_identity(self) -> UiGlyphRasterDemandIdentity {
        self.miss
    }
    pub const fn batch_identity(self) -> UiGlyphRasterBatchIdentity {
        self.batch
    }

    pub const fn layout_identity(self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }
    pub const fn lane(self) -> UiGlyphRasterLane {
        self.lane
    }
    pub const fn records(self) -> &'batch [UiColorRasterRecordView<'pixels>] {
        self.records
    }
}

#[cfg(test)]
#[path = "raster_batch_view_tests.rs"]
mod tests;
