use std::collections::BTreeMap;

use worth_query_admission::facade::resource_admission::{
    admit_execution_resource_plan, WorthQueryAdmittedExecutionResourcePlan,
    WorthQueryAdmittedWorkflowResourcePlan, WorthQueryExecutionResourceAdmissionCounters,
    WorthQueryExecutionResourceSupport, WorthQueryExecutionResourceSupportSnapshot,
};
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

use super::{WorthQueryDirectExecutionResourceAttempt, WorthQueryWorkflowExecutionResourceAttempt};

fn safe_point() -> WorthQueryCancellationSafePointFamily {
    WorthQueryCancellationSafePointFamily::new("execution-chunk").unwrap()
}

fn provider() -> WorthQueryExecutionProviderFamily {
    WorthQueryExecutionProviderFamily::new("execution-provider").unwrap()
}

fn access() -> WorthQueryExecutionAccessProductFamily {
    WorthQueryExecutionAccessProductFamily::new("execution-access").unwrap()
}

fn allocator() -> WorthQueryExecutionAllocatorFamily {
    WorthQueryExecutionAllocatorFamily::new("execution-arena").unwrap()
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

fn admitted_plan(binding: &str, limit: u64) -> WorthQueryAdmittedExecutionResourcePlan {
    let contract =
        WorthQueryExecutionResourceContract::declared([WorthQueryExecutionStrategyContract::new(
            WorthQueryExecutionStrategyName::new("bounded").unwrap(),
            envelope(limit),
            WorthQueryExecutionProviderRequirements::new(provider(), access(), allocator()),
        )])
        .unwrap();
    let request = WorthQueryExecutionResourceRequest::bounded(limit, limit, safe_point());
    let support = WorthQueryExecutionResourceSupportSnapshot::new(
        WorthQueryExecutionResourceSupport::new(provider(), access(), allocator(), envelope(limit)),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
    );

    admit_execution_resource_plan(
        binding,
        &contract,
        &request,
        support,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap()
}

#[test]
fn direct_attempt_mints_one_session_and_binds_immutable_evidence() {
    let attempt = WorthQueryDirectExecutionResourceAttempt::start(admitted_plan("direct", 8));

    assert_eq!(attempt.resources().counters().provider_session_mints, 1);
    assert_eq!(
        attempt.provider_session().attempt_identity(),
        attempt.resources().identity()
    );
    assert_eq!(
        attempt.evidence().admission_identity(),
        attempt.resources().identity()
    );
    assert_eq!(
        attempt.evidence().provider_session_identity(),
        attempt.provider_session().identity()
    );
    assert_eq!(
        attempt.evidence().provider_session_attempt_identity(),
        attempt.provider_session().attempt_identity()
    );
}

#[test]
fn repeated_direct_attempts_receive_unique_provider_sessions() {
    let admitted = admitted_plan("repeat", 8);
    let first = WorthQueryDirectExecutionResourceAttempt::start(admitted.clone());
    let second = WorthQueryDirectExecutionResourceAttempt::start(admitted);

    assert_eq!(
        first.provider_session().attempt_identity(),
        second.provider_session().attempt_identity()
    );
    assert_ne!(
        first.provider_session().identity(),
        second.provider_session().identity()
    );
    assert_ne!(first.evidence().identity(), second.evidence().identity());
}

#[test]
fn workflow_attempt_mints_only_the_operation_session() {
    let operation = admitted_plan("workflow", 8);
    let stage = admitted_plan("workflow-stage", 4);
    let mut stages = BTreeMap::new();
    stages.insert("stage".to_owned(), stage);
    let attempt = WorthQueryWorkflowExecutionResourceAttempt::start(
        WorthQueryAdmittedWorkflowResourcePlan::new(operation, stages),
    );

    assert_eq!(attempt.resources().counters().provider_session_mints, 1);
    assert_eq!(
        attempt
            .operation_resources()
            .counters()
            .provider_session_mints,
        1
    );
    assert_eq!(
        attempt
            .resources()
            .stage("stage")
            .unwrap()
            .counters()
            .provider_session_mints,
        0
    );
    assert_eq!(
        attempt.provider_session().attempt_identity(),
        attempt.resources().identity()
    );
    assert_eq!(
        attempt.evidence().admission_identity(),
        attempt.operation_resources().identity()
    );
}
