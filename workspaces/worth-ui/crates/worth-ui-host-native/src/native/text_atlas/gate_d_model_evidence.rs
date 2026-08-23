//! Independent Gate D model evidence assembled from the native atlas boundary.

use super::{
    model_oracle::{IndependentAtlasModel, ModelDemand, ModelKey, ModelPin},
    settlement::UiNativeTextAtlasSnapshot,
};

pub(crate) fn assert_independent_committed_transaction(
    demands: &[worth_ui_host_contract::UiGlyphRasterDemandBatchView<'_>],
    pins: worth_ui_host_contract::UiGlyphRasterPinTransitionView<'_>,
    receipt: worth_ui_host_contract::UiGlyphRasterTransactionReceipt,
    snapshot: UiNativeTextAtlasSnapshot,
) {
    let modeled = demands
        .iter()
        .flat_map(|demand| demand.records())
        .map(|record| {
            ModelDemand::for_key(
                ModelKey::from_native(record.key()),
                record.extent().width(),
                record.extent().height(),
            )
        })
        .collect::<Vec<_>>();
    let additions = model_pins(pins.additions());
    let releases = model_pins(pins.releases());
    let mut model = IndependentAtlasModel::new(314_105);
    let expected = model
        .admit(&modeled, &additions, &releases)
        .expect("independent Gate-D model admits the qualified transaction");
    assert_eq!(receipt.generation(), expected.generation);
    assert_eq!(usize::try_from(receipt.misses()).unwrap(), expected.misses);
    assert_eq!(usize::try_from(receipt.hits()).unwrap(), expected.hits);
    assert_eq!(
        usize::try_from(receipt.evictions()).unwrap(),
        expected.evictions
    );
    assert_eq!(receipt.staged_bytes(), expected.staged_bytes);
    assert_eq!(
        receipt.physical_staged_bytes(),
        expected.physical_staged_bytes
    );
    assert_eq!(
        usize::try_from(receipt.peak_entries()).unwrap(),
        expected.peak_entries
    );
    assert_eq!(receipt.peak_texel_bytes(), expected.peak_texel_bytes);
    let expected_snapshot = model.snapshot();
    assert_eq!(snapshot.generation.get(), expected_snapshot.generation);
    assert_eq!(
        snapshot.alpha_entries as usize,
        expected_snapshot.alpha_entries
    );
    assert_eq!(
        snapshot.color_entries as usize,
        expected_snapshot.color_entries
    );
    assert_eq!(snapshot.pins as usize, expected_snapshot.pins);
}

fn model_pins(pins: &[worth_ui_host_contract::UiGlyphRasterPinRequest]) -> Vec<ModelPin> {
    pins.iter()
        .map(|pin| {
            ModelPin::new(
                pin.layout_identity().digest(),
                ModelKey::from_native(pin.key()),
            )
        })
        .collect()
}

pub(crate) fn assert_gate_d_model_boundaries() {
    super::boundary_tests::independent_model_and_production_share_every_planned_page_and_origin();
    super::boundary_tests::production_and_independent_model_share_8192_entry_boundary();
    super::boundary_tests::production_and_model_share_8_mib_staging_and_513_extent_denials();
    super::boundary_tests::physical_only_staging_and_complete_key_twins_match_production();
    super::boundary_tests::production_color_saturation_and_mixed_pages_stop_at_36_mib();
    super::placement_model_tests::model_and_production_share_multi_page_color_alpha_and_reused_placements();
    super::pinning_capacity_tests::fully_pinned_page_denies_with_named_cause_without_mutating_predecessor();
    super::ownership_tests::pinned_entry_survives_page_saturation_and_unpinned_entry_is_evicted();
    super::ownership_tests::releasing_one_of_two_layout_pins_keeps_the_shared_entry_protected();
    super::ownership_tests::committed_hit_refreshes_recency_before_the_next_saturation_eviction();
    super::ownership_tests::released_pin_is_eligible_for_atomic_same_transaction_replacement();
    super::recovery_identity_tests::reconstructive_recovery_never_reuses_generation_entry_or_reservation_identity();
}
