use std::collections::HashMap;

use super::super::completion::validate_produced_keys;
use super::*;
use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiGlyphRasterAttribution,
    UiGlyphRasterFractionalOrigin, UiGlyphRasterKeyInput, UiGlyphRasterPalette, UiGlyphRasterSize,
    UiGlyphRasterSource, UiGlyphVariationCoordinates, UiQualifiedFontFaceIdentity,
    UiQualifiedTextLayoutIdentity, UiTextOriginalRange, UiTextProfileGeneration,
};

#[test]
fn aggregate_completion_rejects_missing_duplicate_and_foreign_color_outputs() {
    let first = key(1);
    let second = key(2);
    let admission = UiColorRasterTransactionAdmission {
        identity: [4; 32],
        demand_batches: 2,
        unique_records: 2,
        predicted_bytes: 8,
        key_probes: 2,
        validation_checks: 2,
        provenance_checks: 2,
        admitted_keys: Box::new([first, second]),
        expected_batches: Box::new([]),
        expected_attributions: Box::new([HashMap::from([
            (first, attribution()),
            (second, attribution()),
        ])]),
    };
    assert_eq!(
        validate_produced_keys(&admission, [(0, first, attribution(), 4)]),
        Err(UiGlyphRasterizationDenial::TransactionOutputMismatch)
    );
    assert_eq!(
        validate_produced_keys(
            &admission,
            [
                (0, first, attribution(), 4),
                (0, first, attribution(), 4),
                (0, second, attribution(), 0),
            ],
        ),
        Err(UiGlyphRasterizationDenial::TransactionOutputMismatch)
    );
    assert_eq!(
        validate_produced_keys(
            &admission,
            [(0, first, attribution(), 4), (0, key(3), attribution(), 4)],
        ),
        Err(UiGlyphRasterizationDenial::TransactionOutputMismatch)
    );
    assert_eq!(
        validate_produced_keys(
            &admission,
            [(0, first, attribution(), 4), (0, second, attribution(), 4)],
        ),
        Ok(8)
    );
    let wrong = UiGlyphRasterAttribution::from_text_mechanics(
        UiQualifiedTextLayoutIdentity::from_text_mechanics([9; 32]),
        UiTextOriginalRange::new(3, 4).unwrap(),
    );
    assert_eq!(
        validate_produced_keys(
            &admission,
            [(0, first, wrong, 4), (0, second, attribution(), 4)]
        ),
        Err(UiGlyphRasterizationDenial::TransactionOutputMismatch)
    );
}

fn key(glyph_id: u32) -> UiGlyphRasterKey {
    UiGlyphRasterKey::from_text_mechanics(UiGlyphRasterKeyInput {
        font_collection: UiFontCollectionGeneration::new(1).unwrap(),
        font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([1; 32]),
        profile: UiTextProfileGeneration::new(1).unwrap(),
        face: UiQualifiedFontFaceIdentity::from_text_mechanics([2; 32], 0),
        glyph_id,
        variations: UiGlyphVariationCoordinates::empty(),
        palette: UiGlyphRasterPalette::new(0),
        size: UiGlyphRasterSize::from_millipoints(14_000).unwrap(),
        source: UiGlyphRasterSource::ColorOutline,
        dpi_milli: 1_000,
        origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
    })
    .unwrap()
}

fn attribution() -> UiGlyphRasterAttribution {
    UiGlyphRasterAttribution::from_text_mechanics(
        UiQualifiedTextLayoutIdentity::from_text_mechanics([1; 32]),
        UiTextOriginalRange::new(0, 2).unwrap(),
    )
}
