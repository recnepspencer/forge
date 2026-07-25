use worth_query_declaration::facade::domain_computation::{
    WorthQueryCancellationSafePointFamily, WorthQueryExecutionDegradation, WorthQueryExecutionMode,
    WorthQueryExecutionResourceRequest, WorthQueryPartialEffectPosture,
    WorthQueryResourceDimension, WorthQueryResourceLimitRequest, WorthQuerySemanticScaleAxis,
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
        WorthQueryExecutionResourceSupport::new(
            provider(),
            access(),
            allocator(),
            envelope(limit),
            std::sync::Arc::new(
                WorthQueryFixedExecutionCapacity::mint("admission-test", 8).unwrap(),
            ),
        ),
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
            std::sync::Arc::new(
                WorthQueryFixedExecutionCapacity::mint("foreign-admission-test", 8).unwrap(),
            ),
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

#[test]
fn partial_effect_posture_is_admitted_independently_from_degradation() {
    let partial = envelope(8)
        .with_partial_effect_posture(WorthQueryPartialEffectPosture::PartialEffectsMayRemain);
    let contract =
        WorthQueryExecutionResourceContract::declared([WorthQueryExecutionStrategyContract::new(
            WorthQueryExecutionStrategyName::new("partial-effects").unwrap(),
            partial.clone(),
            WorthQueryExecutionProviderRequirements::new(provider(), access(), allocator()),
        )])
        .unwrap();
    let support = support_with_capacity(
        partial,
        std::sync::Arc::new(WorthQueryFixedExecutionCapacity::mint("partial-effect", 1).unwrap()),
    );

    let denial = admit_execution_resource_plan(
        "binding",
        &contract,
        &request(8),
        support.clone(),
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap_err();
    assert_eq!(
        denial.kind(),
        &WorthQueryExecutionResourceAdmissionDenialKind::PartialEffectPostureUnsupported
    );

    let admitted = admit_execution_resource_plan(
        "binding",
        &contract,
        &request(8)
            .allow_partial_effect_posture(WorthQueryPartialEffectPosture::PartialEffectsMayRemain),
        support,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap();
    assert_eq!(
        admitted.envelope().partial_effect_posture(),
        WorthQueryPartialEffectPosture::PartialEffectsMayRemain
    );
}

#[test]
fn every_named_degradation_requires_explicit_request_consent() {
    for degradation in [
        WorthQueryExecutionDegradation::PartialResult,
        WorthQueryExecutionDegradation::YieldedProgress,
        WorthQueryExecutionDegradation::RetainedProgress,
    ] {
        let degraded = WorthQueryExecutionResourceEnvelope::new(
            WorthQuerySemanticScaleRequest::bounded(8),
            WorthQueryResourceLimitRequest::bounded(8),
            WorthQueryExecutionMode::Synchronous,
            Some(degradation),
            safe_point(),
        );
        let contract = WorthQueryExecutionResourceContract::declared([
            WorthQueryExecutionStrategyContract::new(
                WorthQueryExecutionStrategyName::new(degradation.as_str()).unwrap(),
                degraded.clone(),
                WorthQueryExecutionProviderRequirements::new(provider(), access(), allocator()),
            ),
        ])
        .unwrap();
        let support = support_with_capacity(
            degraded,
            std::sync::Arc::new(
                WorthQueryFixedExecutionCapacity::mint(degradation.as_str(), 1).unwrap(),
            ),
        );

        let denied = admit_execution_resource_plan(
            "binding",
            &contract,
            &request(8),
            support.clone(),
            WorthQueryExecutionResourceAdmissionCounters::default(),
        )
        .unwrap_err();
        assert_eq!(
            denied.kind(),
            &WorthQueryExecutionResourceAdmissionDenialKind::DegradationPostureUnsupported
        );
        let admitted = admit_execution_resource_plan(
            "binding",
            &contract,
            &request(8).allow_degradation(degradation),
            support,
            WorthQueryExecutionResourceAdmissionCounters::default(),
        )
        .unwrap();
        assert_eq!(admitted.envelope().degradation(), Some(degradation));
    }
}

#[test]
fn one_axis_at_a_time_changes_only_the_resource_decision() {
    for axis in WorthQuerySemanticScaleAxis::ALL {
        let request = WorthQueryExecutionResourceRequest::new(
            WorthQuerySemanticScaleRequest::bounded(1).with(axis, 9),
            WorthQueryResourceLimitRequest::bounded(1),
            safe_point(),
        )
        .unwrap();
        assert_single_axis_denial(request);
    }
    for dimension in WorthQueryResourceDimension::ALL {
        let request = WorthQueryExecutionResourceRequest::new(
            WorthQuerySemanticScaleRequest::bounded(1),
            WorthQueryResourceLimitRequest::bounded(1).with(dimension, 9),
            safe_point(),
        )
        .unwrap();
        assert_single_axis_denial(request);
    }
}

fn assert_single_axis_denial(request: WorthQueryExecutionResourceRequest) {
    let denial = admit_execution_resource_plan(
        "binding",
        &contract(8),
        &request,
        support(8),
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap_err();
    assert_eq!(
        denial.kind(),
        &WorthQueryExecutionResourceAdmissionDenialKind::ResourceCeilingExceeded
    );
    assert_eq!(denial.counters().strategy_checks, 1);
    assert_eq!(denial.counters().envelope_dimension_checks, 31);
    assert_eq!(denial.counters().support_snapshot_checks, 0);
    assert_eq!(denial.counters().capacity_reservations, 0);
    assert_eq!(denial.counters().provider_session_mints, 0);
}

#[test]
fn consuming_capacity_reservation_saturates_and_drop_releases_the_exact_pool() {
    let capacity = std::sync::Arc::new(
        WorthQueryFixedExecutionCapacity::mint("one-shot-capacity", 1).unwrap(),
    );
    let support = support_with_capacity(envelope(8), capacity);
    let first = admit_execution_resource_plan(
        "binding",
        &contract(8),
        &request(8),
        support.clone(),
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap();
    let second = admit_execution_resource_plan(
        "binding",
        &contract(8),
        &request(8),
        support.clone(),
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap();
    let retry = admit_execution_resource_plan(
        "binding",
        &contract(8),
        &request(8),
        support,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap();

    let reserved = reserve_execution_resource_plan(first).expect("first arrival reserves");
    assert!(reserve_execution_resource_plan(second).is_none());
    drop(reserved);

    assert!(reserve_execution_resource_plan(retry).is_some());
}

#[test]
fn equal_capacity_labels_cannot_merge_distinct_provider_authorities() {
    let first: std::sync::Arc<dyn WorthQueryExecutionCapacityPort> =
        std::sync::Arc::new(WorthQueryFixedExecutionCapacity::new("shared-label", 1).unwrap());
    let second: std::sync::Arc<dyn WorthQueryExecutionCapacityPort> =
        std::sync::Arc::new(WorthQueryFixedExecutionCapacity::new("shared-label", 1).unwrap());
    let operation = admitted_with_support("operation", support_with_capacity(envelope(8), first));
    let stage = admitted_with_support(
        "operation:stage",
        support_with_capacity(envelope(8), second),
    );
    let workflow = WorthQueryAdmittedWorkflowResourcePlan::assemble(
        operation,
        [("stage".to_owned(), stage)].into_iter().collect(),
    );

    assert!(reserve_workflow_resource_plan(workflow).is_none());
}

fn admitted_with_support(
    binding: &str,
    support: WorthQueryExecutionResourceSupportSnapshot,
) -> WorthQueryAdmittedExecutionResourcePlan {
    admit_execution_resource_plan(
        binding,
        &contract(8),
        &request(8),
        support,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap()
}

fn support_with_capacity(
    resource_envelope: WorthQueryExecutionResourceEnvelope,
    capacity: std::sync::Arc<dyn WorthQueryExecutionCapacityPort>,
) -> WorthQueryExecutionResourceSupportSnapshot {
    WorthQueryExecutionResourceSupportSnapshot::new(
        WorthQueryExecutionResourceSupport::new(
            provider(),
            access(),
            allocator(),
            resource_envelope,
            capacity,
        ),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
    )
}
