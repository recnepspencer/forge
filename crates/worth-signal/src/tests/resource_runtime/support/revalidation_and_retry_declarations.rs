use super::*;

pub(in crate::tests::resource_runtime) fn forced_revalidation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_revalidation_policy(
        ResourceRevalidationPolicyDeclaration::ExplicitOrActiveHandleForced,
    )
}

pub(in crate::tests::resource_runtime) fn forced_revalidation_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
) -> ResourceNodeDeclaration {
    timeout_resource_declaration(node, timeout_ms).with_revalidation_policy(
        ResourceRevalidationPolicyDeclaration::ExplicitOrActiveHandleForced,
    )
}

pub(in crate::tests::resource_runtime) fn stale_after_revalidation_resource_declaration(
    node: NodeId,
    stale_after_ms: u64,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_stale_after_policy(ResourceStaleAfterPolicyDeclaration::RuntimeStaleAfter {
            stale_after: TemporalDuration::temporal_duration(stale_after_ms).unwrap(),
        })
        .with_revalidation_policy(
            ResourceRevalidationPolicyDeclaration::ExplicitOrStaleAfterFulfilled,
        )
}

pub(in crate::tests::resource_runtime) fn dependency_change_revalidation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_revalidation_policy(ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChange)
}

pub(in crate::tests::resource_runtime) fn observer_demand_revalidation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_revalidation_policy(ResourceRevalidationPolicyDeclaration::ExplicitOrObserverDemand)
}

pub(in crate::tests::resource_runtime) fn dependency_change_observer_demand_revalidation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_revalidation_policy(
        ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrObserverDemand,
    )
}

pub(in crate::tests::resource_runtime) fn terminal_state_revalidation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_revalidation_policy(ResourceRevalidationPolicyDeclaration::ExplicitOrTerminalState)
}

pub(in crate::tests::resource_runtime) fn fulfilled_lifecycle_revalidation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_revalidation_policy(
        ResourceRevalidationPolicyDeclaration::ExplicitOrFulfilledLifecycle,
    )
}

pub(in crate::tests::resource_runtime) fn exponential_retry_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
    initial_retry_delay_ms: u64,
    multiplier: u32,
) -> ResourceNodeDeclaration {
    timeout_resource_declaration(node, timeout_ms).with_retry_policy(
        ResourceRetryPolicyDeclaration::ExponentialBackoff {
            initial_delay: TemporalDuration::temporal_duration(initial_retry_delay_ms).unwrap(),
            multiplier,
        },
    )
}

pub(in crate::tests::resource_runtime) fn capped_retry_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
    initial_retry_delay_ms: u64,
    multiplier: u32,
    max_retry_delay_ms: u64,
) -> ResourceNodeDeclaration {
    timeout_resource_declaration(node, timeout_ms).with_retry_policy(
        ResourceRetryPolicyDeclaration::CappedExponentialBackoff {
            initial_delay: TemporalDuration::temporal_duration(initial_retry_delay_ms).unwrap(),
            multiplier,
            max_delay: TemporalDuration::temporal_duration(max_retry_delay_ms).unwrap(),
        },
    )
}

pub(in crate::tests::resource_runtime) fn retry_guarded_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
    retry_delay_ms: u64,
    max_attempts: u32,
    jitter_ms: u64,
) -> ResourceNodeDeclaration {
    retry_timeout_resource_declaration(node, timeout_ms, retry_delay_ms)
        .with_retry_max_attempts(max_attempts)
        .with_retry_deterministic_jitter(TemporalDuration::temporal_duration(jitter_ms).unwrap())
}

pub(in crate::tests::resource_runtime) fn retry_budgeted_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
    retry_delay_ms: u64,
    scope: ResourceRetryBudgetScope,
    retry_budget_limit: u32,
) -> ResourceNodeDeclaration {
    retry_timeout_resource_declaration(node, timeout_ms, retry_delay_ms)
        .with_retry_budget(scope, retry_budget_limit)
}

pub(in crate::tests::resource_runtime) fn retry_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
    retry_delay_ms: u64,
) -> ResourceNodeDeclaration {
    timeout_resource_declaration(node, timeout_ms).with_retry_policy(
        ResourceRetryPolicyDeclaration::FixedDelay {
            delay: TemporalDuration::temporal_duration(retry_delay_ms).unwrap(),
        },
    )
}

pub(in crate::tests::resource_runtime) fn retry_total_request_lifetime_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
    retry_delay_ms: u64,
) -> ResourceNodeDeclaration {
    total_request_lifetime_timeout_resource_declaration(node, timeout_ms).with_retry_policy(
        ResourceRetryPolicyDeclaration::FixedDelay {
            delay: TemporalDuration::temporal_duration(retry_delay_ms).unwrap(),
        },
    )
}
