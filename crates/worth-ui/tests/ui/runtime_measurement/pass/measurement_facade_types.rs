use worth_ui::facade::{
    WorthUiComplexityContract, WorthUiCounterCaptureRichness, WorthUiFoundationalCounterBridge,
    WorthUiFrameCostCounter, WorthUiMeasurementBoundary, WorthUiMeasurementQueryEvidence,
    WorthUiRuntimeCounterFamily,
};

fn main() {
    let packet = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            1,
        ))
        .with_capture_richness(WorthUiCounterCaptureRichness::Full)
        .with_query_evidence(WorthUiMeasurementQueryEvidence::subscription_selection_diagnostics(7))
        .seal()
        .expect("packet seals");

    let certified = packet
        .certify_against(
            WorthUiComplexityContract::hot_path("reload.candidate_admission")
                .requires_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
                .requires_counter_family(WorthUiRuntimeCounterFamily::reload_candidate_admission()),
        )
        .expect("certifies");

    let evidence = WorthUiFoundationalCounterBridge::lower_certified_packet(&certified)
        .expect("lowers");
    let _rows = evidence.counter_rows();
}
