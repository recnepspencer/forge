use worth_query_admission::facade::resource_admission::{
    WorthQueryAdmittedExecutionResourcePlan, WorthQueryExecutionResourceAdmissionCounters,
    WorthQueryExecutionResourceSupport, WorthQueryExecutionResourceSupportSnapshot,
    WorthQueryFixedExecutionCapacity,
};
use worth_query_admission::integration::{
    admit_execution_resource_plan, reserve_execution_resource_plan,
};
use worth_query_declaration::facade::domain_computation::{
    WorthQueryCancellationSafePointFamily, WorthQueryExecutionMode,
    WorthQueryExecutionResourceRequest, WorthQueryPartialEffectPosture,
    WorthQueryResourceDimension, WorthQueryResourceLimitRequest, WorthQueryRetainedProgressPosture,
    WorthQuerySemanticScaleRequest, WorthQueryYieldedStatePosture,
};
use worth_query_installation::facade::{
    WorthQueryExecutionAccessProductFamily, WorthQueryExecutionAllocatorFamily,
    WorthQueryExecutionProviderFamily, WorthQueryExecutionProviderRequirements,
    WorthQueryExecutionResourceContract, WorthQueryExecutionResourceEnvelope,
    WorthQueryExecutionStrategyContract, WorthQueryExecutionStrategyName,
    WorthQueryInstallationGeneration,
};

use super::WorthQueryDirectExecutionResourceAttempt;
use crate::domain_computation::execution_runtime::WorthQueryExecutionRuntimeInstaller;
use crate::domain_computation::operation_binding::direct_authority;

pub(super) fn runtime() -> crate::domain_computation::WorthQueryExecutionRuntime {
    WorthQueryExecutionRuntimeInstaller::new()
        .install(
            WorthQueryInstallationGeneration::initial(),
            std::iter::empty(),
        )
        .unwrap()
        .into_parts()
        .0
}

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
        WorthQueryResourceLimitRequest::bounded(limit)
            .with(WorthQueryResourceDimension::RetainedBytes, 4_096),
        WorthQueryExecutionMode::Synchronous,
        None,
        safe_point(),
    )
}

pub(crate) fn admitted_plan(binding: &str, limit: u64) -> WorthQueryAdmittedExecutionResourcePlan {
    admitted_plan_with_support(binding, limit, limit, std::iter::empty())
}

pub(crate) fn admitted_yield_plan(
    binding: &str,
    limit: u64,
) -> WorthQueryAdmittedExecutionResourcePlan {
    let executor = execution_resource_support_with_yield(binding, limit);
    admit_plan_with_supports(binding, limit, executor, Vec::new())
}

pub(crate) fn admitted_plan_with_graph_support(
    binding: &str,
    limit: u64,
    graph_role: &str,
    graph_support: WorthQueryExecutionResourceSupport,
) -> WorthQueryAdmittedExecutionResourcePlan {
    let executor =
        execution_resource_support_for_envelope(binding, graph_support.envelope().clone());
    admit_plan_with_supports(
        binding,
        limit,
        executor,
        vec![(graph_role.to_owned(), graph_support)],
    )
}

pub(crate) fn execution_resource_support(
    binding: &str,
    limit: u64,
) -> WorthQueryExecutionResourceSupport {
    execution_resource_support_for_envelope(binding, envelope(limit))
}

pub(crate) fn execution_resource_support_with_partial_effects(
    binding: &str,
    limit: u64,
) -> WorthQueryExecutionResourceSupport {
    execution_resource_support_for_envelope(
        binding,
        envelope(limit)
            .with_partial_effect_posture(WorthQueryPartialEffectPosture::PartialEffectsMayRemain),
    )
}

pub(crate) fn execution_resource_support_with_yield(
    binding: &str,
    limit: u64,
) -> WorthQueryExecutionResourceSupport {
    execution_resource_support_for_envelope(
        binding,
        envelope(limit)
            .with_yielded_state_posture(WorthQueryYieldedStatePosture::ProviderCheckpoint)
            .with_retained_progress_posture(
                WorthQueryRetainedProgressPosture::RetainAttemptCapacity,
            ),
    )
}

pub(crate) fn execution_resource_support_with_yield_and_partial_effects(
    binding: &str,
    limit: u64,
) -> WorthQueryExecutionResourceSupport {
    execution_resource_support_for_envelope(
        binding,
        envelope(limit)
            .with_partial_effect_posture(WorthQueryPartialEffectPosture::PartialEffectsMayRemain)
            .with_yielded_state_posture(WorthQueryYieldedStatePosture::ProviderCheckpoint)
            .with_retained_progress_posture(
                WorthQueryRetainedProgressPosture::RetainAttemptCapacity,
            ),
    )
}

pub(crate) fn execution_resource_support_for_envelope(
    binding: &str,
    envelope: WorthQueryExecutionResourceEnvelope,
) -> WorthQueryExecutionResourceSupport {
    WorthQueryExecutionResourceSupport::new(
        provider(),
        access(),
        allocator(),
        envelope,
        std::sync::Arc::new(
            WorthQueryFixedExecutionCapacity::new(format!("provider-session-test:{binding}"), 8)
                .unwrap(),
        ),
    )
}

fn admitted_plan_with_support<'a>(
    binding: &str,
    limit: u64,
    support_limit: u64,
    graph_roles: impl Iterator<Item = &'a str>,
) -> WorthQueryAdmittedExecutionResourcePlan {
    let executor = execution_resource_support(binding, support_limit);
    let graph_providers = graph_roles
        .map(|role| (role.to_owned(), executor.clone()))
        .collect();
    admit_plan_with_supports(binding, limit, executor, graph_providers)
}

fn admit_plan_with_supports(
    binding: &str,
    limit: u64,
    executor: WorthQueryExecutionResourceSupport,
    graph_providers: Vec<(String, WorthQueryExecutionResourceSupport)>,
) -> WorthQueryAdmittedExecutionResourcePlan {
    let envelope = executor.envelope().clone();
    let contract =
        WorthQueryExecutionResourceContract::declared([WorthQueryExecutionStrategyContract::new(
            WorthQueryExecutionStrategyName::new("bounded").unwrap(),
            envelope.clone(),
            WorthQueryExecutionProviderRequirements::new(
                executor.provider().clone(),
                executor.access_product().clone(),
                executor.allocator().clone(),
            ),
        )])
        .unwrap();
    let mut request = WorthQueryExecutionResourceRequest::bounded(
        limit,
        limit,
        envelope.cancellation_safe_point().clone(),
    )
    .allow_mode(envelope.mode())
    .allow_partial_effect_posture(envelope.partial_effect_posture())
    .allow_yielded_state_posture(envelope.yielded_state_posture())
    .allow_retained_progress_posture(envelope.retained_progress_posture());
    if let Some(degradation) = envelope.degradation() {
        request = request.allow_degradation(degradation);
    }
    let support = WorthQueryExecutionResourceSupportSnapshot::new(
        executor,
        Vec::new(),
        graph_providers,
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

pub(super) fn direct_attempt(
    binding: &str,
    limit: u64,
) -> WorthQueryDirectExecutionResourceAttempt {
    let resources = admitted_plan(binding, limit);
    let runtime = runtime();
    let authority = direct_authority(&runtime, &resources);
    let reserved = reserve_execution_resource_plan(resources).unwrap();
    WorthQueryDirectExecutionResourceAttempt::start(reserved, &authority)
}

#[test]
fn direct_attempt_mints_one_session_and_binds_immutable_evidence() {
    let attempt = direct_attempt("direct", 8);

    assert_eq!(attempt.resources().counters().provider_session_mints, 1);
    assert_eq!(
        attempt.provider_session().attempt_identity(),
        attempt.attempt_identity().as_str()
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
    let first = direct_attempt("repeat", 8);
    let second = direct_attempt("repeat", 8);

    assert_ne!(
        first.provider_session().attempt_identity(),
        second.provider_session().attempt_identity()
    );
    assert_eq!(
        first.provider_session().attempt_identity(),
        first.attempt_identity().as_str()
    );
    assert_eq!(
        second.provider_session().attempt_identity(),
        second.attempt_identity().as_str()
    );
    assert_ne!(
        first.provider_session().identity(),
        second.provider_session().identity()
    );
    assert_ne!(first.evidence().identity(), second.evidence().identity());
}

#[test]
fn provider_session_seals_direct_evidence_to_its_bound_operation() {
    let attempt = direct_attempt("direct-evidence", 8);
    let expected_basis_identity = attempt.binding_authority().basis_identity().to_owned();
    let binding = attempt
        .provider_session()
        .bind_direct_domain_evidence("snapshot:1", "output:1")
        .unwrap();

    assert_eq!(binding.binding_identity(), "direct-evidence");
    assert_eq!(binding.operation_identity(), "installed-operation");
    assert_eq!(binding.basis_identity(), expected_basis_identity);
    assert_eq!(binding.run_identity(), None);
    assert_eq!(binding.stage_identity(), None);
    assert_eq!(binding.execution_snapshot_identity(), "snapshot:1");
    assert_eq!(binding.output_occurrence_identity(), "output:1");
}
