use std::sync::Arc;

use worth_query_admission::facade::resource_admission::{
    WorthQueryAdmittedExecutionResourcePlan, WorthQueryExecutionResourceAdmissionCounters,
    WorthQueryExecutionResourceSupport, WorthQueryExecutionResourceSupportSnapshot,
    WorthQueryFixedExecutionCapacity,
};
use worth_query_admission::integration::admit_execution_resource_plan;
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

pub(super) fn admitted_plan(
    binding_identity: &str,
) -> (WorthQueryAdmittedExecutionResourcePlan, String) {
    admitted_plan_with_support_limit(binding_identity, 2)
}

pub(super) fn admitted_plan_with_support_limit(
    binding_identity: &str,
    support_limit: u64,
) -> (WorthQueryAdmittedExecutionResourcePlan, String) {
    let safe_point = WorthQueryCancellationSafePointFamily::new("operation-boundary").unwrap();
    let envelope = WorthQueryExecutionResourceEnvelope::new(
        WorthQuerySemanticScaleRequest::bounded(2),
        WorthQueryResourceLimitRequest::bounded(2),
        WorthQueryExecutionMode::Synchronous,
        None,
        safe_point.clone(),
    );
    let provider = WorthQueryExecutionProviderFamily::new("installed-provider").unwrap();
    let access = WorthQueryExecutionAccessProductFamily::new("installed-access").unwrap();
    let allocator = WorthQueryExecutionAllocatorFamily::new("installed-arena").unwrap();
    let contract =
        WorthQueryExecutionResourceContract::declared([WorthQueryExecutionStrategyContract::new(
            WorthQueryExecutionStrategyName::new("installed-strategy").unwrap(),
            envelope.clone(),
            WorthQueryExecutionProviderRequirements::new(
                provider.clone(),
                access.clone(),
                allocator.clone(),
            ),
        )])
        .unwrap();
    let contract_identity = contract.canonical_identity();
    let support = WorthQueryExecutionResourceSupportSnapshot::new(
        WorthQueryExecutionResourceSupport::new(
            provider,
            access,
            allocator,
            WorthQueryExecutionResourceEnvelope::new(
                WorthQuerySemanticScaleRequest::bounded(support_limit),
                WorthQueryResourceLimitRequest::bounded(support_limit),
                WorthQueryExecutionMode::Synchronous,
                None,
                safe_point.clone(),
            ),
            Arc::new(WorthQueryFixedExecutionCapacity::mint("operation-binding-test", 8).unwrap()),
        ),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        None,
    );
    let plan = admit_execution_resource_plan(
        binding_identity,
        &contract,
        &WorthQueryExecutionResourceRequest::bounded(2, 2, safe_point),
        support,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap();
    (plan, contract_identity)
}
