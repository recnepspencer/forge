//! Same-lineage reconstruction identity monotonicity.

use super::ownership::UiNativeTextAtlas;
use super::settlement::UiNativeTextAtlasCommitOutcome;
use super::transaction::{UiNativeTextAtlasExternalOutcome, UiNativeTextAtlasUpload};
use super::{UiNativeTextAtlasDemand, UiNativeTextAtlasPinTransition};
use crate::native::text_atlas::boundary_tests::{demand, key};
use worth_ui_host_contract::UiGlyphRasterSource;

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

#[test]
pub(super) fn reconstructive_recovery_never_reuses_generation_entry_or_reservation_identity() {
    let atlas = UiNativeTextAtlas::new();
    let first = demand(key(70_000, UiGlyphRasterSource::AlphaOutline), 1, 1);
    let first_plan = atlas
        .plan_demands(&[first], &UiNativeTextAtlasPinTransition::default())
        .unwrap();
    let first_reservation = first_plan.reservation;
    let UiNativeTextAtlasCommitOutcome::Committed(first_receipt) = atlas.settle(
        first_plan,
        &[upload(first)],
        UiNativeTextAtlasExternalOutcome::Submitted,
    ) else {
        panic!("the first atlas generation must commit");
    };
    let first_entry = atlas
        .core
        .borrow()
        .alpha
        .entries
        .get(&first.key())
        .unwrap()
        .identity
        .get();

    let uncertain = demand(key(70_001, UiGlyphRasterSource::AlphaOutline), 1, 1);
    let uncertain_plan = atlas
        .plan_demands(&[uncertain], &UiNativeTextAtlasPinTransition::default())
        .unwrap();
    let uncertain_reservation = uncertain_plan.reservation;
    let UiNativeTextAtlasCommitOutcome::EffectsIndeterminate(recovery) = atlas.settle(
        uncertain_plan,
        &[upload(uncertain)],
        UiNativeTextAtlasExternalOutcome::EffectsIndeterminate,
    ) else {
        panic!("partial effects must quarantine the atlas");
    };
    assert!(atlas.recover(&recovery));
    assert!(atlas.snapshot().generation.get() > first_receipt.generation.get());

    let rebuilt = demand(key(70_002, UiGlyphRasterSource::AlphaOutline), 1, 1);
    let rebuilt_plan = atlas
        .plan_demands(&[rebuilt], &UiNativeTextAtlasPinTransition::default())
        .unwrap();
    assert!(rebuilt_plan.reservation > uncertain_reservation);
    assert!(uncertain_reservation > first_reservation);
    let UiNativeTextAtlasCommitOutcome::Committed(rebuilt_receipt) = atlas.settle(
        rebuilt_plan,
        &[upload(rebuilt)],
        UiNativeTextAtlasExternalOutcome::Submitted,
    ) else {
        panic!("reconstruction must commit under a fresh generation");
    };
    let rebuilt_entry = atlas
        .core
        .borrow()
        .alpha
        .entries
        .get(&rebuilt.key())
        .unwrap()
        .identity
        .get();
    assert!(rebuilt_receipt.generation.get() > recovery.generation().get());
    assert!(rebuilt_entry > first_entry + 1);
}
