use forge_runtime_bridge::facade::BridgeSubscriptionReferenceWorkloadInspection;
use std::sync::Arc;

fn main() {
    let _inspection = BridgeSubscriptionReferenceWorkloadInspection {
        reference_workload_report_digest: Arc::from("workload"),
        manifest_digest: Arc::from("manifest"),
        offline_audit_report_digest: Arc::from("audit"),
        outcome_summary_digest: Arc::from("summary"),
        coverage_report_digest: Arc::from("coverage"),
        counter_digest: Arc::from("counters"),
        lane_report_digests: Vec::new(),
        comparison_report_digests: Vec::new(),
        lane_report_count: 0,
        comparison_report_count: 0,
        host_log_dependency_count: 0,
        live_state_dependency_count: 0,
        canonical_basis: Arc::from("basis"),
        digest: Arc::from("digest"),
    };
}
