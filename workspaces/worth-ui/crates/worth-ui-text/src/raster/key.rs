//! Text-owned exact qualified raster key.
//!
//! The key is raster equivalence only. Layout, mechanic, paint-span, and
//! original-range attribution live on demand and draw records.

pub use worth_ui_host_contract::{UiGlyphRasterKey, UiGlyphRasterKeyInput};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGlyphRasterKeyDenial {
    ZeroDpi,
    ZeroSize,
    VariationCapacityExceeded,
}

pub fn admit_raster_key(
    input: UiGlyphRasterKeyInput,
) -> Result<UiGlyphRasterKey, UiGlyphRasterKeyDenial> {
    if input.size.millipoints() == 0 {
        return Err(UiGlyphRasterKeyDenial::ZeroSize);
    }
    UiGlyphRasterKey::from_text_mechanics(input).ok_or(UiGlyphRasterKeyDenial::ZeroDpi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_ui_host_contract::{
        UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiGlyphRasterFractionalOrigin,
        UiGlyphRasterPalette, UiGlyphRasterSize, UiGlyphRasterSource, UiGlyphVariationCoordinates,
        UiQualifiedFontFaceIdentity, UiTextProfileGeneration,
    };

    #[test]
    fn complete_key_admission_rejects_zero_dpi() {
        let denied = admit_raster_key(UiGlyphRasterKeyInput {
            font_collection: UiFontCollectionGeneration::new(1).unwrap(),
            font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([2; 32]),
            profile: UiTextProfileGeneration::new(1).unwrap(),
            face: UiQualifiedFontFaceIdentity::from_text_mechanics([1; 32], 0),
            glyph_id: 4,
            variations: UiGlyphVariationCoordinates::empty(),
            palette: UiGlyphRasterPalette::new(0),
            size: UiGlyphRasterSize::from_millipoints(12_000).unwrap(),
            source: UiGlyphRasterSource::AlphaOutline,
            dpi_milli: 0,
            origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
        });
        assert_eq!(denied, Err(UiGlyphRasterKeyDenial::ZeroDpi));
    }
}
