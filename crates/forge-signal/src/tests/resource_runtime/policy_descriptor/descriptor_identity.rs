use super::*;

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
