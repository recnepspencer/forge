//! Independent atlas oracle and hostile transaction controls.

use super::model_oracle::{IndependentAtlasModel, ModelDemand, ModelKey, ModelPin, ModelSource};
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

fn demand(key: UiGlyphRasterKey) -> UiNativeTextAtlasDemand {
    UiNativeTextAtlasDemand::from_native_geometry(
        UiGlyphRasterDemandIdentity::from_text_mechanics([8; 32]),
        key,
        512,
        512,
        512 * 512,
    )
}

fn upload(key: UiGlyphRasterKey) -> UiNativeTextAtlasUpload {
    UiNativeTextAtlasUpload::from_text_mechanics(key, 512, 512, 512, vec![0; 512 * 512], [0; 32])
}

#[test]
fn independent_model_matches_production_receipts_and_snapshots() {
    let glyph = key(100);
    let mut model = IndependentAtlasModel::new(17);
    let model_receipt = model
        .admit(
            &[ModelDemand::for_key(ModelKey::from_native(glyph), 512, 512)],
            &[],
            &[],
        )
        .unwrap();
    let atlas = UiNativeTextAtlas::new();
    let plan = atlas
        .plan_demands(&[demand(glyph)], &Default::default())
        .unwrap();
    assert!(matches!(
        atlas.settle(
            plan,
            &[upload(glyph)],
            UiNativeTextAtlasExternalOutcome::Submitted
        ),
        UiNativeTextAtlasCommitOutcome::Committed(_)
    ));
    let snapshot = atlas.snapshot();
    assert_eq!(model_receipt.generation, snapshot.generation.get());
    assert_eq!(model_receipt.misses, snapshot.alpha_entries as usize);
    assert_eq!(model.snapshot().alpha_pages, snapshot.alpha_pages as usize);
    assert_eq!(
        model.snapshot().alpha_entries,
        snapshot.alpha_entries as usize
    );

    let hit_receipt = model
        .admit(
            &[ModelDemand {
                logical_bytes: 0,
                physical_bytes: 0,
                ..ModelDemand::for_key(ModelKey::from_native(glyph), 512, 512)
            }],
            &[],
            &[],
        )
        .unwrap();
    let hit = atlas
        .plan_demands(&[demand(glyph)], &Default::default())
        .unwrap();
    let UiNativeTextAtlasCommitOutcome::Committed(receipt) =
        atlas.settle(hit, &[], UiNativeTextAtlasExternalOutcome::Submitted)
    else {
        panic!("a retained glyph must settle as a hit");
    };
    assert_eq!(hit_receipt.hits, usize::try_from(receipt.hits).unwrap());
}

fn model_demand(source: ModelSource, glyph: u32, width: u32, height: u32) -> ModelDemand {
    ModelDemand::for_key(ModelKey::synthetic(source, glyph), width, height)
}

#[test]
fn independent_model_proves_exact_entry_and_extent_boundaries() {
    let mut model = IndependentAtlasModel::new(21);
    let at_limit = (0..8_192)
        .map(|glyph| model_demand(ModelSource::Alpha, glyph, 1, 1))
        .collect::<Vec<_>>();
    assert!(model.admit(&at_limit, &[], &[]).is_ok());
    assert_eq!(model.snapshot().alpha_entries, 8_192);
    let replacement = model
        .admit(&[model_demand(ModelSource::Alpha, 8_192, 1, 1)], &[], &[])
        .expect("the exact entry cap permits deterministic unpinned replacement");
    assert_eq!(replacement.peak_entries, 8_192);
    assert_eq!(replacement.evictions, 1);
    assert_eq!(model.snapshot().alpha_entries, 8_192);

    let mut extents = IndependentAtlasModel::new(22);
    assert!(extents
        .admit(&[model_demand(ModelSource::Alpha, 1, 512, 512)], &[], &[])
        .is_ok());
    assert_eq!(
        extents.admit(&[model_demand(ModelSource::Alpha, 2, 513, 512)], &[], &[]),
        Err(super::model_oracle::ModelDenial::Extent)
    );
}

#[test]
fn independent_model_proves_staging_pin_release_and_mixed_texel_caps() {
    let mut model = IndependentAtlasModel::new(23);
    let staging_at_limit = (0..8)
        .map(|glyph| model_demand(ModelSource::Color, glyph, 512, 512))
        .collect::<Vec<_>>();
    let receipt = model.admit(&staging_at_limit, &[], &[]).unwrap();
    assert_eq!(receipt.staged_bytes, 8 * 1_024 * 1_024);
    let staging_over = (0..9)
        .map(|glyph| model_demand(ModelSource::Color, 100 + glyph, 512, 512))
        .collect::<Vec<_>>();
    assert_eq!(
        model.admit(&staging_over, &[], &[]),
        Err(super::model_oracle::ModelDenial::Staging)
    );

    let mut pinned = IndependentAtlasModel::new(24);
    let old = model_demand(ModelSource::Alpha, 100, 512, 512);
    pinned.admit(&[old], &[], &[]).unwrap();
    let first_owner = ModelPin::new([1; 32], old.key);
    let second_owner = ModelPin::new([2; 32], old.key);
    pinned
        .admit(&[], &[first_owner, second_owner], &[])
        .unwrap();
    for glyph in 101..116 {
        pinned
            .admit(
                &[model_demand(ModelSource::Alpha, glyph, 512, 512)],
                &[],
                &[],
            )
            .unwrap();
    }
    let replacement = model_demand(ModelSource::Alpha, 200, 512, 512);
    pinned.admit(&[], &[], &[first_owner]).unwrap();
    assert_eq!(pinned.snapshot().pins, 1);
    let retained_owner = pinned
        .admit(
            &[replacement],
            &[ModelPin::new([3; 32], replacement.key)],
            &[],
        )
        .unwrap();
    assert!(!retained_owner.evicted_keys.contains(&old.key));
    let after_release = model_demand(ModelSource::Alpha, 201, 512, 512);
    let released_owner = pinned
        .admit(
            &[after_release],
            &[ModelPin::new([4; 32], after_release.key)],
            &[second_owner],
        )
        .unwrap();
    assert!(released_owner.evicted_keys.contains(&old.key));

    let mut mixed = IndependentAtlasModel::new(25);
    for glyph in 0..16 {
        mixed
            .admit(
                &[model_demand(ModelSource::Alpha, glyph, 512, 512)],
                &[],
                &[],
            )
            .unwrap();
    }
    for glyph in 0..32 {
        mixed
            .admit(
                &[model_demand(ModelSource::Color, glyph, 512, 512)],
                &[],
                &[],
            )
            .unwrap();
    }
    assert_eq!(mixed.snapshot().alpha_pages, 4);
    assert_eq!(mixed.snapshot().color_pages, 2);
    assert_eq!(
        mixed.admit(&[], &[], &[]).unwrap().peak_texel_bytes,
        36 * 1_024 * 1_024
    );
}

#[test]
fn independent_model_recovery_is_lineage_bound_and_reconstructive() {
    let mut model = IndependentAtlasModel::new(31);
    let recovery = model.indeterminate();
    assert_eq!(
        model.admit(&[model_demand(ModelSource::Alpha, 1, 1, 1)], &[], &[]),
        Err(super::model_oracle::ModelDenial::Reconstruction)
    );
    assert_eq!(
        model.recover(super::model_oracle::ModelRecovery {
            lineage: 32,
            generation: recovery.generation,
        }),
        Err(super::model_oracle::ModelDenial::RecoveryOwner)
    );
    model.recover(recovery).unwrap();
    assert!(!model.snapshot().quarantined);
    assert_eq!(model.snapshot().generation, recovery.generation);
    let rebuilt = model
        .admit(&[model_demand(ModelSource::Alpha, 1, 1, 1)], &[], &[])
        .unwrap();
    assert!(rebuilt.generation > recovery.generation);
}

#[test]
fn rejected_and_indeterminate_settlement_preserve_predecessor_state() {
    let key = key(11);
    let atlas = UiNativeTextAtlas::new();
    let before = atlas.snapshot();
    let rejected = atlas
        .plan_demands(&[demand(key)], &Default::default())
        .unwrap();
    assert_eq!(
        atlas.settle(rejected, &[], UiNativeTextAtlasExternalOutcome::Rejected),
        UiNativeTextAtlasCommitOutcome::Denied(super::UiNativeTextAtlasDenial::UploadRejected)
    );
    assert_eq!(atlas.snapshot(), before);

    let indeterminate = atlas
        .plan_demands(&[demand(key)], &Default::default())
        .unwrap();
    let candidate = indeterminate.candidate_generation();
    let outcome = atlas.settle(
        indeterminate,
        &[upload(key)],
        UiNativeTextAtlasExternalOutcome::EffectsIndeterminate,
    );
    let UiNativeTextAtlasCommitOutcome::EffectsIndeterminate(recovery) = outcome else {
        panic!("indeterminate effects must carry recovery authority");
    };
    assert_eq!(recovery.snapshot().demand_identity().digest(), [8; 32]);
    assert_eq!(recovery.snapshot().generation(), candidate);
    assert_eq!(atlas.snapshot(), before);
}

#[test]
fn malformed_duplicate_geometry_and_close_are_hostile_controls() {
    let key = key(12);
    let atlas = UiNativeTextAtlas::new();
    let first = demand(key);
    let conflicting =
        UiNativeTextAtlasDemand::from_native_geometry(first.identity(), key, 256, 512, 256 * 512);
    assert!(matches!(
        atlas.plan_demands(&[first, conflicting], &Default::default()),
        Err(super::UiNativeTextAtlasDenial::RasterGeometryMismatch)
    ));
    let plan = atlas
        .plan_demands(&[demand(key)], &Default::default())
        .unwrap();
    let _ = atlas.settle(
        plan,
        &[upload(key)],
        UiNativeTextAtlasExternalOutcome::Submitted,
    );
    assert_eq!(atlas.census().alpha_entries, 1);
    assert!(atlas.clear());
    assert!(atlas.census().is_zero());
}

#[test]
fn generation_exhaustion_does_not_leave_a_reservation_behind() {
    let atlas = UiNativeTextAtlas::new();
    {
        let mut core = atlas.core.borrow_mut();
        core.generation = super::UiNativeTextAtlasGeneration::new(u64::MAX).unwrap();
        assert_eq!(core.next_reservation, 1);
    }
    assert!(matches!(
        atlas.plan_demands(&[demand(key(13))], &Default::default()),
        Err(super::UiNativeTextAtlasDenial::GenerationExhausted)
    ));
    let snapshot = atlas.snapshot();
    assert!(!snapshot.reservation_active);
    assert_eq!(snapshot.generation.get(), u64::MAX);
    assert_eq!(atlas.core.borrow().next_reservation, 1);
}

#[test]
fn indeterminate_quarantines_hits_until_consumed_reconstructive_recovery() {
    let atlas = UiNativeTextAtlas::new();
    let key = key(901);
    let plan = atlas
        .plan_demands(&[demand(key)], &Default::default())
        .unwrap();
    let recovery_generation = plan.candidate_generation();
    let outcome = atlas.settle(
        plan,
        &[upload(key)],
        UiNativeTextAtlasExternalOutcome::EffectsIndeterminate,
    );
    let UiNativeTextAtlasCommitOutcome::EffectsIndeterminate(recovery) = outcome else {
        panic!("partial external effects must produce recovery authority");
    };
    assert_eq!(atlas.snapshot().alpha_entries, 0);
    assert!(matches!(
        atlas.plan_demands(&[demand(key)], &Default::default()),
        Err(super::UiNativeTextAtlasDenial::ReconstructionRequired)
    ));
    assert_eq!(recovery.generation(), recovery_generation);
    assert!(atlas.recover(&recovery));
    assert!(!atlas.snapshot().reservation_active);
    assert!(atlas
        .plan_demands(&[demand(key)], &Default::default())
        .is_ok());
}

#[test]
pub(super) fn recovery_lineage_rejects_a_same_generation_foreign_atlas() {
    let first = UiNativeTextAtlas::new();
    let second = UiNativeTextAtlas::new();
    let glyph = key(902);
    let plan = first
        .plan_demands(&[demand(glyph)], &Default::default())
        .unwrap();
    let UiNativeTextAtlasCommitOutcome::EffectsIndeterminate(recovery) = first.settle(
        plan,
        &[upload(glyph)],
        UiNativeTextAtlasExternalOutcome::EffectsIndeterminate,
    ) else {
        panic!("partial external effects must produce recovery authority");
    };

    assert!(!second.recover(&recovery));
    assert!(second
        .plan_demands(&[demand(glyph)], &Default::default())
        .is_ok());
    assert!(first.recover(&recovery));
}
