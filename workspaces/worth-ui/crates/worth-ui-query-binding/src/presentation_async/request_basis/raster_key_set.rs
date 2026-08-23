use worth_query::facade::foundation::WorthQueryAsyncRequestIdentityPart as Part;
use worth_ui_host_contract::{UiGlyphRasterKey, UiGlyphRasterSource};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WorthUiPresentationRasterKeySetBasis {
    keys: Box<[UiGlyphRasterKey]>,
}

impl WorthUiPresentationRasterKeySetBasis {
    pub(crate) fn from_runtime(mut keys: Vec<UiGlyphRasterKey>) -> Self {
        keys.sort_by_cached_key(key_sort_parts);
        keys.dedup();
        Self {
            keys: keys.into_boxed_slice(),
        }
    }

    pub fn keys(&self) -> &[UiGlyphRasterKey] {
        &self.keys
    }

    pub fn contains(&self, key: UiGlyphRasterKey) -> bool {
        self.keys.iter().any(|candidate| *candidate == key)
    }
}

pub(super) fn key_sort_parts(key: &UiGlyphRasterKey) -> Vec<Part> {
    let face = key.face();
    let origin = key.fractional_origin();
    let mut parts = vec![
        Part::unsigned("font-generation", key.font_collection_generation().get()),
        Part::bytes32("font-lineage", key.font_collection_lineage().digest()),
        Part::unsigned("profile", key.profile_generation().get()),
        Part::bytes32("font-bytes", face.font_bytes_digest()),
        Part::unsigned("face-index", u64::from(face.face_index())),
        Part::bytes32("selection", face.selection_digest()),
        Part::unsigned("glyph", u64::from(key.glyph_id())),
        Part::unsigned("palette", u64::from(key.palette().index())),
        Part::unsigned("size", u64::from(key.size().millipoints())),
        Part::unsigned("source", raster_source_ordinal(key.source())),
        Part::unsigned("dpi", u64::from(key.dpi_milli())),
        Part::unsigned("origin-x", u64::from(origin.x_over_64() as u16)),
        Part::unsigned("origin-y", u64::from(origin.y_over_64() as u16)),
    ];
    for (index, axis) in key.variations().records().enumerate() {
        parts.extend([
            Part::bytes4(format!("axis.{index:02}.tag"), axis.axis()),
            Part::unsigned(
                format!("axis.{index:02}.value"),
                u64::from(axis.value_milli() as u32),
            ),
        ]);
    }
    parts
}

const fn raster_source_ordinal(source: UiGlyphRasterSource) -> u64 {
    match source {
        UiGlyphRasterSource::ColorOutline => 0,
        UiGlyphRasterSource::ColorBitmap => 1,
        UiGlyphRasterSource::AlphaOutline => 2,
        UiGlyphRasterSource::LastResort => 3,
    }
}
