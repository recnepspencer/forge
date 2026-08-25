//! Independent model assertion for native atlas transactions.

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
