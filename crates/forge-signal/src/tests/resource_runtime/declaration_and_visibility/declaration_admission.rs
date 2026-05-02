use super::*;

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

pub(crate) fn raw_completion(
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
