use super::support::*;
use super::*;

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
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");

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
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
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
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");

    let lifecycle_rollback = resource_async_lifecycle_rollback_workload();

    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate resource state");
    let restore_report = runtime
        .latest_resource_branch_restore_report()
        .expect("resource restore should publish branch evidence");
    let replay = runtime.reconstruct_resource_replay_summary();
    let diagnostics = runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction();
    let inflight_pressure = resource_async_inflight_pressure_workload();

    let bundle = resource_certification_builder()
        .with_async_resource_lifecycle_parity(&replay, &replay, &diagnostics, &diagnostics)
        .expect("lifecycle parity evidence should be accepted")
        .with_out_of_order_completion_supersession(second_admission)
        .expect("supersession evidence should be accepted")
        .with_async_rollback_observation_equivalence(
            lifecycle_rollback.rollback_report,
            lifecycle_rollback.rollback_observation,
            lifecycle_rollback.control_commit_observation,
            &lifecycle_rollback.pre_rollback_replay,
            &lifecycle_rollback.post_rollback_replay,
            &lifecycle_rollback.diagnostics_after_rollback,
        )
        .expect("rollback evidence should be accepted")
        .with_async_branch_restore_replay_equivalence(restore_report, &replay)
        .expect("branch/replay evidence should be accepted")
        .with_async_inflight_boundedness(
            inflight_pressure.runtime_summary,
            &inflight_pressure.replay_after_restore,
            inflight_pressure.telemetry,
            inflight_pressure.pressure_performance,
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
    let inflight_pressure = resource_async_inflight_pressure_workload();
    assert!(ResourceCertificationRecord::passing(
        ResourceCertificationFamily::AsyncInflightBoundedness,
        "",
        performance,
    )
    .is_err());
    let builder_err = resource_certification_builder()
        .with_async_inflight_boundedness(
            inflight_pressure.runtime_summary,
            &inflight_pressure.replay_after_restore,
            inflight_pressure.telemetry,
            inflight_pressure.pressure_performance,
        )
        .expect("first lifecycle record should be accepted")
        .with_async_inflight_boundedness(
            inflight_pressure.runtime_summary,
            &inflight_pressure.replay_after_restore,
            inflight_pressure.telemetry,
            inflight_pressure.pressure_performance,
        )
        .expect_err("duplicate builder family must reject before bundle construction");
    assert!(format!("{builder_err}").contains("duplicate certification family evidence"));
}
