use super::*;

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
