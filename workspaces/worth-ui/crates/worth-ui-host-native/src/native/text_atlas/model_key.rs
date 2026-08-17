//! Independent canonical key and demand projection for atlas model tests.

use worth_ui_host_contract::{UiGlyphRasterKey, UiGlyphRasterSource};

const CANONICAL_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(super) enum ModelSource {
    Alpha,
    Color,
}

impl ModelSource {
    pub(super) const fn channels(self) -> u64 {
        match self {
            Self::Alpha => 1,
            Self::Color => 4,
        }
    }

    fn from_native(source: UiGlyphRasterSource) -> Self {
        match source {
            UiGlyphRasterSource::ColorOutline | UiGlyphRasterSource::ColorBitmap => Self::Color,
            UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort => Self::Alpha,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(super) struct ModelKey {
    source: ModelSource,
    glyph: u32,
    canonical: [u8; CANONICAL_BYTES],
    canonical_len: u16,
}

impl ModelKey {
    pub(super) fn synthetic(source: ModelSource, glyph: u32) -> Self {
        let mut builder = CanonicalBuilder::new();
        builder.push_u64(1);
        builder.push(&[0; 32]);
        builder.push_u64(1);
        builder.push(&[0; 32]);
        builder.push_u32(0);
        builder.push(&[0; 32]);
        builder.push_u32(glyph);
        builder.push_u8(0);
        builder.push_u16(0);
        builder.push_u32(1);
        builder.push_u8(model_source_byte(source));
        builder.push_u32(1_000);
        builder.push_i16(0);
        builder.push_i16(0);
        builder.finish(source, glyph)
    }

    pub(super) fn from_native(key: UiGlyphRasterKey) -> Self {
        let mut builder = CanonicalBuilder::new();
        builder.push_u64(key.font_collection_generation().get());
        builder.push(&key.font_collection_lineage().digest());
        builder.push_u64(key.profile_generation().get());
        builder.push(&key.face().font_bytes_digest());
        builder.push_u32(key.face().face_index());
        builder.push(&key.face().selection_digest());
        builder.push_u32(key.glyph_id());
        builder.push_u8(u8::try_from(key.variations().len()).unwrap_or(u8::MAX));
        for variation in key.variations().records() {
            builder.push(&variation.axis());
            builder.push_i32(variation.value_milli());
        }
        builder.push_u16(key.palette().index());
        builder.push_u32(key.size().millipoints());
        builder.push_u8(source_byte(key.source()));
        builder.push_u32(key.dpi_milli());
        builder.push_i16(key.fractional_origin().x_over_64());
        builder.push_i16(key.fractional_origin().y_over_64());
        builder.finish(ModelSource::from_native(key.source()), key.glyph_id())
    }

    pub(super) const fn source(self) -> ModelSource {
        self.source
    }

    pub(super) fn canonical(self) -> Vec<u8> {
        self.canonical[..usize::from(self.canonical_len)].to_vec()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(super) struct ModelPin {
    layout: [u8; 32],
    key: ModelKey,
}

impl ModelPin {
    pub(super) fn new(layout: [u8; 32], key: ModelKey) -> Self {
        Self { layout, key }
    }

    pub(super) const fn key(self) -> ModelKey {
        self.key
    }
}

struct CanonicalBuilder {
    bytes: [u8; CANONICAL_BYTES],
    len: usize,
}

impl CanonicalBuilder {
    fn new() -> Self {
        Self {
            bytes: [0; CANONICAL_BYTES],
            len: 0,
        }
    }

    fn push(&mut self, value: &[u8]) {
        let end = self.len.saturating_add(value.len()).min(CANONICAL_BYTES);
        let copied = end.saturating_sub(self.len);
        self.bytes[self.len..end].copy_from_slice(&value[..copied]);
        self.len = end;
    }

    fn push_u8(&mut self, value: u8) {
        self.push(&[value]);
    }

    fn push_u16(&mut self, value: u16) {
        self.push(&value.to_le_bytes());
    }

    fn push_u32(&mut self, value: u32) {
        self.push(&value.to_le_bytes());
    }

    fn push_u64(&mut self, value: u64) {
        self.push(&value.to_le_bytes());
    }

    fn push_i16(&mut self, value: i16) {
        self.push(&value.to_le_bytes());
    }

    fn push_i32(&mut self, value: i32) {
        self.push(&value.to_le_bytes());
    }

    fn finish(self, source: ModelSource, glyph: u32) -> ModelKey {
        ModelKey {
            source,
            glyph,
            canonical: self.bytes,
            canonical_len: u16::try_from(self.len).unwrap_or(u16::MAX),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ModelDemand {
    pub(super) key: ModelKey,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) logical_bytes: u64,
    pub(super) physical_bytes: u64,
}

impl ModelDemand {
    pub(super) fn for_key(key: ModelKey, width: u32, height: u32) -> Self {
        let channels = key.source().channels();
        let logical_bytes = u64::from(width)
            .saturating_mul(u64::from(height))
            .saturating_mul(channels);
        let row_bytes = u64::from(width).saturating_mul(channels);
        let padded_row = row_bytes.saturating_add(255) / 256 * 256;
        Self {
            key,
            width,
            height,
            logical_bytes,
            physical_bytes: padded_row.saturating_mul(u64::from(height)),
        }
    }
}

fn source_byte(source: UiGlyphRasterSource) -> u8 {
    match source {
        UiGlyphRasterSource::ColorOutline => 0,
        UiGlyphRasterSource::ColorBitmap => 1,
        UiGlyphRasterSource::AlphaOutline => 2,
        UiGlyphRasterSource::LastResort => 3,
    }
}

fn model_source_byte(source: ModelSource) -> u8 {
    match source {
        ModelSource::Color => 0,
        ModelSource::Alpha => 2,
    }
}
