//! Causal fully-pinned saturation denial.

use super::model_oracle::{IndependentAtlasModel, ModelDemand, ModelDenial, ModelKey, ModelPin};
use super::ownership::UiNativeTextAtlas;
use super::settlement::UiNativeTextAtlasCommitOutcome;
use super::transaction::{
    UiNativeTextAtlasDemand, UiNativeTextAtlasExternalOutcome, UiNativeTextAtlasPinRequest,
    UiNativeTextAtlasPinTransition, UiNativeTextAtlasUpload,
};
use super::UiNativeTextAtlasDenial;
use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiGlyphRasterDemandIdentity,
    UiGlyphRasterFractionalOrigin, UiGlyphRasterKey, UiGlyphRasterKeyInput, UiGlyphRasterPalette,
    UiGlyphRasterSize, UiGlyphRasterSource, UiGlyphVariationCoordinates,
    UiQualifiedFontFaceIdentity, UiQualifiedTextLayoutIdentity, UiTextProfileGeneration,
};

#[test]
pub(super) fn fully_pinned_page_denies_with_named_cause_without_mutating_predecessor() {
    let atlas = UiNativeTextAtlas::new();
    let mut model = IndependentAtlasModel::new(91);
    let mut model_pins = Vec::new();
    let native_seed = (0..16).map(|glyph| demand(key(glyph))).collect::<Vec<_>>();
    let native_pins = native_seed
        .iter()
        .enumerate()
        .map(|(glyph, demand)| {
            let layout = UiQualifiedTextLayoutIdentity::from_text_mechanics([glyph as u8; 32]);
            model_pins.push(ModelPin::new(
                layout.digest(),
                ModelKey::from_native(demand.key()),
            ));
            UiNativeTextAtlasPinRequest::from_text_mechanics(layout, demand.key())
        })
        .collect::<Vec<_>>();
    let seed_transition = UiNativeTextAtlasPinTransition::from_text_mechanics(native_pins, []);
    let seed_plan = atlas.plan_demands(&native_seed, &seed_transition).unwrap();
    let seed_uploads = native_seed.iter().copied().map(upload).collect::<Vec<_>>();
    assert!(matches!(
        atlas.settle(
            seed_plan,
            &seed_uploads,
            UiNativeTextAtlasExternalOutcome::Submitted
        ),
        UiNativeTextAtlasCommitOutcome::Committed(_)
    ));
    let model_seed = (0..16)
        .map(|glyph| ModelDemand::for_key(ModelKey::from_native(key(glyph)), 512, 512))
        .collect::<Vec<_>>();
    model.admit(&model_seed, &[], &[]).unwrap();
    model.admit(&[], &model_pins, &[]).unwrap();
    let before = atlas.snapshot();
    let replacement = key(99);
    assert_eq!(
        model.admit(
            &[ModelDemand::for_key(
                ModelKey::from_native(replacement),
                512,
                512
            )],
            &[],
            &[],
        ),
        Err(ModelDenial::Pinned)
    );
    assert!(matches!(
        atlas.plan_demands(&[demand(replacement)], &Default::default()),
        Err(UiNativeTextAtlasDenial::PinnedCapacityExceeded)
    ));
    assert_eq!(atlas.snapshot(), before);
}

fn demand(key: UiGlyphRasterKey) -> UiNativeTextAtlasDemand {
    UiNativeTextAtlasDemand::from_native_geometry(
        UiGlyphRasterDemandIdentity::from_text_mechanics([9; 32]),
        key,
        512,
        512,
        512 * 512,
    )
}

fn upload(demand: UiNativeTextAtlasDemand) -> UiNativeTextAtlasUpload {
    UiNativeTextAtlasUpload::from_text_mechanics(
        demand.key(),
        demand.width(),
        demand.height(),
        demand.width(),
        vec![0; usize::try_from(demand.staged_bytes()).unwrap()],
        [0; 32],
    )
}

fn key(glyph_id: u32) -> UiGlyphRasterKey {
    UiGlyphRasterKey::from_text_mechanics(UiGlyphRasterKeyInput {
        font_collection: UiFontCollectionGeneration::new(1).unwrap(),
        font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([3; 32]),
        profile: UiTextProfileGeneration::new(1).unwrap(),
        face: UiQualifiedFontFaceIdentity::from_text_mechanics([4; 32], 0),
        glyph_id,
        variations: UiGlyphVariationCoordinates::empty(),
        palette: UiGlyphRasterPalette::new(0),
        size: UiGlyphRasterSize::from_millipoints(12_000).unwrap(),
        source: UiGlyphRasterSource::AlphaOutline,
        dpi_milli: 1_000,
        origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
    })
    .unwrap()
}
