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

// runtime execution denials share one compiler process.
mod covered_001 { include!("../../runtime_reload_counter_boundary/fail/raw_reload_counter_receipt_cannot_lower_to_foundational.rs"); }
mod covered_002 { include!("../../runtime_steady_frame_counter_boundary/fail/raw_steady_frame_receipt_cannot_lower_to_foundational.rs"); }
