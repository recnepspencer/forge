use super::model_oracle::{IndependentAtlasModel, ModelDemand, ModelKey};
use super::ownership::UiNativeTextAtlas;
use super::settlement::UiNativeTextAtlasCommitOutcome;
use super::transaction::{
    UiNativeTextAtlasDemand, UiNativeTextAtlasExternalOutcome, UiNativeTextAtlasUpload,
};
use super::UiNativeTextAtlasDenial;
use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiGlyphRasterDemandIdentity,
    UiGlyphRasterFractionalOrigin, UiGlyphRasterKey, UiGlyphRasterKeyInput, UiGlyphRasterPalette,
    UiGlyphRasterSize, UiGlyphRasterSource, UiGlyphVariationCoordinates,
    UiQualifiedFontFaceIdentity, UiQualifiedTextVariationRecord, UiTextProfileGeneration,
};

pub(super) fn key(glyph: u32, source: UiGlyphRasterSource) -> UiGlyphRasterKey {
    UiGlyphRasterKey::from_text_mechanics(UiGlyphRasterKeyInput {
        font_collection: UiFontCollectionGeneration::new(1).unwrap(),
        font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([3; 32]),
        profile: UiTextProfileGeneration::new(1).unwrap(),
        face: UiQualifiedFontFaceIdentity::from_text_mechanics([4; 32], 0),
        glyph_id: glyph,
        variations: UiGlyphVariationCoordinates::empty(),
        palette: UiGlyphRasterPalette::new(0),
        size: UiGlyphRasterSize::from_millipoints(12_000).unwrap(),
        source,
        dpi_milli: 1_000,
        origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
    })
    .unwrap()
}

fn key_variant(
    glyph: u32,
    dpi_milli: u32,
    palette: u16,
    variation_milli: Option<i32>,
) -> UiGlyphRasterKey {
    let variations = variation_milli.map_or_else(UiGlyphVariationCoordinates::empty, |value| {
        UiGlyphVariationCoordinates::from_records(&[
            UiQualifiedTextVariationRecord::from_text_mechanics(*b"wght", value),
        ])
        .unwrap()
    });
    UiGlyphRasterKey::from_text_mechanics(UiGlyphRasterKeyInput {
        font_collection: UiFontCollectionGeneration::new(1).unwrap(),
        font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([3; 32]),
        profile: UiTextProfileGeneration::new(1).unwrap(),
        face: UiQualifiedFontFaceIdentity::from_text_mechanics([4; 32], 0),
        glyph_id: glyph,
        variations,
        palette: UiGlyphRasterPalette::new(palette),
        size: UiGlyphRasterSize::from_millipoints(12_000).unwrap(),
        source: UiGlyphRasterSource::AlphaOutline,
        dpi_milli,
        origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
    })
    .unwrap()
}

pub(super) fn demand(key: UiGlyphRasterKey, width: u32, height: u32) -> UiNativeTextAtlasDemand {
    let channels = match key.source() {
        UiGlyphRasterSource::ColorOutline | UiGlyphRasterSource::ColorBitmap => 4,
        UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort => 1,
    };
    UiNativeTextAtlasDemand::from_native_geometry(
        UiGlyphRasterDemandIdentity::from_text_mechanics([8; 32]),
        key,
        width,
        height,
        u64::from(width) * u64::from(height) * channels,
    )
}

fn upload(demand: UiNativeTextAtlasDemand) -> UiNativeTextAtlasUpload {
    let channels = match demand.key().source() {
        UiGlyphRasterSource::ColorOutline | UiGlyphRasterSource::ColorBitmap => 4,
        UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort => 1,
    };
    UiNativeTextAtlasUpload::from_text_mechanics(
        demand.key(),
        demand.width(),
        demand.height(),
        demand.width() * channels,
        vec![0; usize::try_from(demand.staged_bytes()).unwrap()],
        [0; 32],
    )
}

fn model_demand(key: UiGlyphRasterKey, width: u32, height: u32) -> ModelDemand {
    ModelDemand::for_key(ModelKey::from_native(key), width, height)
}

#[test]
pub(super) fn independent_model_and_production_share_every_planned_page_and_origin() {
    let demands = [
        demand(key(50, UiGlyphRasterSource::AlphaOutline), 128, 64),
        demand(key(51, UiGlyphRasterSource::AlphaOutline), 256, 128),
        demand(key(52, UiGlyphRasterSource::AlphaOutline), 64, 256),
        demand(key(53, UiGlyphRasterSource::ColorOutline), 320, 96),
    ];
    let mut model = IndependentAtlasModel::new(76);
    let modeled = demands
        .iter()
        .map(|item| model_demand(item.key(), item.width(), item.height()))
        .collect::<Vec<_>>();
    let receipt = model.admit(&modeled, &[], &[]).unwrap();
    let atlas = UiNativeTextAtlas::new();
    let plan = atlas.plan_demands(&demands, &Default::default()).unwrap();
    for placement in &receipt.placements {
        let native_key = demands
            .iter()
            .find(|item| ModelKey::from_native(item.key()) == placement.key)
            .unwrap()
            .key();
        assert_eq!(
            plan.placement_for(native_key),
            Some((u32::try_from(placement.page).unwrap(), placement.origin))
        );
    }
    let uploads = demands.iter().copied().map(upload).collect::<Vec<_>>();
    assert!(matches!(
        atlas.settle(plan, &uploads, UiNativeTextAtlasExternalOutcome::Submitted),
        UiNativeTextAtlasCommitOutcome::Committed(_)
    ));
}

#[test]
pub(super) fn production_and_independent_model_share_8192_entry_boundary() {
    let atlas = UiNativeTextAtlas::new();
    let production_seed = (0..8_191)
        .map(|glyph| demand(key(glyph, UiGlyphRasterSource::AlphaOutline), 1, 1))
        .collect::<Vec<_>>();
    let seed_plan = atlas
        .plan_demands(&production_seed, &Default::default())
        .unwrap();
    let seed_uploads = production_seed
        .iter()
        .copied()
        .map(upload)
        .collect::<Vec<_>>();
    assert!(matches!(
        atlas.settle(
            seed_plan,
            &seed_uploads,
            UiNativeTextAtlasExternalOutcome::Submitted
        ),
        UiNativeTextAtlasCommitOutcome::Committed(_)
    ));
    let new_demand = demand(key(9_000, UiGlyphRasterSource::AlphaOutline), 1, 1);
    let mut model = IndependentAtlasModel::new(77);
    let model_seed = (0..8_191)
        .map(|glyph| model_demand(key(glyph, UiGlyphRasterSource::AlphaOutline), 1, 1))
        .collect::<Vec<_>>();
    model.admit(&model_seed, &[], &[]).unwrap();
    let model_receipt = model
        .admit(&[model_demand(new_demand.key(), 1, 1)], &[], &[])
        .unwrap();
    let plan = atlas
        .plan_demands(&[new_demand], &Default::default())
        .unwrap();
    let UiNativeTextAtlasCommitOutcome::Committed(receipt) = atlas.settle(
        plan,
        &[upload(new_demand)],
        UiNativeTextAtlasExternalOutcome::Submitted,
    ) else {
        panic!("the 8192nd entry must be admitted");
    };
    assert_eq!(model_receipt.peak_entries, receipt.peak_entries as usize);
    assert_eq!(atlas.snapshot().alpha_entries, 8_192);
    assert_eq!(model.snapshot().alpha_entries, 8_192);

    let over = demand(key(9_001, UiGlyphRasterSource::AlphaOutline), 1, 1);
    let model_over = model
        .admit(&[model_demand(over.key(), 1, 1)], &[], &[])
        .expect("an unpinned full atlas replaces its canonical oldest entry");
    let plan = atlas.plan_demands(&[over], &Default::default()).unwrap();
    assert_eq!(plan.candidate_overlay_entries(), 1);
    assert_eq!(plan.evicted_keys().len(), 1);
    assert_eq!(
        ModelKey::from_native(plan.evicted_keys()[0]),
        model_over.evicted_keys[0]
    );
    let UiNativeTextAtlasCommitOutcome::Committed(receipt) = atlas.settle(
        plan,
        &[upload(over)],
        UiNativeTextAtlasExternalOutcome::Submitted,
    ) else {
        panic!("the replacement must commit without exceeding the entry cap");
    };
    assert_eq!(receipt.peak_entries, 8_192);
    assert_eq!(atlas.snapshot().alpha_entries, 8_192);

    let hit_plan = atlas.plan_demands(&[over], &Default::default()).unwrap();
    assert_eq!(hit_plan.candidate_overlay_entries(), 0);
    assert_eq!(hit_plan.hit_keys(), &[over.key()]);
}

#[test]
pub(super) fn production_and_model_share_8_mib_staging_and_513_extent_denials() {
    let exact = (0..8)
        .map(|glyph| demand(key(glyph, UiGlyphRasterSource::ColorOutline), 512, 512))
        .collect::<Vec<_>>();
    let atlas = UiNativeTextAtlas::new();
    let mut model = IndependentAtlasModel::new(78);
    let model_demands = exact
        .iter()
        .map(|demand| model_demand(demand.key(), demand.width(), demand.height()))
        .collect::<Vec<_>>();
    let model_receipt = model.admit(&model_demands, &[], &[]).unwrap();
    let plan = atlas.plan_demands(&exact, &Default::default()).unwrap();
    assert_eq!(plan.staged_bytes(), 8 * 1_024 * 1_024);
    let uploads = exact.iter().copied().map(upload).collect::<Vec<_>>();
    let UiNativeTextAtlasCommitOutcome::Committed(receipt) =
        atlas.settle(plan, &uploads, UiNativeTextAtlasExternalOutcome::Submitted)
    else {
        panic!("8 MiB staging must be admitted");
    };
    assert_eq!(model_receipt.staged_bytes, receipt.staged_bytes);
    assert_eq!(
        model.snapshot().color_entries,
        atlas.snapshot().color_entries as usize
    );

    let over = (0..9)
        .map(|glyph| {
            demand(
                key(100 + glyph, UiGlyphRasterSource::ColorOutline),
                512,
                512,
            )
        })
        .collect::<Vec<_>>();
    assert!(matches!(
        atlas.plan_demands(&over, &Default::default()),
        Err(UiNativeTextAtlasDenial::StagingCapacityExceeded)
    ));
    let too_large = demand(key(300, UiGlyphRasterSource::AlphaOutline), 513, 512);
    assert!(matches!(
        atlas.plan_demands(&[too_large], &Default::default()),
        Err(UiNativeTextAtlasDenial::GlyphExtentExceeded)
    ));
}

#[test]
pub(super) fn physical_only_staging_and_complete_key_twins_match_production() {
    let physical_only = (0..65)
        .map(|glyph| demand(key(glyph, UiGlyphRasterSource::AlphaOutline), 1, 512))
        .collect::<Vec<_>>();
    let mut model = IndependentAtlasModel::new(79);
    let model_physical = physical_only
        .iter()
        .map(|demand| ModelDemand::for_key(ModelKey::from_native(demand.key()), 1, 512))
        .collect::<Vec<_>>();
    assert_eq!(
        model.admit(&model_physical, &[], &[]),
        Err(super::model_oracle::ModelDenial::Staging)
    );
    assert!(matches!(
        UiNativeTextAtlas::new().plan_demands(&physical_only, &Default::default()),
        Err(UiNativeTextAtlasDenial::StagingCapacityExceeded)
    ));

    let baseline = key_variant(900, 1_000, 0, None);
    let twins = [
        key_variant(900, 1_001, 0, None),
        key_variant(900, 1_000, 1, None),
        key_variant(900, 1_000, 0, Some(400_000)),
    ];
    for twin in twins {
        assert_ne!(ModelKey::from_native(baseline), ModelKey::from_native(twin));
        let atlas = UiNativeTextAtlas::new();
        let plan = atlas
            .plan_demands(
                &[demand(baseline, 1, 1), demand(twin, 1, 1)],
                &Default::default(),
            )
            .unwrap();
        assert_eq!(plan.miss_demands().len(), 2);
    }
}

#[test]
pub(super) fn production_color_saturation_and_mixed_pages_stop_at_36_mib() {
    let atlas = UiNativeTextAtlas::new();
    for batch in 0..4 {
        let demands = (0..8)
            .map(|offset| {
                demand(
                    key(batch * 8 + offset, UiGlyphRasterSource::ColorOutline),
                    512,
                    512,
                )
            })
            .collect::<Vec<_>>();
        let plan = atlas.plan_demands(&demands, &Default::default()).unwrap();
        let uploads = demands.iter().copied().map(upload).collect::<Vec<_>>();
        assert!(matches!(
            atlas.settle(plan, &uploads, UiNativeTextAtlasExternalOutcome::Submitted),
            UiNativeTextAtlasCommitOutcome::Committed(_)
        ));
    }
    for batch in 0..2 {
        let demands = (0..8)
            .map(|offset| {
                demand(
                    key(
                        1_000 + batch * 8 + offset,
                        UiGlyphRasterSource::AlphaOutline,
                    ),
                    512,
                    512,
                )
            })
            .collect::<Vec<_>>();
        let plan = atlas.plan_demands(&demands, &Default::default()).unwrap();
        let uploads = demands.iter().copied().map(upload).collect::<Vec<_>>();
        assert!(matches!(
            atlas.settle(plan, &uploads, UiNativeTextAtlasExternalOutcome::Submitted),
            UiNativeTextAtlasCommitOutcome::Committed(_)
        ));
    }
    let snapshot = atlas.snapshot();
    assert_eq!(snapshot.alpha_pages, 4);
    assert_eq!(snapshot.color_pages, 2);
    let replacement = demand(key(2_000, UiGlyphRasterSource::ColorOutline), 512, 512);
    let plan = atlas
        .plan_demands(&[replacement], &Default::default())
        .unwrap();
    assert_eq!(plan.candidate_page_counts(), (4, 2));
    let uploads = [upload(replacement)];
    let UiNativeTextAtlasCommitOutcome::Committed(receipt) =
        atlas.settle(plan, &uploads, UiNativeTextAtlasExternalOutcome::Submitted)
    else {
        panic!("saturated color pages must evict deterministically");
    };
    assert_eq!(receipt.peak_texel_bytes, 36 * 1_024 * 1_024);
    assert_eq!(atlas.snapshot().alpha_pages, 4);
    assert_eq!(atlas.snapshot().color_pages, 2);
}
