use crate::runtime::{
    WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessAdmissionPosture,
    WorthQueryGraphReadAccessComplexityCounters, WorthQueryGraphReadAccessExecutionCounters,
    WorthQueryGraphReadAccessReceiptSummary, WorthQueryReadDenial, WorthQueryRuntimeError,
};

pub fn read_composition_denial(error: WorthQueryRuntimeError) -> WorthQueryReadDenial {
    match error {
        WorthQueryRuntimeError::ReadCompositionDenied(denial) => denial,
        other => panic!("expected read composition denial, got {other:?}"),
    }
}

pub fn assert_admitted_summary(summary: &WorthQueryGraphReadAccessReceiptSummary) {
    assert!(summary.has_admitted_access_plan());
    assert!(!summary.plan_digest().is_empty());
    assert!(!summary.admission_digest().is_empty());
    assert!(!summary.requirement_set_digest().is_empty());
    assert!(!summary
        .graph_index_inventory_match_report_digest()
        .is_empty());
}

pub fn assert_success_counters_are_executor_observed(
    counters: &WorthQueryGraphReadAccessComplexityCounters,
) {
    assert_eq!(counters.executor_entry_count(), 1);
    assert_eq!(counters.executor_strategy_rediscovery_count(), 0);
    assert_eq!(counters.edge_scan_execution_count(), 0);
    assert_eq!(counters.per_result_neighbor_lookup_count(), 0);
    assert_eq!(counters.persistent_artifact_bypass_count(), 0);
}

pub fn assert_pre_execution_graph_access_denial(
    denial: &WorthQueryReadDenial,
) -> &WorthQueryGraphReadAccessAdmission {
    let admission = denial
        .graph_read_access_admission()
        .expect("denial should carry graph read access admission");
    let counters = denial
        .graph_read_access_execution_counters()
        .expect("denial should carry pre-execution counters");

    assert!(!admission.is_admitted());
    assert_eq!(
        admission.posture(),
        &WorthQueryGraphReadAccessAdmissionPosture::Denied
    );
    assert_pre_execution_counters(counters);
    admission
}

pub fn assert_pre_execution_counters(counters: &WorthQueryGraphReadAccessExecutionCounters) {
    assert_eq!(counters.executor_entry_count(), 0);
    assert_eq!(counters.strategy_recompute_count(), 0);
    assert_eq!(counters.edge_scan_count(), 0);
    assert_eq!(counters.per_result_neighbor_lookup_count(), 0);
    assert_eq!(counters.persistent_artifact_bypass_count(), 0);
    assert_eq!(counters.materialized_row_count(), 0);
}
