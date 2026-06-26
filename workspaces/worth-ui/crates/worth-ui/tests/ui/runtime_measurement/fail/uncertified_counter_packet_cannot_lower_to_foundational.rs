use worth_ui::facade::{
    WorthUiFoundationalCounterBridge, WorthUiFrameCostCounter, WorthUiMeasurementBoundary,
    WorthUiRuntimeCounterFamily,
};

fn main() {
    let packet = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            1,
        ))
        .seal()
        .expect("packet seals");

    let _ = WorthUiFoundationalCounterBridge::lower_certified_packet(&packet);
}
