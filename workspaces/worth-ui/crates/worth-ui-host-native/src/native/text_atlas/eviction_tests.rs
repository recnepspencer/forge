//! Registration-order and equal-epoch eviction twins.

use super::entry::UiAtlasEntry;
use super::model_oracle::{IndependentAtlasModel, ModelDemand, ModelKey};
use super::ownership::UiNativeTextAtlas;
use super::placement::UiAtlasPage;
use super::settlement::UiNativeTextAtlasCommitOutcome;
use super::transaction::{
    UiNativeTextAtlasDemand, UiNativeTextAtlasExternalOutcome, UiNativeTextAtlasUpload,
};
use super::UiAtlasEntryIdentity;
use std::collections::HashSet;
use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiGlyphRasterDemandIdentity,
    UiGlyphRasterFractionalOrigin, UiGlyphRasterKey, UiGlyphRasterKeyInput, UiGlyphRasterPalette,
    UiGlyphRasterSize, UiGlyphRasterSource, UiGlyphVariationCoordinates,
    UiQualifiedFontFaceIdentity, UiQualifiedTextVariationRecord, UiTextProfileGeneration,
};

#[test]
pub(crate) fn equal_epoch_eviction_matches_model_and_ignores_registration_order() {
    let forward = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let reverse = [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let forward_model = settle_model(&forward);
    let reverse_model = settle_model(&reverse);
    let forward_native = settle_native(&forward);
    let reverse_native = settle_native(&reverse);

    assert_eq!(forward_model, reverse_model);
    assert_eq!(forward_native, reverse_native);
    assert_eq!(
        ModelKey::from_native(forward_native),
        forward_model,
        "production and independent canonical-key ordering must agree"
    );
}

#[test]
pub(crate) fn every_complete_key_field_participates_in_equal_epoch_eviction_order() {
    for pair in complete_key_field_twins() {
        let expected = pair
            .map(ModelKey::from_native)
            .into_iter()
            .min_by_key(|key| key.canonical())
            .unwrap();
        for ordered in [pair, [pair[1], pair[0]]] {
            let mut model = IndependentAtlasModel::new(62);
            model
                .admit(
                    &ordered.map(|key| ModelDemand::for_key(ModelKey::from_native(key), 1, 1)),
                    &[],
                    &[],
                )
                .unwrap();
            model.force_equal_epoch_for_test(9);
            assert_eq!(model.evict_one_for_test(), Some(expected));
            assert_eq!(evict_native_pair(ordered), expected);
        }
    }
}

fn evict_native_pair(keys: [UiGlyphRasterKey; 2]) -> ModelKey {
    let atlas = UiNativeTextAtlas::new();
    {
        let mut core = atlas.core.borrow_mut();
        core.completed_use_epoch = 9;
        for (slot, key) in keys.into_iter().enumerate() {
            let store = match key.source() {
                UiGlyphRasterSource::ColorOutline | UiGlyphRasterSource::ColorBitmap => {
                    &mut *core.color
                }
                UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort => {
                    &mut *core.alpha
                }
            };
            let (page, rect) = store.allocate(1, 1).unwrap();
            store.insert(UiAtlasEntry {
                identity: UiAtlasEntryIdentity::from_native_host(slot as u64 + 1).unwrap(),
                key,
                page,
                rect,
                staged_bytes: 1,
                digest: [0; 32],
                pin_count: 0,
                completed_use_epoch: 9,
            });
        }
        let super::ownership::AtlasCore { alpha, color, .. } = &mut *core;
        let evicted = super::eviction::evict_one(alpha, color, &HashSet::new()).unwrap();
        ModelKey::from_native(evicted)
    }
}

fn complete_key_field_twins() -> Vec<[UiGlyphRasterKey; 2]> {
    let base = key_input();
    let variation = UiGlyphVariationCoordinates::from_records(&[
        UiQualifiedTextVariationRecord::from_text_mechanics(*b"wght", 700_000),
    ])
    .unwrap();
    vec![
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                font_collection: UiFontCollectionGeneration::new(2).unwrap(),
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics(
                    [5; 32],
                ),
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                profile: UiTextProfileGeneration::new(2).unwrap(),
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                face: UiQualifiedFontFaceIdentity::from_application_text_mechanics(
                    [5; 32], 0, [4; 32],
                ),
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                face: UiQualifiedFontFaceIdentity::from_application_text_mechanics(
                    [4; 32], 1, [4; 32],
                ),
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                face: UiQualifiedFontFaceIdentity::from_application_text_mechanics(
                    [4; 32], 0, [5; 32],
                ),
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                glyph_id: 2,
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                variations: variation,
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                palette: UiGlyphRasterPalette::new(1),
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                size: UiGlyphRasterSize::from_millipoints(13_000).unwrap(),
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                source: UiGlyphRasterSource::ColorOutline,
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                dpi_milli: 1_500,
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(1, 0),
                ..base
            }),
        ],
        [
            from_input(base),
            from_input(UiGlyphRasterKeyInput {
                origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 1),
                ..base
            }),
        ],
    ]
}

fn key_input() -> UiGlyphRasterKeyInput {
    UiGlyphRasterKeyInput {
        font_collection: UiFontCollectionGeneration::new(1).unwrap(),
        font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([3; 32]),
        profile: UiTextProfileGeneration::new(1).unwrap(),
        face: UiQualifiedFontFaceIdentity::from_application_text_mechanics([4; 32], 0, [4; 32]),
        glyph_id: 1,
        variations: UiGlyphVariationCoordinates::empty(),
        palette: UiGlyphRasterPalette::new(0),
        size: UiGlyphRasterSize::from_millipoints(12_000).unwrap(),
        source: UiGlyphRasterSource::AlphaOutline,
        dpi_milli: 1_000,
        origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(0, 0),
    }
}

fn from_input(input: UiGlyphRasterKeyInput) -> UiGlyphRasterKey {
    UiGlyphRasterKey::from_text_mechanics(input).unwrap()
}

fn settle_model(order: &[u32]) -> ModelKey {
    let mut model = IndependentAtlasModel::new(61);
    let seed = order
        .iter()
        .map(|glyph| ModelDemand::for_key(ModelKey::from_native(key(*glyph)), 512, 512))
        .collect::<Vec<_>>();
    model.admit(&seed, &[], &[]).unwrap();
    model.force_equal_epoch_for_test(7);
    let receipt = model
        .admit(
            &[ModelDemand::for_key(
                ModelKey::from_native(key(99)),
                512,
                512,
            )],
            &[],
            &[],
        )
        .unwrap();
    assert_eq!(receipt.evicted_keys.len(), 1);
    receipt.evicted_keys[0]
}

fn settle_native(order: &[u32]) -> UiGlyphRasterKey {
    let atlas = UiNativeTextAtlas::new();
    {
        let mut core = atlas.core.borrow_mut();
        core.alpha.pages = (0..4).map(|_| UiAtlasPage::new(1_024, 1_024)).collect();
        core.completed_use_epoch = 7;
        for (slot, glyph) in order.iter().enumerate() {
            let entry_key = key(*glyph);
            let page_index = slot / 4;
            let rect = core.alpha.pages[page_index].allocate(512, 512).unwrap();
            core.alpha.entries.insert(
                entry_key,
                UiAtlasEntry {
                    identity: UiAtlasEntryIdentity::from_native_host(*glyph as u64 + 1).unwrap(),
                    key: entry_key,
                    page: u32::try_from(page_index).unwrap(),
                    rect,
                    staged_bytes: 512 * 512,
                    digest: [0; 32],
                    pin_count: 0,
                    completed_use_epoch: 7,
                },
            );
        }
    }
    let replacement = demand(key(99));
    let plan = atlas
        .plan_demands(&[replacement], &Default::default())
        .unwrap();
    let evicted = plan.evicted_keys()[0];
    let upload = UiNativeTextAtlasUpload::from_text_mechanics(
        replacement.key(),
        512,
        512,
        512,
        vec![0; 512 * 512],
        [0; 32],
    );
    assert!(matches!(
        atlas.settle(plan, &[upload], UiNativeTextAtlasExternalOutcome::Submitted),
        UiNativeTextAtlasCommitOutcome::Committed(_)
    ));
    evicted
}

fn demand(key: UiGlyphRasterKey) -> UiNativeTextAtlasDemand {
    UiNativeTextAtlasDemand::from_native_geometry(
        UiGlyphRasterDemandIdentity::from_text_mechanics([4; 32]),
        key,
        512,
        512,
        512 * 512,
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
