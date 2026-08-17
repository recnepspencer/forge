use super::*;
use crate::qualified_text::raster_key::{
    UiGlyphRasterFractionalOrigin, UiGlyphRasterKey, UiGlyphRasterKeyInput, UiGlyphRasterPalette,
    UiGlyphRasterSize, UiGlyphRasterSource, UiGlyphVariationCoordinates,
};
use crate::{
    UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiQualifiedFontFaceIdentity,
    UiQualifiedTextLayoutIdentity, UiTextOriginalRange, UiTextProfileGeneration,
};

fn key() -> UiGlyphRasterKey {
    UiGlyphRasterKey::from_text_mechanics(UiGlyphRasterKeyInput {
        font_collection: UiFontCollectionGeneration::new(1).unwrap(),
        font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([10; 32]),
        profile: UiTextProfileGeneration::new(1).unwrap(),
        face: UiQualifiedFontFaceIdentity::from_text_mechanics([9; 32], 0),
        glyph_id: 3,
        variations: UiGlyphVariationCoordinates::empty(),
        palette: UiGlyphRasterPalette::new(0),
        size: UiGlyphRasterSize::from_millipoints(14_000).unwrap(),
        source: UiGlyphRasterSource::AlphaOutline,
        dpi_milli: 1_000,
        origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
    })
    .unwrap()
}

fn record_input(pixels: &[u8], stride: u32) -> UiGlyphRasterRecordViewInput<'_> {
    UiGlyphRasterRecordViewInput {
        key: key(),
        attribution: UiGlyphRasterAttribution::from_text_mechanics(
            UiQualifiedTextLayoutIdentity::from_text_mechanics([1; 32]),
            UiTextOriginalRange::new(0, 1).unwrap(),
        ),
        bearing: UiGlyphRasterBearing::from_sixty_fourths(8, -4),
        extent: UiGlyphRasterExtent::new(2, 2).unwrap(),
        stride,
        pixels,
        digest: UiGlyphRasterContentDigest::from_text_mechanics([11; 32]),
    }
}

#[test]
fn alpha_and_color_views_enforce_distinct_byte_shapes() {
    let alpha_pixels = [1, 2, 3, 4];
    let color_pixels = [0; 16];
    let alpha = UiAlphaRasterRecordView::from_text_mechanics(record_input(&alpha_pixels, 2));
    let color_as_alpha =
        UiAlphaRasterRecordView::from_text_mechanics(record_input(&color_pixels, 8));
    let color = UiColorRasterRecordView::from_text_mechanics(record_input(&color_pixels, 8));
    let alpha_as_color =
        UiColorRasterRecordView::from_text_mechanics(record_input(&alpha_pixels, 2));
    assert!(alpha.is_ok());
    assert_eq!(
        color_as_alpha.err(),
        Some(UiGlyphRasterViewDenial::ByteLengthMismatch {
            expected: 4,
            actual: 16,
        })
    );
    assert!(color.is_ok());
    assert_eq!(
        alpha_as_color.err(),
        Some(UiGlyphRasterViewDenial::ByteLengthMismatch {
            expected: 16,
            actual: 4,
        })
    );
}
