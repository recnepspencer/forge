use crate::facade::*;
use crate::tests::async_node_support::{
    async_node_capability_declaration, async_node_capability_with_dependents,
    raw_async_node_completion, AsyncNodeTestRuntime as TestRuntime,
};

#[test]
fn async_node_hierarchy_late_descendant_completion_switches_from_cancelled_to_stale_across_restore()
{
    let mut graph = SignalGraph::new();
    let parent = graph.node().build();
    let child = graph.node().build();
    let grandchild = graph.node().build();
    graph
        .append_dependency(child, parent, Aspect::new(0))
        .expect("child should depend on parent");
    graph
        .append_dependency(grandchild, child, Aspect::new(0))
        .expect("grandchild should depend on child");

    let mut runtime = TestRuntime::build(graph);
    let parent_handle = runtime
        .attach_async_capability(async_node_capability_with_dependents(parent, [child]))
        .expect("parent capability should attach");
    let child_handle = runtime
        .attach_async_capability(async_node_capability_with_dependents(child, [grandchild]))
        .expect("child capability should attach");
    let grandchild_handle = runtime
        .attach_async_capability(async_node_capability_declaration(grandchild))
        .expect("grandchild capability should attach");

    let parent_request = runtime
        .admit_async_node_request(parent_handle.request_intent())
        .expect("parent request should admit")
        .resource_admission()
        .expect("parent request should expose resource admission")
        .admitted_request();
    let child_request = runtime
        .admit_async_node_request(child_handle.request_intent())
        .expect("child request should admit")
        .resource_admission()
        .expect("child request should expose resource admission")
        .admitted_request();
    let grandchild_request = runtime
        .admit_async_node_request(grandchild_handle.request_intent())
        .expect("grandchild request should admit")
        .resource_admission()
        .expect("grandchild request should expose resource admission")
        .admitted_request();
    let late_grandchild_completion = raw_async_node_completion(
        grandchild_request.handle(),
        grandchild_request.attempt(),
        grandchild_handle.payload_contract_digest().clone(),
        48,
    );
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");

    let cancellation = runtime
        .cancel_async_node_request(
            parent_request.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("hierarchy cancellation should succeed");
    let propagation = cancellation
        .cancellation()
        .dependent_propagation()
        .expect("hierarchy cancellation should propagate");
    let cancelled_handles = propagation
        .cancelled_dependents()
        .iter()
        .map(CancelledResourceRequest::handle)
        .collect::<Vec<_>>();
    let cancelled_completion_report =
        runtime.admit_resource_completion(late_grandchild_completion.clone());
    let cancelled_hierarchy = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("cancelled hierarchy summary should materialize");

    assert_eq!(propagation.parent(), parent_request.handle());
    assert_eq!(
        cancelled_handles,
        vec![child_request.handle(), grandchild_request.handle()]
    );
    assert_eq!(
        cancelled_completion_report
            .denied_completion()
            .expect("late grandchild completion should deny after hierarchy cancellation")
            .class(),
        CompletionDenialClass::Cancelled
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancelled_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(grandchild_request.handle())
            .expect("grandchild request should remain retained after cancellation")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
    assert_eq!(
        cancelled_hierarchy.active_request_handles().len(),
        0,
        "cancelled hierarchy replay should expose active lineage only, not retained cancelled records"
    );

    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate pre-cancellation hierarchy truth");
    let stale_completion_report = runtime.admit_resource_completion(late_grandchild_completion);
    let restored_hierarchy = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("restored hierarchy summary should materialize");

    assert_eq!(
        stale_completion_report
            .denied_completion()
            .expect("pre-restore descendant completion should deny as stale after restore rekeys epochs")
            .class(),
        CompletionDenialClass::Stale
    );
    assert!(
        runtime
            .in_flight_resource_request(grandchild_request.handle())
            .is_none(),
        "restored hierarchy should not still retain drifted descendant handle identity"
    );
    assert_eq!(
        restored_hierarchy
            .active_request_handles()
            .iter()
            .map(|handle| (handle.request_id(), handle.generation()))
            .collect::<Vec<_>>(),
        vec![
            (
                parent_request.handle().request_id(),
                parent_request.handle().generation()
            ),
            (
                child_request.handle().request_id(),
                child_request.handle().generation()
            ),
            (
                grandchild_request.handle().request_id(),
                grandchild_request.handle().generation()
            ),
        ],
        "restore should preserve semantic lineage even though concrete handle epochs rekey"
    );
    assert_ne!(
        restored_hierarchy.replay_digest(),
        cancelled_hierarchy.replay_digest(),
        "restored hierarchy must not keep the drifted cancelled replay story"
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_stale_completion_denial_count,
        1
    );
}
