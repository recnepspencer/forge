use worth_ui::facade::{
    WorthUiCandidateAdmissionCounters, WorthUiReloadCounterBoundary,
    WorthUiReloadCounterStopStage, WorthUiReloadLoweringFoundationalBridge,
};

fn main() {
    let receipt = WorthUiReloadCounterBoundary::stopped_at(
        WorthUiReloadCounterStopStage::CandidateAdmission,
    )
    .record_admission_counters(WorthUiCandidateAdmissionCounters::default())
    .seal()
    .expect("receipt seals");

    let _ = WorthUiReloadLoweringFoundationalBridge::lower(&receipt);
}
