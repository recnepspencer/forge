//! Phase 5 raster authority contracts owned by text mechanics.
//!
//! These records are inert: they freeze the identity, scale, lane, format,
//! byte-shape, and cost that a future rasterizer must produce. They do not
//! rasterize glyphs, consult fonts, upload GPU data, or own an atlas.

use std::{marker::PhantomData, sync::Arc};

use worth_ui_host_contract::{
    UiQualifiedFontFaceIdentity, UiQualifiedTextLayoutIdentity, UiTextOriginalRange,
    UiTextScaleGeneration,
};

const MAX_RASTER_EDGE: u32 = 16_384;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGlyphRasterLane {
    Ordinary,
    Reconstruction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct UiGlyphRasterScale {
    dpi_milli: u32,
    text_scale: UiTextScaleGeneration,
}

impl UiGlyphRasterScale {
    pub const fn new(dpi_milli: u32, text_scale: UiTextScaleGeneration) -> Option<Self> {
        if dpi_milli == 0 {
            None
        } else {
            Some(Self {
                dpi_milli,
                text_scale,
            })
        }
    }

    pub const fn dpi_milli(self) -> u32 {
        self.dpi_milli
    }

    pub const fn text_scale_generation(self) -> UiTextScaleGeneration {
        self.text_scale
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGlyphRasterExtent {
    width: u32,
    height: u32,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGlyphRasterAdmissionDenial {
    ByteLengthOverflow,
    ByteLengthMismatch { expected: usize, actual: usize },
    ForeignLayout,
}

pub struct UiAlphaRasterKind(PhantomData<()>);
pub struct UiColorRasterKind(PhantomData<()>);

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::UiAlphaRasterKind {}
    impl Sealed for super::UiColorRasterKind {}
}

pub trait UiGlyphRasterFormat: sealed::Sealed {
    const CHANNELS: usize;
}

impl UiGlyphRasterFormat for UiAlphaRasterKind {
    const CHANNELS: usize = 1;
}

impl UiGlyphRasterFormat for UiColorRasterKind {
    const CHANNELS: usize = 4;
}

pub struct UiGlyphRasterRecord<Kind> {
    layout: UiQualifiedTextLayoutIdentity,
    face: UiQualifiedFontFaceIdentity,
    glyph_id: u32,
    cluster: UiTextOriginalRange,
    extent: UiGlyphRasterExtent,
    pixels: Arc<[u8]>,
    _format: PhantomData<Kind>,
}

struct UiGlyphRasterRecordInput {
    layout: UiQualifiedTextLayoutIdentity,
    face: UiQualifiedFontFaceIdentity,
    glyph_id: u32,
    cluster: UiTextOriginalRange,
    extent: UiGlyphRasterExtent,
    pixels: Arc<[u8]>,
}

impl<Kind: UiGlyphRasterFormat> UiGlyphRasterRecord<Kind> {
    fn from_text_mechanics(
        input: UiGlyphRasterRecordInput,
    ) -> Result<Self, UiGlyphRasterAdmissionDenial> {
        let expected = usize::try_from(input.extent.width)
            .ok()
            .and_then(|width| {
                usize::try_from(input.extent.height)
                    .ok()
                    .and_then(|height| width.checked_mul(height))
            })
            .and_then(|pixels| pixels.checked_mul(Kind::CHANNELS))
            .ok_or(UiGlyphRasterAdmissionDenial::ByteLengthOverflow)?;
        if input.pixels.len() != expected {
            return Err(UiGlyphRasterAdmissionDenial::ByteLengthMismatch {
                expected,
                actual: input.pixels.len(),
            });
        }
        Ok(Self {
            layout: input.layout,
            face: input.face,
            glyph_id: input.glyph_id,
            cluster: input.cluster,
            extent: input.extent,
            pixels: input.pixels,
            _format: PhantomData,
        })
    }

    pub const fn layout_identity(&self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }
    pub const fn face_identity(&self) -> UiQualifiedFontFaceIdentity {
        self.face
    }
    pub const fn glyph_id(&self) -> u32 {
        self.glyph_id
    }
    pub const fn cluster(&self) -> UiTextOriginalRange {
        self.cluster
    }
    pub const fn extent(&self) -> UiGlyphRasterExtent {
        self.extent
    }
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

pub struct UiGlyphRasterBatch<Kind> {
    layout: UiQualifiedTextLayoutIdentity,
    scale: UiGlyphRasterScale,
    lane: UiGlyphRasterLane,
    records: Box<[UiGlyphRasterRecord<Kind>]>,
}

impl<Kind: UiGlyphRasterFormat> UiGlyphRasterBatch<Kind> {
    pub(crate) fn from_text_mechanics(
        layout: UiQualifiedTextLayoutIdentity,
        scale: UiGlyphRasterScale,
        lane: UiGlyphRasterLane,
        records: impl IntoIterator<Item = UiGlyphRasterRecord<Kind>>,
    ) -> Result<Self, UiGlyphRasterAdmissionDenial> {
        let records: Box<[_]> = records.into_iter().collect();
        if records.iter().any(|record| record.layout != layout) {
            return Err(UiGlyphRasterAdmissionDenial::ForeignLayout);
        }
        Ok(Self {
            layout,
            scale,
            lane,
            records,
        })
    }

    pub const fn layout_identity(&self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }
    pub const fn scale(&self) -> UiGlyphRasterScale {
        self.scale
    }
    pub const fn lane(&self) -> UiGlyphRasterLane {
        self.lane
    }
    pub fn records(&self) -> &[UiGlyphRasterRecord<Kind>] {
        &self.records
    }
}

pub type UiAlphaRasterBatch = UiGlyphRasterBatch<UiAlphaRasterKind>;
pub type UiColorRasterBatch = UiGlyphRasterBatch<UiColorRasterKind>;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiGlyphRasterLaneCost {
    requested_glyphs: u32,
    cache_hits: u32,
    rasterized_glyphs: u32,
    rasterized_pixels: u64,
    produced_bytes: u64,
}

struct UiGlyphRasterLaneCostInput {
    requested_glyphs: u32,
    cache_hits: u32,
    rasterized_glyphs: u32,
    rasterized_pixels: u64,
    produced_bytes: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiGlyphRasterCost {
    ordinary: UiGlyphRasterLaneCost,
    reconstructive: UiGlyphRasterLaneCost,
}

impl UiGlyphRasterCost {
    pub(crate) const fn from_text_mechanics(
        ordinary: UiGlyphRasterLaneCost,
        reconstructive: UiGlyphRasterLaneCost,
    ) -> Self {
        Self {
            ordinary,
            reconstructive,
        }
    }

    pub const fn ordinary(self) -> UiGlyphRasterLaneCost {
        self.ordinary
    }
    pub const fn reconstructive(self) -> UiGlyphRasterLaneCost {
        self.reconstructive
    }
}

impl UiGlyphRasterLaneCost {
    const fn from_text_mechanics(input: UiGlyphRasterLaneCostInput) -> Self {
        Self {
            requested_glyphs: input.requested_glyphs,
            cache_hits: input.cache_hits,
            rasterized_glyphs: input.rasterized_glyphs,
            rasterized_pixels: input.rasterized_pixels,
            produced_bytes: input.produced_bytes,
        }
    }

    pub const fn requested_glyphs(self) -> u32 {
        self.requested_glyphs
    }
    pub const fn cache_hits(self) -> u32 {
        self.cache_hits
    }
    pub const fn rasterized_glyphs(self) -> u32 {
        self.rasterized_glyphs
    }
    pub const fn rasterized_pixels(self) -> u64 {
        self.rasterized_pixels
    }
    pub const fn produced_bytes(self) -> u64 {
        self.produced_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_and_color_records_enforce_distinct_byte_shapes() {
        let layout = UiQualifiedTextLayoutIdentity::from_text_mechanics([1; 32]);
        let face = UiQualifiedFontFaceIdentity::from_text_mechanics([2; 32], 0);
        let cluster = UiTextOriginalRange::new(0, 4).unwrap();
        let extent = UiGlyphRasterExtent::new(2, 2).unwrap();
        let input = || UiGlyphRasterRecordInput {
            layout,
            face,
            glyph_id: 7,
            cluster,
            extent,
            pixels: Arc::from([255; 4]),
        };
        let alpha = UiGlyphRasterRecord::<UiAlphaRasterKind>::from_text_mechanics(input());
        let color = UiGlyphRasterRecord::<UiColorRasterKind>::from_text_mechanics(input());

        let batch = UiGlyphRasterBatch::from_text_mechanics(
            layout,
            UiGlyphRasterScale::new(1_500, UiTextScaleGeneration::new(1).unwrap()).unwrap(),
            UiGlyphRasterLane::Ordinary,
            [alpha.unwrap()],
        )
        .unwrap();
        assert_eq!(batch.records().len(), 1);
        let cost = UiGlyphRasterLaneCost::from_text_mechanics(UiGlyphRasterLaneCostInput {
            requested_glyphs: 3,
            cache_hits: 1,
            rasterized_glyphs: 2,
            rasterized_pixels: 40,
            produced_bytes: 40,
        });
        assert_eq!(cost.rasterized_glyphs(), 2);
        assert_eq!(
            color.err(),
            Some(UiGlyphRasterAdmissionDenial::ByteLengthMismatch {
                expected: 16,
                actual: 4,
            })
        );
    }
}
