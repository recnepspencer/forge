use std::sync::{Arc, Mutex};

use crate::data::resource::{
    built_in_policy_registrations, FrozenResourcePolicyDescriptorSet, LoweredResourcePolicyBundle,
    ResourceReplayAvailabilityClass, ResourceReplayAvailabilityDenialClass,
    ResourceReplayDecisionClass, ResourceReplayDecisionPlan, ResourceReplayPolicyDeclaration,
    ValidatedResourcePolicyDeclaration,
};
use crate::facade::*;
use crate::tests::support::version_ab;

use super::resource_closeout_assertions::{
    assert_hostile_evidence_shape, assert_performance_closeout_claim_shape,
    required_hostile_evidence_row, required_performance_claim_row, required_scenario_row,
};

type TestRuntime = SignalRuntime<(), (), (), (), ()>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResourceObservationRecord {
    observer_id: u64,
    handle_id: u64,
    matched_node_count: usize,
    touched: bool,
    recomputed: bool,
    meaningful_change: bool,
    trigger_matched: bool,
}

struct ResourceObservationListener {
    calls: Arc<Mutex<Vec<ResourceObservationRecord>>>,
}

impl ObservationListener<(), (), (), (), ()> for ResourceObservationListener {
    fn on_observation(
        &self,
        _ctx: ObservationReadContext<'_, (), (), (), (), ()>,
        notice: &ObservationNotice<'_>,
    ) {
        self.calls
            .lock()
            .expect("resource observation mutex poisoned")
            .push(ResourceObservationRecord {
                observer_id: notice.observer_id().get(),
                handle_id: notice.handle_id().get(),
                matched_node_count: notice.matched_nodes().len(),
                touched: notice.touched(),
                recomputed: notice.recomputed(),
                meaningful_change: notice.meaningful_change(),
                trigger_matched: notice.trigger_matched(),
            });
    }
}

fn resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    ResourceNodeDeclaration::new(
        ResourceNodeId::from_node(node),
        ResourcePayloadContract::new(ResourcePayloadContractId::new(7))
            .with_max_payload_bytes(1024),
    )
}

fn hide_pending_output_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_output_continuity_policy(ResourceOutputContinuityPolicyDeclaration::HideWhilePending)
}

fn hide_after_timeout_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_output_continuity_policy(ResourceOutputContinuityPolicyDeclaration::HideAfterTimeout)
}

fn hide_after_rejection_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node).with_output_continuity_policy(
        ResourceOutputContinuityPolicyDeclaration::HideAfterRejection,
    )
}

fn hide_after_cancellation_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node).with_output_continuity_policy(
        ResourceOutputContinuityPolicyDeclaration::HideAfterCancellation,
    )
}

fn hide_after_supersession_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node).with_output_continuity_policy(
        ResourceOutputContinuityPolicyDeclaration::HideAfterSupersession,
    )
}

fn retain_all_transitions_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::RetainAllTransitions)
}

fn terminal_summaries_only_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly)
}

fn compact_cancelled_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::CompactCancelled)
}

fn compact_superseded_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::CompactSuperseded)
}

fn retained_only_diagnostics_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly)
}

fn budgeted_diagnostics_resource_declaration(
    node: NodeId,
    max_replay_reconstruction_width: u32,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_diagnostics_policy(
        ResourceDiagnosticsPolicyDeclaration::BudgetedExpansion {
            max_replay_reconstruction_width,
        },
    )
}

fn forensic_diagnostics_resource_declaration(
    node: NodeId,
    max_replay_reconstruction_width: u32,
    max_forensic_reconstruction_width: u32,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_diagnostics_policy(
        ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
            max_replay_reconstruction_width,
            max_forensic_reconstruction_width,
        },
    )
}

fn deny_cold_diagnostics_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::DenyColdExpansion)
}

fn identical_only_replay_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node).with_replay_policy(ResourceReplayPolicyDeclaration::IdenticalOnly)
}

fn retention_only_replay_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_replay_policy(ResourceReplayPolicyDeclaration::CompatibleRetentionNarrowing)
}

fn diagnostics_only_replay_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_replay_policy(ResourceReplayPolicyDeclaration::CompatibleDiagnosticsRichnessChange)
}

fn parameter_expansion_only_replay_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_replay_policy(ResourceReplayPolicyDeclaration::CompatibleParameterExpansion)
}

fn deny_on_unknown_or_missing_replay_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_replay_policy(ResourceReplayPolicyDeclaration::DenyOnUnknownOrMissing)
}

fn compact_timed_out_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    timeout_resource_declaration(node, 3)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::CompactTimedOut)
}

fn compatible_policy_registry_for(
    kind: ResourcePolicyKind,
    semantic_name: &str,
) -> FrozenResourcePolicyRegistry {
    let registrations = built_in_policy_registrations()
        .into_iter()
        .map(|registration| {
            if registration.kind() == kind && registration.semantic_name().as_str() == semantic_name
            {
                ResourcePolicyRegistration::new(
                    registration.id(),
                    registration.kind(),
                    registration.semantic_name().clone(),
                    ResourcePolicyVersion::new(2, 0),
                    registration.cost_contract(),
                    ResourcePolicyCompatibilityPosture::CompatibleVersion,
                )
            } else {
                registration
            }
        })
        .collect();
    FrozenResourcePolicyRegistry::new(registrations).expect("compatible registry should freeze")
}

fn lifecycle_only_observation_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleOnly)
}

fn denied_completion_observation_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node).with_observation_policy(
        ResourceObservationPolicyDeclaration::LifecycleOutputAndDeniedCompletion,
    )
}

fn retry_schedule_observation_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    retry_timeout_resource_declaration(node, 3, 7).with_observation_policy(
        ResourceObservationPolicyDeclaration::LifecycleOutputAndRetrySchedule,
    )
}

fn timeout_resource_declaration(node: NodeId, timeout_ms: u64) -> ResourceNodeDeclaration {
    resource_declaration(node).with_timeout_policy(ResourceTimeoutPolicyDeclaration::FixedTimeout {
        timeout: TemporalDuration::temporal_duration(timeout_ms).unwrap(),
    })
}

fn total_request_lifetime_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::TotalRequestLifetimeTimeout {
            timeout: TemporalDuration::temporal_duration(timeout_ms).unwrap(),
        },
    )
}

fn heartbeat_extension_timeout_resource_declaration(
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

fn transaction_inherited_deadline_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_timeout_policy(ResourceTimeoutPolicyDeclaration::TransactionInheritedDeadline)
}

fn runtime_inherited_deadline_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_timeout_policy(ResourceTimeoutPolicyDeclaration::RuntimeInheritedDeadline)
}

fn runtime_denial_only_cancellation_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_cancellation_policy(ResourceCancellationPolicyDeclaration::RuntimeDenialOnly)
}

fn graceful_cancellation_resource_declaration(
    node: NodeId,
    grace_ms: u64,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_cancellation_grace_period(TemporalDuration::temporal_duration(grace_ms).unwrap())
}

fn dependent_cancellation_resource_declaration(
    node: NodeId,
    dependents: impl IntoIterator<Item = NodeId>,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_declared_dependent_cancellation_nodes(
        dependents.into_iter().map(ResourceNodeId::from_node),
    )
}

fn overlap_retained_host_work_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node).with_supersession_policy(
        ResourceSupersessionPolicyDeclaration::OverlappingGenerationRetainsOldHostWork,
    )
}

fn overlap_cancelled_host_work_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node).with_supersession_policy(
        ResourceSupersessionPolicyDeclaration::OverlappingGenerationCancelsOldHostWork,
    )
}

fn intent_equivalent_coalescing_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node).with_supersession_policy(
        ResourceSupersessionPolicyDeclaration::IntentEquivalentCoalescesToActive,
    )
}

fn retry_transaction_inherited_deadline_resource_declaration(
    node: NodeId,
    retry_delay_ms: u64,
) -> ResourceNodeDeclaration {
    transaction_inherited_deadline_resource_declaration(node).with_retry_policy(
        ResourceRetryPolicyDeclaration::FixedDelay {
            delay: TemporalDuration::temporal_duration(retry_delay_ms).unwrap(),
        },
    )
}

fn terminal_timeout_resource_declaration(node: NodeId, timeout_ms: u64) -> ResourceNodeDeclaration {
    resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::TerminalTimeout {
            timeout: TemporalDuration::temporal_duration(timeout_ms).unwrap(),
        },
    )
}

fn revalidation_eligible_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::RevalidationEligibleTimeout {
            timeout: TemporalDuration::temporal_duration(timeout_ms).unwrap(),
        },
    )
}

fn forced_revalidation_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node).with_revalidation_policy(
        ResourceRevalidationPolicyDeclaration::ExplicitOrActiveHandleForced,
    )
}

fn forced_revalidation_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
) -> ResourceNodeDeclaration {
    timeout_resource_declaration(node, timeout_ms).with_revalidation_policy(
        ResourceRevalidationPolicyDeclaration::ExplicitOrActiveHandleForced,
    )
}

fn stale_after_revalidation_resource_declaration(
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

fn dependency_change_revalidation_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_revalidation_policy(ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChange)
}

fn observer_demand_revalidation_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_revalidation_policy(ResourceRevalidationPolicyDeclaration::ExplicitOrObserverDemand)
}

fn dependency_change_observer_demand_revalidation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_revalidation_policy(
        ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrObserverDemand,
    )
}

fn terminal_state_revalidation_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_revalidation_policy(ResourceRevalidationPolicyDeclaration::ExplicitOrTerminalState)
}

fn fulfilled_lifecycle_revalidation_resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    resource_declaration(node).with_revalidation_policy(
        ResourceRevalidationPolicyDeclaration::ExplicitOrFulfilledLifecycle,
    )
}

fn exponential_retry_timeout_resource_declaration(
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

fn capped_retry_timeout_resource_declaration(
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

fn retry_guarded_timeout_resource_declaration(
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

fn retry_budgeted_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
    retry_delay_ms: u64,
    scope: ResourceRetryBudgetScope,
    retry_budget_limit: u32,
) -> ResourceNodeDeclaration {
    retry_timeout_resource_declaration(node, timeout_ms, retry_delay_ms)
        .with_retry_budget(scope, retry_budget_limit)
}

fn retry_timeout_resource_declaration(
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

fn retry_total_request_lifetime_timeout_resource_declaration(
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

fn schedule_timed_out_retry(
    runtime: &mut TestRuntime,
    node: NodeId,
) -> ResourceRetryScheduleReport {
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit for retry scheduling")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached before retry scheduling");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach timeout before retry scheduling");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should become ready before retry scheduling");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should succeed before retry scheduling");

    runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("timed-out retry should return a schedule report")
}

#[test]
fn resource_policy_lowering_records_built_in_descriptor_identity() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    runtime
        .declare_resource_node(retry_timeout_resource_declaration(first, 3, 7))
        .expect("first declaration should lower through built-in policy registry");
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(second, 5, 7))
        .expect("second declaration should lower through built-in policy registry");

    let first_descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(first))
        .expect("first descriptor should exist");
    let second_descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(second))
        .expect("second descriptor should exist");

    assert_eq!(
        first_descriptor
            .resolved_policy_bundle()
            .retry()
            .descriptor()
            .semantic_name()
            .as_str(),
        "signal.resource.retry.fixed-delay"
    );
    assert_eq!(
        first_descriptor
            .resolved_policy_bundle()
            .timeout()
            .parameter_digest()
            .as_str(),
        "timeout:fixed-timeout:3"
    );
    assert_eq!(
        first_descriptor
            .cancellation_decision_plan()
            .semantic_name(),
        "signal.resource.cancellation.best-effort-host-signal-and-runtime-denial"
    );
    assert_eq!(
        first_descriptor
            .supersession_decision_plan()
            .semantic_name(),
        "signal.resource.supersession.new-generation-supersedes-prior"
    );
    assert_eq!(
        first_descriptor
            .supersession_decision_plan()
            .overlap_disposition(),
        ResourceSupersessionOverlapDisposition::NoOverlapAdmission
    );
    assert_eq!(
        first_descriptor
            .supersession_decision_plan()
            .old_host_work_posture(),
        ResourceSupersessionOldHostWorkPosture::LeaveRunning
    );
    assert_ne!(
        first_descriptor
            .resolved_policy_bundle()
            .bundle_digest()
            .as_str(),
        second_descriptor
            .resolved_policy_bundle()
            .bundle_digest()
            .as_str()
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_resolution_count,
        2
    );
}

#[test]
fn resource_diagnostics_policy_budget_parameter_changes_frozen_descriptor_digest() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(first, 2))
        .expect("first diagnostics declaration should lower");
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(second, 8))
        .expect("second diagnostics declaration should lower");

    let first_descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(first))
        .expect("first descriptor should exist");
    let second_descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(second))
        .expect("second descriptor should exist");

    assert_eq!(
        first_descriptor.diagnostics_decision_plan().descriptor_id(),
        second_descriptor
            .diagnostics_decision_plan()
            .descriptor_id(),
        "budgeted diagnostics should share one built-in descriptor identity"
    );
    assert_ne!(
        first_descriptor
            .lowered_policy_bundle()
            .diagnostics()
            .frozen_digest(),
        second_descriptor
            .lowered_policy_bundle()
            .diagnostics()
            .frozen_digest(),
        "changing diagnostics budget must change the frozen descriptor digest"
    );
}

#[test]
fn resource_retry_decision_plan_scales_fixed_exponential_and_capped_backoff() {
    let mut graph = SignalGraph::new();
    let fixed = graph.node().build();
    let exponential = graph.node().build();
    let capped = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    runtime
        .declare_resource_node(retry_timeout_resource_declaration(fixed, 3, 7))
        .expect("fixed-delay retry declaration should lower");
    runtime
        .declare_resource_node(exponential_retry_timeout_resource_declaration(
            exponential,
            3,
            2,
            2,
        ))
        .expect("exponential retry declaration should lower");
    runtime
        .declare_resource_node(capped_retry_timeout_resource_declaration(
            capped, 3, 3, 3, 10,
        ))
        .expect("capped retry declaration should lower");
    let fixed_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(fixed)))
        .expect("fixed request should admit")
        .admitted_request()
        .handle();
    let exponential_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            exponential,
        )))
        .expect("exponential request should admit")
        .admitted_request()
        .handle();
    let capped_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            capped,
        )))
        .expect("capped request should admit")
        .admitted_request()
        .handle();

    let fixed_plan = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(fixed))
        .expect("fixed descriptor should exist")
        .retry_decision_plan();
    assert_eq!(fixed_plan.class(), ResourceRetryDecisionClass::FixedDelay);
    assert_eq!(
        fixed_plan
            .delay_for_attempt(fixed_handle, ResourceAttemptId::ZERO)
            .expect("fixed delay should exist")
            .get(),
        7
    );
    assert_eq!(
        fixed_plan
            .delay_for_attempt(fixed_handle, ResourceAttemptId::new(4))
            .expect("fixed delay should stay constant")
            .get(),
        7
    );

    let exponential_plan = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(exponential))
        .expect("exponential descriptor should exist")
        .retry_decision_plan();
    assert_eq!(
        exponential_plan.class(),
        ResourceRetryDecisionClass::ExponentialBackoff
    );
    assert_eq!(
        exponential_plan
            .delay_for_attempt(exponential_handle, ResourceAttemptId::ZERO)
            .expect("initial exponential delay should exist")
            .get(),
        2
    );
    assert_eq!(
        exponential_plan
            .delay_for_attempt(exponential_handle, ResourceAttemptId::new(1))
            .expect("second exponential delay should exist")
            .get(),
        4
    );
    assert_eq!(
        exponential_plan
            .delay_for_attempt(exponential_handle, ResourceAttemptId::new(2))
            .expect("third exponential delay should exist")
            .get(),
        8
    );

    let capped_plan = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(capped))
        .expect("capped descriptor should exist")
        .retry_decision_plan();
    assert_eq!(
        capped_plan.class(),
        ResourceRetryDecisionClass::CappedExponentialBackoff
    );
    assert_eq!(
        capped_plan
            .delay_for_attempt(capped_handle, ResourceAttemptId::ZERO)
            .expect("initial capped delay should exist")
            .get(),
        3
    );
    assert_eq!(
        capped_plan
            .delay_for_attempt(capped_handle, ResourceAttemptId::new(1))
            .expect("second capped delay should exist")
            .get(),
        9
    );
    assert_eq!(
        capped_plan
            .delay_for_attempt(capped_handle, ResourceAttemptId::new(2))
            .expect("capped delay should saturate to max")
            .get(),
        10
    );
}

#[test]
fn resource_supersession_decision_plan_records_overlap_and_old_host_work_posture() {
    let mut graph = SignalGraph::new();
    let retain = graph.node().build();
    let cancel = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    runtime
        .declare_resource_node(overlap_retained_host_work_resource_declaration(retain))
        .expect("retained-host-work overlap declaration should lower");
    runtime
        .declare_resource_node(overlap_cancelled_host_work_resource_declaration(cancel))
        .expect("old-host-work cancel overlap declaration should lower");

    let retain_descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(retain))
        .expect("retained-host-work descriptor should exist");
    let cancel_descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(cancel))
        .expect("cancel-host-work descriptor should exist");

    assert_eq!(
        retain_descriptor
            .supersession_decision_plan()
            .semantic_name(),
        "signal.resource.supersession.overlapping-generation-retains-old-host-work"
    );
    assert_eq!(
        retain_descriptor
            .supersession_decision_plan()
            .overlap_disposition(),
        ResourceSupersessionOverlapDisposition::ExplicitOverlapAdmission
    );
    assert_eq!(
        retain_descriptor
            .supersession_decision_plan()
            .old_host_work_posture(),
        ResourceSupersessionOldHostWorkPosture::LeaveRunning
    );
    assert_eq!(
        cancel_descriptor
            .supersession_decision_plan()
            .semantic_name(),
        "signal.resource.supersession.overlapping-generation-cancels-old-host-work"
    );
    assert_eq!(
        cancel_descriptor
            .supersession_decision_plan()
            .old_host_work_posture(),
        ResourceSupersessionOldHostWorkPosture::AdvisoryCancelRequested
    );
}

#[test]
fn resource_retry_freeze_digest_tracks_max_attempts_and_deterministic_jitter() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let third = graph.node().build();
    let fourth = graph.node().build();
    let fifth = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    runtime
        .declare_resource_node(retry_guarded_timeout_resource_declaration(
            first, 3, 7, 3, 5,
        ))
        .expect("guarded retry declaration should lower");
    runtime
        .declare_resource_node(retry_guarded_timeout_resource_declaration(
            second, 3, 7, 4, 5,
        ))
        .expect("max-attempt drift declaration should lower");
    runtime
        .declare_resource_node(retry_guarded_timeout_resource_declaration(
            third, 3, 7, 3, 6,
        ))
        .expect("jitter drift declaration should lower");
    runtime
        .declare_resource_node(
            retry_guarded_timeout_resource_declaration(fourth, 3, 7, 3, 5)
                .with_retry_budget(ResourceRetryBudgetScope::Runtime, 2),
        )
        .expect("runtime budget declaration should lower");
    runtime
        .declare_resource_node(
            retry_guarded_timeout_resource_declaration(fifth, 3, 7, 3, 5)
                .with_retry_budget(ResourceRetryBudgetScope::ResourceNode, 2),
        )
        .expect("node budget declaration should lower");

    let first_bundle = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(first))
        .expect("first descriptor should exist")
        .resolved_policy_bundle()
        .clone();
    let second_bundle = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(second))
        .expect("second descriptor should exist")
        .resolved_policy_bundle()
        .clone();
    let third_bundle = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(third))
        .expect("third descriptor should exist")
        .resolved_policy_bundle()
        .clone();
    let fourth_bundle = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(fourth))
        .expect("fourth descriptor should exist")
        .resolved_policy_bundle()
        .clone();
    let fifth_bundle = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(fifth))
        .expect("fifth descriptor should exist")
        .resolved_policy_bundle()
        .clone();

    assert_ne!(
        first_bundle.retry().parameter_digest().as_str(),
        second_bundle.retry().parameter_digest().as_str()
    );
    assert_ne!(
        first_bundle.retry().parameter_digest().as_str(),
        third_bundle.retry().parameter_digest().as_str()
    );
    assert_ne!(
        first_bundle.retry().frozen_digest().as_str(),
        second_bundle.retry().frozen_digest().as_str()
    );
    assert_ne!(
        first_bundle.retry().frozen_digest().as_str(),
        third_bundle.retry().frozen_digest().as_str()
    );
    assert_ne!(
        fourth_bundle.retry().parameter_digest().as_str(),
        fifth_bundle.retry().parameter_digest().as_str()
    );
    assert_ne!(
        fourth_bundle.retry().frozen_digest().as_str(),
        fifth_bundle.retry().frozen_digest().as_str()
    );
}

#[test]
fn resource_retry_attempt_limit_denies_before_temporal_wake_allocation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            retry_timeout_resource_declaration(node, 3, 7).with_retry_max_attempts(1),
        )
        .expect("attempt-limited retry declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake");

    let report = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("attempt-limit denial should remain report-shaped");
    let denied = report
        .denied_retry()
        .expect("attempt-limited retry should deny before wake allocation");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryAttemptLimitReached
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 0);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_attempt_limit_denial_count,
        1
    );
}

#[test]
fn resource_retry_runtime_budget_exhaustion_denies_before_temporal_wake_allocation() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            first,
            3,
            7,
            ResourceRetryBudgetScope::Runtime,
            1,
        ))
        .expect("first runtime-budget retry declaration should lower");
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            second,
            3,
            7,
            ResourceRetryBudgetScope::Runtime,
            1,
        ))
        .expect("second runtime-budget retry declaration should lower");

    let first_report = schedule_timed_out_retry(&mut runtime, first);
    let first_scheduled = first_report
        .scheduled_retry()
        .expect("first runtime-budget retry should schedule");
    assert_eq!(
        first_scheduled.retry_budget_scope(),
        Some(ResourceRetryBudgetScope::Runtime)
    );
    assert_eq!(first_scheduled.retry_budget_limit(), Some(1));
    assert_eq!(first_scheduled.retry_budget_usage(), Some(1));
    assert_eq!(
        first_report.performance().retry_budget_scope_touch_count(),
        1
    );

    let second_report = schedule_timed_out_retry(&mut runtime, second);
    let denied = second_report
        .denied_retry()
        .expect("second runtime-budget retry should deny");
    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryBudgetExhausted
    );
    assert_eq!(
        denied.retry_budget_scope(),
        Some(ResourceRetryBudgetScope::Runtime)
    );
    assert_eq!(denied.retry_budget_limit(), Some(1));
    assert_eq!(denied.retry_budget_usage(), Some(1));
    assert_eq!(second_report.performance().temporal_wake_footprint(), 0);
    assert_eq!(
        second_report.performance().retry_budget_scope_touch_count(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_budget_exhaustion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_budget_scope_touch_count,
        2
    );
}

#[test]
fn resource_retry_node_budget_scope_accumulates_across_requests_for_same_node() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            node,
            3,
            7,
            ResourceRetryBudgetScope::ResourceNode,
            1,
        ))
        .expect("node-budget retry declaration should lower");

    let first_report = schedule_timed_out_retry(&mut runtime, node);
    assert!(first_report.scheduled_retry().is_some());

    let second_report = schedule_timed_out_retry(&mut runtime, node);
    let denied = second_report
        .denied_retry()
        .expect("second node-budget retry should deny");
    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryBudgetExhausted
    );
    assert_eq!(
        denied.retry_budget_scope(),
        Some(ResourceRetryBudgetScope::ResourceNode)
    );
    assert_eq!(denied.retry_budget_limit(), Some(1));
    assert_eq!(denied.retry_budget_usage(), Some(1));
    assert_eq!(second_report.performance().temporal_wake_footprint(), 0);
}

#[test]
fn resource_retry_request_budget_scope_is_isolated_across_distinct_lineages() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            first,
            3,
            7,
            ResourceRetryBudgetScope::Request,
            1,
        ))
        .expect("first request-budget retry declaration should lower");
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            second,
            3,
            7,
            ResourceRetryBudgetScope::Request,
            1,
        ))
        .expect("second request-budget retry declaration should lower");

    let first_report = schedule_timed_out_retry(&mut runtime, first);
    let second_report = schedule_timed_out_retry(&mut runtime, second);

    assert_eq!(
        first_report
            .scheduled_retry()
            .expect("first request-budget retry should schedule")
            .retry_budget_scope(),
        Some(ResourceRetryBudgetScope::Request)
    );
    assert_eq!(
        second_report
            .scheduled_retry()
            .expect("second request-budget retry should schedule")
            .retry_budget_scope(),
        Some(ResourceRetryBudgetScope::Request)
    );
}

#[test]
fn resource_retry_request_budget_scope_accumulates_across_retry_lineage() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_budgeted_timeout_resource_declaration(
            node,
            3,
            7,
            ResourceRetryBudgetScope::Request,
            1,
        ))
        .expect("request-budget retry declaration should lower");

    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach first timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("first timeout admission should succeed");

    let first_schedule = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("first retry schedule should stay report-shaped");
    let first_scheduled = first_schedule
        .scheduled_retry()
        .expect("first request-budget retry should schedule");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(
                runtime
                    .clock_basis()
                    .current_tick()
                    .get()
                    .saturating_add(first_scheduled.scheduled_delay().get()),
            ),
        ))
        .expect("clock should reach retry backoff");
    let ready_retry = runtime
        .promote_temporal_wake_ready(first_scheduled.backoff_wake_id())
        .expect("retry backoff wake should become ready");
    let retry_report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), ready_retry)
        .expect("scheduled retry should admit");
    let retry = retry_report
        .admitted_retry()
        .expect("scheduled retry should produce admitted artifact");
    let retried_handle = retry.admitted_request().handle();
    let retry_timeout_wake = runtime
        .in_flight_resource_request(retried_handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("retried request should receive timeout wake");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach second timeout");
    let retry_ready_timeout = runtime
        .promote_temporal_wake_ready(retry_timeout_wake)
        .expect("retried timeout wake should become ready");
    runtime
        .admit_resource_timeout(retried_handle, retry_ready_timeout)
        .expect("retried timeout admission should succeed");

    let second_schedule = runtime
        .schedule_resource_retry(retried_handle, ResourceRetryReason::TimedOut)
        .expect("second retry schedule should stay report-shaped");
    let denied = second_schedule
        .denied_retry()
        .expect("request-budget retry should deny once the lineage budget is spent");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryBudgetExhausted
    );
    assert_eq!(
        denied.retry_budget_scope(),
        Some(ResourceRetryBudgetScope::Request)
    );
    assert_eq!(denied.retry_budget_limit(), Some(1));
    assert_eq!(denied.retry_budget_usage(), Some(1));
    assert_eq!(second_schedule.performance().temporal_wake_footprint(), 0);
}

#[test]
fn resource_retry_deterministic_jitter_is_stable_for_same_lineage_and_preserved_across_restore() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = retry_guarded_timeout_resource_declaration(node, 3, 7, 4, 5);

    let mut first_runtime = TestRuntime::build(graph.clone());
    first_runtime
        .declare_resource_node(declaration.clone())
        .expect("first jitter declaration should lower");
    let first_schedule = schedule_timed_out_retry(&mut first_runtime, node);
    let first_scheduled = first_schedule
        .scheduled_retry()
        .expect("first jitter retry should schedule");

    let mut second_runtime = TestRuntime::build(graph);
    second_runtime
        .declare_resource_node(declaration)
        .expect("second jitter declaration should lower");
    let second_schedule = schedule_timed_out_retry(&mut second_runtime, node);
    let second_scheduled = second_schedule
        .scheduled_retry()
        .expect("second jitter retry should schedule");

    assert_eq!(
        first_scheduled.scheduled_delay().get(),
        second_scheduled.scheduled_delay().get()
    );
    assert_eq!(
        first_scheduled.policy_decision_digest().as_str(),
        second_scheduled.policy_decision_digest().as_str()
    );
    assert_eq!(first_scheduled.previous(), second_scheduled.previous());
    assert_eq!(
        first_runtime
            .telemetry()
            .resource
            .resource_retry_jitter_decision_count,
        1
    );
    assert_eq!(
        second_runtime
            .telemetry()
            .resource
            .resource_retry_jitter_decision_count,
        1
    );

    let snapshot = first_runtime.capture_snapshot();
    first_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("post-snapshot mutation should succeed");
    first_runtime
        .restore_snapshot(&snapshot)
        .expect("restore should preserve pending retry schedule");
    first_runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3u64.saturating_add(first_scheduled.scheduled_delay().get())),
        ))
        .expect("clock should reach restored jitter backoff");

    let restored_schedule = first_runtime
        .promote_temporal_wake_ready(first_scheduled.backoff_wake_id())
        .expect("restored jitter wake should still become ready");
    assert_eq!(restored_schedule.id(), first_scheduled.backoff_wake_id());
    assert_eq!(
        first_runtime
            .telemetry()
            .resource
            .resource_retry_policy_decision_count,
        1
    );
}

#[test]
fn resource_named_retry_and_timeout_policies_deny_before_descriptor_lowering() {
    let mut graph = SignalGraph::new();
    let retry_node = graph.node().build();
    let timeout_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    let retry_error = runtime
        .declare_resource_node(resource_declaration(retry_node).with_retry_policy(
            ResourceRetryPolicyDeclaration::Named {
                name: ResourcePolicyName::new("signal.resource.retry.fixed-delay"),
            },
        ))
        .expect_err("named retry policy should deny in the first ship runtime");
    assert!(
        retry_error
            .to_string()
            .contains("not executable in the first ship runtime"),
        "unexpected retry error: {retry_error}"
    );

    let timeout_error = runtime
        .declare_resource_node(resource_declaration(timeout_node).with_timeout_policy(
            ResourceTimeoutPolicyDeclaration::Named {
                name: ResourcePolicyName::new("signal.resource.timeout.fixed-timeout"),
            },
        ))
        .expect_err("named timeout policy should deny in the first ship runtime");
    assert!(
        timeout_error
            .to_string()
            .contains("not executable in the first ship runtime"),
        "unexpected timeout error: {timeout_error}"
    );
}

#[test]
fn resource_policy_unknown_named_policy_denies_before_descriptor_allocation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let declaration =
        resource_declaration(node).with_retry_policy(ResourceRetryPolicyDeclaration::Named {
            name: ResourcePolicyName::new("example.resource.retry.unregistered"),
        });

    let err = runtime
        .declare_resource_node(declaration)
        .expect_err("unknown named retry policy should deny declaration");

    assert!(err
        .to_string()
        .contains("example.resource.retry.unregistered"));
    assert_eq!(runtime.resource_runtime_summary().descriptor_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_resolution_denial_count,
        1
    );
}

#[test]
fn resource_policy_validation_binds_lowered_bundle_to_registry_digest() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = retry_timeout_resource_declaration(node, 3, 7);
    let registry = FrozenResourcePolicyRegistry::built_in();

    let validated = ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &registry)
        .expect("built-in declaration should validate");
    let frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(&validated, &registry)
            .expect("validated declaration should freeze against the same registry digest");
    let lowered = LoweredResourcePolicyBundle::from_frozen_descriptors(&frozen);

    assert_eq!(
        validated.registry_digest().as_str(),
        registry.registry_digest().as_str()
    );
    assert_eq!(
        frozen.registry_digest().as_str(),
        registry.registry_digest().as_str()
    );
    assert_eq!(
        lowered.registry_digest().as_str(),
        registry.registry_digest().as_str()
    );
    assert_eq!(
        lowered.retry().descriptor().semantic_name().as_str(),
        "signal.resource.retry.fixed-delay"
    );
    assert_eq!(
        lowered.timeout().parameter_digest().as_str(),
        "timeout:fixed-timeout:3"
    );
}

#[test]
fn resource_policy_freeze_digest_changes_when_parameters_change() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let registry = FrozenResourcePolicyRegistry::built_in();
    let first_declaration = timeout_resource_declaration(first, 3);
    let second_declaration = timeout_resource_declaration(second, 9);

    let first_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&first_declaration, &registry)
            .expect("first declaration should validate");
    let second_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&second_declaration, &registry)
            .expect("second declaration should validate");
    let first_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(&first_validated, &registry)
            .expect("first declaration should freeze");
    let second_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(&second_validated, &registry)
            .expect("second declaration should freeze");
    let first_lowered = LoweredResourcePolicyBundle::from_frozen_descriptors(&first_frozen);
    let second_lowered = LoweredResourcePolicyBundle::from_frozen_descriptors(&second_frozen);

    assert_eq!(
        first_frozen
            .timeout()
            .descriptor()
            .descriptor_digest()
            .as_str(),
        second_frozen
            .timeout()
            .descriptor()
            .descriptor_digest()
            .as_str()
    );
    assert_ne!(
        first_frozen.timeout().parameter_digest().as_str(),
        second_frozen.timeout().parameter_digest().as_str()
    );
    assert_ne!(
        first_frozen.timeout().frozen_digest().as_str(),
        second_frozen.timeout().frozen_digest().as_str()
    );
    assert_ne!(
        first_lowered.bundle_digest().as_str(),
        second_lowered.bundle_digest().as_str()
    );

    let scoped_declaration = total_request_lifetime_timeout_resource_declaration(first, 3);
    let scoped_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&scoped_declaration, &registry)
            .expect("scoped declaration should validate");
    let scoped_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(&scoped_validated, &registry)
            .expect("scoped declaration should freeze");
    let scoped_lowered = LoweredResourcePolicyBundle::from_frozen_descriptors(&scoped_frozen);

    assert_ne!(
        first_frozen.timeout().parameter_digest().as_str(),
        scoped_frozen.timeout().parameter_digest().as_str()
    );
    assert_ne!(
        first_frozen.timeout().descriptor().semantic_name().as_str(),
        scoped_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str()
    );
    assert_ne!(
        first_lowered.bundle_digest().as_str(),
        scoped_lowered.bundle_digest().as_str()
    );

    let heartbeat_declaration = heartbeat_extension_timeout_resource_declaration(first, 3, 5);
    let heartbeat_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&heartbeat_declaration, &registry)
            .expect("heartbeat declaration should validate");
    let heartbeat_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &heartbeat_validated,
        &registry,
    )
    .expect("heartbeat declaration should freeze");
    let heartbeat_lowered = LoweredResourcePolicyBundle::from_frozen_descriptors(&heartbeat_frozen);
    assert_ne!(
        first_frozen.timeout().parameter_digest().as_str(),
        heartbeat_frozen.timeout().parameter_digest().as_str()
    );
    assert_ne!(
        heartbeat_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str(),
        first_frozen.timeout().descriptor().semantic_name().as_str()
    );
    assert_ne!(
        heartbeat_lowered.bundle_digest().as_str(),
        first_lowered.bundle_digest().as_str()
    );

    let terminal_declaration = terminal_timeout_resource_declaration(first, 3);
    let terminal_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&terminal_declaration, &registry)
            .expect("terminal timeout declaration should validate");
    let terminal_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &terminal_validated,
        &registry,
    )
    .expect("terminal timeout declaration should freeze");
    assert_ne!(
        first_frozen.timeout().descriptor().semantic_name().as_str(),
        terminal_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str()
    );

    let revalidation_declaration = revalidation_eligible_timeout_resource_declaration(first, 3);
    let revalidation_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&revalidation_declaration, &registry)
            .expect("revalidation eligible timeout declaration should validate");
    let revalidation_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &revalidation_validated,
        &registry,
    )
    .expect("revalidation eligible timeout declaration should freeze");
    assert_ne!(
        terminal_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str(),
        revalidation_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str()
    );

    let transaction_deadline_declaration =
        transaction_inherited_deadline_resource_declaration(first);
    let transaction_deadline_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &transaction_deadline_declaration,
        &registry,
    )
    .expect("transaction inherited deadline declaration should validate");
    let transaction_deadline_frozen =
        FrozenResourcePolicyDescriptorSet::from_validated_declaration(
            &transaction_deadline_validated,
            &registry,
        )
        .expect("transaction inherited deadline declaration should freeze");
    assert_ne!(
        transaction_deadline_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str(),
        first_frozen.timeout().descriptor().semantic_name().as_str()
    );

    let runtime_deadline_declaration = runtime_inherited_deadline_resource_declaration(first);
    let runtime_deadline_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &runtime_deadline_declaration,
        &registry,
    )
    .expect("runtime inherited deadline declaration should validate");
    let runtime_deadline_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &runtime_deadline_validated,
        &registry,
    )
    .expect("runtime inherited deadline declaration should freeze");
    assert_ne!(
        transaction_deadline_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str(),
        runtime_deadline_frozen
            .timeout()
            .descriptor()
            .semantic_name()
            .as_str()
    );
}

#[test]
fn resource_policy_malformed_named_policy_denies_before_descriptor_allocation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let declaration =
        resource_declaration(node).with_retry_policy(ResourceRetryPolicyDeclaration::Named {
            name: ResourcePolicyName::new("   "),
        });

    let err = runtime
        .declare_resource_node(declaration)
        .expect_err("malformed named policy should deny declaration");

    assert!(err
        .to_string()
        .contains("malformed resource policy descriptor"));
    assert_eq!(runtime.resource_runtime_summary().descriptor_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_resolution_denial_count,
        1
    );
}

#[test]
fn resource_policy_missing_builtin_descriptor_denies_during_validation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = resource_declaration(node);
    let registrations: Vec<_> = built_in_policy_registrations()
        .into_iter()
        .filter(|registration: &ResourcePolicyRegistration| {
            !matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (ResourcePolicyKind::Retry, "signal.resource.retry.disabled")
            )
        })
        .collect();
    let registry = FrozenResourcePolicyRegistry::new(registrations)
        .expect("custom registry should freeze without one built-in");

    let err = ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &registry)
        .expect_err("missing built-in descriptor should deny validation");

    assert_eq!(
        err,
        ResourcePolicyResolutionError::MissingDescriptor {
            kind: ResourcePolicyKind::Retry,
            name: ResourcePolicyName::new("signal.resource.retry.disabled"),
        }
    );
}

#[test]
fn resource_policy_incompatible_named_descriptor_denies_during_validation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration =
        resource_declaration(node).with_retry_policy(ResourceRetryPolicyDeclaration::Named {
            name: ResourcePolicyName::new("example.resource.retry.incompatible"),
        });
    let mut registrations = built_in_policy_registrations();
    registrations.push(ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(400),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.incompatible"),
        ResourcePolicyVersion::new(2, 0),
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::IncompatibleVersion,
    ));
    let registry =
        FrozenResourcePolicyRegistry::new(registrations).expect("custom registry should freeze");

    let err = ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &registry)
        .expect_err("incompatible named descriptor should deny validation");

    assert_eq!(
        err,
        ResourcePolicyResolutionError::IncompatibleDescriptor {
            kind: ResourcePolicyKind::Retry,
            name: ResourcePolicyName::new("example.resource.retry.incompatible"),
            version: ResourcePolicyVersion::new(2, 0),
            compatibility_posture: ResourcePolicyCompatibilityPosture::IncompatibleVersion,
        }
    );
}

#[test]
fn resource_policy_registry_rejects_duplicate_descriptor_ids() {
    let first = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(99),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.first"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );
    let second = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(99),
        ResourcePolicyKind::Timeout,
        ResourcePolicyName::new("example.resource.timeout.second"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(4),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );

    let err = FrozenResourcePolicyRegistry::new(vec![first, second])
        .expect_err("duplicate policy descriptor ids must deny registry construction");

    assert_eq!(
        err,
        ResourcePolicyRegistryError::DuplicateId(ResourcePolicyDescriptorId::new(99))
    );
}

#[test]
fn resource_policy_registry_digest_is_canonical_across_registration_order() {
    let retry = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(100),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.fixed"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );
    let timeout = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(101),
        ResourcePolicyKind::Timeout,
        ResourcePolicyName::new("example.resource.timeout.fixed"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(4),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );
    let cancellation = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(102),
        ResourcePolicyKind::Cancellation,
        ResourcePolicyName::new("example.resource.cancellation.runtime"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(3),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );

    let forward = FrozenResourcePolicyRegistry::new(vec![
        retry.clone(),
        timeout.clone(),
        cancellation.clone(),
    ])
    .expect("first registry should freeze");
    let reversed = FrozenResourcePolicyRegistry::new(vec![cancellation, timeout, retry])
        .expect("equivalent registry should freeze");

    assert_eq!(forward.descriptor_count(), 3);
    assert_eq!(forward.freeze_report().descriptor_count(), 3);
    assert_eq!(forward.freeze_report().id_index_width(), 3);
    assert_eq!(forward.freeze_report().kind_name_index_width(), 3);
    assert_eq!(
        forward.registry_digest().as_str(),
        reversed.registry_digest().as_str()
    );
    assert_eq!(
        forward.freeze_report().registry_digest().as_str(),
        reversed.freeze_report().registry_digest().as_str()
    );
}

#[test]
fn resource_policy_registry_rejects_duplicate_kind_and_semantic_name() {
    let first = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(110),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.same-name"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );
    let second = ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(111),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.same-name"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(6),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );

    let err = FrozenResourcePolicyRegistry::new(vec![first, second])
        .expect_err("duplicate policy semantic names must deny registry construction");

    assert_eq!(
        err,
        ResourcePolicyRegistryError::DuplicateName {
            kind: ResourcePolicyKind::Retry,
            name: ResourcePolicyName::new("example.resource.retry.same-name")
        }
    );
}

#[test]
fn resource_policy_registry_rejects_malformed_semantic_name() {
    let err = FrozenResourcePolicyRegistry::new(vec![ResourcePolicyRegistration::new(
        ResourcePolicyDescriptorId::new(120),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new(" "),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    )])
    .expect_err("blank semantic names must deny registry construction");

    assert_eq!(
        err,
        ResourcePolicyRegistryError::MalformedDescriptor {
            kind: ResourcePolicyKind::Retry,
            name: ResourcePolicyName::new(" "),
            reason: "resource policy name must not be empty",
        }
    );
}

#[test]
fn resource_policy_incompatible_builtin_descriptor_denies_during_validation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = resource_declaration(node);
    let registrations: Vec<_> = built_in_policy_registrations()
        .into_iter()
        .map(|registration| {
            if matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (ResourcePolicyKind::Retry, "signal.resource.retry.disabled")
            ) {
                ResourcePolicyRegistration::new(
                    registration.id(),
                    registration.kind(),
                    registration.semantic_name().clone(),
                    ResourcePolicyVersion::new(2, 1),
                    registration.cost_contract(),
                    ResourcePolicyCompatibilityPosture::IncompatibleVersion,
                )
            } else {
                registration
            }
        })
        .collect();
    let registry =
        FrozenResourcePolicyRegistry::new(registrations).expect("custom registry should freeze");

    let err = ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &registry)
        .expect_err("incompatible built-in descriptor should deny validation");

    assert_eq!(
        err,
        ResourcePolicyResolutionError::IncompatibleDescriptor {
            kind: ResourcePolicyKind::Retry,
            name: ResourcePolicyName::new("signal.resource.retry.disabled"),
            version: ResourcePolicyVersion::new(2, 1),
            compatibility_posture: ResourcePolicyCompatibilityPosture::IncompatibleVersion,
        }
    );
}

#[test]
fn resource_policy_freeze_denies_registry_digest_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = retry_timeout_resource_declaration(node, 3, 7);
    let validated_registry = FrozenResourcePolicyRegistry::built_in();
    let drifted_registrations = built_in_policy_registrations()
        .into_iter()
        .map(|registration| {
            if matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout"
                )
            ) {
                ResourcePolicyRegistration::new(
                    registration.id(),
                    registration.kind(),
                    registration.semantic_name().clone(),
                    ResourcePolicyVersion::new(2, 0),
                    registration.cost_contract(),
                    registration.compatibility_posture(),
                )
            } else {
                registration
            }
        })
        .collect();
    let drifted_registry = FrozenResourcePolicyRegistry::new(drifted_registrations)
        .expect("alternate registry should still freeze");

    let validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &validated_registry)
            .expect("declaration should validate against the original registry");
    let err = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &validated,
        &drifted_registry,
    )
    .expect_err("freeze must deny when the registry digest drifts after validation");

    assert_eq!(
        err,
        ResourcePolicyResolutionError::RegistryDigestDrift {
            expected: validated_registry.registry_digest().clone(),
            actual: drifted_registry.registry_digest().clone(),
        }
    );
}

#[test]
fn resource_policy_compatibility_accepts_exact_descriptor_match() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let declaration = retry_timeout_resource_declaration(node, 3, 7);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");

    let report = runtime
        .classify_resource_policy_compatibility(&declaration)
        .expect("identical declaration should classify as exactly compatible");

    assert!(report.is_compatible());
    assert_eq!(report.compared_width(), 10);
    assert_eq!(report.incompatible_width(), 0);
    assert_eq!(
        report.historical_registry_digest().as_str(),
        report.current_registry_digest().as_str()
    );
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::ExactDescriptorMatch
    );
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .historical_version(),
        ResourcePolicyVersion::INITIAL
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_compatibility_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_comparison_count,
        10
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        0
    );
}

#[test]
fn resource_policy_restore_compatibility_admits_exact_descriptor_match() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let declaration = retry_timeout_resource_declaration(node, 3, 7);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(&declaration)
        .expect("declared node should classify")
        .expect("identical declaration should admit restore compatibility proof");

    assert!(proof.compatibility().is_compatible());
    assert_eq!(proof.compatibility().compared_width(), 10);
    assert_eq!(
        proof.compatibility_digest().as_str(),
        proof.compatibility().compatibility_digest().as_str()
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_compatibility_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatibility_decision_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatible_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_incompatible_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_policy_decision_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        0
    );
}

#[test]
fn resource_policy_compatibility_denies_parameter_digest_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("resource declaration should lower");

    let report = runtime
        .classify_resource_policy_compatibility(&timeout_resource_declaration(node, 9))
        .expect("compatibility classification should still produce a report");

    assert!(!report.is_compatible());
    assert_eq!(report.incompatible_width(), 1);
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::ParameterDigestDrift
    );
    assert_eq!(
        report.historical_registry_digest().as_str(),
        report.current_registry_digest().as_str()
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatibility_decision_count,
        0
    );
}

#[test]
fn resource_policy_restore_compatibility_denies_parameter_drift_before_current_policy_code_executes(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("resource declaration should lower");

    let denial = runtime
        .admit_resource_policy_restore_compatibility(&timeout_resource_declaration(node, 9))
        .expect("declared node should classify")
        .expect_err("parameter drift must deny restore compatibility");

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ParameterDigestDrift
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Timeout)
    );
    assert_eq!(denial.incompatible_width(), 1);
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::ParameterDigestDrift
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatibility_decision_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatible_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_incompatible_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_missing_policy_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_policy_decision_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        0
    );
}

#[test]
fn resource_policy_restore_compatibility_admits_retention_narrowing_with_unavailable_rich_history()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.terminal-summaries-only",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(node))
        .expect("historical resource declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(&terminal_summaries_only_resource_declaration(
            node,
        ))
        .expect("declared node should classify")
        .expect("compatible retention narrowing should admit restore proof");

    let retention = proof
        .compatibility()
        .family(ResourcePolicyKind::Retention)
        .expect("retention family report should exist");
    assert_eq!(
        retention.class(),
        ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing
    );
    assert_eq!(
        retention.historical_retention_class(),
        Some(ResourceRetentionDecisionClass::RetainAllTransitions)
    );
    assert_eq!(
        retention.current_retention_class(),
        Some(ResourceRetentionDecisionClass::TerminalSummariesOnly)
    );
    assert!(retention.canonical_truth_preserved());
    assert!(retention.retained_history_unavailable());
    assert!(!retention.diagnostics_details_unavailable());
    assert_eq!(proof.retained_history_unavailable_width(), 1);
    assert_eq!(proof.diagnostics_details_unavailable_width(), 0);
    assert_eq!(proof.canonical_truth_preserved_width(), 10);
    assert_eq!(
        proof.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatibility_decision_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatible_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        0
    );
}

#[test]
fn resource_policy_restore_compatibility_admits_diagnostics_richness_change_with_explicit_availability_posture(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.retained-only",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical diagnostics declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(
            &retained_only_diagnostics_resource_declaration(node),
        )
        .expect("declared node should classify")
        .expect("compatible diagnostics change should admit restore proof");

    let diagnostics = proof
        .compatibility()
        .family(ResourcePolicyKind::Diagnostics)
        .expect("diagnostics family report should exist");
    assert_eq!(
        diagnostics.class(),
        ResourcePolicyCompatibilityClass::CompatibleDiagnosticsRichnessChange
    );
    assert_eq!(
        diagnostics.historical_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::BudgetedExpansion)
    );
    assert_eq!(
        diagnostics.current_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::RetainedOnly)
    );
    assert!(diagnostics.canonical_truth_preserved());
    assert!(!diagnostics.retained_history_unavailable());
    assert!(diagnostics.diagnostics_details_unavailable());
    assert_eq!(proof.retained_history_unavailable_width(), 0);
    assert_eq!(proof.diagnostics_details_unavailable_width(), 1);
    assert_eq!(
        proof.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatible_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_descriptor_incompatibility_count,
        0
    );
}

#[test]
fn resource_policy_restore_compatibility_replay_policy_can_deny_retention_narrowing() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.terminal-summaries-only",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(node))
        .expect("historical declaration should lower");

    let denial = runtime
        .admit_resource_policy_restore_compatibility(
            &identical_only_replay_resource_declaration(node)
                .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly),
        )
        .expect("declared node should classify")
        .expect_err("identical-only replay policy should deny retention narrowing");

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Retention)
    );
    assert_eq!(
        denial.replay_decision_class(),
        ResourceReplayDecisionClass::IdenticalOnly
    );
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Retention)
            .expect("retention family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing
    );
}

#[test]
fn resource_policy_restore_compatibility_replay_policy_can_deny_diagnostics_richness_change() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.retained-only",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");

    let denial = runtime
        .admit_resource_policy_restore_compatibility(
            &retention_only_replay_resource_declaration(node)
                .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly),
        )
        .expect("declared node should classify")
        .expect_err("retention-only replay policy should deny diagnostics richness change");

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Diagnostics)
    );
    assert_eq!(
        denial.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleRetentionNarrowing
    );
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Diagnostics)
            .expect("diagnostics family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::CompatibleDiagnosticsRichnessChange
    );
}

#[test]
fn resource_policy_restore_compatibility_proof_constructor_rejects_replay_gated_compatible_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.terminal-summaries-only",
    );
    let historical_declaration = retain_all_transitions_resource_declaration(node);
    let current_declaration = identical_only_replay_resource_declaration(node)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly);
    let historical_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &historical_declaration,
        &compatible_registry,
    )
    .expect("historical declaration should validate");
    let historical_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_validated,
        &compatible_registry,
    )
    .expect("historical declaration should freeze");
    let historical_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_frozen);
    let current_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &current_declaration,
        &compatible_registry,
    )
    .expect("current declaration should validate");
    let current_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &current_validated,
        &compatible_registry,
    )
    .expect("current declaration should freeze");
    let replay_plan = ResourceReplayDecisionPlan::lower(
        current_validated.declaration().replay_policy(),
        current_frozen.replay(),
    )
    .expect("replay plan should lower");
    let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(91),
        ResourceNodeId::from_node(node),
        &historical_lowered,
        &current_validated,
        &compatible_registry,
    )
    .expect("compatibility classification should succeed");

    assert!(report.is_compatible());
    assert!(
        ResourcePolicyRestoreCompatibilityProof::from_compatibility(report, &replay_plan).is_err(),
        "proof constructor must reject replay-gated compatible drift"
    );
}

#[test]
fn resource_policy_restore_compatibility_denies_retention_widening_even_under_compatible_posture() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.retain-all-transitions",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(terminal_summaries_only_resource_declaration(node))
        .expect("historical declaration should lower");

    let denial = runtime
        .admit_resource_policy_restore_compatibility(&retain_all_transitions_resource_declaration(
            node,
        ))
        .expect("declared node should classify")
        .expect_err("retention widening should still deny restore compatibility");

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::DecisionSemanticsDrift
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Retention)
    );
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Retention)
            .expect("retention family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::DecisionSemanticsDrift
    );
}

#[test]
fn resource_policy_restore_compatibility_retention_narrowing_names_exact_retention_posture() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.compact-cancelled",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(node))
        .expect("historical declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(&compact_cancelled_resource_declaration(node))
        .expect("declared node should classify")
        .expect("compatible retention narrowing should admit restore proof");
    let retention = proof
        .compatibility()
        .family(ResourcePolicyKind::Retention)
        .expect("retention family report should exist");

    assert_eq!(
        retention.class(),
        ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing
    );
    assert_eq!(
        retention.historical_retention_class(),
        Some(ResourceRetentionDecisionClass::RetainAllTransitions)
    );
    assert_eq!(
        retention.current_retention_class(),
        Some(ResourceRetentionDecisionClass::CompactCancelled)
    );
    assert!(retention.retained_history_unavailable());
}

#[test]
fn resource_policy_restore_compatibility_diagnostics_richness_change_distinguishes_retained_only_from_deny_cold(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();

    let retained_only_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.retained-only",
    );
    let mut retained_only_runtime = TestRuntime::builder(graph.clone())
        .with_kernel_defaults()
        .resource_policy_registry(retained_only_registry)
        .build();
    retained_only_runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical diagnostics declaration should lower");
    let retained_only_proof = retained_only_runtime
        .admit_resource_policy_restore_compatibility(
            &retained_only_diagnostics_resource_declaration(node),
        )
        .expect("declared node should classify")
        .expect("retained-only diagnostics change should admit restore proof");

    let deny_cold_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.deny-cold-expansion",
    );
    let mut deny_cold_runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(deny_cold_registry)
        .build();
    deny_cold_runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical diagnostics declaration should lower");
    let deny_cold_proof = deny_cold_runtime
        .admit_resource_policy_restore_compatibility(&deny_cold_diagnostics_resource_declaration(
            node,
        ))
        .expect("declared node should classify")
        .expect("deny-cold diagnostics change should admit restore proof");

    let retained_only_family = retained_only_proof
        .compatibility()
        .family(ResourcePolicyKind::Diagnostics)
        .expect("retained-only diagnostics family should exist");
    let deny_cold_family = deny_cold_proof
        .compatibility()
        .family(ResourcePolicyKind::Diagnostics)
        .expect("deny-cold diagnostics family should exist");

    assert_eq!(
        retained_only_family.current_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::RetainedOnly)
    );
    assert_eq!(
        deny_cold_family.current_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::DenyColdExpansion)
    );
    assert_ne!(
        retained_only_proof.compatibility_digest().as_str(),
        deny_cold_proof.compatibility_digest().as_str()
    );
}

#[test]
fn resource_policy_restore_compatibility_admits_parameter_expansion_and_names_defaulted_parameter()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.forensic-expansion-budget",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical diagnostics declaration should lower");

    let proof = runtime
        .admit_resource_policy_restore_compatibility(
            &parameter_expansion_only_replay_resource_declaration(node).with_diagnostics_policy(
                ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
                    max_replay_reconstruction_width: 5,
                    max_forensic_reconstruction_width: 5,
                },
            ),
        )
        .expect("declared node should classify")
        .expect("compatible parameter expansion should admit restore proof");

    let diagnostics = proof
        .compatibility()
        .family(ResourcePolicyKind::Diagnostics)
        .expect("diagnostics family report should exist");
    assert_eq!(
        diagnostics.class(),
        ResourcePolicyCompatibilityClass::CompatibleParameterExpansion
    );
    assert_eq!(
        diagnostics.historical_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::BudgetedExpansion)
    );
    assert_eq!(
        diagnostics.current_diagnostics_class(),
        Some(ResourceDiagnosticsDecisionClass::ForensicExpansionBudget)
    );
    assert_eq!(
        diagnostics.defaulted_parameter_names(),
        ["max_forensic_reconstruction_width"]
    );
    assert!(diagnostics.canonical_truth_preserved());
    assert!(!diagnostics.retained_history_unavailable());
    assert!(!diagnostics.diagnostics_details_unavailable());
    assert_eq!(proof.retained_history_unavailable_width(), 0);
    assert_eq!(proof.diagnostics_details_unavailable_width(), 0);
    assert_eq!(
        proof.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleParameterExpansion
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_compatible_count,
        1
    );
}

#[test]
fn resource_policy_restore_compatibility_replay_policy_can_deny_parameter_expansion() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.forensic-expansion-budget",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");

    let denial = runtime
        .admit_resource_policy_restore_compatibility(
            &forensic_diagnostics_resource_declaration(node, 5, 5).with_replay_policy(
                ResourceReplayPolicyDeclaration::CompatibleDiagnosticsRichnessChange,
            ),
        )
        .expect("declared node should classify")
        .expect_err("diagnostics-richness replay policy should deny parameter expansion");

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Diagnostics)
    );
    assert_eq!(
        denial.replay_decision_class(),
        ResourceReplayDecisionClass::CompatibleDiagnosticsRichnessChange
    );
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Diagnostics)
            .expect("diagnostics family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::CompatibleParameterExpansion
    );
}

#[test]
fn resource_replay_availability_retained_when_restore_is_compatible_and_history_is_present() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let report = runtime
        .resource_replay_availability(&resource_declaration(node))
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Retained);
    assert!(report.restore_compatibility().is_some());
    assert!(report.restore_compatibility_denial().is_none());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(report.retained_history_unavailable_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_retained_count,
        1
    );
}

#[test]
fn resource_replay_availability_omits_cold_reconstruction_when_history_is_unavailable_and_no_budget_is_requested(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = terminal_summaries_only_resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::FixedTimeout {
            timeout: TemporalDuration::temporal_duration(3).unwrap(),
        },
    );
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should advance");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should succeed");
    let compaction = runtime.compact_resource_lifecycle_history(1);
    assert_eq!(compaction.retained_history_unavailable_count(), 1);

    let report = runtime
        .resource_replay_availability(&declaration)
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Omitted);
    assert!(report.restore_compatibility().is_some());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(report.retained_history_unavailable_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_omitted_count,
        1
    );
}

#[test]
fn resource_replay_availability_reports_unavailable_when_cold_reconstruction_is_policy_denied() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = terminal_summaries_only_resource_declaration(node)
        .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly)
        .with_timeout_policy(ResourceTimeoutPolicyDeclaration::FixedTimeout {
            timeout: TemporalDuration::temporal_duration(3).unwrap(),
        });
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should advance");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should succeed");
    runtime.compact_resource_lifecycle_history(1);

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &declaration,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Unavailable);
    assert!(report.restore_compatibility().is_some());
    assert!(report.diagnostics_summary().is_none());
    assert_eq!(
        report
            .diagnostics_denial()
            .expect("diagnostics denial should be present")
            .class(),
        ResourceDiagnosticsExpansionDenialClass::PolicyRetainedOnly
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_unavailable_count,
        1
    );
}

#[test]
fn resource_replay_availability_reconstructs_when_history_is_unavailable_and_budget_admits_cold_work(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = terminal_summaries_only_resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::FixedTimeout {
            timeout: TemporalDuration::temporal_duration(3).unwrap(),
        },
    );
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should advance");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should succeed");
    runtime.compact_resource_lifecycle_history(1);

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &declaration,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("replay availability should classify");

    assert_eq!(
        report.class(),
        ResourceReplayAvailabilityClass::Reconstructed
    );
    assert!(report.restore_compatibility().is_some());
    assert!(report.diagnostics_summary().is_some());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_reconstructed_count,
        1
    );
}

#[test]
fn resource_replay_availability_denied_by_restore_compatibility_does_not_attempt_cold_reconstruction(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("resource declaration should lower");

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &timeout_resource_declaration(node, 9),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Denied);
    assert!(report.restore_compatibility().is_none());
    assert!(report.restore_compatibility_denial().is_some());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_denied_count,
        1
    );
}

#[test]
fn resource_replay_availability_replay_policy_gate_denial_stays_zero_cold() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let compatible_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Retention,
        "signal.resource.retention.terminal-summaries-only",
    );
    let mut runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(compatible_registry)
        .build();
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(node))
        .expect("historical declaration should lower");

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &identical_only_replay_resource_declaration(node)
                .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Denied);
    let denial = report
        .restore_compatibility_denial()
        .expect("restore compatibility denial should be present");
    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_availability_denied_count,
        1
    );
}

#[test]
fn resource_replay_availability_digest_includes_replay_decision_provenance() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();

    let diagnostics_only_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.retained-only",
    );
    let mut diagnostics_only_runtime = TestRuntime::builder(graph.clone())
        .with_kernel_defaults()
        .resource_policy_registry(diagnostics_only_registry)
        .build();
    diagnostics_only_runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");
    let diagnostics_only_report = diagnostics_only_runtime
        .resource_replay_availability(
            &diagnostics_only_replay_resource_declaration(node)
                .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly),
        )
        .expect("diagnostics-only replay availability should classify");

    let combined_registry = compatible_policy_registry_for(
        ResourcePolicyKind::Diagnostics,
        "signal.resource.diagnostics.retained-only",
    );
    let mut combined_runtime = TestRuntime::builder(graph)
        .with_kernel_defaults()
        .resource_policy_registry(combined_registry)
        .build();
    combined_runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 5))
        .expect("historical declaration should lower");
    let combined_report = combined_runtime
        .resource_replay_availability(
            &resource_declaration(node)
                .with_replay_policy(
                    ResourceReplayPolicyDeclaration::CompatibleRetentionNarrowingAndDiagnosticsRichnessChange,
                )
                .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly),
        )
        .expect("combined replay availability should classify");

    assert_eq!(
        diagnostics_only_report.class(),
        ResourceReplayAvailabilityClass::Retained
    );
    assert_eq!(
        combined_report.class(),
        ResourceReplayAvailabilityClass::Retained
    );
    assert_ne!(
        diagnostics_only_report.availability_digest(),
        combined_report.availability_digest(),
        "availability digest must reflect replay-decision provenance"
    );
}

#[test]
fn resource_replay_availability_strict_replay_policy_denies_pruned_budget_history_before_cold_work()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let payload_digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();

    for request_id in [ResourceRequestId::new(910), ResourceRequestId::new(911)] {
        runtime
            .admit_resource_completion(RawCompletionEnvelope::new(
                request_id,
                ResourceGeneration::new(1),
                ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
                ResourceAttemptId::ZERO,
                payload_digest.clone(),
                32,
            ))
            .denied_completion()
            .expect("unknown request should retain denied completion evidence");
    }

    runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_denied_completion_limit(1),
    );

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &deny_on_unknown_or_missing_replay_resource_declaration(node),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Denied);
    assert_eq!(
        report.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert!(report.restore_compatibility().is_some());
    assert!(report.restore_compatibility_denial().is_none());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(report.retained_history_unavailable_count(), 0);
    assert_eq!(report.denied_completion_unavailable_count(), 1);
    assert_eq!(report.retry_lineage_unavailable_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_budget_history_unavailable_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        0
    );
}

#[test]
fn resource_replay_availability_strict_replay_policy_denies_pruned_lifecycle_history_before_cold_work(
) {
    let mut graph = SignalGraph::new();
    let first_node = graph.node().build();
    let second_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(deny_on_unknown_or_missing_replay_resource_declaration(
            first_node,
        ))
        .expect("first declaration should lower");
    runtime
        .declare_resource_node(deny_on_unknown_or_missing_replay_resource_declaration(
            second_node,
        ))
        .expect("second declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            first_node,
        )))
        .expect("first request should admit")
        .admitted_request();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            second_node,
        )))
        .expect("second request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(first.handle(), ResourceCancellationReason::HostRequested)
        .expect("first cancellation should admit");
    runtime
        .cancel_resource_request(second.handle(), ResourceCancellationReason::HostRequested)
        .expect("second cancellation should admit");

    runtime.compact_resource_lifecycle_history_with_budget(
        2,
        ResourceRetentionCompactionBudget::unbounded().with_retained_lifecycle_history_limit(1),
    );

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &deny_on_unknown_or_missing_replay_resource_declaration(first_node),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("strict replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Denied);
    assert_eq!(
        report.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert!(report.restore_compatibility().is_some());
    assert!(report.restore_compatibility_denial().is_none());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert!(
        report.retained_history_unavailable_count() > 0,
        "strict replay denial should be driven by typed unavailable lifecycle history"
    );
    assert_eq!(report.denied_completion_unavailable_count(), 0);
    assert_eq!(report.retry_lineage_unavailable_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_budget_history_unavailable_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        0
    );
}

#[test]
fn resource_replay_availability_strict_replay_policy_denies_pruned_retry_lineage_before_cold_work()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = retry_timeout_resource_declaration(node, 3, 7)
        .with_replay_policy(ResourceReplayPolicyDeclaration::DenyOnUnknownOrMissing);
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(declaration.clone())
        .expect("resource declaration should lower");

    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let first_timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("initial timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach first timeout");
    let first_ready_timeout = runtime
        .promote_temporal_wake_ready(first_timeout_wake)
        .expect("initial timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), first_ready_timeout)
        .expect("initial timeout should admit");

    let first_schedule = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("first retry schedule should return report");
    let first_scheduled = first_schedule
        .scheduled_retry()
        .expect("first retry should schedule");
    let first_retry_ordinal = first_scheduled.retry_ordinal();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(
                runtime
                    .clock_basis()
                    .current_tick()
                    .get()
                    .saturating_add(first_scheduled.scheduled_delay().get()),
            ),
        ))
        .expect("clock should reach first retry backoff");
    let first_ready_retry = runtime
        .promote_temporal_wake_ready(first_scheduled.backoff_wake_id())
        .expect("first retry wake should become ready");
    let first_retry_report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), first_ready_retry)
        .expect("first scheduled retry should admit");
    let first_retry_request = first_retry_report
        .admitted_retry()
        .expect("first retry should produce admitted retry artifact")
        .admitted_request();

    let second_timeout_wake = runtime
        .in_flight_resource_request(first_retry_request.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("retried request should attach timeout wake");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach second timeout");
    let second_ready_timeout = runtime
        .promote_temporal_wake_ready(second_timeout_wake)
        .expect("second timeout wake should become ready");
    runtime
        .admit_resource_timeout(first_retry_request.handle(), second_ready_timeout)
        .expect("second timeout should admit");

    let second_schedule = runtime
        .schedule_resource_retry(first_retry_request.handle(), ResourceRetryReason::TimedOut)
        .expect("second retry schedule should return report");
    let second_scheduled = second_schedule
        .scheduled_retry()
        .expect("second retry should schedule");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(
                runtime
                    .clock_basis()
                    .current_tick()
                    .get()
                    .saturating_add(second_scheduled.scheduled_delay().get()),
            ),
        ))
        .expect("clock should reach second retry backoff");
    let second_ready_retry = runtime
        .promote_temporal_wake_ready(second_scheduled.backoff_wake_id())
        .expect("second retry wake should become ready");
    runtime
        .admit_scheduled_resource_retry(first_retry_request.handle(), second_ready_retry)
        .expect("second scheduled retry should admit");

    runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_retry_lineage_limit(1),
    );

    let report = runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &declaration,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("strict replay availability should classify");

    assert_eq!(report.class(), ResourceReplayAvailabilityClass::Denied);
    assert_eq!(
        report.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert!(report.restore_compatibility().is_some());
    assert!(report.restore_compatibility_denial().is_none());
    assert!(report.diagnostics_summary().is_none());
    assert!(report.diagnostics_denial().is_none());
    assert_eq!(report.retained_history_unavailable_count(), 0);
    assert_eq!(report.denied_completion_unavailable_count(), 0);
    assert_eq!(report.retry_lineage_unavailable_count(), 1);
    assert!(
        runtime
            .retained_retry_lineage_availability(first_retry_ordinal)
            .is_some(),
        "strict denial should come from typed unavailable retry-lineage evidence"
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_budget_history_unavailable_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        0
    );
}

#[test]
fn resource_replay_availability_default_lane_omits_pruned_budget_history_where_strict_lane_denies()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let payload_digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();

    for request_id in [ResourceRequestId::new(920), ResourceRequestId::new(921)] {
        runtime
            .admit_resource_completion(RawCompletionEnvelope::new(
                request_id,
                ResourceGeneration::new(1),
                ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
                ResourceAttemptId::ZERO,
                payload_digest.clone(),
                32,
            ))
            .denied_completion()
            .expect("unknown request should retain denied completion evidence");
    }

    runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_denied_completion_limit(1),
    );

    let default_report = runtime
        .resource_replay_availability(&resource_declaration(node))
        .expect("default replay availability should classify");
    let strict_report = runtime
        .resource_replay_availability(&deny_on_unknown_or_missing_replay_resource_declaration(
            node,
        ))
        .expect("strict replay availability should classify");

    assert_eq!(
        default_report.class(),
        ResourceReplayAvailabilityClass::Omitted
    );
    assert_eq!(default_report.denial_class(), None);
    assert_eq!(
        strict_report.class(),
        ResourceReplayAvailabilityClass::Denied
    );
    assert_eq!(
        strict_report.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert_ne!(
        default_report.availability_digest(),
        strict_report.availability_digest(),
        "strict budget-history denial must not collapse into default omitted availability"
    );
}

#[test]
fn resource_replay_availability_budget_history_denial_is_distinct_from_restore_compatibility_denial(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut strict_runtime = TestRuntime::build(graph.clone());
    strict_runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let payload_digest = strict_runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();
    for request_id in [ResourceRequestId::new(930), ResourceRequestId::new(931)] {
        strict_runtime
            .admit_resource_completion(RawCompletionEnvelope::new(
                request_id,
                ResourceGeneration::new(1),
                ResourceBranchEpoch::new(strict_runtime.graph().current_branch().id, 0),
                ResourceAttemptId::ZERO,
                payload_digest.clone(),
                32,
            ))
            .denied_completion()
            .expect("unknown request should retain denied completion evidence");
    }
    strict_runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_denied_completion_limit(1),
    );
    let budget_history_denied = strict_runtime
        .resource_replay_availability(&deny_on_unknown_or_missing_replay_resource_declaration(
            node,
        ))
        .expect("strict replay availability should classify");

    let mut restore_runtime = TestRuntime::build(graph);
    restore_runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("timeout declaration should lower");
    let restore_denied = restore_runtime
        .resource_replay_availability_with_cold_reconstruction_budget(
            &timeout_resource_declaration(node, 9),
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("restore-denied replay availability should classify");

    assert_eq!(
        budget_history_denied.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable)
    );
    assert_eq!(
        restore_denied.denial_class(),
        Some(ResourceReplayAvailabilityDenialClass::RestoreCompatibilityDenied)
    );
    assert_ne!(
        budget_history_denied.availability_digest(),
        restore_denied.availability_digest(),
        "budget-history denial and restore-compatibility denial must remain distinct"
    );
}

#[test]
fn resource_policy_compatibility_denies_missing_historical_descriptor() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let historical_declaration = timeout_resource_declaration(node, 3);
    let historical_registry = FrozenResourcePolicyRegistry::built_in();
    let historical_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &historical_declaration,
        &historical_registry,
    )
    .expect("historical declaration should validate");
    let historical_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_validated,
        &historical_registry,
    )
    .expect("historical declaration should freeze");
    let historical_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_frozen);
    let current_registrations: Vec<_> = built_in_policy_registrations()
        .into_iter()
        .filter(|registration| {
            !matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout"
                )
            )
        })
        .collect();
    let current_registry = FrozenResourcePolicyRegistry::new(current_registrations)
        .expect("current registry should freeze");
    let current_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(node),
        &current_registry,
    )
    .expect("current declaration should validate against the reduced registry");

    let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(77),
        ResourceNodeId::from_node(node),
        &historical_lowered,
        &current_validated,
        &current_registry,
    )
    .expect("compatibility classification should return a report");

    assert!(!report.is_compatible());
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::MissingDescriptor
    );
    assert_ne!(
        report.historical_registry_digest().as_str(),
        report.current_registry_digest().as_str()
    );
}

#[test]
fn resource_policy_restore_compatibility_denies_missing_historical_descriptor() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let historical_declaration = timeout_resource_declaration(node, 3);
    let historical_registry = FrozenResourcePolicyRegistry::built_in();
    let historical_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &historical_declaration,
        &historical_registry,
    )
    .expect("historical declaration should validate");
    let historical_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_validated,
        &historical_registry,
    )
    .expect("historical declaration should freeze");
    let historical_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_frozen);
    let current_registrations: Vec<_> = built_in_policy_registrations()
        .into_iter()
        .filter(|registration| {
            !matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout"
                )
            )
        })
        .collect();
    let current_registry = FrozenResourcePolicyRegistry::new(current_registrations)
        .expect("current registry should freeze");
    let current_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(node),
        &current_registry,
    )
    .expect("current declaration should validate against the reduced registry");

    let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(77),
        ResourceNodeId::from_node(node),
        &historical_lowered,
        &current_validated,
        &current_registry,
    )
    .expect("compatibility classification should return a report");
    let current_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &current_validated,
        &current_registry,
    )
    .expect("current declaration should freeze");
    let replay_plan = ResourceReplayDecisionPlan::lower(
        current_validated.declaration().replay_policy(),
        current_frozen.replay(),
    )
    .expect("default replay policy should lower");
    let denial = DeniedResourcePolicyRestoreCompatibility::from_compatibility(report, &replay_plan);

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::MissingDescriptor
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Timeout)
    );
    assert_eq!(denial.incompatible_width(), 1);
    assert_eq!(
        denial
            .compatibility()
            .families()
            .iter()
            .filter(|family| !family.class().is_compatible())
            .count(),
        1
    );
}

#[test]
fn resource_policy_compatibility_denies_incompatible_version_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = timeout_resource_declaration(node, 3);
    let historical_registry = FrozenResourcePolicyRegistry::built_in();
    let historical_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &historical_registry)
            .expect("historical declaration should validate");
    let historical_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_validated,
        &historical_registry,
    )
    .expect("historical declaration should freeze");
    let historical_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_frozen);
    let current_registrations = built_in_policy_registrations()
        .into_iter()
        .map(|registration| {
            if matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout"
                )
            ) {
                ResourcePolicyRegistration::new(
                    registration.id(),
                    registration.kind(),
                    registration.semantic_name().clone(),
                    ResourcePolicyVersion::new(2, 0),
                    registration.cost_contract(),
                    ResourcePolicyCompatibilityPosture::IncompatibleVersion,
                )
            } else {
                registration
            }
        })
        .collect();
    let current_registry =
        FrozenResourcePolicyRegistry::new(current_registrations).expect("registry should freeze");
    let current_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(node),
        &current_registry,
    )
    .expect(
        "current declaration should validate while the historical descriptor remains incompatible",
    );

    let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(78),
        ResourceNodeId::from_node(node),
        &historical_lowered,
        &current_validated,
        &current_registry,
    )
    .expect("compatibility classification should return a report");

    assert!(!report.is_compatible());
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::VersionIncompatible
    );
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .current_compatibility_posture()
            .expect("current posture should exist"),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch
    );
    assert_ne!(
        report.historical_registry_digest().as_str(),
        report.current_registry_digest().as_str()
    );
    assert_eq!(
        report
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .historical_version(),
        ResourcePolicyVersion::INITIAL
    );
}

#[test]
fn resource_policy_restore_compatibility_denies_incompatible_version_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let declaration = timeout_resource_declaration(node, 3);
    let historical_registry = FrozenResourcePolicyRegistry::built_in();
    let historical_validated =
        ValidatedResourcePolicyDeclaration::from_declaration(&declaration, &historical_registry)
            .expect("historical declaration should validate");
    let historical_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &historical_validated,
        &historical_registry,
    )
    .expect("historical declaration should freeze");
    let historical_lowered =
        LoweredResourcePolicyBundle::from_frozen_descriptors(&historical_frozen);
    let current_registrations = built_in_policy_registrations()
        .into_iter()
        .map(|registration| {
            if matches!(
                (registration.kind(), registration.semantic_name().as_str()),
                (
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout"
                )
            ) {
                ResourcePolicyRegistration::new(
                    registration.id(),
                    registration.kind(),
                    registration.semantic_name().clone(),
                    ResourcePolicyVersion::new(2, 0),
                    registration.cost_contract(),
                    ResourcePolicyCompatibilityPosture::IncompatibleVersion,
                )
            } else {
                registration
            }
        })
        .collect();
    let current_registry =
        FrozenResourcePolicyRegistry::new(current_registrations).expect("registry should freeze");
    let current_validated = ValidatedResourcePolicyDeclaration::from_declaration(
        &resource_declaration(node),
        &current_registry,
    )
    .expect(
        "current declaration should validate while the historical descriptor remains incompatible",
    );

    let report = ResourcePolicyCompatibilityReport::classify_against_validated_declaration(
        ResourceDescriptorId::new(78),
        ResourceNodeId::from_node(node),
        &historical_lowered,
        &current_validated,
        &current_registry,
    )
    .expect("compatibility classification should return a report");
    let current_frozen = FrozenResourcePolicyDescriptorSet::from_validated_declaration(
        &current_validated,
        &current_registry,
    )
    .expect("current declaration should freeze");
    let replay_plan = ResourceReplayDecisionPlan::lower(
        current_validated.declaration().replay_policy(),
        current_frozen.replay(),
    )
    .expect("default replay policy should lower");
    let denial = DeniedResourcePolicyRestoreCompatibility::from_compatibility(report, &replay_plan);

    assert_eq!(
        denial.class(),
        ResourcePolicyRestoreCompatibilityDenialClass::VersionIncompatible
    );
    assert_eq!(
        denial.primary_incompatible_kind(),
        Some(ResourcePolicyKind::Timeout)
    );
    assert_eq!(
        denial
            .compatibility()
            .family(ResourcePolicyKind::Timeout)
            .expect("timeout family report should exist")
            .class(),
        ResourcePolicyCompatibilityClass::VersionIncompatible
    );
    assert_eq!(
        denial
            .compatibility()
            .families()
            .iter()
            .filter(|family| !family.class().is_compatible())
            .count(),
        1
    );
}

#[test]
fn built_in_resource_policy_registry_exposes_freeze_evidence() {
    let registry = FrozenResourcePolicyRegistry::built_in();
    let report = registry.freeze_report();
    let descriptor_count = registry.descriptor_count();

    assert_eq!(
        descriptor_count as usize,
        built_in_policy_registrations().len()
    );
    assert_eq!(report.descriptor_count(), descriptor_count);
    assert_eq!(report.id_index_width(), descriptor_count);
    assert_eq!(report.kind_name_index_width(), descriptor_count);
    assert_eq!(
        report.registry_digest().as_str(),
        registry.registry_digest().as_str()
    );
    assert!(report
        .registry_digest()
        .as_str()
        .starts_with("resource-policy-registry:"));
    assert!(report
        .registry_digest()
        .as_str()
        .contains("signal.resource.retry.disabled"));
}

#[test]
fn resource_lifecycle_policy_initial_class_is_compile_time_constrained_to_unrequested() {
    let policy =
        ResourceLifecyclePolicyDeclaration::new(ResourceInitialLifecycleClass::unrequested());
    assert_eq!(policy.initial(), ResourceLifecycleClass::Unrequested);

    let encoded = serde_json::to_string(&ResourceInitialLifecycleClass::unrequested()).unwrap();
    assert_eq!(encoded, "\"Unrequested\"");
    let decoded: ResourceInitialLifecycleClass = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.lifecycle(), ResourceLifecycleClass::Unrequested);

    let rejected = serde_json::from_str::<ResourceInitialLifecycleClass>("\"Pending\"");
    assert!(rejected
        .expect_err("runtime lifecycle classes must not deserialize as initial policy")
        .to_string()
        .contains("Unrequested"));
    let policy_encoded = serde_json::to_string(&policy).unwrap();
    assert_eq!(policy_encoded, "{\"initial\":\"Unrequested\"}");
    let policy_decoded: ResourceLifecyclePolicyDeclaration =
        serde_json::from_str(&policy_encoded).unwrap();
    assert_eq!(
        policy_decoded.initial(),
        ResourceLifecycleClass::Unrequested
    );
    let rejected_policy =
        serde_json::from_str::<ResourceLifecyclePolicyDeclaration>("{\"initial\":\"Fulfilled\"}");
    assert!(rejected_policy
        .expect_err("policy declarations must reject terminal initial lifecycle data")
        .to_string()
        .contains("Unrequested"));
    let mut declaration_graph = SignalGraph::new();
    let declaration_node = declaration_graph.node().build();
    let declaration = resource_declaration(declaration_node);
    let mut declaration_value =
        serde_json::to_value(&declaration).expect("resource declaration should serialize");
    declaration_value["lifecycle_policy"]["initial"] = serde_json::json!("TimedOut");
    let rejected_declaration = serde_json::from_value::<ResourceNodeDeclaration>(declaration_value);
    assert!(rejected_declaration
        .expect_err("resource declarations must reject impossible initial lifecycle policy data")
        .to_string()
        .contains("Unrequested"));

    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let report = runtime
        .declare_resource_node(resource_declaration(node).with_lifecycle_policy(policy))
        .expect("resource declaration should accept the constrained initial policy");

    assert_eq!(
        report.lifecycle().lifecycle(),
        ResourceLifecycleClass::Unrequested
    );
    assert_eq!(
        report.transition().from(),
        ResourceLifecycleClass::Unrequested
    );
    assert_eq!(
        report.transition().to(),
        ResourceLifecycleClass::Unrequested
    );
}

fn raw_completion(
    runtime: &TestRuntime,
    node: NodeId,
    handle: ResourceRequestHandle,
    attempt: ResourceAttemptId,
    payload_byte_len: u64,
) -> RawCompletionEnvelope {
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should exist")
        .payload_contract_digest()
        .clone();
    RawCompletionEnvelope::new(
        handle.request_id(),
        handle.generation(),
        handle.branch_epoch(),
        attempt,
        digest,
        payload_byte_len,
    )
}

#[test]
fn resource_declaration_lowers_into_runtime_owned_descriptor() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    let report = runtime
        .declare_resource_node(resource_declaration(node))
        .expect("live node resource declaration should lower");

    assert_eq!(report.descriptor_id(), ResourceDescriptorId::new(0));
    assert_eq!(report.lifecycle().node(), ResourceNodeId::from_node(node));
    assert_eq!(
        report.lifecycle().lifecycle(),
        ResourceLifecycleClass::Unrequested
    );
    assert_eq!(
        report.lifecycle().output_continuity(),
        ResourceOutputContinuity::NoPriorOutput
    );
    assert_eq!(
        report.transition().kind(),
        ResourceLifecycleTransitionKind::DeclarationInitialized
    );
    assert_eq!(
        report.lifecycle().lifecycle_ordinal(),
        report.transition().ordinal()
    );
    assert_eq!(report.performance().input_width(), 1);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().lifecycle_transition_count(), 1);
    assert_eq!(report.performance().broad_scan_denial_count(), 0);

    let summary = runtime.resource_runtime_summary();
    assert_eq!(summary.descriptor_count(), 1);
    assert_eq!(summary.declared_resource_node_count(), 1);
    assert_eq!(summary.next_descriptor_id(), ResourceDescriptorId::new(1));

    let descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should be indexed by resource node id");
    assert_eq!(descriptor.node(), ResourceNodeId::from_node(node));
    assert_eq!(descriptor.descriptor_id(), ResourceDescriptorId::new(0));
    assert_eq!(
        descriptor.payload_contract_digest().as_str(),
        "payload-contract:7:1024"
    );

    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_declaration_lowering_count,
        1
    );
    assert_eq!(runtime.telemetry().resource.resource_descriptor_count, 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        1
    );
}

#[test]
fn resource_declaration_rejects_non_live_node_owner() {
    let graph = SignalGraph::new();
    let mut runtime = TestRuntime::build(graph);

    let err = runtime
        .declare_resource_node(resource_declaration(NodeId::new(99, 0)))
        .expect_err("resource declarations must be owned by live signal nodes");

    assert!(err.to_string().contains("non-live owner"));
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_non_live_owner_denial_count,
        1
    );
    assert_eq!(runtime.resource_runtime_summary().descriptor_count(), 0);
}

#[test]
fn resource_declaration_rejects_duplicate_node_without_relowering() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("first declaration should lower");
    let err = runtime
        .declare_resource_node(resource_declaration(node))
        .expect_err("duplicate resource declarations for one node should be denied");

    assert!(err
        .to_string()
        .contains("already has a lowered resource descriptor"));
    assert_eq!(runtime.resource_runtime_summary().descriptor_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_duplicate_declaration_denial_count,
        1
    );
}

#[test]
fn resource_request_admission_creates_pending_in_flight_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(node))
        .expect("resource declaration should lower");

    let report = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("declared resource node should admit a request");
    let admitted = report.admitted_request();
    let handle = admitted.handle();

    assert_eq!(handle.request_id(), ResourceRequestId::new(0));
    assert_eq!(handle.generation(), ResourceGeneration::new(1));
    assert_eq!(admitted.attempt(), ResourceAttemptId::ZERO);
    assert_eq!(report.lifecycle().node(), ResourceNodeId::from_node(node));
    assert_eq!(
        report.lifecycle().lifecycle(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        report.lifecycle().output_continuity(),
        ResourceOutputContinuity::NoPriorOutput
    );
    assert_eq!(
        report.transition().kind(),
        ResourceLifecycleTransitionKind::RequestAdmitted
    );
    assert_eq!(
        report.transition().from(),
        ResourceLifecycleClass::Unrequested
    );
    assert_eq!(report.transition().to(), ResourceLifecycleClass::Pending);
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::RequestAdmission
    );
    assert_eq!(report.performance().lifecycle_transition_count(), 1);
    assert_eq!(
        report.performance().density_strategy(),
        ResourceDensityStrategy::SparseIndexedLookup
    );

    let in_flight = runtime
        .in_flight_resource_request(handle)
        .expect("request handle should resolve through hot in-flight lookup");
    assert_eq!(in_flight.handle(), handle);
    assert_eq!(in_flight.node(), ResourceNodeId::from_node(node));
    assert_eq!(in_flight.lifecycle(), ResourceLifecycleClass::Pending);
    assert_eq!(in_flight.status(), ResourceInFlightStatus::Active);

    let summary = runtime.resource_runtime_summary();
    assert_eq!(summary.in_flight_request_count(), 1);
    assert_eq!(summary.active_in_flight_node_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_request_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_hot_in_flight_lookup_count,
        1
    );
}

#[test]
fn resource_request_admission_denies_undeclared_resource_node() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    let err = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect_err("request admission must require a lowered descriptor");

    assert!(err.to_string().contains("undeclared resource node"));
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_undeclared_owner_denial_count,
        1
    );
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        0
    );
}

#[test]
fn resource_pending_visibility_can_preserve_prior_output_without_mutating_lifecycle() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(node))
        .expect("resource declaration should lower");

    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("matching completion should admit");
    let mut ctx = ();
    runtime
        .transaction(&mut ctx, |tx| {
            let staging = tx.stage_admitted_resource_completion(admitted_completion)?;
            tx.commit_staged_resource_completion(staging.staged_effect())?;
            Ok(())
        })
        .expect("completion transaction should commit");

    let report = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("follow-up request should admit");

    assert_eq!(
        report.lifecycle().lifecycle(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        report.lifecycle().output_continuity(),
        ResourceOutputContinuity::PriorOutputPreserved
    );
    assert_eq!(
        report.transition().output_continuity(),
        ResourceOutputContinuity::PriorOutputPreserved
    );
    assert_eq!(
        report
            .performance()
            .output_continuity_classification_width(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_previous_output_preserved_count,
        1
    );
}

#[test]
fn resource_pending_visibility_hide_and_preserve_share_lifecycle_but_not_visibility_digest() {
    fn drive_pending_visibility(
        hide_while_pending: bool,
    ) -> (
        ResourceRequestAdmissionReport,
        ResourceReplayReconstructionReport,
        TestRuntime,
    ) {
        let mut graph = SignalGraph::new();
        let node = graph.node().build();
        let mut runtime = TestRuntime::build(graph);
        runtime
            .declare_resource_node(if hide_while_pending {
                hide_pending_output_resource_declaration(node)
            } else {
                resource_declaration(node)
            })
            .expect("resource declaration should lower");

        let admitted_request = runtime
            .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
            .expect("initial request should admit")
            .admitted_request();
        let admitted_completion = runtime
            .admit_resource_completion(raw_completion(
                &runtime,
                node,
                admitted_request.handle(),
                admitted_request.attempt(),
                64,
            ))
            .admitted_completion()
            .expect("matching completion should admit");
        let mut ctx = ();
        runtime
            .transaction(&mut ctx, |tx| {
                let staging = tx.stage_admitted_resource_completion(admitted_completion)?;
                tx.commit_staged_resource_completion(staging.staged_effect())?;
                Ok(())
            })
            .expect("completion transaction should commit");

        let report = runtime
            .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
            .expect("follow-up request should admit");
        let replay = runtime.reconstruct_resource_replay_summary();
        (report, replay, runtime)
    }

    let (preserve_report, preserve_replay, _) = drive_pending_visibility(false);
    let (hide_report, hide_replay, hide_runtime) = drive_pending_visibility(true);

    assert_eq!(
        preserve_report.lifecycle().lifecycle(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        hide_report.lifecycle().lifecycle(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        preserve_replay.lifecycle_digest(),
        hide_replay.lifecycle_digest()
    );
    assert_ne!(
        preserve_replay.output_continuity_digest(),
        hide_replay.output_continuity_digest()
    );
    assert_eq!(
        hide_report.lifecycle().output_continuity(),
        ResourceOutputContinuity::OutputUnavailableByPolicy
    );
    assert_eq!(
        hide_runtime
            .telemetry()
            .resource
            .resource_previous_output_hidden_count,
        1
    );
}

#[test]
fn resource_timeout_reclassifies_hidden_pending_output_when_terminal_policy_preserves() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            hide_pending_output_resource_declaration(node).with_timeout_policy(
                ResourceTimeoutPolicyDeclaration::FixedTimeout {
                    timeout: TemporalDuration::temporal_duration(5).unwrap(),
                },
            ),
        )
        .expect("resource declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request();
    let first_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            first.handle(),
            first.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("initial completion should admit");
    let mut ctx = ();
    runtime
        .transaction(&mut ctx, |tx| {
            let staging = tx.stage_admitted_resource_completion(first_completion)?;
            tx.commit_staged_resource_completion(staging.staged_effect())?;
            Ok(())
        })
        .expect("completion transaction should commit");

    let pending_report = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("follow-up request should admit");
    let pending = pending_report.admitted_request();
    assert_eq!(
        pending_report.lifecycle().output_continuity(),
        ResourceOutputContinuity::OutputUnavailableByPolicy
    );

    let decisions_before_timeout = runtime
        .telemetry()
        .resource
        .resource_output_continuity_decision_count;
    let timeout_wake = runtime
        .in_flight_resource_request(pending.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("pending request should have a timeout wake");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(8)),
        ))
        .expect("clock should advance past timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should promote after the clock advance");
    let timeout_report = runtime
        .admit_resource_timeout(pending.handle(), ready_timeout)
        .expect("timeout admission should succeed");

    assert_eq!(
        timeout_report
            .lifecycle()
            .expect("admitted timeout should report lifecycle truth")
            .output_continuity(),
        ResourceOutputContinuity::PriorOutputPreserved
    );
    assert_eq!(
        timeout_report
            .performance()
            .output_continuity_classification_width(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_output_continuity_decision_count,
        decisions_before_timeout + 1
    );
}

#[test]
fn resource_timeout_without_prior_output_does_not_charge_terminal_visibility_classification() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");

    let pending = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request();
    let decisions_before_timeout = runtime
        .telemetry()
        .resource
        .resource_output_continuity_decision_count;
    let timeout_wake = runtime
        .in_flight_resource_request(pending.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("pending request should retain timeout wake");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(8)),
        ))
        .expect("clock should advance past timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should promote");
    let timeout_report = runtime
        .admit_resource_timeout(pending.handle(), ready_timeout)
        .expect("timeout admission should succeed");

    assert_eq!(
        timeout_report
            .lifecycle()
            .expect("admitted timeout should report lifecycle truth")
            .output_continuity(),
        ResourceOutputContinuity::NoPriorOutput
    );
    assert_eq!(
        timeout_report
            .performance()
            .output_continuity_classification_width(),
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_output_continuity_decision_count,
        decisions_before_timeout
    );
}

#[test]
fn resource_cancellation_reclassifies_hidden_pending_output_when_terminal_policy_preserves() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(hide_pending_output_resource_declaration(node))
        .expect("resource declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request();
    let first_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            first.handle(),
            first.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("initial completion should admit");
    let mut ctx = ();
    runtime
        .transaction(&mut ctx, |tx| {
            let staging = tx.stage_admitted_resource_completion(first_completion)?;
            tx.commit_staged_resource_completion(staging.staged_effect())?;
            Ok(())
        })
        .expect("completion transaction should commit");

    let pending_report = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("follow-up request should admit");
    let pending = pending_report.admitted_request();
    let decisions_before_cancellation = runtime
        .telemetry()
        .resource
        .resource_output_continuity_decision_count;

    let cancellation_report = runtime
        .cancel_resource_request(pending.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should succeed");

    assert_eq!(
        cancellation_report
            .lifecycle()
            .expect("admitted cancellation should report lifecycle truth")
            .output_continuity(),
        ResourceOutputContinuity::PriorOutputPreserved
    );
    assert_eq!(
        cancellation_report
            .performance()
            .output_continuity_classification_width(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_output_continuity_decision_count,
        decisions_before_cancellation + 1
    );
}

#[test]
fn resource_cancellation_without_prior_output_does_not_charge_terminal_visibility_classification() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(node))
        .expect("resource declaration should lower");

    let pending = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request();
    let decisions_before_cancellation = runtime
        .telemetry()
        .resource
        .resource_output_continuity_decision_count;

    let cancellation_report = runtime
        .cancel_resource_request(pending.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should succeed");

    assert_eq!(
        cancellation_report
            .lifecycle()
            .expect("admitted cancellation should report lifecycle truth")
            .output_continuity(),
        ResourceOutputContinuity::NoPriorOutput
    );
    assert_eq!(
        cancellation_report
            .performance()
            .output_continuity_classification_width(),
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_output_continuity_decision_count,
        decisions_before_cancellation
    );
}

#[test]
fn resource_timeout_visibility_hide_and_preserve_share_lifecycle_but_not_visibility_digest() {
    fn drive_timeout_visibility(
        hide_after_timeout: bool,
    ) -> (
        ResourceTimeoutReport,
        ResourceReplayReconstructionReport,
        TestRuntime,
    ) {
        let mut graph = SignalGraph::new();
        let node = graph.node().build();
        let mut runtime = TestRuntime::build(graph);
        let declaration = if hide_after_timeout {
            hide_after_timeout_resource_declaration(node)
        } else {
            resource_declaration(node)
        }
        .with_timeout_policy(ResourceTimeoutPolicyDeclaration::FixedTimeout {
            timeout: TemporalDuration::temporal_duration(5).unwrap(),
        });
        runtime
            .declare_resource_node(declaration)
            .expect("resource declaration should lower");

        let first = runtime
            .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
            .expect("initial request should admit")
            .admitted_request();
        let first_completion = runtime
            .admit_resource_completion(raw_completion(
                &runtime,
                node,
                first.handle(),
                first.attempt(),
                64,
            ))
            .admitted_completion()
            .expect("initial completion should admit");
        let mut ctx = ();
        runtime
            .transaction(&mut ctx, |tx| {
                let staging = tx.stage_admitted_resource_completion(first_completion)?;
                tx.commit_staged_resource_completion(staging.staged_effect())?;
                Ok(())
            })
            .expect("completion transaction should commit");

        let pending = runtime
            .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
            .expect("follow-up request should admit")
            .admitted_request();
        let timeout_wake = runtime
            .in_flight_resource_request(pending.handle())
            .and_then(|in_flight| in_flight.timeout_wake_id())
            .expect("pending request should retain timeout wake");
        runtime
            .advance_clock(ClockAdvanceRequest::new(
                ClockDomain::MonotonicExecution,
                ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(8)),
            ))
            .expect("clock should advance past timeout");
        let ready_timeout = runtime
            .promote_temporal_wake_ready(timeout_wake)
            .expect("timeout wake should promote");
        let timeout_report = runtime
            .admit_resource_timeout(pending.handle(), ready_timeout)
            .expect("timeout admission should succeed");
        let replay = runtime.reconstruct_resource_replay_summary();
        (timeout_report, replay, runtime)
    }

    let (preserve_report, preserve_replay, _) = drive_timeout_visibility(false);
    let (hide_report, hide_replay, hide_runtime) = drive_timeout_visibility(true);

    assert_eq!(
        preserve_report
            .lifecycle()
            .expect("preserve timeout should admit")
            .lifecycle(),
        ResourceLifecycleClass::TimedOut
    );
    assert_eq!(
        hide_report
            .lifecycle()
            .expect("hide timeout should admit")
            .lifecycle(),
        ResourceLifecycleClass::TimedOut
    );
    assert_eq!(
        preserve_replay.lifecycle_digest(),
        hide_replay.lifecycle_digest()
    );
    assert_ne!(
        preserve_replay.output_continuity_digest(),
        hide_replay.output_continuity_digest()
    );
    assert_eq!(
        hide_report
            .lifecycle()
            .expect("hide timeout should retain lifecycle")
            .output_continuity(),
        ResourceOutputContinuity::OutputUnavailableByPolicy
    );
    assert_eq!(
        hide_runtime
            .telemetry()
            .resource
            .resource_previous_output_hidden_count,
        1
    );
}

#[test]
fn resource_cancellation_visibility_can_hide_previous_output() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(hide_after_cancellation_resource_declaration(node))
        .expect("resource declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request();
    let first_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            first.handle(),
            first.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("initial completion should admit");
    let mut ctx = ();
    runtime
        .transaction(&mut ctx, |tx| {
            let staging = tx.stage_admitted_resource_completion(first_completion)?;
            tx.commit_staged_resource_completion(staging.staged_effect())?;
            Ok(())
        })
        .expect("completion transaction should commit");

    let pending = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("follow-up request should admit")
        .admitted_request();
    let cancellation_report = runtime
        .cancel_resource_request(pending.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should succeed");

    assert_eq!(
        cancellation_report
            .lifecycle()
            .expect("cancellation should report lifecycle")
            .output_continuity(),
        ResourceOutputContinuity::OutputUnavailableByPolicy
    );
    assert_eq!(
        cancellation_report
            .performance()
            .output_continuity_classification_width(),
        1
    );
}

#[test]
fn resource_rejection_visibility_can_hide_previous_output() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(hide_after_rejection_resource_declaration(node))
        .expect("resource declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request();
    let first_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            first.handle(),
            first.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("initial completion should admit");
    let mut ctx = ();
    runtime
        .transaction(&mut ctx, |tx| {
            let staging = tx.stage_admitted_resource_completion(first_completion)?;
            tx.commit_staged_resource_completion(staging.staged_effect())?;
            Ok(())
        })
        .expect("completion transaction should commit");

    let pending = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("follow-up request should admit")
        .admitted_request();
    let rejection_report = runtime
        .reject_resource_request(pending.handle(), ResourceRejectionReason::SemanticFailure)
        .expect("rejection should succeed");

    assert_eq!(
        rejection_report
            .lifecycle()
            .expect("rejection should report lifecycle")
            .lifecycle(),
        ResourceLifecycleClass::Rejected
    );
    assert_eq!(
        rejection_report
            .lifecycle()
            .expect("rejection should report lifecycle")
            .output_continuity(),
        ResourceOutputContinuity::OutputUnavailableByPolicy
    );
    assert_eq!(
        rejection_report
            .rejected_request()
            .expect("rejection should retain rejected request")
            .reason(),
        ResourceRejectionReason::SemanticFailure
    );
    assert_eq!(
        rejection_report
            .performance()
            .output_continuity_classification_width(),
        1
    );
}

#[test]
fn resource_rejection_without_prior_output_does_not_charge_terminal_visibility_classification() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let pending = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request();
    let decisions_before_rejection = runtime
        .telemetry()
        .resource
        .resource_output_continuity_decision_count;

    let rejection_report = runtime
        .reject_resource_request(pending.handle(), ResourceRejectionReason::HostFailure)
        .expect("rejection should succeed");

    assert_eq!(
        rejection_report
            .lifecycle()
            .expect("rejection should report lifecycle")
            .output_continuity(),
        ResourceOutputContinuity::NoPriorOutput
    );
    assert_eq!(
        rejection_report
            .performance()
            .output_continuity_classification_width(),
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_output_continuity_decision_count,
        decisions_before_rejection
    );
}

#[test]
fn resource_supersession_visibility_policy_can_hide_previous_output() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(hide_after_supersession_resource_declaration(node))
        .expect("resource declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request();
    let first_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            first.handle(),
            first.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("initial completion should admit");
    let mut ctx = ();
    runtime
        .transaction(&mut ctx, |tx| {
            let staging = tx.stage_admitted_resource_completion(first_completion)?;
            tx.commit_staged_resource_completion(staging.staged_effect())?;
            Ok(())
        })
        .expect("completion transaction should commit");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("pending follow-up should admit");
    let third_report = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("third request should supersede active pending request");

    assert_eq!(
        third_report.lifecycle().output_continuity(),
        ResourceOutputContinuity::PriorOutputPreserved
    );
    assert_eq!(
        third_report
            .supersession_record()
            .expect("fresh request should retain supersession record")
            .lifecycle_transition()
            .output_continuity(),
        ResourceOutputContinuity::OutputUnavailableByPolicy
    );
    assert_eq!(
        third_report
            .performance()
            .output_continuity_classification_width(),
        2
    );
}

#[test]
fn resource_supersession_without_prior_output_counts_only_pending_visibility_classification() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(hide_after_supersession_resource_declaration(node))
        .expect("resource declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit");
    let decisions_before_supersession = runtime
        .telemetry()
        .resource
        .resource_output_continuity_decision_count;

    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede the active pending request");

    assert_eq!(
        second.lifecycle().output_continuity(),
        ResourceOutputContinuity::NoPriorOutput
    );
    assert_eq!(
        second
            .supersession_record()
            .expect("supersession should be retained")
            .lifecycle_transition()
            .output_continuity(),
        ResourceOutputContinuity::NoPriorOutput
    );
    assert_eq!(
        second
            .performance()
            .output_continuity_classification_width(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_output_continuity_decision_count,
        decisions_before_supersession + 1
    );
    assert_eq!(
        first.admitted_request().handle(),
        second
            .superseded_request()
            .expect("second request should supersede the first")
    );
}

#[test]
fn resource_request_admission_supersedes_prior_active_request_for_node() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should admit as the active generation");
    let second_handle = second.admitted_request().handle();

    assert_eq!(second.superseded_request(), Some(first));
    let supersession = second
        .supersession_record()
        .expect("second admission should return explicit supersession lineage");
    assert_eq!(supersession.previous(), first);
    assert_eq!(supersession.replacing(), second_handle);
    assert!(
        supersession.overlap_admission().is_none(),
        "default newest-generation-wins supersession should not claim explicit overlap admission"
    );
    assert_eq!(
        supersession.supersession_ordinal(),
        ResourceSupersessionOrdinal::new(1)
    );
    let superseded_transition = second
        .superseded_transition()
        .expect("second admission should return the supersession transition");
    assert_eq!(supersession.lifecycle_transition(), superseded_transition);
    assert_eq!(
        superseded_transition.kind(),
        ResourceLifecycleTransitionKind::RequestSuperseded
    );
    assert_eq!(
        superseded_transition.from(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        superseded_transition.to(),
        ResourceLifecycleClass::Superseded
    );
    assert_eq!(second_handle.request_id(), ResourceRequestId::new(1));
    assert_eq!(second.performance().lifecycle_transition_count(), 2);
    assert_eq!(
        second.performance().density_strategy(),
        ResourceDensityStrategy::BurstySortedDeduplicated
    );
    let superseded = runtime
        .in_flight_resource_request(first)
        .expect("superseded request remains retained for later denial");
    assert_eq!(superseded.status(), ResourceInFlightStatus::Superseded);
    assert_eq!(superseded.superseded_by(), Some(second_handle));
    assert_eq!(
        runtime
            .in_flight_resource_request(second_handle)
            .expect("new request is active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        2
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_superseded_in_flight_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_supersession_record_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_supersession_lineage_width,
        2
    );
}

#[test]
fn resource_overlap_supersession_retains_old_host_work_evidence_and_denies_late_completion() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(overlap_retained_host_work_resource_declaration(node))
        .expect("overlap-retained-host-work declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should admit and supersede the first as runtime authority");

    let supersession = second
        .supersession_record()
        .expect("overlap supersession should retain explicit lineage");
    let overlap = supersession
        .overlap_admission()
        .expect("overlap policy should emit an explicit overlap admission artifact");
    assert_eq!(overlap.previous(), first);
    assert_eq!(overlap.replacing(), second.admitted_request().handle());
    assert_eq!(
        overlap.policy_decision_digest().as_str(),
        supersession.policy_decision_digest().as_str()
    );
    assert!(
        overlap.old_host_work_cancellation_advisory().is_none(),
        "retained-host-work overlap should not claim old-host-work cancellation advisory"
    );

    let denied = runtime.admit_resource_completion(RawCompletionEnvelope::new(
        first.request_id(),
        first.generation(),
        first.branch_epoch(),
        ResourceAttemptId::ZERO,
        ResourcePayloadContractDigest::new("payload-contract:7:1024"),
        8,
    ));
    let denied = denied
        .denied_completion()
        .expect("late completion for overlap-retained loser should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Superseded);

    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_overlapping_generation_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_old_host_work_retained_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_old_host_work_advisory_cancelled_count,
        0
    );
}

#[test]
fn resource_overlap_supersession_can_request_old_host_work_advisory_cancel() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(overlap_cancelled_host_work_resource_declaration(node))
        .expect("overlap-cancelled-host-work declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should admit and emit overlap advisory evidence");

    let supersession = second
        .supersession_record()
        .expect("supersession should be explicit");
    let overlap = supersession
        .overlap_admission()
        .expect("overlap-cancel policy should retain overlap admission evidence");
    let advisory = overlap
        .old_host_work_cancellation_advisory()
        .expect("overlap-cancel policy should emit old-host-work advisory evidence");
    assert_eq!(
        advisory.policy_decision_digest().as_str(),
        supersession.policy_decision_digest().as_str()
    );
    assert_eq!(overlap.previous(), first);
    assert_eq!(overlap.replacing(), second.admitted_request().handle());
    assert_ne!(
        first.request_id(),
        second.admitted_request().handle().request_id()
    );
    assert_ne!(
        first.generation(),
        second.admitted_request().handle().generation()
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(first)
            .expect("older overlapping request should remain retained as superseded")
            .status(),
        ResourceInFlightStatus::Superseded
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(second.admitted_request().handle())
            .expect("winner should remain the only active authority")
            .status(),
        ResourceInFlightStatus::Active
    );
    let denied = runtime.admit_resource_completion(raw_completion(
        &runtime,
        node,
        first,
        ResourceAttemptId::ZERO,
        8,
    ));
    let denied = denied
        .denied_completion()
        .expect("late completion must still deny even if old host work kept running");
    assert_eq!(denied.class(), CompletionDenialClass::Superseded);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_overlapping_generation_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_old_host_work_retained_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_old_host_work_advisory_cancelled_count,
        1
    );
}

#[test]
fn resource_overlap_supersession_replay_retains_superseded_denial_evidence_after_restore() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(overlap_retained_host_work_resource_declaration(node))
        .expect("overlap-retained-host-work declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should admit and supersede first")
        .admitted_request()
        .handle();

    let denied = runtime.admit_resource_completion(raw_completion(
        &runtime,
        node,
        first,
        ResourceAttemptId::ZERO,
        32,
    ));
    assert_eq!(
        denied
            .denied_completion()
            .expect("superseded completion should be retained as denial evidence")
            .class(),
        CompletionDenialClass::Superseded
    );

    let snapshot = runtime.capture_snapshot();
    let expected = runtime.reconstruct_resource_replay_summary();

    runtime
        .cancel_resource_request(second, ResourceCancellationReason::HostRequested)
        .expect("post-snapshot mutation should change the replay surface");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate overlap supersession state");
    let replayed = runtime.reconstruct_resource_replay_summary();

    assert_eq!(replayed.denied_completion_width(), 1);
    assert_eq!(replayed.descriptor_digest(), expected.descriptor_digest());
    assert_eq!(
        replayed.lifecycle_summary_width(),
        expected.lifecycle_summary_width()
    );
    assert_eq!(replayed.in_flight_width(), expected.in_flight_width());
    assert_eq!(
        replayed.denied_completion_digest(),
        expected.denied_completion_digest(),
        "superseded completion denial evidence must survive replay/restore unchanged"
    );
}

#[test]
fn resource_intent_equivalence_coalescing_preserves_winner_and_loser_lineage() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(intent_equivalent_coalescing_resource_declaration(node))
        .expect("coalescing declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit");
    let winner = first.admitted_request();
    let coalesced = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("equivalent second request should coalesce");
    let record = coalesced
        .intent_equivalence_coalescing()
        .expect("coalescing policy should retain explicit winner/loser evidence");
    let loser = record.coalesced_request();

    assert_eq!(coalesced.admitted_request(), winner);
    assert!(coalesced.supersession_record().is_none());
    assert_eq!(record.winner(), winner.handle());
    assert_ne!(loser.handle(), winner.handle());
    assert_eq!(
        record.lifecycle_transition().kind(),
        ResourceLifecycleTransitionKind::RequestSuperseded
    );
    assert_eq!(
        record.policy_decision_digest().as_str(),
        runtime
            .resource_descriptor_for_node(ResourceNodeId::from_node(node))
            .expect("descriptor should remain declared")
            .supersession_decision_plan()
            .decision_digest()
            .as_str()
    );
    assert!(
        record
            .intent_digest()
            .as_str()
            .starts_with("resource-intent:"),
        "coalescing evidence should retain canonical intent digest truth"
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(loser.handle())
            .expect("coalesced loser should remain retained for late-completion denial")
            .status(),
        ResourceInFlightStatus::Superseded
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_intent_equivalence_coalescing_count,
        1
    );
}

#[test]
fn resource_intent_equivalence_coalescing_denies_late_completion_for_coalesced_loser() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(intent_equivalent_coalescing_resource_declaration(node))
        .expect("coalescing declaration should lower");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("winner request should admit");
    let coalesced = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("equivalent loser should coalesce");
    let loser = coalesced
        .intent_equivalence_coalescing()
        .expect("coalescing evidence should exist")
        .coalesced_request();
    let late = raw_completion(&runtime, node, loser.handle(), loser.attempt(), 64);

    let report = runtime.admit_resource_completion(late);
    let denied = report
        .denied_completion()
        .expect("late completion for coalesced loser should be denied");

    assert_eq!(denied.class(), CompletionDenialClass::Superseded);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_superseded_completion_denial_count,
        1
    );
}

#[test]
fn resource_cancellation_marks_request_cancelled_and_removes_active_frontier() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted.handle();

    let report = runtime
        .cancel_resource_request(handle, ResourceCancellationReason::HostRequested)
        .expect("cancellation should retire timeout side effects cleanly");

    let cancelled = report
        .cancelled_request()
        .expect("active pending request should cancel");
    let lifecycle = report
        .lifecycle()
        .expect("admitted cancellation should report lifecycle truth");
    let transition = report
        .transition()
        .expect("admitted cancellation should report transition truth");
    assert!(report.denied_cancellation().is_none());

    assert_eq!(cancelled.handle(), handle);
    assert_eq!(
        cancelled.reason(),
        ResourceCancellationReason::HostRequested
    );
    assert!(
        cancelled
            .policy_decision_digest()
            .as_str()
            .starts_with("resource-policy-cancellation-plan:"),
        "cancellation artifact should retain lowered cancellation decision proof"
    );
    assert!(
        cancelled.host_advisory().is_some(),
        "default best-effort cancellation policy should emit host advisory evidence"
    );
    assert_eq!(
        cancelled.cancellation_ordinal(),
        ResourceCancellationOrdinal::new(1)
    );
    assert_eq!(lifecycle.node(), ResourceNodeId::from_node(node));
    assert_eq!(lifecycle.lifecycle(), ResourceLifecycleClass::Cancelled);
    assert_eq!(
        transition.kind(),
        ResourceLifecycleTransitionKind::RequestCancelled
    );
    assert_eq!(transition.from(), ResourceLifecycleClass::Pending);
    assert_eq!(transition.to(), ResourceLifecycleClass::Cancelled);
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::Cancellation
    );
    assert_eq!(report.performance().input_width(), 1);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().denied_count(), 0);
    assert_eq!(report.performance().lifecycle_transition_count(), 1);

    let in_flight = runtime
        .in_flight_resource_request(handle)
        .expect("cancelled request remains retained for late completion denial");
    assert_eq!(in_flight.lifecycle(), ResourceLifecycleClass::Cancelled);
    assert_eq!(in_flight.status(), ResourceInFlightStatus::Cancelled);
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        0
    );
    assert_eq!(runtime.telemetry().resource.resource_cancellation_count, 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancellation_policy_decision_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_runtime_hard_cancellation_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_host_cancellation_advisory_count,
        1
    );
}

#[test]
fn resource_cancellation_denies_stale_handle_without_mutating_active_request() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first")
        .admitted_request()
        .handle();

    let report = runtime
        .cancel_resource_request(first, ResourceCancellationReason::HostRequested)
        .expect("denied cancellation should not trip temporal cleanup");

    let denied = report
        .denied_cancellation()
        .expect("stale superseded handle should be denied");
    assert_eq!(
        denied.class(),
        ResourceCancellationDenialClass::NonActiveRequest
    );
    assert!(report.cancelled_request().is_none());
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancellation_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_non_active_cancellation_denial_count,
        1
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(second)
            .expect("current request should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
}

#[test]
fn resource_runtime_denial_only_cancellation_omits_host_advisory_evidence() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(runtime_denial_only_cancellation_resource_declaration(node))
        .expect("runtime denial only declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();

    let report = runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("runtime denial only cancellation should admit");
    let cancelled = report
        .cancelled_request()
        .expect("active request should cancel");

    assert!(cancelled.host_advisory().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancellation_policy_decision_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_runtime_hard_cancellation_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_host_cancellation_advisory_count,
        0
    );
}

#[test]
fn resource_cancellation_reports_declared_grace_window() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(graceful_cancellation_resource_declaration(node, 25))
        .expect("graceful cancellation declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();

    let report = runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("graceful cancellation should admit");
    let cancelled = report
        .cancelled_request()
        .expect("graceful cancellation should retain the cancelled request");

    assert_eq!(
        cancelled
            .grace_window()
            .expect("declared grace window should be retained")
            .duration(),
        TemporalDuration::temporal_duration(25).unwrap()
    );
    assert!(report.dependent_propagation().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancellation_grace_period_count,
        1
    );
}

#[test]
fn resource_cancellation_only_propagates_across_declared_dependent_footprint() {
    let mut graph = SignalGraph::new();
    let parent = graph.node().build();
    let child = graph.node().build();
    let grandchild = graph.node().build();
    let sibling = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(dependent_cancellation_resource_declaration(parent, [child]))
        .expect("parent declaration should lower");
    runtime
        .declare_resource_node(dependent_cancellation_resource_declaration(
            child,
            [grandchild],
        ))
        .expect("child declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(grandchild))
        .expect("grandchild declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(sibling))
        .expect("sibling declaration should lower");

    let parent_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            parent,
        )))
        .expect("parent request should admit")
        .admitted_request()
        .handle();
    let child_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(child)))
        .expect("child request should admit")
        .admitted_request()
        .handle();
    let grandchild_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            grandchild,
        )))
        .expect("grandchild request should admit")
        .admitted_request()
        .handle();
    let sibling_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            sibling,
        )))
        .expect("sibling request should admit")
        .admitted_request()
        .handle();

    let report = runtime
        .cancel_resource_request(parent_handle, ResourceCancellationReason::HostRequested)
        .expect("parent cancellation should admit");
    let propagation = report
        .dependent_propagation()
        .expect("declared dependent footprint should emit propagation evidence");
    let propagated_handles = propagation
        .cancelled_dependents()
        .iter()
        .map(CancelledResourceRequest::handle)
        .collect::<Vec<_>>();
    let propagated_reasons = propagation
        .cancelled_dependents()
        .iter()
        .map(CancelledResourceRequest::reason)
        .collect::<Vec<_>>();

    assert_eq!(propagation.parent(), parent_handle);
    assert_eq!(propagation.cancelled_dependent_width(), 2);
    assert_eq!(propagated_handles, vec![child_handle, grandchild_handle]);
    assert_eq!(
        propagated_reasons,
        vec![
            ResourceCancellationReason::RuntimePolicy,
            ResourceCancellationReason::RuntimePolicy,
        ]
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(parent_handle)
            .expect("parent should remain retained")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(child_handle)
            .expect("child should remain retained")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(grandchild_handle)
            .expect("grandchild should remain retained")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(sibling_handle)
            .expect("undeclared sibling should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        1
    );
    assert_eq!(runtime.telemetry().resource.resource_cancellation_count, 3);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_dependent_cancellation_propagation_count,
        2
    );
    assert_eq!(report.performance().input_width(), 3);
    assert_eq!(report.performance().admitted_count(), 3);
    assert_eq!(report.performance().lifecycle_transition_count(), 3);
}

#[test]
fn resource_dependent_cancellation_retires_child_timeout_wakes_with_the_cancelled_footprint() {
    let mut graph = SignalGraph::new();
    let parent = graph.node().build();
    let child = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            timeout_resource_declaration(parent, 10)
                .with_declared_dependent_cancellation_nodes([ResourceNodeId::from_node(child)]),
        )
        .expect("parent declaration should lower");
    runtime
        .declare_resource_node(timeout_resource_declaration(child, 10))
        .expect("child declaration should lower");

    let parent_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            parent,
        )))
        .expect("parent request should admit")
        .admitted_request()
        .handle();
    let child_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(child)))
        .expect("child request should admit")
        .admitted_request()
        .handle();

    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 2);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 0);

    let report = runtime
        .cancel_resource_request(parent_handle, ResourceCancellationReason::HostRequested)
        .expect("parent cancellation should retire timeout wakes across the footprint");

    assert!(report.dependent_propagation().is_some());
    assert_eq!(
        runtime
            .in_flight_resource_request(child_handle)
            .expect("child should remain retained")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 2);
}

#[test]
fn resource_request_admission_with_timeout_policy_schedules_temporal_wake() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");

    let report = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("timeout policy should schedule a runtime-owned wake");
    let handle = report.admitted_request().handle();

    let in_flight = runtime
        .in_flight_resource_request(handle)
        .expect("admitted request should be retained in flight");
    assert_eq!(in_flight.timeout_wake_id(), Some(TemporalWakeId::new(0)));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_timeout_temporal_wake_footprint,
        1
    );
}

#[test]
fn resource_timeout_wake_owner_does_not_alias_node_temporal_owner() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .after(5)
        .expect("temporal evaluation condition should be valid")
        .build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("resource timeout policy should schedule resource-owned wake");
    let node_wake = runtime
        .admit_node_temporal_wake(node)
        .expect("node temporal wake admission should remain independent");

    assert!(node_wake.is_some());
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 2);
}

#[test]
fn resource_timeout_admission_requires_ready_temporal_wake_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let wake_id = runtime
        .in_flight_resource_request(handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached to request");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("authoritative clock should advance to timeout tick");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should promote when due");
    let report = runtime
        .admit_resource_timeout(handle, ready)
        .expect("timeout admission should consume temporal wake cleanly");

    let timed_out = report
        .timed_out_request()
        .expect("matching ready wake should admit timeout");
    let timeout_plan = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("timeout descriptor should exist")
        .timeout_decision_plan()
        .clone();
    assert_eq!(timed_out.handle(), handle);
    assert_eq!(timed_out.timeout_duration().get(), 5);
    assert!(
        timed_out
            .policy_decision_digest()
            .as_str()
            .starts_with("resolved-timeout-decision:"),
        "timeout artifact should retain resolved timeout admission proof"
    );
    assert!(
        timed_out
            .policy_decision_digest()
            .as_str()
            .contains(timeout_plan.decision_digest().as_str()),
        "resolved timeout digest should remain anchored to the lowered timeout plan digest"
    );
    assert_eq!(
        timed_out.lifecycle_transition().kind(),
        ResourceLifecycleTransitionKind::RequestTimedOut
    );
    assert_eq!(
        report
            .lifecycle()
            .expect("timeout should report lifecycle")
            .lifecycle(),
        ResourceLifecycleClass::TimedOut
    );
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::TimeoutAdmission
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 1);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        0
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("timed out request remains retained for late completion denial")
            .status(),
        ResourceInFlightStatus::TimedOut
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_timeout_admission_count,
        1
    );
}

#[test]
fn resource_total_request_lifetime_timeout_denies_timeout_triggered_retry_after_lineage_deadline() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_total_request_lifetime_timeout_resource_declaration(
            node, 5, 7,
        ))
        .expect("total-lifetime timeout declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let first_timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("initial timeout wake should attach");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("clock should reach lifetime timeout");
    let first_ready_timeout = runtime
        .promote_temporal_wake_ready(first_timeout_wake)
        .expect("initial timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), first_ready_timeout)
        .expect("initial timeout admission should succeed");

    let retry_schedule_report = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("retry scheduling should stay report-shaped");
    let denied = retry_schedule_report
        .denied_retry()
        .expect("total request lifetime timeout should deny timeout-triggered retry");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryTimeoutWindowExhausted
    );
    assert_eq!(
        retry_schedule_report
            .performance()
            .temporal_wake_footprint(),
        0
    );
    assert_eq!(
        runtime
            .resource_descriptor_for_node(ResourceNodeId::from_node(node))
            .expect("descriptor should exist")
            .timeout_decision_plan()
            .class(),
        ResourceTimeoutDecisionClass::TotalRequestLifetimeTimeout
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_timeout_window_exhaustion_denial_count,
        1
    );
}

#[test]
fn resource_timeout_admission_denies_wrong_ready_wake_without_mutation() {
    let mut graph = SignalGraph::new();
    let first_node = graph.node().build();
    let second_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(first_node, 5))
        .expect("first declaration should lower");
    runtime
        .declare_resource_node(timeout_resource_declaration(second_node, 5))
        .expect("second declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            first_node,
        )))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            second_node,
        )))
        .expect("second request should admit")
        .admitted_request()
        .handle();
    let second_wake = runtime
        .in_flight_resource_request(second)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("second timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("authoritative clock should advance");
    let wrong_ready = runtime
        .promote_temporal_wake_ready(second_wake)
        .expect("second wake should promote");
    let report = runtime
        .admit_resource_timeout(first, wrong_ready)
        .expect("wrong wake denial should not trip temporal cleanup");

    let denied = report
        .denied_timeout()
        .expect("wrong ready wake should be denied");
    assert_eq!(denied.class(), ResourceTimeoutDenialClass::WakeMismatch);
    assert!(report.timed_out_request().is_none());
    assert_eq!(
        runtime
            .in_flight_resource_request(first)
            .expect("first request should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_timeout_wake_mismatch_denial_count,
        1
    );
}

#[test]
fn resource_timeout_heartbeat_extension_reschedules_active_timeout_wake() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(heartbeat_extension_timeout_resource_declaration(node, 5, 7))
        .expect("heartbeat timeout declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let first_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("initial timeout wake should exist");

    let report = runtime
        .extend_resource_timeout_heartbeat(admitted.handle())
        .expect("heartbeat extension should return report");
    let extended = report
        .extended_heartbeat()
        .expect("active request should admit heartbeat extension");

    assert_eq!(extended.previous_timeout_wake_id(), first_wake);
    assert_eq!(extended.extension_duration().get(), 7);
    assert_eq!(
        extended.extended_timeout_wake().due_tick(),
        ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(7))
    );
    assert_ne!(extended.extended_timeout_wake().id(), first_wake);
    assert_eq!(
        runtime
            .in_flight_resource_request(admitted.handle())
            .and_then(|in_flight| in_flight.timeout_wake_id()),
        Some(extended.extended_timeout_wake().id())
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_progress_heartbeat_extension_count,
        1
    );
}

#[test]
fn resource_timeout_heartbeat_extension_denies_terminal_request() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(heartbeat_extension_timeout_resource_declaration(node, 5, 7))
        .expect("heartbeat timeout declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("clock should reach timeout");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should succeed");

    let report = runtime
        .extend_resource_timeout_heartbeat(admitted.handle())
        .expect("heartbeat extension denial should still return report");
    let denied = report
        .denied_extension()
        .expect("timed out request should deny heartbeat extension");

    assert_eq!(
        denied.class(),
        ResourceTimeoutHeartbeatExtensionDenialClass::NonActiveRequest
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_timeout_heartbeat_extension_denial_count,
        1
    );
}

#[test]
fn resource_timeout_revalidation_eligible_classification_is_retained_in_timeout_artifact() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(revalidation_eligible_timeout_resource_declaration(node, 5))
        .expect("revalidation eligible timeout declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("clock should reach timeout");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should become ready");
    let report = runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should succeed");
    let timed_out = report
        .timed_out_request()
        .expect("revalidation-eligible timeout should still admit timeout");

    assert_eq!(
        timed_out.outcome_class(),
        ResourceTimeoutOutcomeClass::RevalidationEligible
    );
    assert_eq!(
        runtime
            .resource_descriptor_for_node(ResourceNodeId::from_node(node))
            .expect("descriptor should exist")
            .timeout_decision_plan()
            .class(),
        ResourceTimeoutDecisionClass::RevalidationEligibleTimeout
    );
}

#[test]
fn resource_transaction_inherited_deadline_times_out_with_transaction_authority() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(transaction_inherited_deadline_resource_declaration(node))
        .expect("transaction inherited deadline declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::with_transaction_deadline(
            ResourceNodeId::from_node(node),
            TemporalDuration::temporal_duration(6).unwrap(),
        ))
        .expect("request with inherited deadline should admit")
        .admitted_request();
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(6),
        ))
        .expect("clock should reach inherited deadline");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("deadline wake should become ready");
    let report = runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should succeed");
    let timed_out = report
        .timed_out_request()
        .expect("deadline timeout should produce timeout artifact");

    assert_eq!(timed_out.timeout_duration().get(), 6);
    assert_eq!(
        timed_out.deadline_authority(),
        ResourceTimeoutDeadlineAuthority::TransactionIntent
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_deadline_inherited_count,
        1
    );
}

#[test]
fn resource_transaction_inherited_deadline_denies_missing_transaction_deadline() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(transaction_inherited_deadline_resource_declaration(node))
        .expect("transaction inherited deadline declaration should lower");

    let err = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect_err("missing transaction deadline should deny request admission");

    assert!(err.to_string().contains("transaction-inherited deadline"));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
}

#[test]
fn resource_runtime_inherited_deadline_uses_runtime_config_authority() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(runtime_inherited_deadline_resource_declaration(node))
        .expect("runtime inherited deadline declaration should lower");
    runtime
        .config_mut()
        .set_resource_runtime_deadline(TemporalDuration::temporal_duration(8).unwrap());

    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should inherit runtime deadline")
        .admitted_request();
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(8),
        ))
        .expect("clock should reach runtime deadline");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("deadline wake should become ready");
    let report = runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should succeed");
    let timed_out = report
        .timed_out_request()
        .expect("runtime deadline timeout should produce timeout artifact");

    assert_eq!(timed_out.timeout_duration().get(), 8);
    assert_eq!(
        timed_out.deadline_authority(),
        ResourceTimeoutDeadlineAuthority::RuntimeConfig
    );
}

#[test]
fn resource_inherited_deadline_retry_denies_when_backoff_outlives_preserved_deadline() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_transaction_inherited_deadline_resource_declaration(
            node, 7,
        ))
        .expect("inherited deadline retry declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::with_transaction_deadline(
            ResourceNodeId::from_node(node),
            TemporalDuration::temporal_duration(3).unwrap(),
        ))
        .expect("request with inherited deadline should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach inherited deadline");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should promote when due");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume inherited deadline wake");

    let schedule = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("retry scheduling should remain report-shaped");
    let scheduled = schedule
        .scheduled_retry()
        .expect("retry backoff should still schedule before admission-time denial");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should reach retry backoff");
    let ready_retry = runtime
        .promote_temporal_wake_ready(scheduled.backoff_wake_id())
        .expect("retry wake should become ready");
    let report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), ready_retry)
        .expect("expired inherited deadline should stay report-shaped");
    let performance = report.performance();
    let denied = report
        .denied_retry()
        .expect("expired inherited deadline must deny retry admission");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryTimeoutWindowExhausted
    );
    assert_eq!(performance.temporal_wake_footprint(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        0
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(admitted.handle())
            .expect("timed out request should stay retained for audit")
            .status(),
        ResourceInFlightStatus::TimedOut
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_timeout_window_exhaustion_denial_count,
        1
    );
}

#[test]
fn resource_supersession_retires_prior_timeout_wake_before_it_can_drive_timeout() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let first_wake = runtime
        .in_flight_resource_request(first)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("first timeout wake should be attached");

    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first");

    let supersession = second
        .supersession_record()
        .expect("supersession should be explicit");
    assert_eq!(supersession.previous(), first);
    assert_eq!(supersession.replacing(), second.admitted_request().handle());
    assert!(
        supersession
            .policy_decision_digest()
            .as_str()
            .starts_with("resource-policy-supersession-plan:"),
        "supersession record should retain lowered supersession decision proof"
    );
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_supersession_policy_decision_count,
        1
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("authoritative clock should advance");
    let err = runtime
        .promote_temporal_wake_ready(first_wake)
        .expect_err("superseded timeout wake must not become ready truth");
    assert!(!err.to_string().is_empty());
    assert_eq!(
        runtime
            .in_flight_resource_request(first)
            .expect("first request should remain retained as superseded")
            .superseded_by(),
        Some(second.admitted_request().handle())
    );
}

#[test]
fn resource_retry_after_timeout_preserves_attempt_lineage_and_backoff_wake_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(node, 3, 7))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake")
        .timed_out_request()
        .expect("timeout should admit");

    let schedule = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("retry scheduling should use runtime backoff");
    let scheduled = schedule
        .scheduled_retry()
        .expect("timed-out request with retry policy should schedule retry");
    let retry_plan = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("retry descriptor should exist")
        .retry_decision_plan()
        .clone();
    assert_eq!(scheduled.previous(), admitted.handle());
    assert_eq!(scheduled.next_attempt(), ResourceAttemptId::new(1));
    assert_eq!(scheduled.scheduled_delay().get(), 7);
    assert_eq!(
        scheduled.policy_decision_digest().as_str(),
        retry_plan.decision_digest().as_str()
    );
    assert_eq!(
        schedule.performance().boundary(),
        ResourceBoundaryKind::RetrySchedule
    );
    assert_eq!(schedule.performance().temporal_wake_footprint(), 1);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should reach retry backoff");
    let ready_retry = runtime
        .promote_temporal_wake_ready(scheduled.backoff_wake_id())
        .expect("retry backoff wake should become ready");
    let report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), ready_retry)
        .expect("retry admission should consume backoff wake");
    let retry = report
        .admitted_retry()
        .expect("matching backoff wake should admit retry");
    let retry_request = retry.admitted_request();

    assert_eq!(retry.scheduled().previous(), admitted.handle());
    assert_eq!(retry_request.attempt(), ResourceAttemptId::new(1));
    assert_eq!(
        retry_request.handle().generation(),
        admitted.handle().generation()
    );
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::RetryAdmission
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 1);
    assert_eq!(
        runtime
            .in_flight_resource_request(retry_request.handle())
            .expect("retry request should be retained")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime.telemetry().resource.resource_retry_schedule_count,
        1
    );
    assert_eq!(
        runtime.telemetry().resource.resource_retry_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_temporal_wake_footprint,
        2
    );
}

#[test]
fn resource_retry_schedule_denies_disabled_policy_without_temporal_wake() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake");

    let report = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("disabled policy denial should stay report-shaped");
    let denied = report
        .denied_retry()
        .expect("retry policy disabled should deny retry scheduling");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryPolicyDisabled
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 0);
    assert_eq!(
        runtime.telemetry().resource.resource_retry_schedule_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_policy_disabled_denial_count,
        1
    );
}

#[test]
fn resource_retry_schedule_denies_duplicate_without_allocating_second_wake() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(node, 3, 7))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake");

    let first = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("first retry scheduling should admit");
    let scheduled = first
        .scheduled_retry()
        .expect("first retry should carry a pending backoff wake");
    let second = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("duplicate retry scheduling should stay report-shaped");
    let denied = second
        .denied_retry()
        .expect("duplicate retry should be denied before temporal allocation");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryAlreadyScheduled
    );
    assert_eq!(second.performance().temporal_wake_footprint(), 0);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should reach original retry backoff");
    assert_eq!(
        runtime
            .promote_temporal_wake_ready(scheduled.backoff_wake_id())
            .expect("original retry wake should remain the only schedulable wake")
            .id(),
        scheduled.backoff_wake_id()
    );
    assert_eq!(
        runtime.telemetry().resource.resource_retry_schedule_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_already_scheduled_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_temporal_wake_footprint,
        1
    );
}

#[test]
fn resource_retry_admission_denies_if_newer_request_wins_before_backoff_ready() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(node, 3, 7))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake");
    let schedule = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("retry scheduling should use runtime backoff");
    let scheduled = schedule
        .scheduled_retry()
        .expect("retry should be scheduled");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("fresh admission should win before retry backoff fires");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should reach retry backoff");
    let promote_err = runtime
        .promote_temporal_wake_ready(scheduled.backoff_wake_id())
        .expect_err("superseded retry backoff wake must be retired before promotion");
    assert!(
        promote_err
            .to_string()
            .contains("cannot promote unknown scheduled temporal wake"),
        "unexpected promotion error: {promote_err}"
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_superseded_denial_count,
        0
    );
}

#[test]
fn resource_pending_retry_handle_is_rekeyed_across_restore_epoch() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(node, 3, 7))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake");
    let schedule = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("retry scheduling should use runtime backoff");
    let scheduled = schedule
        .scheduled_retry()
        .expect("retry should be scheduled");
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("state should mutate after snapshot");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should rekey pending retry handle identity");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should reach retry backoff");
    let ready_retry = runtime
        .promote_temporal_wake_ready(scheduled.backoff_wake_id())
        .expect("restored retry backoff wake should become ready");
    let report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), ready_retry)
        .expect("stale retry handle denial should be report-shaped");
    let denied = report
        .denied_retry()
        .expect("pre-restore retry handle must not admit after restore");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::UnknownOrStaleRequest
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_stale_retry_denial_count,
        1
    );
}

#[test]
fn resource_revalidation_admits_new_generation_when_no_request_is_active() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("live declared resource should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("no-active revalidation should admit");
    let admitted = revalidation.admitted_request();

    assert_eq!(revalidation.expected_active(), None);
    assert_eq!(admitted.handle().generation(), ResourceGeneration::new(1));
    assert_eq!(admitted.attempt(), ResourceAttemptId::ZERO);
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::RevalidationAdmission
    );
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().lifecycle_transition_count(), 1);
    assert_eq!(
        runtime
            .in_flight_resource_request(admitted.handle())
            .expect("revalidation request should be retained")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_admission_count,
        1
    );
}

#[test]
fn resource_revalidation_coalesces_duplicate_explicit_refresh_while_pending() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let first = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("first explicit revalidation should admit")
        .admitted_revalidation()
        .expect("first explicit revalidation should be admitted")
        .admitted_request();

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("duplicate explicit revalidation should coalesce");
    let revalidation = report
        .admitted_revalidation()
        .expect("duplicate explicit revalidation should still be admitted");
    let coalescing = revalidation
        .coalescing()
        .expect("coalesced revalidation should retain explicit winner/loser evidence");
    let loser = coalescing.coalesced_request();

    assert_eq!(revalidation.admitted_request(), first);
    assert_eq!(
        revalidation.freshness_decision().class(),
        ResourceRevalidationFreshnessClass::ExplicitIntent
    );
    assert_eq!(coalescing.winner(), first.handle());
    assert_ne!(loser.handle(), first.handle());
    assert_eq!(report.performance().coalescing_width(), 1);
    assert_eq!(
        runtime
            .in_flight_resource_request(first.handle())
            .expect("winner should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(loser.handle())
            .expect("coalesced loser should be retained")
            .status(),
        ResourceInFlightStatus::Superseded
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_coalesced_count,
        1
    );
}

#[test]
fn resource_revalidation_requires_expected_handle_when_active_request_exists() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit");

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("expected-handle denial should be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("ambient active request should require explicit expected handle");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle
    );
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_active_requires_expected_denial_count,
        1
    );
}

#[test]
fn resource_revalidation_supersedes_only_the_expected_active_handle() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let first_wake = runtime
        .in_flight_resource_request(first)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::with_expected_active(
            ResourceNodeId::from_node(node),
            first,
        ))
        .expect("expected active request should revalidate");
    let revalidation = report
        .admitted_revalidation()
        .expect("expected active revalidation should admit");
    let admitted = revalidation.admitted_request();
    let supersession = revalidation
        .supersession_record()
        .expect("revalidation should retain explicit supersession lineage");

    assert_eq!(revalidation.expected_active(), Some(first));
    assert_eq!(supersession.previous(), first);
    assert_eq!(supersession.replacing(), admitted.handle());
    assert_eq!(admitted.handle().generation(), ResourceGeneration::new(2));
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert!(runtime.promote_temporal_wake_ready(first_wake).is_err());
    assert_eq!(
        runtime
            .in_flight_resource_request(first)
            .expect("prior request should be retained")
            .status(),
        ResourceInFlightStatus::Superseded
    );
}

#[test]
fn resource_revalidation_denies_stale_expected_handle_after_newer_generation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first")
        .admitted_request()
        .handle();

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::with_expected_active(
            ResourceNodeId::from_node(node),
            first,
        ))
        .expect("stale expected denial should be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("stale expected handle must not overwrite newer active request");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ExpectedActiveRequestMismatch
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(second)
            .expect("newer active request should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_expected_mismatch_denial_count,
        1
    );
}

#[test]
fn resource_forced_revalidation_requires_policy_enabled_active_handle_proof() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let active = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let proof = runtime
        .prove_active_resource_revalidation_handle(active)
        .expect("active request should mint revalidation proof");

    let report = runtime
        .force_revalidate_resource_node(proof)
        .expect("policy-disabled force should still produce a report");
    let denied = report
        .denied_revalidation()
        .expect("explicit-intent-only policy must deny forced revalidation");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ForcedRevalidationPolicyDisabled
    );
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_forced_revalidation_policy_denial_count,
        1
    );
}

#[test]
fn resource_forced_revalidation_supersedes_proven_active_handle_when_policy_allows() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(forced_revalidation_timeout_resource_declaration(node, 5))
        .expect("forced revalidation declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let first_wake = runtime
        .in_flight_resource_request(first)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    let proof = runtime
        .prove_active_resource_revalidation_handle(first)
        .expect("active request should mint revalidation proof");

    let report = runtime
        .force_revalidate_resource_node(proof.clone())
        .expect("forced revalidation should admit");
    let revalidation = report
        .admitted_revalidation()
        .expect("forced revalidation should be admitted");
    let admitted = revalidation.admitted_request();
    let supersession = revalidation
        .supersession_record()
        .expect("forced revalidation should retain supersession lineage");
    let descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should remain visible");

    assert_eq!(revalidation.expected_active(), Some(first));
    assert_eq!(revalidation.forced_active_handle(), Some(first));
    assert_eq!(supersession.previous(), first);
    assert_eq!(supersession.replacing(), admitted.handle());
    assert_eq!(
        revalidation.decision_digest().as_str(),
        descriptor
            .revalidation_decision_plan()
            .decision_digest()
            .as_str()
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 1);
    assert!(runtime.promote_temporal_wake_ready(first_wake).is_err());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_forced_revalidation_count,
        1
    );
}

#[test]
fn resource_forced_revalidation_denies_stale_active_handle_proof_after_supersession() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(forced_revalidation_resource_declaration(node))
        .expect("forced revalidation declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let proof = runtime
        .prove_active_resource_revalidation_handle(first)
        .expect("first active request should mint proof");
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first")
        .admitted_request()
        .handle();

    let report = runtime
        .force_revalidate_resource_node(proof)
        .expect("stale proof denial should be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("stale active-handle proof must deny");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ActiveHandleProofMismatch
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(second)
            .expect("newer active request should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_active_handle_proof_mismatch_denial_count,
        1
    );
}

#[test]
fn resource_dependency_change_revalidation_revalidates_invalidated_node_when_policy_allows() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let node = graph.node().build();
    graph
        .depends_on(node, source, Aspect::new(0))
        .expect("dependency edge should admit");
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(dependency_change_revalidation_resource_declaration(node))
        .expect("dependency-change declaration should lower");
    mark_dirty(runtime.graph_mut(), source, Aspect::new(0))
        .expect("dependency invalidation should mark resource node non-clean");
    assert!(matches!(
        runtime
            .graph()
            .get_state(node)
            .expect("resource node should exist"),
        NodeState::Dirty | NodeState::MaybeStale
    ));

    let proof = runtime
        .prove_dependency_change_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("invalidated node should mint dependency-change proof");
    let report = runtime
        .revalidate_resource_node_for_dependency_change(proof.clone())
        .expect("dependency-change proof should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("dependency-change proof should admit");
    let descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should remain visible");

    assert_eq!(revalidation.expected_active(), None);
    assert_eq!(revalidation.forced_active_handle(), None);
    assert_eq!(
        revalidation
            .dependency_change_proof()
            .expect("admitted dependency-change revalidation should retain proof")
            .node_state(),
        proof.node_state()
    );
    assert_eq!(
        revalidation.decision_digest().as_str(),
        descriptor
            .revalidation_decision_plan()
            .decision_digest()
            .as_str()
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_dependency_change_revalidation_count,
        1
    );
}

#[test]
fn resource_dependency_change_revalidation_denies_forged_state_mismatch_proof() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let node = graph.node().build();
    graph
        .depends_on(node, source, Aspect::new(0))
        .expect("dependency edge should admit");
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(dependency_change_revalidation_resource_declaration(node))
        .expect("dependency-change declaration should lower");
    mark_dirty(runtime.graph_mut(), source, Aspect::new(0))
        .expect("dependency invalidation should mark resource node non-clean");
    let decision_digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .revalidation_decision_plan()
        .decision_digest()
        .clone();
    let forged = DependencyChangeResourceRevalidationProof::new(
        ResourceNodeId::from_node(node),
        NodeState::Clean,
        decision_digest,
    );

    let report = runtime
        .revalidate_resource_node_for_dependency_change(forged)
        .expect("forged proof denial should still be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("forged dependency-change proof must deny");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::DependencyChangeProofMismatch
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_dependency_change_proof_mismatch_denial_count,
        1
    );
}

#[test]
fn resource_dependency_change_revalidation_does_not_bypass_active_request_rule() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let node = graph.node().build();
    graph
        .depends_on(node, source, Aspect::new(0))
        .expect("dependency edge should admit");
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(dependency_change_revalidation_resource_declaration(node))
        .expect("dependency-change declaration should lower");
    let active = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request()
        .handle();
    mark_dirty(runtime.graph_mut(), source, Aspect::new(0))
        .expect("dependency invalidation should mark resource node non-clean");
    let proof = runtime
        .prove_dependency_change_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("invalidated active node should still mint invalidation proof");

    let report = runtime
        .revalidate_resource_node_for_dependency_change(proof)
        .expect("active-request denial should still be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("dependency-change proof must not bypass active overwrite rules");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(active)
            .expect("active request should remain authoritative")
            .status(),
        ResourceInFlightStatus::Active
    );
}

#[test]
fn resource_observer_demand_and_dependency_change_revalidation_do_not_coalesce_across_distinct_freshness_causes(
) {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let node = graph.node().build();
    graph
        .depends_on(node, source, Aspect::new(0))
        .expect("dependency edge should admit");
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            dependency_change_observer_demand_revalidation_resource_declaration(node),
        )
        .expect("combined revalidation declaration should lower");
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::new(Mutex::new(Vec::new())),
        }),
    );

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        node,
        &|view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
        EvaluationRequestMode::Default,
    )
    .expect("evaluation should succeed");
    tx.commit().expect("commit should succeed");

    mark_dirty(runtime.graph_mut(), node, Aspect::new(0))
        .expect("dependency invalidation should mark node dirty");
    let dependency_proof = runtime
        .prove_dependency_change_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("dirty node should mint dependency-change proof");
    let dependency_report = runtime
        .revalidate_resource_node_for_dependency_change(dependency_proof)
        .expect("dependency-change proof should admit");
    let dependency_revalidation = dependency_report
        .admitted_revalidation()
        .expect("dependency-change proof should admit");
    assert_eq!(
        dependency_revalidation.freshness_decision().class(),
        ResourceRevalidationFreshnessClass::DependencyChange
    );

    let observer_proof = runtime
        .prove_observer_demand_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("committed observation should still mint observer-demand proof");
    let report = runtime
        .revalidate_resource_node_for_observer_demand(observer_proof)
        .expect("distinct freshness race should still be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("distinct freshness cause must not silently coalesce");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_coalesced_count,
        0
    );
}

#[test]
fn resource_observer_demand_revalidation_revalidates_using_committed_observation_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(observer_demand_revalidation_resource_declaration(node))
        .expect("observer-demand declaration should lower");
    let calls = Arc::new(Mutex::new(Vec::<ResourceObservationRecord>::new()));
    let observation_handle = runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::clone(&calls),
        }),
    );
    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        node,
        &|view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
        EvaluationRequestMode::Default,
    )
    .expect("evaluation should succeed");
    let observation = tx.commit().expect("commit should succeed").observation;
    let delivered = calls
        .lock()
        .expect("resource observation mutex poisoned")
        .clone();
    assert_eq!(observation.delivered_event_count, 1);
    assert_eq!(delivered.len(), 1);
    assert_eq!(
        delivered[0].observer_id,
        observation_handle.observer_id().get()
    );

    let proof = runtime
        .prove_observer_demand_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("committed observation should mint observer-demand proof");
    let report = runtime
        .revalidate_resource_node_for_observer_demand(proof.clone())
        .expect("observer-demand proof should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("observer-demand proof should admit");

    assert_eq!(revalidation.expected_active(), None);
    assert_eq!(revalidation.forced_active_handle(), None);
    assert_eq!(
        revalidation
            .observer_demand_proof()
            .expect("admitted observer-demand revalidation should retain proof")
            .observation_digest(),
        proof.observation_digest()
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_observer_demand_revalidation_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_observer_demand_proof_check_count,
        1
    );
}

#[test]
fn resource_observer_demand_revalidation_requires_committed_not_rollback_suppressed_observation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(observer_demand_revalidation_resource_declaration(node))
        .expect("observer-demand declaration should lower");
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::new(Mutex::new(Vec::new())),
        }),
    );

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        node,
        &|view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
        EvaluationRequestMode::Default,
    )
    .expect("evaluation should succeed");
    let observation = tx.rollback().expect("rollback should succeed").observation;
    assert_eq!(observation.delivered_event_count, 0);
    assert_eq!(observation.rollback_suppressed_event_count, 1);

    let err = runtime
        .prove_observer_demand_resource_revalidation(ResourceNodeId::from_node(node))
        .expect_err("rollback-suppressed observation must not mint observer-demand proof");
    assert!(err
        .to_string()
        .contains("without committed matching observation"));
}

#[test]
fn resource_observer_demand_revalidation_denies_forged_observation_proof() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(observer_demand_revalidation_resource_declaration(node))
        .expect("observer-demand declaration should lower");
    let observation_handle = runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::new(Mutex::new(Vec::new())),
        }),
    );
    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        node,
        &|view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
        EvaluationRequestMode::Default,
    )
    .expect("evaluation should succeed");
    tx.commit().expect("commit should succeed");

    let decision_digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .revalidation_decision_plan()
        .decision_digest()
        .clone();
    let forged = ObserverDemandResourceRevalidationProof::new(
        ResourceNodeId::from_node(node),
        observation_handle.observer_id().get(),
        observation_handle.handle_id().get() + 1,
        String::from("forged-observation-digest"),
        decision_digest,
    );

    let report = runtime
        .revalidate_resource_node_for_observer_demand(forged)
        .expect("forged observer-demand proof denial should still be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("forged observer-demand proof must deny");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ObserverDemandProofMismatch
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_observer_demand_proof_mismatch_denial_count,
        1
    );
}

#[test]
fn resource_observer_demand_revalidation_does_not_bypass_active_request_rule() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(observer_demand_revalidation_resource_declaration(node))
        .expect("observer-demand declaration should lower");
    let active = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request()
        .handle();
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::new(Mutex::new(Vec::new())),
        }),
    );
    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        node,
        &|view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
        EvaluationRequestMode::Default,
    )
    .expect("evaluation should succeed");
    tx.commit().expect("commit should succeed");
    let proof = runtime
        .prove_observer_demand_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("committed observation should mint observer-demand proof");

    let report = runtime
        .revalidate_resource_node_for_observer_demand(proof)
        .expect("active-request denial should still be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("observer-demand proof must not bypass active overwrite rules");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(active)
            .expect("active request should remain authoritative")
            .status(),
        ResourceInFlightStatus::Active
    );
}

#[test]
fn resource_terminal_state_revalidation_revalidates_timed_out_node_when_policy_allows() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            terminal_state_revalidation_resource_declaration(node).with_timeout_policy(
                ResourceTimeoutPolicyDeclaration::FixedTimeout {
                    timeout: TemporalDuration::temporal_duration(1).unwrap(),
                },
            ),
        )
        .expect("terminal-state declaration should lower");
    let handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let timeout_wake = runtime
        .in_flight_resource_request(handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .expect("clock should reach timeout");
    let ready = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should promote");
    runtime
        .admit_resource_timeout(handle, ready)
        .expect("timeout should admit");

    let proof = runtime
        .prove_terminal_state_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("timed-out node should mint terminal-state proof");
    let report = runtime
        .revalidate_resource_node_for_terminal_state(proof.clone())
        .expect("terminal-state proof should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("terminal-state proof should admit");

    assert_eq!(
        revalidation
            .terminal_state_proof()
            .expect("admitted terminal-state revalidation should retain proof")
            .lifecycle(),
        ResourceLifecycleClass::TimedOut
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_terminal_state_revalidation_count,
        1
    );
}

#[test]
fn resource_terminal_state_revalidation_denies_when_proof_lifecycle_drifts_to_pending() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(terminal_state_revalidation_resource_declaration(node))
        .expect("terminal-state declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage");
    runtime
        .commit_staged_resource_completion(staging.staged_effect())
        .expect("completion should commit");
    let proof = runtime
        .prove_terminal_state_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("fulfilled terminal node should mint terminal-state proof");
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("fresh request should move lifecycle back to pending");

    let report = runtime
        .revalidate_resource_node_for_terminal_state(proof)
        .expect("drifted terminal-state proof denial should still be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("drifted terminal-state proof must deny");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::TerminalStateProofMismatch
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_terminal_state_proof_mismatch_denial_count,
        1
    );
}

#[test]
fn resource_terminal_state_revalidation_denies_stale_proof_after_node_reenters_same_terminal_class()
{
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            terminal_state_revalidation_resource_declaration(node).with_timeout_policy(
                ResourceTimeoutPolicyDeclaration::FixedTimeout {
                    timeout: TemporalDuration::temporal_duration(1).unwrap(),
                },
            ),
        )
        .expect("terminal-state declaration should lower");

    let first_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let first_timeout_wake = runtime
        .in_flight_resource_request(first_handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("first timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .expect("clock should reach first timeout");
    let first_ready = runtime
        .promote_temporal_wake_ready(first_timeout_wake)
        .expect("first timeout wake should promote");
    runtime
        .admit_resource_timeout(first_handle, first_ready)
        .expect("first timeout should admit");

    let stale_proof = runtime
        .prove_terminal_state_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("first timed-out node should mint proof");
    let second_handle = runtime
        .revalidate_resource_node_for_terminal_state(stale_proof.clone())
        .expect("fresh terminal-state proof should admit revalidation")
        .admitted_revalidation()
        .expect("terminal-state revalidation should admit")
        .admitted_request()
        .handle();
    let second_timeout_wake = runtime
        .in_flight_resource_request(second_handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("second timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(2),
        ))
        .expect("clock should reach second timeout");
    let second_ready = runtime
        .promote_temporal_wake_ready(second_timeout_wake)
        .expect("second timeout wake should promote");
    runtime
        .admit_resource_timeout(second_handle, second_ready)
        .expect("second timeout should admit");

    let report = runtime
        .revalidate_resource_node_for_terminal_state(stale_proof)
        .expect("stale terminal-state proof should deny as a report");
    let denied = report
        .denied_revalidation()
        .expect("stale proof must deny after lifecycle ordinal changed");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::TerminalStateProofMismatch
    );
}

#[test]
fn resource_fulfilled_lifecycle_revalidation_revalidates_fulfilled_node_when_policy_allows() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(fulfilled_lifecycle_revalidation_resource_declaration(node))
        .expect("fulfilled-lifecycle declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage");
    runtime
        .commit_staged_resource_completion(staging.staged_effect())
        .expect("completion should commit");

    let proof = runtime
        .prove_fulfilled_lifecycle_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("fulfilled node should mint fulfilled-lifecycle proof");
    let report = runtime
        .revalidate_resource_node_for_fulfilled_lifecycle(proof.clone())
        .expect("fulfilled-lifecycle proof should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("fulfilled-lifecycle proof should admit");

    assert_eq!(
        revalidation
            .fulfilled_lifecycle_proof()
            .expect("admitted fulfilled-lifecycle revalidation should retain proof")
            .decision_digest()
            .as_str(),
        proof.decision_digest().as_str()
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_fulfilled_lifecycle_revalidation_count,
        1
    );
}

#[test]
fn resource_fulfilled_lifecycle_revalidation_denies_stale_proof_after_node_reenters_fulfilled() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(fulfilled_lifecycle_revalidation_resource_declaration(node))
        .expect("fulfilled-lifecycle declaration should lower");

    let first_admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request();
    let first_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            first_admitted.handle(),
            first_admitted.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("first completion should admit");
    let first_staging = runtime
        .stage_admitted_resource_completion(first_completion)
        .expect("first completion should stage");
    runtime
        .commit_staged_resource_completion(first_staging.staged_effect())
        .expect("first completion should commit");

    let stale_proof = runtime
        .prove_fulfilled_lifecycle_resource_revalidation(ResourceNodeId::from_node(node))
        .expect("first fulfilled node should mint proof");
    let second_admitted = runtime
        .revalidate_resource_node_for_fulfilled_lifecycle(stale_proof.clone())
        .expect("fresh fulfilled-lifecycle proof should admit revalidation")
        .admitted_revalidation()
        .expect("fulfilled-lifecycle revalidation should admit")
        .admitted_request();
    let second_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            second_admitted.handle(),
            second_admitted.attempt(),
            96,
        ))
        .admitted_completion()
        .expect("second completion should admit");
    let second_staging = runtime
        .stage_admitted_resource_completion(second_completion)
        .expect("second completion should stage");
    runtime
        .commit_staged_resource_completion(second_staging.staged_effect())
        .expect("second completion should commit");

    let report = runtime
        .revalidate_resource_node_for_fulfilled_lifecycle(stale_proof)
        .expect("stale fulfilled-lifecycle proof should deny as a report");
    let denied = report
        .denied_revalidation()
        .expect("stale fulfilled proof must deny after lifecycle ordinal changed");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::FulfilledLifecycleProofMismatch
    );
}

#[test]
fn resource_fulfilled_lifecycle_revalidation_cannot_mint_from_non_fulfilled_terminal_state() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            fulfilled_lifecycle_revalidation_resource_declaration(node).with_timeout_policy(
                ResourceTimeoutPolicyDeclaration::FixedTimeout {
                    timeout: TemporalDuration::temporal_duration(1).unwrap(),
                },
            ),
        )
        .expect("fulfilled-lifecycle timeout declaration should lower");
    let handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let timeout_wake = runtime
        .in_flight_resource_request(handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .expect("clock should reach timeout");
    let ready = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should promote");
    runtime
        .admit_resource_timeout(handle, ready)
        .expect("timeout should admit");

    let err = runtime
        .prove_fulfilled_lifecycle_resource_revalidation(ResourceNodeId::from_node(node))
        .expect_err("timed-out node must not mint fulfilled-lifecycle proof");
    assert!(err
        .to_string()
        .contains("fulfilled-lifecycle revalidation proof"));
}

#[test]
fn resource_stale_after_completion_schedules_ready_revalidation_wake_and_revalidates() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(stale_after_revalidation_resource_declaration(node, 3))
        .expect("stale-after declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage");
    runtime
        .commit_staged_resource_completion(staging.staged_effect())
        .expect("staged completion should commit");

    let stale_after_wake = runtime
        .active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node))
        .expect("fulfilled node should retain a stale-after wake");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach stale-after due tick");
    let ready_wake = runtime
        .promote_temporal_wake_ready(stale_after_wake)
        .expect("stale-after wake should promote when due");

    let report = runtime
        .admit_stale_after_resource_revalidation(ResourceNodeId::from_node(node), ready_wake)
        .expect("stale-after ready wake should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("stale-after ready wake should revalidate");

    assert_eq!(revalidation.expected_active(), None);
    assert_eq!(revalidation.forced_active_handle(), None);
    assert_eq!(
        revalidation
            .stale_after_ready_wake()
            .expect("admitted stale-after revalidation should retain ready wake")
            .id(),
        stale_after_wake
    );
    assert_eq!(
        revalidation.admitted_request().handle().generation(),
        ResourceGeneration::new(2)
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 0);
    assert_eq!(
        runtime.active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node)),
        None
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_stale_after_revalidation_count,
        1
    );
}

#[test]
fn resource_stale_after_revalidation_denies_when_policy_does_not_allow_it() {
    let mut graph = SignalGraph::new();
    let resource_node = graph.node().build();
    let timeout_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(resource_node).with_stale_after_policy(
            ResourceStaleAfterPolicyDeclaration::RuntimeStaleAfter {
                stale_after: TemporalDuration::temporal_duration(3).unwrap(),
            },
        ))
        .expect("policy-disabled stale-after declaration should still lower");
    runtime
        .declare_resource_node(timeout_resource_declaration(timeout_node, 1))
        .expect("timeout declaration should lower");
    let timeout_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            timeout_node,
        )))
        .expect("timeout request should admit")
        .admitted_request()
        .handle();
    let timeout_wake = runtime
        .in_flight_resource_request(timeout_handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .expect("clock should reach timeout due tick");
    let stray_ready = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should promote");

    let report = runtime
        .admit_stale_after_resource_revalidation(
            ResourceNodeId::from_node(resource_node),
            stray_ready,
        )
        .expect("policy-disabled stale-after should still return a report");
    let denied = report
        .denied_revalidation()
        .expect("policy-disabled stale-after revalidation must deny");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::StaleAfterRevalidationPolicyDisabled
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_stale_after_policy_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_policy_decision_count,
        1
    );
}

#[test]
fn resource_stale_after_revalidation_denies_before_fulfillment_even_with_ready_wake() {
    let mut graph = SignalGraph::new();
    let pending_node = graph.node().build();
    let timeout_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(stale_after_revalidation_resource_declaration(
            pending_node,
            3,
        ))
        .expect("stale-after declaration should lower");
    runtime
        .declare_resource_node(timeout_resource_declaration(timeout_node, 1))
        .expect("timeout declaration should lower");
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            pending_node,
        )))
        .expect("pending request should admit");
    let timeout_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            timeout_node,
        )))
        .expect("timeout request should admit")
        .admitted_request()
        .handle();
    let timeout_wake = runtime
        .in_flight_resource_request(timeout_handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .expect("clock should reach timeout due tick");
    let stray_ready = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should promote");

    let report = runtime
        .admit_stale_after_resource_revalidation(
            ResourceNodeId::from_node(pending_node),
            stray_ready,
        )
        .expect("fulfilled-only denial should be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("pending node must not admit stale-after revalidation");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::StaleAfterRequiresFulfilledLifecycle
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_stale_after_fulfilled_only_denial_count,
        1
    );
}

#[test]
fn resource_new_request_retires_stale_after_wake_before_it_can_fire() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(stale_after_revalidation_resource_declaration(node, 3))
        .expect("stale-after declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage");
    runtime
        .commit_staged_resource_completion(staging.staged_effect())
        .expect("completion should commit");
    let stale_after_wake = runtime
        .active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node))
        .expect("fulfilled node should retain a stale-after wake");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("fresh request should supersede stale-after wake");

    assert_eq!(
        runtime.active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node)),
        None
    );
    assert!(runtime
        .promote_temporal_wake_ready(stale_after_wake)
        .is_err());
}

#[test]
fn resource_stale_after_revalidation_survives_snapshot_restore_with_same_ready_wake_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(stale_after_revalidation_resource_declaration(node, 3))
        .expect("stale-after declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage");
    runtime
        .commit_staged_resource_completion(staging.staged_effect())
        .expect("completion should commit");
    let stale_after_wake = runtime
        .active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node))
        .expect("fulfilled node should retain stale-after wake before restore");
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("post-snapshot mutation should change active state");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate stale-after state");

    let restored_wake = runtime
        .active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node))
        .expect("restore should preserve stale-after wake evidence");
    assert_eq!(restored_wake, stale_after_wake);
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach restored stale-after due tick");
    let ready_wake = runtime
        .promote_temporal_wake_ready(restored_wake)
        .expect("restored stale-after wake should promote");

    let report = runtime
        .admit_stale_after_resource_revalidation(ResourceNodeId::from_node(node), ready_wake)
        .expect("restored stale-after ready wake should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("restored stale-after wake should still revalidate");

    assert_eq!(
        revalidation.admitted_request().handle().generation(),
        ResourceGeneration::new(2)
    );
    assert_eq!(
        runtime.active_resource_stale_after_wake_for_node(ResourceNodeId::from_node(node)),
        None
    );
}

#[test]
fn resource_retry_and_revalidation_decision_artifacts_remain_distinct() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            forced_revalidation_resource_declaration(node).with_retry_policy(
                ResourceRetryPolicyDeclaration::FixedDelay {
                    delay: TemporalDuration::temporal_duration(3).unwrap(),
                },
            ),
        )
        .expect("retry and revalidation declaration should lower");

    let descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should be visible");

    assert_ne!(
        descriptor.retry_decision_plan().decision_digest().as_str(),
        descriptor
            .revalidation_decision_plan()
            .decision_digest()
            .as_str()
    );
    assert_eq!(
        descriptor.revalidation_decision_plan().class(),
        ResourceRevalidationDecisionClass::ExplicitOrActiveHandleForced
    );
}

#[test]
fn resource_snapshot_restore_rekeys_in_flight_handles_to_new_restore_epoch() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let pre_restore = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit before snapshot")
        .admitted_request()
        .handle();
    assert_eq!(pre_restore.branch_epoch().restore_epoch(), 0);
    let boundary_envelopes_at_snapshot = runtime
        .telemetry()
        .resource
        .resource_boundary_performance_envelope_count;
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should mutate resource state before restore");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate captured resource state");
    let restore_report = runtime
        .latest_resource_branch_restore_report()
        .expect("resource restore should publish a report");

    assert!(
        runtime.in_flight_resource_request(pre_restore).is_none(),
        "pre-restore handles must not resolve after branch restore changes the resource epoch"
    );
    assert_eq!(
        restore_report.performance().boundary(),
        ResourceBoundaryKind::BranchRestore
    );
    assert_eq!(restore_report.performance().cost_contract().get(), 13);
    assert_eq!(
        restore_report.performance().cost_posture(),
        ResourceCostPosture::Verified
    );
    assert_eq!(restore_report.restored_in_flight_width(), 1);
    assert_eq!(restore_report.retained_summary_width(), 1);
    assert_eq!(restore_report.broad_rebuild_denial_count(), 1);
    assert_eq!(restore_report.performance().broad_scan_denial_count(), 1);
    assert_eq!(
        runtime.telemetry().resource.resource_branch_restore_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_branch_restore_in_flight_width,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_branch_restore_retained_summary_width,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_branch_restore_broad_rebuild_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        boundary_envelopes_at_snapshot + 1
    );

    let post_restore = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("restored resource state should admit a new epoch-safe request");
    assert_eq!(
        post_restore
            .superseded_request()
            .expect("restored in-flight request should be superseded")
            .branch_epoch()
            .restore_epoch(),
        1
    );
    assert_eq!(
        post_restore
            .admitted_request()
            .handle()
            .branch_epoch()
            .restore_epoch(),
        1
    );
}

#[test]
fn resource_replay_reconstruction_digest_matches_after_snapshot_restore() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();
    runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            ResourceRequestId::new(9_999),
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown request should produce retained denial");
    let snapshot = runtime.capture_snapshot();
    let expected = runtime.reconstruct_resource_replay_summary();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("post-snapshot request should mutate resource state");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate captured resource state");
    let boundary_envelopes_before_replay = runtime
        .telemetry()
        .resource
        .resource_boundary_performance_envelope_count;
    let replayed = runtime.reconstruct_resource_replay_summary();

    assert_eq!(
        replayed.performance().boundary(),
        ResourceBoundaryKind::ReplayReconstruction
    );
    assert_eq!(replayed.performance().cost_contract().get(), 14);
    assert_eq!(
        replayed.performance().cost_posture(),
        ResourceCostPosture::Debt
    );
    assert_eq!(replayed.descriptor_width(), 1);
    assert_eq!(replayed.lifecycle_summary_width(), 1);
    assert_eq!(replayed.denied_completion_width(), 1);
    assert_eq!(replayed.in_flight_width(), 0);
    assert_eq!(replayed.retained_history_unavailable_count(), 0);
    assert_eq!(replayed.descriptor_digest(), expected.descriptor_digest());
    assert_eq!(replayed.lifecycle_digest(), expected.lifecycle_digest());
    assert_eq!(
        replayed.output_continuity_digest(),
        expected.output_continuity_digest()
    );
    assert_eq!(
        replayed.denied_completion_digest(),
        expected.denied_completion_digest()
    );
    assert_eq!(replayed.in_flight_digest(), expected.in_flight_digest());
    assert_eq!(replayed.replay_digest(), expected.replay_digest());
    assert_eq!(replayed.performance().input_width(), 3);
    assert_eq!(replayed.performance().lifecycle_transition_count(), 1);
    assert_eq!(replayed.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_lifecycle_width,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_denial_width,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        boundary_envelopes_before_replay + 1
    );
}

#[test]
fn resource_certification_bundle_requires_all_named_phase10_families() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let first_admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit");
    let first_request = first_admission.admitted_request();
    let second_admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first request");
    let superseded_request = second_admission
        .superseded_request()
        .expect("second admission should retain supersession evidence");
    assert_eq!(superseded_request, first_request.handle());
    let second_request = second_admission.admitted_request();
    let snapshot = runtime.capture_snapshot();

    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            second_request.handle(),
            second_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("current completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("admitted completion should stage");
    let rollback = runtime.rollback_staged_resource_completion(staging.staged_effect());

    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate resource state");
    let restore_report = runtime
        .latest_resource_branch_restore_report()
        .expect("resource restore should publish branch evidence");
    let replay = runtime.reconstruct_resource_replay_summary();

    let bundle = resource_certification_builder()
        .with_async_resource_lifecycle_parity(&replay)
        .expect("lifecycle parity evidence should be accepted")
        .with_out_of_order_completion_supersession(second_admission)
        .expect("supersession evidence should be accepted")
        .with_async_rollback_observation_equivalence(rollback)
        .expect("rollback evidence should be accepted")
        .with_async_branch_restore_replay_equivalence(restore_report, &replay)
        .expect("branch/replay evidence should be accepted")
        .with_async_inflight_boundedness(
            runtime.resource_runtime_summary(),
            first_admission.performance(),
        )
        .expect("boundedness evidence should be accepted")
        .build()
        .expect("complete resource certification bundle should pass");

    assert!(bundle.passed());
    assert_eq!(
        bundle.schema_version(),
        RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION
    );
    assert_eq!(
        bundle.records().len(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len()
    );
    assert_eq!(
        bundle.summary().passed_family_count(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(bundle.summary().missing_family_count(), 0);
    assert_eq!(bundle.summary().duplicate_family_count(), 0);
    assert!(bundle.failures().is_empty());
    assert!(bundle
        .records()
        .iter()
        .all(|record| record.performance().cost_contract().get() > 0));
}

#[test]
fn resource_certification_bundle_reports_missing_duplicate_and_parity_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit");
    let performance = admission.performance();
    let missing_supersession = resource_certification_builder()
        .with_out_of_order_completion_supersession(admission)
        .expect_err("supersession family must require real supersession evidence");
    assert!(format!("{missing_supersession}")
        .contains("requires request admission with supersession evidence"));

    let lifecycle = ResourceCertificationRecord::passing(
        ResourceCertificationFamily::AsyncResourceLifecycleParity,
        "lifecycle",
        performance,
    )
    .expect("non-empty evidence digest should certify a record");
    let duplicate_lifecycle = ResourceCertificationRecord::passing(
        ResourceCertificationFamily::AsyncResourceLifecycleParity,
        "lifecycle-duplicate",
        performance,
    )
    .expect("duplicate family is reported at bundle construction");
    let partial = resource_certification_bundle([lifecycle.clone(), duplicate_lifecycle]);

    assert!(!partial.passed());
    assert_eq!(partial.summary().missing_family_count(), 4);
    assert_eq!(partial.summary().duplicate_family_count(), 1);
    assert!(partial.failures().iter().any(|failure| matches!(
        failure,
        ResourceCertificationFailure::DuplicateFamily {
            family: ResourceCertificationFamily::AsyncResourceLifecycleParity,
            count: 2
        }
    )));

    let complete = resource_certification_fixture_bundle(ResourceRequestId::new(9_999));
    let drifted = resource_certification_fixture_bundle(ResourceRequestId::new(9_998));
    let parity = resource_certification_bundle_parity_report(&complete, &drifted);

    assert!(!parity.parity());
    assert!(parity
        .mismatch_classes()
        .contains(&ResourceCertificationBundleMismatchClass::BundleDigestMismatch));
    assert!(parity
        .mismatch_classes()
        .contains(&ResourceCertificationBundleMismatchClass::RecordSetMismatch));
    assert!(ResourceCertificationRecord::passing(
        ResourceCertificationFamily::AsyncInflightBoundedness,
        "",
        performance,
    )
    .is_err());
    let builder_err = resource_certification_builder()
        .with_async_inflight_boundedness(runtime.resource_runtime_summary(), performance)
        .expect("first lifecycle record should be accepted")
        .with_async_inflight_boundedness(runtime.resource_runtime_summary(), performance)
        .expect_err("duplicate builder family must reject before bundle construction");
    assert!(format!("{builder_err}").contains("duplicate certification family evidence"));
}

#[test]
fn resource_milestone_b_certification_run_requires_complete_passing_bundle() {
    let (complete, hostile_evidence, summary_read, diagnostics_summary, diagnostics_denial) =
        resource_certification_fixture_artifacts(ResourceRequestId::new(9_999));
    let scenario_matrix = resource_milestone_b_scenario_matrix(&complete, &hostile_evidence)
        .expect("complete passing resource bundle should produce scenario matrix");
    let performance_closeout = resource_milestone_b_performance_closeout(
        &scenario_matrix,
        summary_read,
        diagnostics_summary,
        diagnostics_denial,
    )
    .expect("complete passing resource evidence should produce performance closeout");
    let run = resource_milestone_b_certification_run(
        complete.clone(),
        scenario_matrix.clone(),
        performance_closeout.clone(),
    )
    .expect("complete passing resource bundle should close milestone B certification");

    assert!(run.passed());
    assert!(scenario_matrix.passed());
    assert!(performance_closeout.passed());
    assert_eq!(
        run.schema_version(),
        RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION
    );
    assert_eq!(run.bundle().bundle_digest(), complete.bundle_digest());
    assert_eq!(run.scenario_matrix(), &scenario_matrix);
    assert_eq!(run.performance_closeout(), &performance_closeout);
    assert_eq!(
        scenario_matrix.schema_version(),
        RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(scenario_matrix.bundle_digest(), complete.bundle_digest());
    assert_eq!(
        scenario_matrix.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len()
    );
    assert_eq!(
        hostile_evidence.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS.len()
    );
    assert_eq!(
        hostile_evidence.schema_version(),
        RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION
    );
    assert_eq!(
        performance_closeout.schema_version(),
        RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION
    );
    assert_eq!(
        performance_closeout.scenario_matrix_digest(),
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        performance_closeout.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len()
    );
    assert_eq!(
        scenario_matrix.summary().required_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
    );
    assert_eq!(
        scenario_matrix.summary().certified_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
    );
    assert_eq!(scenario_matrix.summary().failed_scenario_count(), 0);
    assert_eq!(
        scenario_matrix.summary().bundle_digest(),
        complete.bundle_digest()
    );
    assert_eq!(
        performance_closeout.summary().required_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        performance_closeout.summary().certified_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(performance_closeout.summary().failed_claim_count(), 0);
    assert_eq!(
        performance_closeout.summary().scenario_matrix_digest(),
        scenario_matrix.matrix_digest()
    );

    for scenario in REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS {
        let row = required_scenario_row(&scenario_matrix, scenario);
        assert_eq!(row.certification_family(), scenario.certification_family());
        assert_eq!(
            row.completion_denial_class(),
            scenario.completion_denial_class()
        );
        assert!(row.passed());
        assert!(!row.evidence_digest().is_empty());
    }
    for scenario in REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS {
        let evidence_row = required_hostile_evidence_row(&hostile_evidence, scenario);
        assert_hostile_evidence_shape(evidence_row);

        let matrix_row = required_scenario_row(&scenario_matrix, scenario);
        assert_eq!(
            matrix_row.evidence_kind(),
            ResourceMilestoneBScenarioEvidenceKind::HostileCompletionDenial
        );
        assert_eq!(
            matrix_row.completion_denial_class(),
            scenario.completion_denial_class()
        );
        assert_eq!(
            matrix_row.performance(),
            evidence_row.performance(),
            "hostile matrix row should preserve the source evidence performance envelope"
        );
        assert!(matrix_row.certification_family().is_none());
        assert!(matrix_row.passed());
    }
    for claim in REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS {
        assert_performance_closeout_claim_shape(required_performance_claim_row(
            &performance_closeout,
            claim,
        ));
    }
    assert_eq!(
        run.summary().required_family_count(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(
        run.summary().certified_family_count(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(run.summary().failed_family_count(), 0);
    assert_eq!(run.summary().bundle_digest(), complete.bundle_digest());
    assert_eq!(
        run.summary().required_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
    );
    assert_eq!(
        run.summary().certified_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
    );
    assert_eq!(
        run.summary().scenario_matrix_digest(),
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        run.summary().required_performance_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        run.summary().certified_performance_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        run.summary().performance_closeout_digest(),
        performance_closeout.closeout_digest()
    );
    assert!(!run.run_digest().is_empty());
    let serialized_run =
        serde_json::to_value(&run).expect("closeout certification run should serialize");
    assert_eq!(
        serialized_run["scenarioMatrix"]["matrixDigest"],
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        serialized_run["summary"]["scenarioMatrixDigest"],
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        serialized_run["summary"]["performanceCloseoutDigest"],
        performance_closeout.closeout_digest()
    );

    let incomplete = resource_certification_bundle([]);
    let err = resource_milestone_b_scenario_matrix(&incomplete, &hostile_evidence)
        .expect_err("incomplete certification bundle must not become scenario evidence");
    assert!(format!("{err}").contains("resource certification bundle failed"));
    let err = resource_milestone_b_certification_run(
        incomplete,
        scenario_matrix.clone(),
        performance_closeout.clone(),
    )
    .expect_err("incomplete certification bundle must not become a milestone run");
    assert!(format!("{err}").contains("resource certification bundle failed"));
    let misclassified_hostile = resource_milestone_b_hostile_scenario_evidence(
        resource_late_cancelled_completion_report(),
        resource_late_cancelled_completion_report(),
        resource_late_timed_out_completion_report(),
        resource_malformed_completion_report(),
    )
    .expect_err("hostile scenario evidence must reject the wrong denial class per row");
    assert!(format!("{misclassified_hostile}").contains("requires Superseded denial evidence"));

    let (
        drifted,
        drifted_hostile_evidence,
        drifted_summary_read,
        drifted_diagnostics_summary,
        drifted_diagnostics_denial,
    ) = resource_certification_fixture_artifacts(ResourceRequestId::new(9_998));
    let drifted_matrix = resource_milestone_b_scenario_matrix(&drifted, &drifted_hostile_evidence)
        .expect("drifted but complete bundle should produce its own scenario matrix");
    let drifted_performance_closeout = resource_milestone_b_performance_closeout(
        &drifted_matrix,
        drifted_summary_read,
        drifted_diagnostics_summary,
        drifted_diagnostics_denial,
    )
    .expect("drifted but complete evidence should produce performance closeout");
    let wrong_matrix_err = resource_milestone_b_certification_run(
        complete,
        drifted_matrix.clone(),
        drifted_performance_closeout.clone(),
    )
    .expect_err("scenario matrix from a different bundle must not close the run");
    assert!(format!("{wrong_matrix_err}").contains("same bundle"));
    let wrong_performance_err = resource_milestone_b_certification_run(
        drifted.clone(),
        drifted_matrix.clone(),
        performance_closeout,
    )
    .expect_err("performance closeout from a different matrix must not close the run");
    assert!(format!("{wrong_performance_err}").contains("same scenario matrix"));
    let drifted_run = resource_milestone_b_certification_run(
        drifted,
        drifted_matrix,
        drifted_performance_closeout,
    )
    .expect("drifted but complete bundle should still produce its own run");
    assert_ne!(
        run.bundle().bundle_digest(),
        drifted_run.bundle().bundle_digest()
    );
    assert_ne!(
        run.scenario_matrix().matrix_digest(),
        drifted_run.scenario_matrix().matrix_digest()
    );
    assert_ne!(run.run_digest(), drifted_run.run_digest());
}

fn resource_certification_fixture_bundle(
    retained_denial_request_id: ResourceRequestId,
) -> ResourceCertificationBundle {
    resource_certification_fixture_artifacts(retained_denial_request_id).0
}

fn resource_certification_fixture_artifacts(
    retained_denial_request_id: ResourceRequestId,
) -> (
    ResourceCertificationBundle,
    ResourceMilestoneBHostileScenarioEvidence,
    ResourceRuntimeSummaryReadReport,
    ResourceDiagnosticsSummary,
    ResourceDiagnosticsExpansionDenial,
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should exist")
        .payload_contract_digest()
        .clone();
    runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            retained_denial_request_id,
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown request should produce retained denial evidence");
    let first_admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit");
    let second_admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first request");
    let second_request = second_admission.admitted_request();
    let snapshot = runtime.capture_snapshot();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            second_request.handle(),
            second_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("current completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("admitted completion should stage");
    let rollback = runtime.rollback_staged_resource_completion(staging.staged_effect());
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate resource state");
    let restore = runtime
        .latest_resource_branch_restore_report()
        .expect("resource restore should publish branch evidence");
    let replay = runtime.reconstruct_resource_replay_summary();

    let bundle = resource_certification_builder()
        .with_async_resource_lifecycle_parity(&replay)
        .expect("lifecycle evidence should be accepted")
        .with_out_of_order_completion_supersession(second_admission)
        .expect("supersession evidence should be accepted")
        .with_async_rollback_observation_equivalence(rollback)
        .expect("rollback evidence should be accepted")
        .with_async_branch_restore_replay_equivalence(restore, &replay)
        .expect("branch/replay evidence should be accepted")
        .with_async_inflight_boundedness(
            runtime.resource_runtime_summary(),
            first_admission.performance(),
        )
        .expect("boundedness evidence should be accepted")
        .build()
        .expect("complete fixture bundle should pass");
    let summary_read = runtime.resource_runtime_summary_read_report();
    let diagnostics_summary =
        runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction();
    let diagnostics_denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::retained_summary_only(),
        )
        .expect_err("retained-only diagnostics budget should deny cold reconstruction");
    let hostile_evidence = resource_milestone_b_hostile_scenario_evidence(
        resource_late_superseded_completion_report(),
        resource_late_cancelled_completion_report(),
        resource_late_timed_out_completion_report(),
        resource_malformed_completion_report(),
    )
    .expect("hostile completion evidence should cover required denial lanes");
    (
        bundle,
        hostile_evidence,
        summary_read,
        diagnostics_summary,
        diagnostics_denial,
    )
}

fn resource_late_superseded_completion_report() -> ResourceCompletionAdmissionReport {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request();
    let stale_first = raw_completion(&runtime, node, first.handle(), first.attempt(), 64);
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first");
    runtime.admit_resource_completion(stale_first)
}

fn resource_late_cancelled_completion_report() -> ResourceCompletionAdmissionReport {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let late = raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should retire request");
    runtime.admit_resource_completion(late)
}

fn resource_late_timed_out_completion_report() -> ResourceCompletionAdmissionReport {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let late = raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("authoritative clock should advance");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should promote");
    runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should consume wake");
    runtime.admit_resource_completion(late)
}

fn resource_malformed_completion_report() -> ResourceCompletionAdmissionReport {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted.handle();
    runtime.admit_resource_completion(RawCompletionEnvelope::new(
        handle.request_id(),
        handle.generation(),
        handle.branch_epoch(),
        admitted.attempt(),
        ResourcePayloadContractDigest::new("payload-contract:999:1024"),
        64,
    ))
}

#[test]
fn resource_diagnostics_summary_preserves_truth_and_exposes_replay_debt() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should exist")
        .payload_contract_digest()
        .clone();
    runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            ResourceRequestId::new(9_999),
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown completion should retain denial provenance");

    let runtime_summary_before = runtime.resource_runtime_summary();
    let replay_count_before = runtime
        .telemetry()
        .resource
        .resource_replay_reconstruction_count;
    let allocation_telemetry_before = runtime.telemetry().resource;
    let diagnostics = runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction();

    assert_eq!(
        diagnostics.schema_version(),
        RESOURCE_DIAGNOSTICS_SUMMARY_SCHEMA_VERSION
    );
    assert_eq!(diagnostics.runtime_summary(), runtime_summary_before);
    assert_eq!(runtime.resource_runtime_summary(), runtime_summary_before);
    assert!(diagnostics.latest_branch_restore_report().is_none());
    assert_eq!(
        diagnostics
            .replay_reconstruction()
            .performance()
            .cost_posture(),
        ResourceCostPosture::Debt
    );
    assert_eq!(
        diagnostics.performance().boundary(),
        ResourceBoundaryKind::DiagnosticsExpansion
    );
    assert_eq!(
        diagnostics.performance().cost_posture(),
        ResourceCostPosture::Debt
    );
    assert_eq!(
        diagnostics
            .expansion_budget()
            .max_replay_reconstruction_width(),
        u32::MAX
    );
    assert_eq!(
        diagnostics.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::BudgetedExpansion
    );
    assert!(
        !diagnostics.policy_decision_digest().as_str().is_empty(),
        "diagnostics summary should retain the effective diagnostics policy digest"
    );
    assert_eq!(
        diagnostics
            .replay_reconstruction()
            .denied_completion_width(),
        1
    );
    assert_eq!(
        diagnostics
            .replay_reconstruction()
            .performance()
            .diagnostics_allocation_count(),
        diagnostics
            .replay_reconstruction()
            .performance()
            .input_width()
    );
    assert_eq!(
        diagnostics.performance().diagnostics_allocation_count(),
        diagnostics
            .replay_reconstruction()
            .performance()
            .input_width()
    );
    assert_eq!(
        diagnostics.performance().facade_report_allocation_count(),
        1
    );
    assert_eq!(diagnostics.performance().operational_allocation_count(), 0);
    assert_eq!(
        diagnostics
            .performance()
            .retained_history_allocation_count(),
        0
    );
    assert!(!diagnostics.provenance_digest().is_empty());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        replay_count_before + 1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count
            - allocation_telemetry_before.resource_diagnostics_allocation_count,
        diagnostics
            .replay_reconstruction()
            .performance()
            .diagnostics_allocation_count() as u64
            + diagnostics.performance().diagnostics_allocation_count() as u64
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before.resource_facade_report_allocation_count,
        diagnostics
            .replay_reconstruction()
            .performance()
            .facade_report_allocation_count() as u64
            + diagnostics.performance().facade_report_allocation_count() as u64
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_operational_allocation_count,
        allocation_telemetry_before.resource_operational_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_history_allocation_count,
        allocation_telemetry_before.resource_retained_history_allocation_count
    );
}

#[test]
fn resource_runtime_summary_read_report_is_zero_cold_reconstruction() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let replay_count_before = runtime
        .telemetry()
        .resource
        .resource_replay_reconstruction_count;
    let diagnostics_expansion_before = runtime
        .telemetry()
        .resource
        .resource_diagnostics_expansion_count;
    let allocation_telemetry_before = runtime.telemetry().resource;
    let report = runtime.resource_runtime_summary_read_report();

    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::SummaryRead
    );
    assert_eq!(
        report.performance().cost_posture(),
        ResourceCostPosture::Verified
    );
    assert_eq!(report.performance().operational_allocation_count(), 0);
    assert_eq!(report.performance().retained_history_allocation_count(), 0);
    assert_eq!(report.performance().diagnostics_allocation_count(), 0);
    assert_eq!(report.performance().facade_report_allocation_count(), 1);
    assert_eq!(report.performance().broad_scan_denial_count(), 0);
    assert_eq!(report.summary(), runtime.resource_runtime_summary());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_summary_read_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        replay_count_before
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        diagnostics_expansion_before
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_operational_allocation_count,
        allocation_telemetry_before.resource_operational_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_history_allocation_count,
        allocation_telemetry_before.resource_retained_history_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count,
        allocation_telemetry_before.resource_diagnostics_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before.resource_facade_report_allocation_count,
        1
    );
}

#[test]
fn resource_diagnostics_summary_respects_cold_reconstruction_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let allocation_telemetry_before = runtime.telemetry().resource;

    let err = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::retained_summary_only(),
        )
        .expect_err("retained-summary-only diagnostics should deny replay reconstruction");

    assert_eq!(
        err.class(),
        ResourceDiagnosticsExpansionDenialClass::ColdReconstructionDisabled
    );
    assert_eq!(
        err.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::BudgetedExpansion
    );
    assert!(
        !err.policy_decision_digest().as_str().is_empty(),
        "diagnostics denial should retain the effective diagnostics policy digest"
    );
    assert_eq!(err.replay_reconstruction_width(), 2);
    assert_eq!(
        err.performance().boundary(),
        ResourceBoundaryKind::DiagnosticsExpansion
    );
    assert_eq!(err.performance().denied_count(), 1);
    assert_eq!(
        err.performance().cost_posture(),
        ResourceCostPosture::DeniedFallback
    );
    assert_eq!(err.performance().operational_allocation_count(), 0);
    assert_eq!(err.performance().retained_history_allocation_count(), 0);
    assert_eq!(err.performance().diagnostics_allocation_count(), 0);
    assert_eq!(err.performance().facade_report_allocation_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_policy_decision_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_operational_allocation_count,
        allocation_telemetry_before.resource_operational_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_history_allocation_count,
        allocation_telemetry_before.resource_retained_history_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count,
        allocation_telemetry_before.resource_diagnostics_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before.resource_facade_report_allocation_count,
        1
    );

    let allocation_telemetry_before_admission = runtime.telemetry().resource;
    let admitted = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(2),
        )
        .expect("budget that admits descriptor plus lifecycle reconstruction should pass");

    assert_eq!(
        admitted.performance().boundary(),
        ResourceBoundaryKind::DiagnosticsExpansion
    );
    assert_eq!(
        admitted
            .expansion_budget()
            .max_replay_reconstruction_width(),
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        1
    );
    assert_eq!(admitted.performance().diagnostics_allocation_count(), 2);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count
            - allocation_telemetry_before_admission.resource_diagnostics_allocation_count,
        4
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before_admission.resource_facade_report_allocation_count,
        2
    );
}

#[test]
fn resource_diagnostics_summary_denies_when_replay_width_exceeds_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(1),
        )
        .expect_err("descriptor plus lifecycle width should exceed budget one");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::ReplayReconstructionBudgetExceeded
    );
    assert_eq!(denial.budget().max_replay_reconstruction_width(), 1);
    assert_eq!(denial.replay_reconstruction_width(), 2);
    assert_eq!(denial.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        0
    );
}

#[test]
fn resource_diagnostics_policy_retained_only_denies_cold_reconstruction_even_when_caller_budget_allows(
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retained_only_diagnostics_resource_declaration(node))
        .expect("retained-only diagnostics declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect_err("retained-only policy should deny cold reconstruction");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::PolicyRetainedOnly
    );
    assert_eq!(
        denial.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::RetainedOnly
    );
    assert_eq!(denial.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_policy_decision_count,
        1
    );
}

#[test]
fn resource_diagnostics_policy_budgeted_expansion_denies_above_descriptor_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(node, 1))
        .expect("budgeted diagnostics declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect_err("descriptor-backed diagnostics budget should cap cold reconstruction");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::PolicyReplayReconstructionBudgetExceeded
    );
    assert_eq!(
        denial.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::BudgetedExpansion
    );
    assert_eq!(denial.replay_reconstruction_width(), 2);
}

#[test]
fn resource_diagnostics_policy_forensic_expansion_budget_denies_above_descriptor_forensic_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(forensic_diagnostics_resource_declaration(node, 2, 1))
        .expect("forensic diagnostics declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction_with_forensic_budget(
                u32::MAX,
                u32::MAX,
            ),
        )
        .expect_err("descriptor forensic budget should deny above-forensic reconstruction");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::PolicyForensicReconstructionBudgetExceeded
    );
    assert_eq!(
        denial.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::ForensicExpansionBudget
    );
    assert_eq!(denial.replay_reconstruction_width(), 2);
    assert_eq!(denial.forensic_reconstruction_width(), 2);
}

#[test]
fn resource_diagnostics_summary_denies_when_caller_forensic_budget_is_tighter_than_replay_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction_with_forensic_budget(
                8, 1,
            ),
        )
        .expect_err("caller forensic budget should deny even when replay budget allows");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::ForensicReconstructionBudgetExceeded
    );
    assert_eq!(denial.replay_reconstruction_width(), 2);
    assert_eq!(denial.forensic_reconstruction_width(), 2);
    assert_eq!(denial.budget().max_replay_reconstruction_width(), 8);
    assert_eq!(denial.budget().max_forensic_reconstruction_width(), 1);
}

#[test]
fn resource_diagnostics_policy_mixed_nodes_use_hard_denial_posture_over_budgeted_nodes() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(budgeted_diagnostics_resource_declaration(first, 5))
        .expect("budgeted diagnostics declaration should lower");
    runtime
        .declare_resource_node(deny_cold_diagnostics_resource_declaration(second))
        .expect("deny-cold diagnostics declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(5),
        )
        .expect_err("deny-cold diagnostics policy should dominate mixed nodes");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::PolicyColdReconstructionDisabled
    );
    assert_eq!(
        denial.policy_decision_class(),
        ResourceDiagnosticsDecisionClass::DenyColdExpansion
    );
}

#[test]
fn resource_diagnostics_summary_digest_tracks_retained_denial_drift() {
    let left = resource_diagnostics_summary_for_unknown_completion(ResourceRequestId::new(9_999));
    let right = resource_diagnostics_summary_for_unknown_completion(ResourceRequestId::new(9_998));

    assert_ne!(left.provenance_digest(), right.provenance_digest());
    assert_ne!(
        left.replay_reconstruction().denied_completion_digest(),
        right.replay_reconstruction().denied_completion_digest()
    );
    assert_eq!(left.runtime_summary(), right.runtime_summary());
}

#[test]
fn resource_diagnostics_summary_digest_tracks_expansion_budget() {
    let strict = resource_diagnostics_summary_for_budget(
        ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(2),
    );
    let loose = resource_diagnostics_summary_for_budget(
        ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(8),
    );

    assert_ne!(strict.provenance_digest(), loose.provenance_digest());
    assert_eq!(
        strict.replay_reconstruction().replay_digest(),
        loose.replay_reconstruction().replay_digest()
    );
    assert_eq!(strict.runtime_summary(), loose.runtime_summary());
}

fn resource_diagnostics_summary_for_budget(
    budget: ResourceDiagnosticsExpansionBudget,
) -> ResourceDiagnosticsSummary {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    runtime
        .try_resource_diagnostics_summary(budget)
        .expect("budget should admit descriptor plus lifecycle reconstruction")
}

fn resource_diagnostics_summary_for_unknown_completion(
    request_id: ResourceRequestId,
) -> ResourceDiagnosticsSummary {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should exist")
        .payload_contract_digest()
        .clone();
    runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            request_id,
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown completion should retain denial provenance");

    runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction()
}

#[test]
fn resource_request_identity_is_not_node_identity() {
    let node = NodeId::new(3, 0);
    let resource_node = ResourceNodeId::from_node(node);
    let request = ResourceRequestId::new(3);

    assert_eq!(resource_node.node(), node);
    assert_eq!(request.get(), node.index() as u64);
}

#[test]
fn resource_completion_admission_accepts_matching_active_request_without_committing_lifecycle() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted_request.handle();

    let report = runtime.admit_resource_completion(raw_completion(
        &runtime,
        node,
        handle,
        admitted_request.attempt(),
        64,
    ));

    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::CompletionAdmission
    );
    assert_eq!(report.performance().input_width(), 1);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().denied_count(), 0);
    assert_eq!(report.performance().lifecycle_transition_count(), 1);
    assert_eq!(
        report.performance().density_strategy(),
        ResourceDensityStrategy::SparseIndexedLookup
    );
    let completion = report
        .admitted_completion()
        .expect("matching envelope should admit");
    assert_eq!(completion.handle(), handle);
    assert_eq!(completion.node(), ResourceNodeId::from_node(node));
    assert_eq!(completion.payload_byte_len(), 64);
    assert_eq!(
        completion.lifecycle_transition().kind(),
        ResourceLifecycleTransitionKind::CompletionAdmitted
    );
    assert_eq!(
        completion.lifecycle_transition().from(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        completion.lifecycle_transition().to(),
        ResourceLifecycleClass::Fulfilled
    );
    assert_eq!(
        completion.completion_ordinal(),
        ResourceCompletionOrdinal::new(1)
    );
    assert!(report.denied_completion().is_none());

    let in_flight = runtime
        .in_flight_resource_request(handle)
        .expect("admission must not retire or mutate in-flight state before apply");
    assert_eq!(in_flight.lifecycle(), ResourceLifecycleClass::Pending);
    assert_eq!(in_flight.status(), ResourceInFlightStatus::Active);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_validation_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_denial_count,
        0
    );
}

#[test]
fn resource_completion_stage_and_commit_apply_lifecycle_once() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted_request.handle();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            handle,
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("matching completion should admit");

    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("active admitted completion should stage");
    assert_eq!(
        staging.performance().boundary(),
        ResourceBoundaryKind::CompletionStaging
    );
    assert_eq!(staging.performance().cost_contract().get(), 9);
    assert_eq!(
        staging.performance().cost_posture(),
        ResourceCostPosture::Verified
    );
    assert_eq!(staging.performance().lifecycle_transition_count(), 0);
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("staging must not mutate request lifecycle")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);

    let commit = runtime
        .commit_staged_resource_completion(staging.staged_effect())
        .expect("staged completion should commit exactly once");

    assert_eq!(
        commit.performance().boundary(),
        ResourceBoundaryKind::CompletionCommit
    );
    assert_eq!(commit.lifecycle().node(), ResourceNodeId::from_node(node));
    assert_eq!(
        commit.lifecycle().lifecycle(),
        ResourceLifecycleClass::Fulfilled
    );
    assert_eq!(
        commit.transition().kind(),
        ResourceLifecycleTransitionKind::CompletionAdmitted
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("fulfilled request remains retained for audit")
            .status(),
        ResourceInFlightStatus::Fulfilled
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        0
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_staging_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_commit_count,
        1
    );
}

#[test]
fn resource_completion_rollback_of_staged_admitted_preserves_pending_request() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted_request.handle();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            handle,
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("matching completion should admit");

    let staged = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("active admitted completion should stage")
        .staged_effect();
    let rollback = runtime.rollback_staged_resource_completion(staged);

    assert_eq!(
        rollback.performance().boundary(),
        ResourceBoundaryKind::CompletionRollback
    );
    assert_eq!(rollback.performance().admitted_count(), 1);
    assert_eq!(rollback.performance().denied_count(), 0);
    assert_eq!(rollback.performance().lifecycle_transition_count(), 0);
    assert_eq!(
        rollback.rolled_back_completion().subject(),
        ResourceCompletionRollbackSubject::Admitted {
            handle,
            node: ResourceNodeId::from_node(node),
            completion_ordinal: ResourceCompletionOrdinal::new(1),
        }
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("rollback must leave request available for a later commit")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        1
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_rollback_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_commit_count,
        0
    );
}

#[test]
fn resource_completion_transaction_commit_delivers_lifecycle_observation_once() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted_request.handle();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            handle,
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("matching completion should admit");
    let calls = Arc::new(Mutex::new(Vec::<ResourceObservationRecord>::new()));
    let observation_handle = runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::clone(&calls),
        }),
    );

    let mut ctx = ();
    let result = runtime
        .transaction(&mut ctx, |tx| {
            let staging = tx.stage_admitted_resource_completion(admitted_completion)?;
            tx.commit_staged_resource_completion(staging.staged_effect())?;
            Ok(())
        })
        .expect("completion transaction should commit");

    let recorded = calls
        .lock()
        .expect("resource observation mutex poisoned")
        .clone();
    assert_eq!(
        recorded,
        vec![ResourceObservationRecord {
            observer_id: observation_handle.observer_id().get(),
            handle_id: observation_handle.handle_id().get(),
            matched_node_count: 1,
            touched: true,
            recomputed: false,
            meaningful_change: true,
            trigger_matched: true,
        }]
    );
    assert_eq!(result.observation.classified_event_count, 1);
    assert_eq!(result.observation.trigger_matched_event_count, 1);
    assert_eq!(result.observation.delivered_event_count, 1);
    assert_eq!(result.observation.rollback_suppressed_event_count, 0);
    assert_eq!(
        result.observation.boundary_events[0]
            .matched_nodes
            .iter()
            .collect::<Vec<_>>(),
        vec![node]
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("fulfilled request remains retained for audit")
            .status(),
        ResourceInFlightStatus::Fulfilled
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime.telemetry().transaction.delivered_observation_count,
        1
    );
}

#[test]
fn resource_completion_transaction_rollback_suppresses_observation_and_restores_state() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted_request.handle();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            handle,
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("matching completion should admit");
    let calls = Arc::new(Mutex::new(Vec::<ResourceObservationRecord>::new()));
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::clone(&calls),
        }),
    );

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    let staging = tx
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage inside transaction");
    tx.commit_staged_resource_completion(staging.staged_effect())
        .expect("completion should mutate transaction-local resource state");
    let result = tx
        .rollback()
        .expect("rollback should restore resource and temporal state");

    assert!(
        calls
            .lock()
            .expect("resource observation mutex poisoned")
            .is_empty(),
        "rollback must suppress completion-driven observation delivery"
    );
    assert_eq!(result.observation.classified_event_count, 1);
    assert_eq!(result.observation.trigger_matched_event_count, 1);
    assert_eq!(result.observation.delivered_event_count, 0);
    assert_eq!(result.observation.rollback_suppressed_event_count, 1);
    assert!(matches!(
        result.observation.boundary_events[0].outcome,
        ObservationBoundaryOutcome::RollbackSuppressed
    ));
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("rollback must restore active request")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_resource_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_temporal_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_suppressed_observation_count,
        1
    );
}

#[test]
fn resource_observation_batch_report_respects_lifecycle_only_and_output_policies_per_node() {
    let mut graph = SignalGraph::new();
    let lifecycle_only_node = graph.node().build();
    let lifecycle_and_output_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(lifecycle_only_observation_resource_declaration(
            lifecycle_only_node,
        ))
        .expect("lifecycle-only declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(lifecycle_and_output_node))
        .expect("lifecycle-and-output declaration should lower");

    let lifecycle_only_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            lifecycle_only_node,
        )))
        .expect("lifecycle-only request should admit")
        .admitted_request();
    let lifecycle_and_output_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            lifecycle_and_output_node,
        )))
        .expect("lifecycle-and-output request should admit")
        .admitted_request();
    let lifecycle_only_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            lifecycle_only_node,
            lifecycle_only_request.handle(),
            lifecycle_only_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("lifecycle-only completion should admit");
    let lifecycle_and_output_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            lifecycle_and_output_node,
            lifecycle_and_output_request.handle(),
            lifecycle_and_output_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("lifecycle-and-output completion should admit");

    let calls = Arc::new(Mutex::new(Vec::<ResourceObservationRecord>::new()));
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [lifecycle_only_node, lifecycle_and_output_node],
        Box::new(ResourceObservationListener {
            calls: Arc::clone(&calls),
        }),
    );

    let mut ctx = ();
    runtime
        .transaction(&mut ctx, |tx| {
            let first = tx.stage_admitted_resource_completion(lifecycle_only_completion)?;
            let second = tx.stage_admitted_resource_completion(lifecycle_and_output_completion)?;
            tx.commit_staged_resource_completion(first.staged_effect())?;
            tx.commit_staged_resource_completion(second.staged_effect())?;
            Ok(())
        })
        .expect("two completions should commit in one transaction");

    let report = runtime
        .latest_resource_observation_batch_report()
        .expect("committed resource observation should materialize");
    let event = &report.events()[0];

    assert_eq!(report.events().len(), 1);
    assert_eq!(event.matched_resource_nodes().len(), 2);
    assert_eq!(event.outcome(), ObservationBoundaryOutcome::Delivered);
    assert_eq!(
        event.matched_resource_nodes()[0].node(),
        ResourceNodeId::from_node(lifecycle_only_node)
    );
    assert_eq!(
        event.matched_resource_nodes()[0].lifecycle(),
        ResourceLifecycleClass::Fulfilled
    );
    assert_eq!(event.matched_resource_nodes()[0].output_continuity(), None);
    assert_eq!(
        event.matched_resource_nodes()[1].node(),
        ResourceNodeId::from_node(lifecycle_and_output_node)
    );
    assert_eq!(
        event.matched_resource_nodes()[1].output_continuity(),
        Some(ResourceOutputContinuity::OutputReplaced)
    );
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::ObservationMaterialization
    );
    assert_eq!(report.performance().input_width(), 1);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().coalescing_width(), 1);
    assert_eq!(
        report
            .performance()
            .output_continuity_classification_width(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_observation_policy_decision_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_observation_candidate_width,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_observation_coalesced_width,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_observation_delivered_width,
        1
    );
}

#[test]
fn resource_observation_batch_report_remains_rollback_safe() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("matching completion should admit");
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::new(Mutex::new(Vec::new())),
        }),
    );

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    let staging = tx
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage");
    tx.commit_staged_resource_completion(staging.staged_effect())
        .expect("completion should mutate transaction-local state");
    tx.rollback()
        .expect("rollback should restore resource state");

    let report = runtime
        .latest_resource_observation_batch_report()
        .expect("rollback-suppressed resource observation should materialize");
    let event = &report.events()[0];

    assert_eq!(
        event.outcome(),
        ObservationBoundaryOutcome::RollbackSuppressed
    );
    assert_eq!(event.matched_resource_nodes().len(), 1);
    assert_eq!(
        event.matched_resource_nodes()[0].lifecycle(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        event.matched_resource_nodes()[0].output_continuity(),
        Some(ResourceOutputContinuity::NoPriorOutput)
    );
    assert_eq!(report.performance().admitted_count(), 0);
    assert_eq!(report.performance().denied_count(), 1);
}

#[test]
fn resource_observation_batch_report_can_include_denied_completion_without_applying_it() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(denied_completion_observation_resource_declaration(node))
        .expect("denied-completion observation declaration should lower");
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::new(Mutex::new(Vec::new())),
        }),
    );
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();

    let denied = runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            admitted.handle().request_id(),
            admitted.handle().generation(),
            admitted.handle().branch_epoch(),
            admitted.attempt(),
            ResourcePayloadContractDigest::new("payload-contract:999:1024"),
            64,
        ))
        .denied_completion()
        .expect("wrong payload contract should deny without apply");
    assert_eq!(denied.class(), CompletionDenialClass::Malformed);

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        node,
        &|view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
        EvaluationRequestMode::Default,
    )
    .expect("evaluation should succeed");
    tx.commit().expect("commit should succeed");

    let report = runtime
        .latest_resource_observation_batch_report()
        .expect("resource observation should materialize denied completion evidence");
    let observed = &report.events()[0].matched_resource_nodes()[0];

    assert_eq!(observed.lifecycle(), ResourceLifecycleClass::Pending);
    assert_eq!(
        observed.output_continuity(),
        Some(ResourceOutputContinuity::NoPriorOutput)
    );
    assert_eq!(
        observed
            .denied_completion()
            .expect("policy should surface denied completion evidence")
            .class(),
        CompletionDenialClass::Malformed
    );
    assert!(observed.scheduled_retry().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_denied_completion_observation_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_schedule_observation_count,
        0
    );

    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted.handle(),
            admitted.attempt(),
            64,
        ))
        .admitted_completion();
    assert!(
        admitted_completion.is_some(),
        "denied-completion observation must not spend or poison real completion authority"
    );
}

#[test]
fn resource_observation_clears_stale_denied_completion_after_authoritative_progress() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let node = graph.node().build();
    graph
        .depends_on(node, source, Aspect::new(0))
        .expect("dependency edge should admit");
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(denied_completion_observation_resource_declaration(node))
        .expect("denied-completion observation declaration should lower");
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::new(Mutex::new(Vec::new())),
        }),
    );
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();

    let denied = runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            admitted.handle().request_id(),
            admitted.handle().generation(),
            admitted.handle().branch_epoch(),
            admitted.attempt(),
            ResourcePayloadContractDigest::new("payload-contract:999:1024"),
            64,
        ))
        .denied_completion()
        .expect("wrong payload contract should deny");
    assert_eq!(denied.class(), CompletionDenialClass::Malformed);

    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted.handle(),
            admitted.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("valid completion should still admit afterward");

    let mut ctx = ();
    runtime
        .transaction(&mut ctx, |tx| {
            let staging = tx.stage_admitted_resource_completion(admitted_completion)?;
            tx.commit_staged_resource_completion(staging.staged_effect())?;
            Ok(())
        })
        .expect("valid completion should commit");

    mark_dirty(runtime.graph_mut(), source, Aspect::new(0))
        .expect("dependency invalidation should create a fresh observation boundary");
    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        node,
        &|view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
        EvaluationRequestMode::Default,
    )
    .expect("evaluation should succeed");
    tx.commit().expect("commit should succeed");

    let report = runtime
        .latest_resource_observation_batch_report()
        .expect("resource observation should materialize current state");
    let observed = &report.events()[0].matched_resource_nodes()[0];
    assert_eq!(observed.lifecycle(), ResourceLifecycleClass::Fulfilled);
    assert!(
        observed.denied_completion().is_none(),
        "fulfilled observation must not leak stale denied-completion evidence"
    );
}

#[test]
fn resource_observation_batch_report_can_include_retry_schedule_without_retry_apply() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_schedule_observation_resource_declaration(node))
        .expect("retry-schedule observation declaration should lower");
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::new(Mutex::new(Vec::new())),
        }),
    );

    let scheduled = schedule_timed_out_retry(&mut runtime, node)
        .scheduled_retry()
        .cloned()
        .expect("timed out request should schedule retry");

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        node,
        &|view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
        EvaluationRequestMode::Default,
    )
    .expect("evaluation should succeed");
    tx.commit().expect("commit should succeed");

    let report = runtime
        .latest_resource_observation_batch_report()
        .expect("resource observation should materialize scheduled retry evidence");
    let observed = &report.events()[0].matched_resource_nodes()[0];

    assert_eq!(observed.lifecycle(), ResourceLifecycleClass::TimedOut);
    assert_eq!(
        observed
            .scheduled_retry()
            .expect("policy should surface scheduled retry evidence")
            .retry_ordinal(),
        scheduled.retry_ordinal()
    );
    assert_eq!(
        observed
            .scheduled_retry()
            .expect("policy should retain scheduled retry reason")
            .reason(),
        ResourceRetryReason::TimedOut
    );
    assert!(observed.denied_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_denied_completion_observation_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_schedule_observation_count,
        1
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(
                runtime
                    .clock_basis()
                    .current_tick()
                    .get()
                    .saturating_add(scheduled.scheduled_delay().get()),
            ),
        ))
        .expect("clock should reach scheduled retry backoff after observation");
    let ready_retry = runtime
        .promote_temporal_wake_ready(scheduled.backoff_wake_id())
        .expect("observation must not consume scheduled retry wake");
    let retry_admission = runtime
        .admit_scheduled_resource_retry(scheduled.previous(), ready_retry)
        .expect("observation must not apply or block the scheduled retry");
    let admitted_retry = retry_admission
        .admitted_retry()
        .expect("ready retry should still admit after observation materialization");
    assert_eq!(
        admitted_retry.scheduled().retry_ordinal(),
        scheduled.retry_ordinal()
    );
}

#[test]
fn resource_observation_clears_superseded_retry_schedule_when_fresh_request_admits() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let node = graph.node().build();
    graph
        .depends_on(node, source, Aspect::new(0))
        .expect("dependency edge should admit");
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_schedule_observation_resource_declaration(node))
        .expect("retry-schedule observation declaration should lower");
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::new(Mutex::new(Vec::new())),
        }),
    );

    let scheduled = schedule_timed_out_retry(&mut runtime, node)
        .scheduled_retry()
        .cloned()
        .expect("timed out request should schedule retry");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("fresh request should supersede the old timed-out lineage");
    assert!(
        runtime
            .promote_temporal_wake_ready(scheduled.backoff_wake_id())
            .is_err(),
        "superseded retry wake should be retired before it can promote"
    );

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        node,
        &|view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
        EvaluationRequestMode::Default,
    )
    .expect("evaluation should succeed");
    tx.commit().expect("commit should succeed");

    let report = runtime
        .latest_resource_observation_batch_report()
        .expect("resource observation should materialize current state");
    let observed = &report.events()[0].matched_resource_nodes()[0];

    assert_eq!(observed.lifecycle(), ResourceLifecycleClass::Pending);
    assert!(
        observed.scheduled_retry().is_none(),
        "fresh request observation must not leak superseded retry schedule"
    );
}

#[test]
fn resource_lifecycle_retention_compaction_moves_terminal_records_out_of_hot_lookup() {
    let mut graph = SignalGraph::new();
    let cancelled_node = graph.node().build();
    let fulfilled_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(cancelled_node))
        .expect("cancelled resource declaration should lower");
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(fulfilled_node))
        .expect("fulfilled resource declaration should lower");
    let cancelled = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            cancelled_node,
        )))
        .expect("cancelled request should admit")
        .admitted_request();
    let fulfilled = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            fulfilled_node,
        )))
        .expect("fulfilled request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(
            cancelled.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancellation should admit");
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            fulfilled_node,
            fulfilled.handle(),
            fulfilled.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit");
    let staged = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage")
        .staged_effect();
    runtime
        .commit_staged_resource_completion(staged)
        .expect("completion should commit");
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        2
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .retained_lifecycle_history_count(),
        0
    );

    let report = runtime.compact_resource_lifecycle_history(1);

    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::LifecycleRetentionCompaction
    );
    assert_eq!(report.selected_terminal_count(), 1);
    assert_eq!(report.reclaimed_in_flight_count(), 1);
    assert_eq!(report.retained_history_write_count(), 1);
    assert_eq!(report.retained_history_pruned_count(), 0);
    assert_eq!(report.retained_history_unavailable_count(), 0);
    assert_eq!(report.retained_history_width(), 1);
    assert_eq!(report.hot_in_flight_width(), 1);
    assert_eq!(report.compacted_terminal_summary_count(), 0);
    assert_eq!(report.performance().input_width(), 1);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().retained_history_allocation_count(), 1);
    assert!(runtime
        .in_flight_resource_request(cancelled.handle())
        .is_none());
    assert!(runtime
        .in_flight_resource_request(fulfilled.handle())
        .is_some());
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        1
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .retained_lifecycle_history_count(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_hot_in_flight_compaction_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_in_flight_reclaimed_record_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_lifecycle_history_write_count,
        1
    );
}

#[test]
fn resource_terminal_summaries_only_compaction_emits_typed_unavailable_history_artifact() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(terminal_summaries_only_resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should admit");

    let report = runtime.compact_resource_lifecycle_history(1);

    assert_eq!(report.reclaimed_in_flight_count(), 1);
    assert_eq!(report.retained_history_write_count(), 0);
    assert_eq!(report.retained_history_pruned_count(), 0);
    assert_eq!(report.retained_history_unavailable_count(), 1);
    assert_eq!(report.compacted_terminal_summary_count(), 1);
    assert_eq!(report.retained_history_width(), 0);
    let availability = runtime
        .retained_history_availability_for_request(admitted.handle().request_id())
        .expect("terminal-summary compaction should retain typed availability evidence");
    assert_eq!(
        availability.class(),
        ResourceRetainedHistoryAvailabilityClass::TerminalSummaryOnly
    );
    assert_eq!(
        availability.retention_decision_class(),
        ResourceRetentionDecisionClass::TerminalSummariesOnly
    );
    let descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should remain available for the node");
    assert_eq!(
        availability.retention_descriptor_id(),
        descriptor.retention_decision_plan().descriptor_id()
    );
    let denied = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted.handle(),
            admitted.attempt(),
            64,
        ))
        .denied_completion()
        .expect("late completion should still deny from compacted terminal summary");
    assert_eq!(denied.class(), CompletionDenialClass::Cancelled);
}

#[test]
fn resource_targeted_retention_compaction_only_reclaims_matching_lifecycle_policy() {
    let mut graph = SignalGraph::new();
    let cancelled_node = graph.node().build();
    let mismatched_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(compact_cancelled_resource_declaration(cancelled_node))
        .expect("cancelled compaction declaration should lower");
    runtime
        .declare_resource_node(compact_superseded_resource_declaration(mismatched_node))
        .expect("mismatched compaction declaration should lower");
    let cancelled = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            cancelled_node,
        )))
        .expect("cancelled request should admit")
        .admitted_request();
    let mismatched = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            mismatched_node,
        )))
        .expect("mismatched request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(
            cancelled.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancelled request should terminate");
    runtime
        .cancel_resource_request(
            mismatched.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("mismatched request should also terminate");

    let report = runtime.compact_resource_lifecycle_history(2);

    assert_eq!(report.selected_terminal_count(), 1);
    assert_eq!(report.reclaimed_in_flight_count(), 1);
    assert_eq!(report.compacted_cancelled_count(), 1);
    assert_eq!(report.compacted_superseded_count(), 0);
    assert!(
        runtime
            .retained_history_availability_for_request(cancelled.handle().request_id())
            .is_some(),
        "matching cancelled policy should produce availability artifact"
    );
    assert!(
        runtime
            .in_flight_resource_request(mismatched.handle())
            .is_some(),
        "non-matching supersession policy should not compact cancelled lifecycle"
    );
}

#[test]
fn resource_timed_out_retention_compaction_only_reclaims_matching_timeout_policy() {
    let mut graph = SignalGraph::new();
    let timed_out_node = graph.node().build();
    let cancelled_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(compact_timed_out_resource_declaration(timed_out_node))
        .expect("timed-out compaction declaration should lower");
    runtime
        .declare_resource_node(compact_timed_out_resource_declaration(cancelled_node))
        .expect("cancelled declaration should still lower");

    let timed_out = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            timed_out_node,
        )))
        .expect("timed-out request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(timed_out.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach for timed-out policy");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should advance to timeout boundary");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(timed_out.handle(), ready_timeout)
        .expect("timed-out request should transition to timed-out lifecycle");

    let cancelled = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            cancelled_node,
        )))
        .expect("cancelled request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(
            cancelled.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancelled request should terminate");

    let report = runtime.compact_resource_lifecycle_history(2);

    assert_eq!(report.selected_terminal_count(), 1);
    assert_eq!(report.reclaimed_in_flight_count(), 1);
    assert_eq!(report.compacted_timed_out_count(), 1);
    assert_eq!(report.compacted_cancelled_count(), 0);
    let availability = runtime
        .retained_history_availability_for_request(timed_out.handle().request_id())
        .expect("matching timed-out policy should produce availability artifact");
    assert_eq!(
        availability.class(),
        ResourceRetainedHistoryAvailabilityClass::CompactTimedOut
    );
    assert_eq!(
        availability.retention_decision_class(),
        ResourceRetentionDecisionClass::CompactTimedOut
    );
    let denied = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            timed_out_node,
            timed_out.handle(),
            timed_out.attempt(),
            64,
        ))
        .denied_completion()
        .expect("late completion after timed-out compaction should still deny as timed out");
    assert_eq!(denied.class(), CompletionDenialClass::TimedOut);
    assert!(
        runtime
            .in_flight_resource_request(cancelled.handle())
            .is_some(),
        "timed-out-only compaction should not reclaim cancelled lifecycle"
    );
}

#[test]
fn resource_lifecycle_retention_compaction_prunes_retained_history_by_explicit_limit() {
    let mut graph = SignalGraph::new();
    let first_node = graph.node().build();
    let second_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(first_node))
        .expect("first resource declaration should lower");
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(second_node))
        .expect("second resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            first_node,
        )))
        .expect("first request should admit")
        .admitted_request();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            second_node,
        )))
        .expect("second request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(first.handle(), ResourceCancellationReason::HostRequested)
        .expect("first cancellation should admit");
    runtime
        .cancel_resource_request(second.handle(), ResourceCancellationReason::HostRequested)
        .expect("second cancellation should admit");

    let report = runtime.compact_resource_lifecycle_history_with_retained_limit(2, 1);

    assert_eq!(report.selected_terminal_count(), 2);
    assert_eq!(report.reclaimed_in_flight_count(), 2);
    assert_eq!(report.retained_history_write_count(), 2);
    assert_eq!(report.retained_history_pruned_count(), 1);
    assert_eq!(report.retained_history_unavailable_count(), 1);
    assert_eq!(report.retained_history_width(), 1);
    assert_eq!(report.hot_in_flight_width(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_lifecycle_history_pruned_count,
        1
    );
    let denied = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            first_node,
            first.handle(),
            first.attempt(),
            64,
        ))
        .denied_completion()
        .expect("pruned retained history completion should deny explicitly");
    assert_eq!(
        denied.class(),
        CompletionDenialClass::RetainedHistoryUnavailable
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_history_unavailable_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_unknown_request_completion_denial_count,
        0
    );
}

#[test]
fn resource_branch_restore_accounts_for_retained_lifecycle_history_width() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should admit");
    let compaction = runtime.compact_resource_lifecycle_history(1);
    assert_eq!(compaction.retained_history_width(), 1);
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("post-snapshot request should mutate resource state");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate retained lifecycle history");
    let restore = runtime
        .latest_resource_branch_restore_report()
        .expect("restore should publish resource branch evidence");

    assert_eq!(restore.restored_in_flight_width(), 0);
    assert_eq!(restore.retained_summary_width(), 2);
    assert_eq!(restore.performance().input_width(), 2);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_branch_restore_retained_summary_width,
        2
    );
}

#[test]
fn resource_retention_budget_prunes_denied_completion_history_with_typed_availability() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();

    let first_denied = runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            ResourceRequestId::new(900),
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest.clone(),
            32,
        ))
        .denied_completion()
        .expect("unknown request should retain denied completion evidence");
    let second_denied = runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            ResourceRequestId::new(901),
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            48,
        ))
        .denied_completion()
        .expect("second unknown request should retain denied completion evidence");

    let report = runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_denied_completion_limit(1),
    );

    assert_eq!(report.selected_terminal_count(), 0);
    assert_eq!(report.reclaimed_in_flight_count(), 0);
    assert_eq!(report.retained_denied_completion_pruned_count(), 1);
    assert_eq!(report.retained_denied_completion_width(), 1);
    assert_eq!(
        runtime.resource_runtime_summary().denied_completion_count(),
        1
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .retained_denied_completion_count(),
        1
    );
    let availability = runtime
        .retained_denied_completion_availability(first_denied.denial_id())
        .expect("oldest denied completion should become typed unavailable history");
    assert_eq!(
        availability.class(),
        ResourceRetainedDeniedCompletionAvailabilityClass::PrunedByRetainedDeniedCompletionLimit
    );
    assert_eq!(availability.denial_id(), first_denied.denial_id());
    assert_eq!(availability.request_id(), first_denied.request_id());
    assert_eq!(availability.node(), first_denied.node());
    assert_eq!(availability.denial_class(), first_denied.class());
    assert!(
        runtime
            .retained_denied_completion_availability(second_denied.denial_id())
            .is_none(),
        "newest denied completion should remain retained rather than pruned"
    );
    let replay = runtime.reconstruct_resource_replay_summary();
    assert_eq!(replay.denied_completion_width(), 1);
    assert_eq!(replay.denied_completion_unavailable_count(), 1);
    assert_eq!(replay.retry_lineage_unavailable_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_denied_completion_count,
        1
    );
}

#[test]
fn resource_retention_budget_prunes_retry_lineage_with_typed_availability() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(node, 3, 7))
        .expect("retry declaration should lower");

    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("initial request should admit")
        .admitted_request();
    let first_timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("initial timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach initial timeout");
    let first_ready_timeout = runtime
        .promote_temporal_wake_ready(first_timeout_wake)
        .expect("initial timeout wake should become ready");
    runtime
        .admit_resource_timeout(admitted.handle(), first_ready_timeout)
        .expect("initial timeout should admit");

    let first_schedule = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("first retry schedule should return report");
    let first_scheduled = first_schedule
        .scheduled_retry()
        .expect("first retry should schedule");
    let first_retry_ordinal = first_scheduled.retry_ordinal();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(
                runtime
                    .clock_basis()
                    .current_tick()
                    .get()
                    .saturating_add(first_scheduled.scheduled_delay().get()),
            ),
        ))
        .expect("clock should reach first retry backoff");
    let first_ready_retry = runtime
        .promote_temporal_wake_ready(first_scheduled.backoff_wake_id())
        .expect("first retry wake should become ready");
    let first_retry_report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), first_ready_retry)
        .expect("first scheduled retry should admit");
    let first_retry = first_retry_report
        .admitted_retry()
        .expect("first retry should produce admitted retry artifact");
    let first_retry_request = first_retry.admitted_request();

    let second_timeout_wake = runtime
        .in_flight_resource_request(first_retry_request.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("retried request should attach timeout wake");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should reach second timeout");
    let second_ready_timeout = runtime
        .promote_temporal_wake_ready(second_timeout_wake)
        .expect("second timeout wake should become ready");
    runtime
        .admit_resource_timeout(first_retry_request.handle(), second_ready_timeout)
        .expect("second timeout should admit");

    let second_schedule = runtime
        .schedule_resource_retry(first_retry_request.handle(), ResourceRetryReason::TimedOut)
        .expect("second retry schedule should return report");
    let second_scheduled = second_schedule
        .scheduled_retry()
        .expect("second retry should schedule");
    let second_retry_ordinal = second_scheduled.retry_ordinal();
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(
                runtime
                    .clock_basis()
                    .current_tick()
                    .get()
                    .saturating_add(second_scheduled.scheduled_delay().get()),
            ),
        ))
        .expect("clock should reach second retry backoff");
    let second_ready_retry = runtime
        .promote_temporal_wake_ready(second_scheduled.backoff_wake_id())
        .expect("second retry wake should become ready");
    runtime
        .admit_scheduled_resource_retry(first_retry_request.handle(), second_ready_retry)
        .expect("second scheduled retry should admit");

    let report = runtime.compact_resource_lifecycle_history_with_budget(
        0,
        ResourceRetentionCompactionBudget::unbounded().with_retained_retry_lineage_limit(1),
    );

    assert_eq!(report.selected_terminal_count(), 0);
    assert_eq!(report.retained_retry_lineage_pruned_count(), 1);
    assert_eq!(report.retained_retry_lineage_width(), 1);
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .retained_retry_lineage_count(),
        1
    );
    let availability = runtime
        .retained_retry_lineage_availability(first_retry_ordinal)
        .expect("oldest retry lineage should become typed unavailable history");
    assert_eq!(
        availability.class(),
        ResourceRetainedRetryLineageAvailabilityClass::PrunedByRetainedRetryLineageLimit
    );
    assert_eq!(availability.retry_ordinal(), first_retry_ordinal);
    assert_eq!(
        availability.previous(),
        admitted.handle(),
        "pruned lineage should still identify the source request handle"
    );
    assert_eq!(availability.reason(), ResourceRetryReason::TimedOut);
    assert_eq!(availability.next_attempt(), ResourceAttemptId::new(1));
    assert_eq!(availability.scheduled_delay().get(), 7);
    let retained = runtime
        .retained_retry_lineage(second_retry_ordinal)
        .expect("newest retry lineage should remain retained");
    assert_eq!(retained.retry_ordinal(), second_retry_ordinal);
    assert_eq!(retained.reason(), ResourceRetryReason::TimedOut);
    let replay = runtime.reconstruct_resource_replay_summary();
    assert_eq!(replay.retained_retry_lineage_width(), 1);
    assert_eq!(replay.retry_lineage_unavailable_count(), 1);
    assert_eq!(replay.denied_completion_unavailable_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_retry_lineage_count,
        1
    );
}

#[test]
fn resource_lifecycle_retention_compaction_preserves_late_completion_denial_class() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should admit");
    let report = runtime.compact_resource_lifecycle_history(1);
    assert_eq!(report.reclaimed_in_flight_count(), 1);

    let late = runtime.admit_resource_completion(raw_completion(
        &runtime,
        node,
        admitted.handle(),
        admitted.attempt(),
        64,
    ));

    let denied = late
        .denied_completion()
        .expect("late compacted cancelled completion should deny");
    assert_eq!(denied.class(), CompletionDenialClass::Cancelled);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancelled_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_unknown_request_completion_denial_count,
        0
    );
}

#[test]
fn resource_completion_duplicate_after_commit_is_retired_without_second_commit() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let raw = raw_completion(
        &runtime,
        node,
        admitted_request.handle(),
        admitted_request.attempt(),
        64,
    );
    let admitted_completion = runtime
        .admit_resource_completion(raw.clone())
        .admitted_completion()
        .expect("first matching completion should admit");
    let staged = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("active admitted completion should stage")
        .staged_effect();

    runtime
        .commit_staged_resource_completion(staged)
        .expect("first completion should commit");
    let duplicate = runtime.admit_resource_completion(raw);

    assert_eq!(
        duplicate
            .denied_completion()
            .expect("duplicate completion should be denied")
            .class(),
        CompletionDenialClass::Retired
    );
    assert!(duplicate.admitted_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_commit_count,
        1
    );
}

#[test]
fn resource_completion_staging_rejects_admitted_proof_after_cancellation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit before cancellation");

    runtime
        .cancel_resource_request(
            admitted_request.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancellation should apply");
    let err = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect_err("admitted completion proof should not stage after lifecycle changes");

    assert!(err.to_string().contains("non-active request"));
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_staging_count,
        0
    );
}

#[test]
fn resource_completion_admission_denies_unknown_request_without_lifecycle_mutation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();

    let report = runtime.admit_resource_completion(RawCompletionEnvelope::new(
        ResourceRequestId::new(999),
        ResourceGeneration::new(1),
        ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
        ResourceAttemptId::ZERO,
        digest,
        32,
    ));

    let denied = report
        .denied_completion()
        .expect("unknown request should produce a retained denial");
    assert_eq!(denied.class(), CompletionDenialClass::UnknownRequest);
    assert!(report.admitted_completion().is_none());
    assert_eq!(
        runtime.resource_runtime_summary().denied_completion_count(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_unknown_request_completion_denial_count,
        1
    );
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        0
    );
}

#[test]
fn resource_completion_batch_admission_canonicalizes_out_of_order_inputs() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(first))
        .expect("first resource declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(second))
        .expect("second resource declaration should lower");
    let first_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(first)))
        .expect("first request should admit")
        .admitted_request();
    let second_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            second,
        )))
        .expect("second request should admit")
        .admitted_request();
    let first_raw = raw_completion(
        &runtime,
        first,
        first_request.handle(),
        first_request.attempt(),
        64,
    );
    let second_raw = raw_completion(
        &runtime,
        second,
        second_request.handle(),
        second_request.attempt(),
        96,
    );
    let boundary_envelopes_before = runtime
        .telemetry()
        .resource
        .resource_boundary_performance_envelope_count;

    let report = runtime.admit_resource_completion_batch([second_raw, first_raw]);

    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::CompletionBatchAdmission
    );
    assert_eq!(report.input_width(), 2);
    assert_eq!(report.deduplicated_width(), 2);
    assert_eq!(report.duplicate_width(), 0);
    assert_eq!(report.admitted_completions().len(), 2);
    assert!(report.denied_completions().is_empty());
    assert_eq!(
        report.admitted_completions()[0].handle(),
        first_request.handle()
    );
    assert_eq!(
        report.admitted_completions()[1].handle(),
        second_request.handle()
    );
    assert_eq!(report.performance().admitted_count(), 2);
    assert_eq!(report.performance().denied_count(), 0);
    assert_eq!(
        report.performance().density_strategy(),
        ResourceDensityStrategy::BurstySortedDeduplicated
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_admission_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_batch_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        boundary_envelopes_before + 1
    );
}

#[test]
fn resource_completion_batch_admission_reports_dense_strategy_without_truth_drift() {
    let mut graph = SignalGraph::new();
    let nodes = [
        graph.node().build(),
        graph.node().build(),
        graph.node().build(),
        graph.node().build(),
    ];
    let mut runtime = TestRuntime::build(graph);
    for node in nodes {
        runtime
            .declare_resource_node(resource_declaration(node))
            .expect("resource declaration should lower");
    }
    let admitted = nodes.map(|node| {
        runtime
            .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
            .expect("request should admit")
            .admitted_request()
    });
    let mut completions = admitted
        .iter()
        .zip(nodes)
        .map(|(request, node)| {
            raw_completion(&runtime, node, request.handle(), request.attempt(), 64)
        })
        .collect::<Vec<_>>();
    completions.reverse();
    let density_before = runtime
        .telemetry()
        .resource
        .resource_density_strategy_selection_count;
    let dense_before = runtime
        .telemetry()
        .resource
        .resource_dense_density_strategy_count;

    let report = runtime.admit_resource_completion_batch(completions);

    assert_eq!(report.input_width(), 4);
    assert_eq!(report.deduplicated_width(), 4);
    assert_eq!(report.admitted_completions().len(), 4);
    assert!(report.denied_completions().is_empty());
    assert_eq!(
        report.performance().density_strategy(),
        ResourceDensityStrategy::DenseSortedDeduplicated
    );
    assert_eq!(
        report
            .admitted_completions()
            .iter()
            .map(|completion| completion.handle())
            .collect::<Vec<_>>(),
        admitted
            .iter()
            .map(|request| request.handle())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_density_strategy_selection_count,
        density_before + 1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_dense_density_strategy_count,
        dense_before + 1
    );
}

#[test]
fn resource_completion_batch_admission_denies_in_batch_duplicate_without_second_admitted_proof() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let raw = raw_completion(
        &runtime,
        node,
        admitted_request.handle(),
        admitted_request.attempt(),
        64,
    );
    let boundary_envelopes_before = runtime
        .telemetry()
        .resource
        .resource_boundary_performance_envelope_count;

    let report = runtime.admit_resource_completion_batch([raw.clone(), raw]);

    assert_eq!(report.input_width(), 2);
    assert_eq!(report.deduplicated_width(), 1);
    assert_eq!(report.duplicate_width(), 1);
    assert_eq!(report.admitted_completions().len(), 1);
    assert_eq!(
        report.admitted_completions()[0].handle(),
        admitted_request.handle()
    );
    assert_eq!(report.denied_completions().len(), 1);
    assert_eq!(
        report.denied_completions()[0].class(),
        CompletionDenialClass::Duplicate
    );
    assert_eq!(report.performance().input_width(), 2);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        report.performance().density_strategy(),
        ResourceDensityStrategy::BurstySortedDeduplicated
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_duplicate_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_validation_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_admission_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        boundary_envelopes_before + 1
    );
}

#[test]
fn resource_completion_batch_admission_denies_contradictory_duplicate_identity() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let accepted = raw_completion(
        &runtime,
        node,
        admitted_request.handle(),
        admitted_request.attempt(),
        64,
    );
    let contradictory = raw_completion(
        &runtime,
        node,
        admitted_request.handle(),
        admitted_request.attempt(),
        96,
    );

    let report = runtime.admit_resource_completion_batch([contradictory, accepted]);

    assert_eq!(report.input_width(), 2);
    assert_eq!(report.deduplicated_width(), 1);
    assert_eq!(report.duplicate_width(), 1);
    assert_eq!(report.admitted_completions().len(), 1);
    assert_eq!(
        report.admitted_completions()[0].handle(),
        admitted_request.handle()
    );
    assert_eq!(report.denied_completions().len(), 1);
    assert_eq!(
        report.denied_completions()[0].class(),
        CompletionDenialClass::Contradictory
    );
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_contradictory_completion_denial_count,
        1
    );
}

#[test]
fn resource_completion_rollback_of_staged_denied_preserves_retained_denial_without_mutation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();

    let denied = runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            ResourceRequestId::new(999),
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown request should produce a retained denial");
    let denial_id = denied.denial_id();
    let request_id = denied.request_id();

    let staging = runtime
        .stage_denied_resource_completion(denied)
        .expect("retained denied completion should stage");
    assert_eq!(
        staging.performance().boundary(),
        ResourceBoundaryKind::CompletionDenialStaging
    );
    assert_eq!(staging.performance().admitted_count(), 0);
    assert_eq!(staging.performance().denied_count(), 1);

    let rollback =
        runtime.rollback_staged_denied_resource_completion(staging.staged_denial_effect());
    assert_eq!(
        rollback.performance().boundary(),
        ResourceBoundaryKind::CompletionRollback
    );
    assert_eq!(rollback.performance().admitted_count(), 0);
    assert_eq!(rollback.performance().denied_count(), 1);
    assert_eq!(
        rollback.rolled_back_completion().subject(),
        ResourceCompletionRollbackSubject::Denied {
            denial_id,
            class: CompletionDenialClass::UnknownRequest,
            request_id,
        }
    );
    assert_eq!(
        runtime.resource_runtime_summary().denied_completion_count(),
        1
    );
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_denial_staging_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_rollback_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_commit_count,
        0
    );
}

#[test]
fn resource_completion_admission_denies_pre_restore_epoch_as_stale() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit before snapshot")
        .admitted_request();
    let stale = raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should mutate before restore");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should rekey in-flight handle epochs");

    let report = runtime.admit_resource_completion(stale);

    let denied = report
        .denied_completion()
        .expect("pre-restore completion should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Stale);
    assert!(report.admitted_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_stale_completion_denial_count,
        1
    );
}

#[test]
fn resource_completion_admission_denies_superseded_request() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request();
    let stale_first = raw_completion(&runtime, node, first.handle(), first.attempt(), 64);
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first");

    let report = runtime.admit_resource_completion(stale_first);

    let denied = report
        .denied_completion()
        .expect("superseded completion should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Superseded);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_superseded_completion_denial_count,
        1
    );
}

#[test]
fn resource_completion_admission_denies_late_success_after_cancellation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let late = raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let cancellation = runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should retire timeout side effects cleanly");
    assert!(cancellation.cancelled_request().is_some());

    let report = runtime.admit_resource_completion(late);

    let denied = report
        .denied_completion()
        .expect("late completion after cancellation should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Cancelled);
    assert!(report.admitted_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancelled_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(admitted.handle())
            .expect("cancelled request remains retained")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
}

#[test]
fn resource_completion_admission_denies_late_success_after_rejection() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let late = raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let rejection = runtime
        .reject_resource_request(admitted.handle(), ResourceRejectionReason::SemanticFailure)
        .expect("rejection should retire timeout side effects cleanly");
    assert!(rejection.rejected_request().is_some());

    let report = runtime.admit_resource_completion(late);

    let denied = report
        .denied_completion()
        .expect("late completion after rejection should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Rejected);
    assert!(report.admitted_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_rejected_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(admitted.handle())
            .expect("rejected request remains retained")
            .status(),
        ResourceInFlightStatus::Rejected
    );
}

#[test]
fn resource_completion_identity_staleness_dominates_cancelled_lifecycle_after_restore() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let pre_restore_completion =
        raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let cancellation = runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should retire timeout side effects cleanly");
    assert!(cancellation.cancelled_request().is_some());
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should mutate state before restore");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should rekey retained cancelled in-flight handles");

    let report = runtime.admit_resource_completion(pre_restore_completion);

    let denied = report
        .denied_completion()
        .expect("pre-restore completion should be stale even when retained request is cancelled");
    assert_eq!(denied.class(), CompletionDenialClass::Stale);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_stale_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancelled_completion_denial_count,
        0
    );
}

#[test]
fn resource_completion_admission_denies_late_success_after_timeout() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let late = raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("authoritative clock should advance");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should promote");
    assert!(runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should consume temporal wake cleanly")
        .timed_out_request()
        .is_some());

    let report = runtime.admit_resource_completion(late);

    let denied = report
        .denied_completion()
        .expect("late completion after timeout should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::TimedOut);
    assert!(report.admitted_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_timed_out_completion_denial_count,
        1
    );
}

#[test]
fn resource_completion_admission_denies_payload_contract_mismatch() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted.handle();

    let report = runtime.admit_resource_completion(RawCompletionEnvelope::new(
        handle.request_id(),
        handle.generation(),
        handle.branch_epoch(),
        admitted.attempt(),
        ResourcePayloadContractDigest::new("payload-contract:999:1024"),
        64,
    ));

    let denied = report
        .denied_completion()
        .expect("wrong payload contract should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Malformed);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_malformed_completion_denial_count,
        1
    );
}

#[test]
fn resource_completion_admission_denies_payload_above_declared_limit() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();

    let report = runtime.admit_resource_completion(raw_completion(
        &runtime,
        node,
        admitted.handle(),
        admitted.attempt(),
        2048,
    ));

    let denied = report
        .denied_completion()
        .expect("oversized payload should be denied before apply");
    assert_eq!(denied.class(), CompletionDenialClass::Partial);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_partial_completion_denial_count,
        1
    );
}
