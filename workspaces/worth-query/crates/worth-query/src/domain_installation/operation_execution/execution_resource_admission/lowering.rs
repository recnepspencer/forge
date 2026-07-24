use worth_query_declaration::facade::domain_computation::{
    WorthQueryExecutionMode, WorthQueryExecutionResourceRequest, WorthQueryResourceDimension,
    WorthQuerySemanticScaleAxis,
};
use worth_query_installation::facade::{
    WorthQueryExecutionResourceContract, WorthQueryExecutionStrategyContract,
};

use super::{
    identity::admitted_plan_identity, WorthQueryAdmittedExecutionResourcePlan,
    WorthQueryExecutionResourceAdmissionCounters, WorthQueryExecutionResourceAdmissionDenial,
    WorthQueryExecutionResourceAdmissionDenialKind as Kind,
    WorthQueryExecutionResourceSupportSnapshot,
};

pub(crate) fn lower_execution_resource_plan(
    binding_identity: &str,
    contract: &WorthQueryExecutionResourceContract,
    request: &WorthQueryExecutionResourceRequest,
    support: WorthQueryExecutionResourceSupportSnapshot,
    mut counters: WorthQueryExecutionResourceAdmissionCounters,
) -> Result<WorthQueryAdmittedExecutionResourcePlan, WorthQueryExecutionResourceAdmissionDenial> {
    counters.resource_contract_lookups += 1;
    contract.validate().map_err(|detail| {
        WorthQueryExecutionResourceAdmissionDenial::new(Kind::ResourceContract, detail, counters)
    })?;
    let mut fitting = Vec::new();
    for strategy in contract.strategies() {
        counters.strategy_checks += 1;
        counters.envelope_dimension_checks +=
            WorthQuerySemanticScaleAxis::ALL.len() + WorthQueryResourceDimension::ALL.len();
        if strategy.envelope().admits(request) {
            fitting.push(strategy);
        }
    }
    for strategy in &fitting {
        counters.support_snapshot_checks += 1;
        if support.supports(strategy) {
            let request_identity = request.canonical_identity();
            let identity =
                admitted_plan_identity(binding_identity, &request_identity, &support, strategy);
            return Ok(WorthQueryAdmittedExecutionResourcePlan::new(
                identity,
                request_identity,
                support,
                (*strategy).clone(),
                counters,
            ));
        }
    }
    if let Some(strategy) = fitting.first() {
        return Err(support_mismatch(strategy, &support, counters));
    }
    if contract.strategies().iter().any(|strategy| {
        strategy.envelope().mode() == WorthQueryExecutionMode::Asynchronous
            && request
                .scale()
                .iter()
                .all(|(axis, value)| value <= strategy.envelope().scale_ceiling(axis))
            && request
                .limits()
                .iter()
                .all(|(dimension, value)| value <= strategy.envelope().resource_ceiling(dimension))
    }) {
        return Err(WorthQueryExecutionResourceAdmissionDenial::new(
            Kind::AsyncExecutionRequired,
            "the bounded request requires a declared asynchronous strategy",
            counters,
        ));
    }
    Err(WorthQueryExecutionResourceAdmissionDenial::new(
        Kind::ResourceCeilingExceeded,
        "no installed strategy admits every requested scale and resource dimension",
        counters,
    ))
}

fn support_mismatch(
    strategy: &WorthQueryExecutionStrategyContract,
    support: &WorthQueryExecutionResourceSupportSnapshot,
    counters: WorthQueryExecutionResourceAdmissionCounters,
) -> WorthQueryExecutionResourceAdmissionDenial {
    let required = strategy.provider_requirements();
    let actual = support.executor();
    let (kind, detail) = if actual.provider() != required.provider() {
        (
            Kind::DifferentProviderRequired,
            "installed provider family does not support the admitted strategy",
        )
    } else if actual.access_product() != required.access_product() {
        (
            Kind::DifferentAccessProductRequired,
            "installed access product does not support the admitted strategy",
        )
    } else if actual.allocator() != required.allocator() {
        (
            Kind::DifferentAllocatorRequired,
            "installed allocator family does not support the admitted strategy",
        )
    } else {
        (
            Kind::Backpressured,
            "provider support is currently below the strategy envelope",
        )
    };
    WorthQueryExecutionResourceAdmissionDenial::new(kind, detail, counters)
}
