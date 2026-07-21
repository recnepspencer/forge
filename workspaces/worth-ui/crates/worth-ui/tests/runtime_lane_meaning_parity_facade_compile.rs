use worth_ui::facade::runtime::{
    WorthUiCrossLaneSemanticFamily, WorthUiCrossLaneSemanticReference, WorthUiLaneMeaningParity,
    WorthUiLaneParityCertification, WorthUiLaneParityCounters, WorthUiLaneParityDenial,
    WorthUiLaneParityDenialReason, WorthUiLaneParityReport, WorthUiLaneTransitionParity,
};

#[test]
fn lane_meaning_parity_types_are_facade_visible() {
    fn assert_visible<T>() {}

    assert_visible::<WorthUiCrossLaneSemanticFamily>();
    assert_visible::<WorthUiCrossLaneSemanticReference>();
    assert_visible::<WorthUiLaneMeaningParity>();
    assert_visible::<WorthUiLaneParityCertification>();
    assert_visible::<WorthUiLaneParityCounters>();
    assert_visible::<WorthUiLaneParityDenial>();
    assert_visible::<WorthUiLaneParityDenialReason>();
    assert_visible::<WorthUiLaneParityReport>();
    assert_visible::<WorthUiLaneTransitionParity>();
}
