use worth_ui::facade::{
    WorthUiFailedActivationReport, WorthUiReloadCheckedStopPosture, WorthUiReloadDenial,
    WorthUiReloadFailure, WorthUiReloadFailureCounters, WorthUiReloadFailureStage,
    WorthUiReloadPreservationReceipt,
};

fn observe_failure(failure: WorthUiReloadFailure) {
    let _ = failure.denial();
    let _ = failure.preservation_receipt();
    let _ = failure.failed_activation_report();
    let _ = failure.counters();
}

fn observe_denial(denial: WorthUiReloadDenial) {
    let _ = denial.stage();
    let _ = denial.upstream_evidence_digest();
    let _ = denial.checked_stop_posture();
}

fn observe_receipt(receipt: WorthUiReloadPreservationReceipt) {
    let _ = receipt.active_artifact_digest();
    let _ = receipt.active_plan_digest();
    let _ = receipt.active_snapshot_digest();
    let _ = receipt.active_lifecycle();
    let _ = receipt.active_status();
    let _ = receipt.active_frame_epoch();
    let _ = receipt.last_valid_artifact_digest();
    let _ = receipt.last_valid_plan_digest();
    let _ = receipt.last_valid_frame_epoch();
}

fn observe_report(report: WorthUiFailedActivationReport) {
    let _ = report.stage();
    let _ = report.checked_stop_posture();
    let _ = report.preserved_active_artifact_digest();
    let _ = report.preserved_active_plan_digest();
    let _ = report.fallback_runtime_created();
    let _ = report.counters();
}

fn observe_counters(counters: WorthUiReloadFailureCounters) {
    let _ = counters.preservation_receipt_count();
    let _ = counters.active_state_mutation_count();
    let _ = counters.durable_state_mutation_count();
    let _ = counters.query_binding_mutation_count();
    let _ = counters.fallback_runtime_creation_count();
    let _ = counters.source_reparse_count();
    let _ = counters.registry_rebuild_count();
    let _ = counters.semantic_replanning_count();
    let _ = counters.query_replanning_count();
}

fn observe_stage(stage: WorthUiReloadFailureStage) {
    let _ = stage;
}

fn observe_checked_stop(posture: WorthUiReloadCheckedStopPosture) {
    let _ = posture.is_query_checked_stop();
}

fn main() {}
