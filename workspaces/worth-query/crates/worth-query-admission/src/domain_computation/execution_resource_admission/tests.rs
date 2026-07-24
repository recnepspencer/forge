use worth_query_declaration::facade::domain_computation::{
    WorthQueryCancellationSafePointFamily, WorthQueryExecutionMode,
    WorthQueryExecutionResourceRequest, WorthQueryResourceLimitRequest,
    WorthQuerySemanticScaleRequest,
};
use worth_query_installation::facade::{
    WorthQueryExecutionAccessProductFamily, WorthQueryExecutionAllocatorFamily,
    WorthQueryExecutionProviderFamily, WorthQueryExecutionProviderRequirements,
    WorthQueryExecutionResourceContract, WorthQueryExecutionResourceEnvelope,
    WorthQueryExecutionStrategyContract, WorthQueryExecutionStrategyName,
};

use super::*;

fn safe_point() -> WorthQueryCancellationSafePointFamily {
    WorthQueryCancellationSafePointFamily::new("admission-chunk").unwrap()
}

fn envelope(limit: u64) -> WorthQueryExecutionResourceEnvelope {
    WorthQueryExecutionResourceEnvelope::new(
        WorthQuerySemanticScaleRequest::bounded(limit),
        WorthQueryResourceLimitRequest::bounded(limit),
        WorthQueryExecutionMode::Synchronous,
        None,
        safe_point(),
    )
}

fn provider() -> WorthQueryExecutionProviderFamily {
    WorthQueryExecutionProviderFamily::new("admission-provider").unwrap()
}

fn access() -> WorthQueryExecutionAccessProductFamily {
    WorthQueryExecutionAccessProductFamily::new("admission-access").unwrap()
}

fn allocator() -> WorthQueryExecutionAllocatorFamily {
    WorthQueryExecutionAllocatorFamily::new("admission-arena").unwrap()
}

fn contract(limit: u64) -> WorthQueryExecutionResourceContract {
    WorthQueryExecutionResourceContract::declared([WorthQueryExecutionStrategyContract::new(
        WorthQueryExecutionStrategyName::new("bounded").unwrap(),
        envelope(limit),
        WorthQueryExecutionProviderRequirements::new(provider(), access(), allocator()),
    )])
    .unwrap()
}

fn support(limit: u64) -> WorthQueryExecutionResourceSupportSnapshot {
    WorthQueryExecutionResourceSupportSnapshot::new(
        WorthQueryExecutionResourceSupport::new(provider(), access(), allocator(), envelope(limit)),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
    )
}

fn request(limit: u64) -> WorthQueryExecutionResourceRequest {
    WorthQueryExecutionResourceRequest::bounded(limit, limit, safe_point())
}

#[test]
fn exact_support_mints_one_resource_admission_plan() {
    let plan = admit_execution_resource_plan(
        "binding",
        &contract(8),
        &request(8),
        support(8),
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap();

    assert_eq!(
        plan.posture(),
        WorthQueryExecutionResourceAdmissionPosture::Exact
    );
    assert_eq!(plan.counters().resource_contract_lookups, 1);
    assert_eq!(plan.counters().support_snapshot_checks, 1);
    assert_eq!(plan.counters().strategy_checks, 1);
}

#[test]
fn over_budget_request_denies_before_support_inspection() {
    let denial = admit_execution_resource_plan(
        "binding",
        &contract(8),
        &request(9),
        support(8),
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        &WorthQueryExecutionResourceAdmissionDenialKind::ResourceCeilingExceeded
    );
    assert_eq!(denial.counters().support_snapshot_checks, 0);
}

#[test]
fn provider_mismatch_is_distinct_from_capacity_denial() {
    let mismatched = WorthQueryExecutionResourceSupportSnapshot::new(
        WorthQueryExecutionResourceSupport::new(
            WorthQueryExecutionProviderFamily::new("foreign-provider").unwrap(),
            access(),
            allocator(),
            envelope(8),
        ),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
    );
    let denial = admit_execution_resource_plan(
        "binding",
        &contract(8),
        &request(8),
        mismatched,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        &WorthQueryExecutionResourceAdmissionDenialKind::DifferentProviderRequired
    );
    assert_eq!(denial.counters().support_snapshot_checks, 1);
}
