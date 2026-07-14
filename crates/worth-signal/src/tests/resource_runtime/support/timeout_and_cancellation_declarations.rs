use super::*;

pub(in crate::tests::resource_runtime) fn lifecycle_only_observation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleOnly)
}

pub(in crate::tests::resource_runtime) fn denied_completion_observation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_observation_policy(
        ResourceObservationPolicyDeclaration::LifecycleOutputAndDeniedCompletion,
    )
}

pub(in crate::tests::resource_runtime) fn retry_schedule_observation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    retry_timeout_resource_declaration(node, 3, 7).with_observation_policy(
        ResourceObservationPolicyDeclaration::LifecycleOutputAndRetrySchedule,
    )
}

pub(in crate::tests::resource_runtime) fn timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_timeout_policy(ResourceTimeoutPolicyDeclaration::FixedTimeout {
        timeout: TemporalDuration::temporal_duration(timeout_ms).unwrap(),
    })
}

pub(in crate::tests::resource_runtime) fn total_request_lifetime_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::TotalRequestLifetimeTimeout {
            timeout: TemporalDuration::temporal_duration(timeout_ms).unwrap(),
        },
    )
}

pub(in crate::tests::resource_runtime) fn heartbeat_extension_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
    heartbeat_extension_ms: u64,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::ProgressHeartbeatExtension {
            timeout: TemporalDuration::temporal_duration(timeout_ms).unwrap(),
            heartbeat_extension: TemporalDuration::temporal_duration(heartbeat_extension_ms)
                .unwrap(),
        },
    )
}

pub(in crate::tests::resource_runtime) fn transaction_inherited_deadline_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_timeout_policy(ResourceTimeoutPolicyDeclaration::TransactionInheritedDeadline)
}

pub(in crate::tests::resource_runtime) fn runtime_inherited_deadline_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_timeout_policy(ResourceTimeoutPolicyDeclaration::RuntimeInheritedDeadline)
}

pub(in crate::tests::resource_runtime) fn runtime_denial_only_cancellation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_cancellation_policy(ResourceCancellationPolicyDeclaration::RuntimeDenialOnly)
}

pub(in crate::tests::resource_runtime) fn graceful_cancellation_resource_declaration(
    node: NodeId,
    grace_ms: u64,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_cancellation_grace_period(TemporalDuration::temporal_duration(grace_ms).unwrap())
}

pub(in crate::tests::resource_runtime) fn dependent_cancellation_resource_declaration(
    node: NodeId,
    dependents: impl IntoIterator<Item = NodeId>,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_declared_dependent_cancellation_nodes(
        dependents.into_iter().map(ResourceNodeId::from_node),
    )
}

pub(in crate::tests::resource_runtime) fn overlap_retained_host_work_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_supersession_policy(
        ResourceSupersessionPolicyDeclaration::OverlappingGenerationRetainsOldHostWork,
    )
}

pub(in crate::tests::resource_runtime) fn overlap_cancelled_host_work_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_supersession_policy(
        ResourceSupersessionPolicyDeclaration::OverlappingGenerationCancelsOldHostWork,
    )
}

pub(in crate::tests::resource_runtime) fn intent_equivalent_coalescing_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_supersession_policy(
        ResourceSupersessionPolicyDeclaration::IntentEquivalentCoalescesToActive,
    )
}

pub(in crate::tests::resource_runtime) fn retry_transaction_inherited_deadline_resource_declaration(
    node: NodeId,
    retry_delay_ms: u64,
) -> ResourceNodeDeclaration {
    transaction_inherited_deadline_resource_declaration(node).with_retry_policy(
        ResourceRetryPolicyDeclaration::FixedDelay {
            delay: TemporalDuration::temporal_duration(retry_delay_ms).unwrap(),
        },
    )
}

pub(in crate::tests::resource_runtime) fn terminal_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::TerminalTimeout {
            timeout: TemporalDuration::temporal_duration(timeout_ms).unwrap(),
        },
    )
}

pub(in crate::tests::resource_runtime) fn revalidation_eligible_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::RevalidationEligibleTimeout {
            timeout: TemporalDuration::temporal_duration(timeout_ms).unwrap(),
        },
    )
}
