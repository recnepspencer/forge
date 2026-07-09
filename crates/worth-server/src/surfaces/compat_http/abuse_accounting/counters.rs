use worth_foundational::facade::{
    counter_backed_performance_receipt, performance, performance_bundle,
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceContractName,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

use crate::{WorthServerBinaryCounterSet, WorthServerExternalCounterSet};

pub(crate) const BUDGET_CHECKS: &str = "compat_http.abuse.budget_checks";
pub(crate) const BUDGET_ADMITTED: &str = "compat_http.abuse.admitted";
pub(crate) const BUDGET_DENIED: &str = "compat_http.abuse.denied";
pub(crate) const TENANT_SCOPE_ASSERTIONS: &str = "compat_http.abuse.tenant_scope_assertions";
pub(crate) const ROUTE_FAMILY_ASSERTIONS: &str = "compat_http.abuse.route_family_assertions";
pub(crate) const BYTE_CLASS_ASSERTIONS: &str = "compat_http.abuse.byte_class_assertions";
pub(crate) const STRUCTURED_LANE_ASSERTIONS: &str = "compat_http.abuse.structured_lane_assertions";
pub(crate) const BINARY_LANE_ASSERTIONS: &str = "compat_http.abuse.binary_lane_assertions";
pub(crate) const METADATA_ONLY_ASSERTIONS: &str = "compat_http.abuse.metadata_only_assertions";
pub(crate) const SLOWLORIS_CUTOFFS: &str = "compat_http.transfer.slowloris_cutoffs";
pub(crate) const DISCONNECT_EVENTS: &str = "compat_http.transfer.disconnect_events";
pub(crate) const BACKPRESSURE_ABORTS: &str = "compat_http.transfer.backpressure_aborts";
pub(crate) const CALLER_CANCELLATIONS: &str = "compat_http.transfer.caller_cancellations";
pub(crate) const RETRY_EVENTS: &str = "compat_http.transfer.retry_events";
pub(crate) const EXPIRY_EVENTS: &str = "compat_http.transfer.expiry_events";
pub(crate) const STAGED_CLEANUP_EVENTS: &str = "compat_http.transfer.staged_cleanup_events";
pub(crate) const CLEANUP_OPERATIONS: &str = "compat_http.transfer.cleanup_operations";
pub(crate) const CLEANUP_STAGED_BYTES: &str = "compat_http.transfer.cleanup_staged_bytes";
pub(crate) const SEMANTIC_TRUTH_DRIFT: &str = "compat_http.transfer.semantic_truth_drift";

pub(crate) fn external_counter_set(
    contract_name: &'static str,
    rows: &[(&'static str, u64)],
) -> WorthServerExternalCounterSet {
    WorthServerExternalCounterSet::new(counter_receipt(contract_name, rows))
}

pub(crate) fn binary_counter_set(
    contract_name: &'static str,
    rows: &[(&'static str, u64)],
) -> WorthServerBinaryCounterSet {
    WorthServerBinaryCounterSet::new(counter_receipt(contract_name, rows))
}

fn counter_receipt(
    contract_name: &'static str,
    rows: &[(&'static str, u64)],
) -> FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim> {
    let claim = performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::WarmPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .exclude_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .exclude_work(FoundationalPerformanceWorkClass::ForensicParity)
        .finish()
        .expect("phase twelve abuse accounting claim should stay valid");
    let bundle = rows.iter().fold(
        performance_bundle(claim).attach_contract_name(
            FoundationalPerformanceContractName::new(contract_name)
                .expect("phase twelve accounting contract name should stay valid"),
        ),
        |bundle, (name, value)| bundle.attach_counter_spec(counter_spec(name, *value)),
    );
    let bundle = bundle
        .finish()
        .expect("phase twelve accounting bundle should stay valid");
    rows.iter()
        .fold(
            counter_backed_performance_receipt(bundle),
            |receipt, (name, value)| receipt.attach_counter_row(counter_row(name, *value)),
        )
        .finish()
        .expect("phase twelve accounting receipt should stay valid")
}

fn counter_spec(
    name: &'static str,
    expected_exact_count: u64,
) -> FoundationalPerformanceCounterSpec {
    FoundationalPerformanceCounterSpec::new(
        FoundationalPerformanceCounterName::new(name)
            .expect("phase twelve counter name should stay valid"),
        FoundationalPerformanceWorkClass::ValidationPlanning,
        expected_exact_count,
    )
}

fn counter_row(name: &'static str, observed_count: u64) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(
        FoundationalPerformanceCounterName::new(name)
            .expect("phase twelve counter name should stay valid"),
        observed_count,
    )
}
