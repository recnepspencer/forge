use crate::facade::*;
use crate::tests::async_node_support::{
    admit_and_commit_async_node_completion, async_node_capability_declaration,
    raw_async_node_completion, AsyncNodeTestRuntime as TestRuntime,
};
use crate::tests::support::{evaluate, version_ab};

#[test]
fn async_node_interior_gate_pending_visibility_reflects_output_continuity_policy() {
    let mut graph = SignalGraph::new();
    let preserve_gate = graph.node().build();
    let hide_gate = graph.node().build();
    let mut eval = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(7, 0)).with_output_identity("gate-v1"))
    };
    evaluate(&mut graph, preserve_gate, &mut eval).expect("preserve gate should evaluate");
    evaluate(&mut graph, hide_gate, &mut eval).expect("hide gate should evaluate");

    let mut runtime = TestRuntime::build(graph);
    let preserve = runtime
        .attach_async_capability(
            async_node_capability_declaration(preserve_gate)
                .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleAndOutput),
        )
        .expect("preserve gate capability should attach");
    let hide = runtime
        .attach_async_capability(
            async_node_capability_declaration(hide_gate)
                .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleAndOutput)
                .with_output_continuity_policy(
                    ResourceOutputContinuityPolicyDeclaration::HideWhilePending,
                ),
        )
        .expect("hide gate capability should attach");

    let preserve_first = runtime
        .admit_async_node_request(preserve.request_intent())
        .expect("preserve gate first request should admit")
        .resource_admission()
        .expect("preserve gate first request should expose resource admission")
        .admitted_request();
    admit_and_commit_async_node_completion(
        &mut runtime,
        preserve_first.handle(),
        preserve_first.attempt(),
        preserve.payload_contract_digest().clone(),
        64,
    );
    let hide_first = runtime
        .admit_async_node_request(hide.request_intent())
        .expect("hide gate first request should admit")
        .resource_admission()
        .expect("hide gate first request should expose resource admission")
        .admitted_request();
    admit_and_commit_async_node_completion(
        &mut runtime,
        hide_first.handle(),
        hide_first.attempt(),
        hide.payload_contract_digest().clone(),
        64,
    );

    runtime
        .admit_async_node_request(preserve.request_intent())
        .expect("preserve gate second request should admit");
    runtime
        .admit_async_node_request(hide.request_intent())
        .expect("hide gate second request should admit");

    let preserve_state = runtime
        .async_node_gate_state_report(preserve_gate)
        .expect("preserve gate state should materialize");
    let hide_state = runtime
        .async_node_gate_state_report(hide_gate)
        .expect("hide gate state should materialize");

    assert_eq!(
        preserve_state.lifecycle_class(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        hide_state.lifecycle_class(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        preserve_state.output_continuity(),
        Some(ResourceOutputContinuity::PriorOutputPreserved)
    );
    assert_eq!(
        hide_state.output_continuity(),
        Some(ResourceOutputContinuity::OutputUnavailableByPolicy)
    );
    assert_eq!(
        preserve_state
            .committed_output_identity()
            .map(OutputIdentity::as_str),
        Some("gate-v1")
    );
    assert_eq!(
        hide_state
            .committed_output_identity()
            .map(OutputIdentity::as_str),
        Some("gate-v1")
    );
    assert_ne!(
        preserve_state.gate_digest(),
        hide_state.gate_digest(),
        "continuity policy drift must perturb gate truth even when lifecycle class stays pending"
    );
    assert!(
        runtime
            .telemetry()
            .resource
            .resource_previous_output_preserved_count
            >= 1,
        "preserve policy should record at least one preserved-output continuity decision"
    );
    assert!(
        runtime
            .telemetry()
            .resource
            .resource_previous_output_hidden_count
            >= 1,
        "hide-while-pending policy should record at least one hidden-output continuity decision"
    );
}

#[test]
fn async_node_interior_gate_rejection_visibility_changes_without_forging_lifecycle_truth() {
    let mut graph = SignalGraph::new();
    let preserve_gate = graph.node().build();
    let hide_gate = graph.node().build();
    let mut eval = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(9, 0)).with_output_identity("gate-v2"))
    };
    evaluate(&mut graph, preserve_gate, &mut eval).expect("preserve gate should evaluate");
    evaluate(&mut graph, hide_gate, &mut eval).expect("hide gate should evaluate");

    let mut runtime = TestRuntime::build(graph);
    let preserve = runtime
        .attach_async_capability(
            async_node_capability_declaration(preserve_gate)
                .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleAndOutput),
        )
        .expect("preserve gate capability should attach");
    let hide = runtime
        .attach_async_capability(
            async_node_capability_declaration(hide_gate)
                .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleAndOutput)
                .with_output_continuity_policy(
                    ResourceOutputContinuityPolicyDeclaration::HideAfterRejection,
                ),
        )
        .expect("hide gate capability should attach");

    let preserve_first = runtime
        .admit_async_node_request(preserve.request_intent())
        .expect("preserve gate first request should admit")
        .resource_admission()
        .expect("preserve gate first request should expose resource admission")
        .admitted_request();
    admit_and_commit_async_node_completion(
        &mut runtime,
        preserve_first.handle(),
        preserve_first.attempt(),
        preserve.payload_contract_digest().clone(),
        64,
    );
    let hide_first = runtime
        .admit_async_node_request(hide.request_intent())
        .expect("hide gate first request should admit")
        .resource_admission()
        .expect("hide gate first request should expose resource admission")
        .admitted_request();
    admit_and_commit_async_node_completion(
        &mut runtime,
        hide_first.handle(),
        hide_first.attempt(),
        hide.payload_contract_digest().clone(),
        64,
    );

    let preserve_second = runtime
        .admit_async_node_request(preserve.request_intent())
        .expect("preserve gate second request should admit")
        .resource_admission()
        .expect("preserve gate second request should expose resource admission")
        .admitted_request();
    let hide_second = runtime
        .admit_async_node_request(hide.request_intent())
        .expect("hide gate second request should admit")
        .resource_admission()
        .expect("hide gate second request should expose resource admission")
        .admitted_request();

    runtime
        .reject_resource_request(
            preserve_second.handle(),
            ResourceRejectionReason::SemanticFailure,
        )
        .expect("preserve gate rejection should succeed");
    runtime
        .reject_resource_request(
            hide_second.handle(),
            ResourceRejectionReason::SemanticFailure,
        )
        .expect("hide gate rejection should succeed");

    let preserve_state = runtime
        .async_node_gate_state_report(preserve_gate)
        .expect("preserve gate rejection state should materialize");
    let hide_state = runtime
        .async_node_gate_state_report(hide_gate)
        .expect("hide gate rejection state should materialize");
    let preserve_late_completion = runtime.admit_resource_completion(raw_async_node_completion(
        preserve_second.handle(),
        preserve_second.attempt(),
        preserve.payload_contract_digest().clone(),
        32,
    ));
    let hide_late_completion = runtime.admit_resource_completion(raw_async_node_completion(
        hide_second.handle(),
        hide_second.attempt(),
        hide.payload_contract_digest().clone(),
        32,
    ));

    assert_eq!(
        preserve_state.lifecycle_class(),
        ResourceLifecycleClass::Rejected
    );
    assert_eq!(
        hide_state.lifecycle_class(),
        ResourceLifecycleClass::Rejected
    );
    assert_eq!(
        preserve_state.output_continuity(),
        Some(ResourceOutputContinuity::PriorOutputPreserved)
    );
    assert_eq!(
        hide_state.output_continuity(),
        Some(ResourceOutputContinuity::OutputUnavailableByPolicy)
    );
    assert_eq!(
        preserve_state
            .committed_output_identity()
            .map(OutputIdentity::as_str),
        Some("gate-v2")
    );
    assert_eq!(
        hide_state
            .committed_output_identity()
            .map(OutputIdentity::as_str),
        Some("gate-v2")
    );
    assert_ne!(
        preserve_state.gate_digest(),
        hide_state.gate_digest(),
        "rejection visibility policy drift must perturb gate truth without forging a different lifecycle class"
    );
    assert_eq!(
        preserve_late_completion
            .denied_completion()
            .expect("late preserve completion should be denied")
            .class(),
        CompletionDenialClass::Rejected
    );
    assert_eq!(
        hide_late_completion
            .denied_completion()
            .expect("late hidden completion should be denied")
            .class(),
        CompletionDenialClass::Rejected
    );
    assert!(
        runtime
            .telemetry()
            .resource
            .resource_previous_output_preserved_count
            >= 1,
        "preserve policy should record at least one preserved-output continuity decision"
    );
    assert!(
        runtime
            .telemetry()
            .resource
            .resource_previous_output_hidden_count
            >= 1,
        "hide-after-rejection policy should record at least one hidden-output continuity decision"
    );
    assert!(
        runtime
            .telemetry()
            .resource
            .resource_output_continuity_decision_count
            >= 4,
        "request and rejection lanes together should exercise multiple continuity decisions"
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_rejected_completion_denial_count,
        2
    );
}
