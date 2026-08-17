use super::model_oracle::{IndependentAtlasModel, ModelDemand, ModelKey};
use super::ownership::UiNativeTextAtlas;
use super::settlement::UiNativeTextAtlasCommitOutcome;
use super::transaction::UiNativeTextAtlasExternalOutcome;
use super::{UiNativeTextAtlasDemand, UiNativeTextAtlasUpload};
use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiGlyphRasterDemandIdentity,
    UiGlyphRasterFractionalOrigin, UiGlyphRasterKey, UiGlyphRasterKeyInput, UiGlyphRasterPalette,
    UiGlyphRasterSize, UiGlyphRasterSource, UiGlyphVariationCoordinates,
    UiQualifiedFontFaceIdentity, UiTextProfileGeneration,
};

#[test]
pub(super) fn model_and_production_share_multi_page_color_alpha_and_reused_placements() {
    let atlas = UiNativeTextAtlas::new();
    let mut model = IndependentAtlasModel::new(91);

    admit_and_compare(
        &atlas,
        &mut model,
        &demands(0, 4, UiGlyphRasterSource::AlphaOutline),
    );
    let alpha_second = demands(4, 1, UiGlyphRasterSource::AlphaOutline);
    let alpha_receipt = admit_and_compare(&atlas, &mut model, &alpha_second);
    assert_eq!(alpha_receipt.placements[0].page, 1);

    for start in [100, 108, 116, 124] {
        admit_and_compare(
            &atlas,
            &mut model,
            &demands(start, 8, UiGlyphRasterSource::ColorOutline),
        );
    }
    let replacement = demands(200, 1, UiGlyphRasterSource::ColorOutline);
    let replacement_receipt = admit_and_compare(&atlas, &mut model, &replacement);
    assert_eq!(replacement_receipt.evictions, 1);
    assert!(replacement_receipt.placements[0].page <= 1);
}

fn admit_and_compare(
    atlas: &UiNativeTextAtlas,
    model: &mut IndependentAtlasModel,
    demands: &[UiNativeTextAtlasDemand],
) -> super::model_records::ModelReceipt {
    let modeled = demands
        .iter()
        .map(|demand| ModelDemand::for_key(ModelKey::from_native(demand.key()), 512, 512))
        .collect::<Vec<_>>();
    let receipt = model.admit(&modeled, &[], &[]).unwrap();
    let plan = atlas.plan_demands(demands, &Default::default()).unwrap();
    for placement in &receipt.placements {
        let key = demands
            .iter()
            .find(|demand| ModelKey::from_native(demand.key()) == placement.key)
            .unwrap()
            .key();
        assert_eq!(
            plan.placement_for(key),
            Some((u32::try_from(placement.page).unwrap(), placement.origin))
        );
    }
    assert_eq!(plan.evicted_keys().len(), receipt.evictions);
    let uploads = demands.iter().copied().map(upload).collect::<Vec<_>>();
    assert!(matches!(
        atlas.settle(plan, &uploads, UiNativeTextAtlasExternalOutcome::Submitted),
        UiNativeTextAtlasCommitOutcome::Committed(_)
    ));
    receipt
}

fn demands(start: u32, count: u32, source: UiGlyphRasterSource) -> Vec<UiNativeTextAtlasDemand> {
    (start..start + count)
        .map(|glyph| {
            UiNativeTextAtlasDemand::from_native_geometry(
                UiGlyphRasterDemandIdentity::from_text_mechanics([12; 32]),
                key(glyph, source),
                512,
                512,
                match source {
                    UiGlyphRasterSource::ColorOutline | UiGlyphRasterSource::ColorBitmap => {
                        512 * 512 * 4
                    }
                    UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort => {
                        512 * 512
                    }
                },
            )
        })
        .collect()
}

fn upload(demand: UiNativeTextAtlasDemand) -> UiNativeTextAtlasUpload {
    let channels = match demand.key().source() {
        UiGlyphRasterSource::ColorOutline | UiGlyphRasterSource::ColorBitmap => 4,
        UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort => 1,
    };
    UiNativeTextAtlasUpload::from_text_mechanics(
        demand.key(),
        512,
        512,
        512 * channels,
        vec![0; usize::try_from(demand.staged_bytes()).unwrap()],
        [0; 32],
    )
}

fn key(glyph_id: u32, source: UiGlyphRasterSource) -> UiGlyphRasterKey {
    UiGlyphRasterKey::from_text_mechanics(UiGlyphRasterKeyInput {
        font_collection: UiFontCollectionGeneration::new(1).unwrap(),
        font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([7; 32]),
        profile: UiTextProfileGeneration::new(1).unwrap(),
        face: UiQualifiedFontFaceIdentity::from_text_mechanics([8; 32], 0),
        glyph_id,
        variations: UiGlyphVariationCoordinates::empty(),
        palette: UiGlyphRasterPalette::new(0),
        size: UiGlyphRasterSize::from_millipoints(12_000).unwrap(),
        source,
        dpi_milli: 1_000,
        origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
    })
    .unwrap()
}
