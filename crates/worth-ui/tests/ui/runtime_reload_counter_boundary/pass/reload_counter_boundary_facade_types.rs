use worth_ui::facade::{
    WorthUiCandidateAdmissionCounters, WorthUiReloadCounterBoundary, WorthUiReloadCounterStopStage,
};

fn main() {
    let receipt = WorthUiReloadCounterBoundary::stopped_at(
        WorthUiReloadCounterStopStage::CandidateAdmission,
    )
    .record_admission_counters(WorthUiCandidateAdmissionCounters::default())
    .seal();

    let _ = receipt;
}
