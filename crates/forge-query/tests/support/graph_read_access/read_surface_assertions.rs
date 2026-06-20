use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadAccessComplexityCounters, ForgeQueryGraphReadAccessExecutionCounters,
    ForgeQueryGraphReadAccessReceiptSummary, ForgeQueryReadDenial, ForgeQueryRuntimeError,
};

pub fn read_composition_denial(error: ForgeQueryRuntimeError) -> ForgeQueryReadDenial {
    match error {
        ForgeQueryRuntimeError::ReadCompositionDenied(denial) => denial,
        other => panic!("expected read composition denial, got {other:?}"),
    }
}

pub fn assert_admitted_summary(summary: &ForgeQueryGraphReadAccessReceiptSummary) {
    assert!(summary.has_admitted_access_plan());
    assert!(!summary.plan_digest().is_empty());
    assert!(!summary.admission_digest().is_empty());
    assert!(!summary.requirement_set_digest().is_empty());
    assert!(!summary
        .graph_index_inventory_match_report_digest()
        .is_empty());
}

pub fn assert_success_counters_are_executor_observed(
    counters: &ForgeQueryGraphReadAccessComplexityCounters,
) {
    assert_eq!(counters.executor_entry_count(), 1);
    assert_eq!(counters.executor_strategy_rediscovery_count(), 0);
    assert_eq!(counters.edge_scan_execution_count(), 0);
    assert_eq!(counters.per_result_neighbor_lookup_count(), 0);
    assert_eq!(counters.persistent_artifact_bypass_count(), 0);
}

pub fn assert_pre_execution_graph_access_denial(
    denial: &ForgeQueryReadDenial,
) -> &ForgeQueryGraphReadAccessAdmission {
    let admission = denial
        .graph_read_access_admission()
        .expect("denial should carry graph read access admission");
    let counters = denial
        .graph_read_access_execution_counters()
        .expect("denial should carry pre-execution counters");

    assert!(!admission.is_admitted());
    assert_eq!(
        admission.posture(),
        &ForgeQueryGraphReadAccessAdmissionPosture::Denied
    );
    assert_pre_execution_counters(counters);
    admission
}

pub fn assert_pre_execution_counters(counters: &ForgeQueryGraphReadAccessExecutionCounters) {
    assert_eq!(counters.executor_entry_count(), 0);
    assert_eq!(counters.strategy_recompute_count(), 0);
    assert_eq!(counters.edge_scan_count(), 0);
    assert_eq!(counters.per_result_neighbor_lookup_count(), 0);
    assert_eq!(counters.persistent_artifact_bypass_count(), 0);
    assert_eq!(counters.materialized_row_count(), 0);
}
