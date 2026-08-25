use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiGlyphRasterDemandIdentity,
    UiGlyphRasterFractionalOrigin, UiGlyphRasterKey, UiGlyphRasterKeyInput, UiGlyphRasterPalette,
    UiGlyphRasterSize, UiGlyphRasterSource, UiGlyphVariationCoordinates,
    UiQualifiedFontFaceIdentity, UiTextProfileGeneration,
};

use super::{
    UiNativeTextAtlas, UiNativeTextAtlasDemand, UiNativeTextAtlasExternalOutcome,
    UiNativeTextAtlasUpload,
};

fn key() -> UiGlyphRasterKey {
    UiGlyphRasterKey::from_text_mechanics(UiGlyphRasterKeyInput {
        font_collection: UiFontCollectionGeneration::new(1).unwrap(),
        font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([1; 32]),
        profile: UiTextProfileGeneration::new(1).unwrap(),
        face: UiQualifiedFontFaceIdentity::from_text_mechanics([2; 32], 0),
        glyph_id: 41,
        variations: UiGlyphVariationCoordinates::empty(),
        palette: UiGlyphRasterPalette::new(0),
        size: UiGlyphRasterSize::from_millipoints(12_000).unwrap(),
        source: UiGlyphRasterSource::AlphaOutline,
        dpi_milli: 1_000,
        origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
    })
    .unwrap()
}

#[test]
fn retained_content_extent_is_the_uploaded_shape_not_the_padded_allocation() {
    let atlas = UiNativeTextAtlas::new();
    let key = key();
    let demand = UiNativeTextAtlasDemand::from_native_geometry(
        UiGlyphRasterDemandIdentity::from_text_mechanics([7; 32]),
        key,
        8,
        9,
        72,
    );
    let plan = atlas.plan_demands(&[demand], &Default::default()).unwrap();
    let upload = UiNativeTextAtlasUpload::from_text_mechanics(key, 6, 7, 6, vec![0; 42], [9; 32]);
    let committed = atlas.settle(plan, &[upload], UiNativeTextAtlasExternalOutcome::Submitted);
    assert!(matches!(
        committed,
        super::settlement::UiNativeTextAtlasCommitOutcome::Committed(_)
    ));
    let view = atlas.entry_view(key).unwrap();
    assert_eq!(view.extent, [6, 7]);
    let core = atlas.core.borrow();
    let entry = core.alpha.entries.get(&key).unwrap();
    assert_eq!([entry.rect.width, entry.rect.height], [8, 9]);
    assert_eq!(entry.content_extent, [6, 7]);
    assert_eq!(atlas.committed_transactions(), 1);
}
