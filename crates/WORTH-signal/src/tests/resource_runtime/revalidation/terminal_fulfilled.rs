use super::*;

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
