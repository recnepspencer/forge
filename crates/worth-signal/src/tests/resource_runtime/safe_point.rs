use super::*;

#[test]
fn safe_point_observation_preserves_exact_signal_lifecycle_and_pressure() {
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
    let queue = runtime
        .bind_resource_managed_queue(admitted, 2)
        .expect("exact admitted request should bind one managed queue");
    let duplicate = runtime
        .bind_resource_managed_queue(admitted, 2)
        .expect_err("one admitted request must not mint a second queue binding");
    assert_eq!(
        duplicate.class(),
        ResourceManagedQueueDenialClass::QueueAlreadyBound
    );
    assert_eq!(duplicate.counters().exact_request_lookup_count(), 1);
    assert_eq!(duplicate.counters().queue_state_mutation_count(), 0);
    runtime
        .enqueue_resource_managed_queue(&queue, 1)
        .expect("first queue item should remain below capacity");

    let active = runtime
        .observe_resource_safe_point(&queue)
        .expect("exact active request should admit a safe-point observation");
    assert_eq!(active.request(), handle);
    assert_eq!(active.status(), ResourceInFlightStatus::Active);
    assert_eq!(
        active.pressure().class(),
        ResourceQueuePressureClass::Available
    );
    assert_eq!(active.ordinal().get(), 0);
    assert_eq!(active.counters().exact_request_lookup_count(), 1);
    assert_eq!(active.counters().pressure_classification_count(), 1);

    runtime
        .enqueue_resource_managed_queue(&queue, 1)
        .expect("second queue item should saturate exact capacity");
    runtime
        .cancel_resource_request(handle, ResourceCancellationReason::HostRequested)
        .expect("Signal should cancel the exact request");
    let cancelled = runtime
        .observe_resource_safe_point(&queue)
        .expect("retained cancelled request should remain observable");
    assert_eq!(cancelled.status(), ResourceInFlightStatus::Cancelled);
    assert_eq!(
        cancelled.pressure().class(),
        ResourceQueuePressureClass::Saturated
    );
    assert_eq!(cancelled.ordinal().get(), 1);
    let enqueue_denial = runtime
        .enqueue_resource_managed_queue(&queue, 1)
        .expect_err("terminal request cannot admit more producer output");
    assert_eq!(
        enqueue_denial.class(),
        ResourceManagedQueueDenialClass::RequestNotActive
    );
    let released = runtime
        .dequeue_resource_managed_queue(&queue, 2)
        .expect("consumer must release admitted occupancy after cancellation");
    assert_eq!(released.kind(), ResourceManagedQueueMutationKind::Dequeued);
    assert_eq!(released.pressure().queue_depth(), 0);
    assert_eq!(released.counters().exact_request_lookup_count(), 1);
    assert_eq!(released.counters().queue_state_mutation_count(), 1);
}

#[test]
fn managed_queue_rejects_overflow_without_changing_pressure_truth() {
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
    let queue = runtime
        .bind_resource_managed_queue(admitted, 2)
        .expect("exact admitted request should bind one managed queue");
    runtime
        .enqueue_resource_managed_queue(&queue, 2)
        .expect("exact capacity should saturate");

    let denial = runtime
        .enqueue_resource_managed_queue(&queue, 1)
        .expect_err("managed queue must deny production beyond capacity");
    assert_eq!(
        denial.class(),
        ResourceManagedQueueDenialClass::CapacityExceeded
    );
    assert_eq!(denial.counters().exact_request_lookup_count(), 1);
    assert_eq!(denial.counters().queue_state_mutation_count(), 0);
    let observation = runtime
        .observe_resource_safe_point(&queue)
        .expect("overflow denial should preserve queue observation");
    assert_eq!(observation.pressure().queue_depth(), 2);
    assert_eq!(
        observation.pressure().class(),
        ResourceQueuePressureClass::Saturated
    );
}

#[test]
fn occupied_terminal_queue_blocks_compaction_until_the_binding_drains_it() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(compact_cancelled_resource_declaration(node))
        .expect("cancelled compaction declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let queue = runtime
        .bind_resource_managed_queue(admitted, 2)
        .expect("managed request should bind one queue");
    runtime
        .enqueue_resource_managed_queue(&queue, 2)
        .expect("queue should retain exact admitted occupancy");
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should preserve retained drain authority");

    let blocked = runtime.compact_resource_lifecycle_history(1);
    assert_eq!(blocked.selected_terminal_count(), 0);
    assert_eq!(blocked.reclaimed_in_flight_count(), 0);
    runtime
        .dequeue_resource_managed_queue(&queue, 2)
        .expect("the original binding must drain terminal occupancy");

    let reclaimed = runtime.compact_resource_lifecycle_history(1);
    assert_eq!(reclaimed.selected_terminal_count(), 1);
    assert_eq!(reclaimed.reclaimed_in_flight_count(), 1);
    assert_eq!(reclaimed.compacted_cancelled_count(), 1);
}

#[test]
fn managed_queue_pressure_changes_in_flight_replay_truth() {
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
    let before_binding = runtime.reconstruct_resource_replay_summary();
    let queue = runtime
        .bind_resource_managed_queue(admitted, 2)
        .expect("managed request should bind one queue");
    let after_binding = runtime.reconstruct_resource_replay_summary();
    runtime
        .enqueue_resource_managed_queue(&queue, 1)
        .expect("queue depth should advance");
    let after_enqueue = runtime.reconstruct_resource_replay_summary();

    assert_ne!(
        before_binding.in_flight_digest(),
        after_binding.in_flight_digest(),
        "queue capacity must participate in replay truth"
    );
    assert_ne!(
        after_binding.in_flight_digest(),
        after_enqueue.in_flight_digest(),
        "queue depth must participate in replay truth"
    );
}

#[test]
fn snapshot_capture_denial_preserves_the_original_queue_binding() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(compact_cancelled_resource_declaration(node))
        .expect("cancelled compaction declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let queue = runtime
        .bind_resource_managed_queue(admitted, 2)
        .expect("managed request should bind one queue");

    let denial = runtime
        .capture_snapshot()
        .expect_err("branch snapshot must not clone live managed queue authority");
    assert_eq!(
        denial,
        SignalError::ManagedQueueBranchTransferDenied {
            bound_queue_count: 1
        }
    );
    runtime
        .enqueue_resource_managed_queue(&queue, 1)
        .expect("denial must leave the original binding usable");
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("request should enter terminal drain posture");
    runtime
        .dequeue_resource_managed_queue(&queue, 1)
        .expect("binding should drain retained occupancy");
    let reclaimed = runtime.compact_resource_lifecycle_history(1);
    assert_eq!(reclaimed.reclaimed_in_flight_count(), 1);
    runtime
        .capture_snapshot()
        .expect("snapshot should succeed after queue drain and terminal compaction");
}

#[test]
fn restore_and_branch_switch_deny_before_invalidating_a_queue_binding() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let sibling = runtime
        .create_branch("managed-queue-sibling")
        .expect("branch creation should succeed before queue authority exists");
    let snapshot = runtime
        .capture_snapshot()
        .expect("baseline snapshot should capture before queue binding");
    let main = runtime.current_branch();
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let queue = runtime
        .bind_resource_managed_queue(admitted, 2)
        .expect("managed request should bind one queue");

    let restore_denial = runtime
        .restore_snapshot(&snapshot)
        .expect_err("restore must not invalidate live queue authority");
    assert_eq!(
        restore_denial,
        SignalError::ManagedQueueBranchTransferDenied {
            bound_queue_count: 1
        }
    );
    let switch_denial = runtime
        .switch_branch(sibling)
        .expect_err("branch switch must not move live queue authority");
    assert_eq!(
        switch_denial,
        SignalError::ManagedQueueBranchTransferDenied {
            bound_queue_count: 1
        }
    );
    assert_eq!(runtime.current_branch(), main);
    runtime
        .enqueue_resource_managed_queue(&queue, 1)
        .expect("both denials must preserve the original binding");
}
