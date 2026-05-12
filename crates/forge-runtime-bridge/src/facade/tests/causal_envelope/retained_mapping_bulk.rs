use super::retained_mapping_support::{
    binding_for, bridge_reference, digest, query_observation_reference,
};
use super::{runtime, BridgeRuntimePolicy};
use crate::facade::{
    BridgeBulkPlanningCounters, BridgeBulkPlanningFailure, BridgeBulkWorkloadRequest,
    BridgeBulkWorkloadSegment, BridgeCanonicalBulkPlanRecord, BridgeCausalEnvelopeAssemblyRequest,
    BridgeCausalEnvelopeDenialKind, BridgeCausalEvidenceFamily, BridgeRouteRequest,
};

#[test]
fn causal_envelope_maps_bulk_planning_by_exact_workload_identity() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let routed = runtime
        .route("commit-causal-bulk-route")
        .expect("route should succeed");
    let bulk_record = retain_bulk_record(&runtime, "target");
    assert_eq!(bulk_record.planning_failure_count(), 1);
    assert!(bulk_record
        .planning_failures()
        .iter()
        .all(|failure| !failure.digest().is_empty()));
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:bulk-planning",
            "causal-anchor:bulk-planning",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:bulk-planning"),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                routed.result().result_summary().route_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeBulkPlanning,
                bulk_record.workload_identity().as_str(),
            ),
        ],
    )
    .expect("request should be valid");

    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect("bulk planning record should bind");

    assert_eq!(envelope.counters().bridge_retained_lookup_count(), 2);
    assert_eq!(envelope.counters().retained_bridge_binding_count(), 2);
    assert_eq!(envelope.counters().bridge_record_scan_fallback_count(), 0);
    assert_eq!(
        binding_for(
            envelope.bindings(),
            BridgeCausalEvidenceFamily::BridgeBulkPlanning,
            bulk_record.workload_identity().as_str()
        )
        .retained_record_digest(),
        Some(bulk_planning_digest(&bulk_record).as_str())
    );
}

#[test]
fn causal_envelope_denies_missing_bulk_planning_without_scan_fallback() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let routed = runtime
        .route("commit-causal-missing-bulk")
        .expect("route should succeed");
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:missing-bulk",
            "causal-anchor:missing-bulk",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:missing-bulk"),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                routed.result().result_summary().route_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeBulkPlanning,
                "missing-bulk-workload",
            ),
        ],
    )
    .expect("request should be valid");

    let denial = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect_err("missing bulk planning record should deny");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::MissingRetainedBridgeRecord
    );
    assert_eq!(
        denial.family(),
        BridgeCausalEvidenceFamily::BridgeBulkPlanning
    );
    assert_eq!(denial.counters().bridge_retained_lookup_count(), 2);
    assert_eq!(denial.counters().retained_bridge_binding_count(), 1);
    assert_eq!(denial.counters().missing_bridge_record_count(), 1);
    assert_eq!(denial.counters().bridge_record_scan_fallback_count(), 0);
}

#[test]
fn causal_envelope_bulk_planning_lookup_cost_ignores_unrelated_records() {
    let mut envelope_identities = Vec::new();

    for unrelated_records in [0, 3, 8] {
        let runtime = runtime(BridgeRuntimePolicy::default());
        for index in 0..unrelated_records {
            retain_bulk_record(&runtime, &format!("noise-{index}"));
        }
        let target_record = retain_bulk_record(&runtime, "target");
        let routed = runtime
            .route("commit-causal-bulk-scale")
            .expect("route should succeed");
        let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
            crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
                "query-admission:bulk-scale",
                "causal-anchor:bulk-scale",
            )
            .expect("query admission summary should be valid"),
            vec![
                query_observation_reference("query-observation:bulk-scale"),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.result().result_summary().route_identity().as_str(),
                ),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeBulkPlanning,
                    target_record.workload_identity().as_str(),
                ),
            ],
        )
        .expect("request should be valid");

        let envelope = runtime
            .diagnostics()
            .assemble_causal_explanation_envelope(request)
            .expect("target bulk planning should bind");

        assert_eq!(
            runtime.diagnostics().bulk_records().len(),
            unrelated_records + 1
        );
        assert_eq!(envelope.counters().bridge_retained_lookup_count(), 2);
        assert_eq!(envelope.counters().retained_bridge_binding_count(), 2);
        assert_eq!(envelope.counters().bridge_record_scan_fallback_count(), 0);
        envelope_identities.push(envelope.identity().identity_digest().to_string());
    }

    assert_eq!(envelope_identities[0], envelope_identities[1]);
    assert_eq!(envelope_identities[1], envelope_identities[2]);
}

fn retain_bulk_record(
    runtime: &crate::facade::RuntimeBridge,
    suffix: &str,
) -> BridgeCanonicalBulkPlanRecord {
    let workload = BridgeBulkWorkloadRequest::new(vec![
        BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(format!(
            "commit-bulk-{suffix}-a"
        ))),
        BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(format!(
            "commit-bulk-{suffix}-b"
        ))),
    ]);
    let plan = runtime
        .plan_bulk_workload(workload)
        .expect("bulk workload should plan");
    runtime.canonicalize_bulk_workload_plan(&plan)
}

fn bulk_planning_digest(record: &BridgeCanonicalBulkPlanRecord) -> String {
    let selected_mode = format!("{:?}", record.selected_mode());
    let planning_failure_count = record.planning_failure_count().to_string();
    let planning_failures_digest = bulk_planning_failures_digest(record.planning_failures());
    let counters_digest = bulk_planning_counters_digest(record.counters());
    digest(
        "bridge-causal-retained-bulk-planning-record",
        &[
            record.workload_identity().as_str(),
            record.schema_version(),
            record.canonical_request_digest(),
            record.normalized_summary_digest(),
            record.canonical_planning_identity().as_str(),
            record.admission_profile_identity().as_str(),
            record.packet_set_digest(),
            record.execution_plan_digest(),
            record.reduced_artifact_digest(),
            selected_mode.as_str(),
            record.decision_log_digest(),
            counters_digest.as_str(),
            planning_failure_count.as_str(),
            planning_failures_digest.as_str(),
        ],
    )
}

fn bulk_planning_counters_digest(counters: &BridgeBulkPlanningCounters) -> String {
    let counter_parts = [
        counters.bulk_workload_count().to_string(),
        counters.bulk_routed_item_count().to_string(),
        counters.bulk_normalized_workload_width().to_string(),
        counters.bulk_packet_count().to_string(),
        counters.bulk_packet_entry_count().to_string(),
        counters.bulk_reduction_input_count().to_string(),
        counters.bulk_reduction_output_count().to_string(),
        counters.bulk_fallback_count().to_string(),
        counters.bulk_packet_queue_depth_peak().to_string(),
        counters.bulk_reducer_input_buffer_peak().to_string(),
        counters.bulk_replay_mismatch_count().to_string(),
        counters.bulk_unsupported_path_count().to_string(),
        counters.bulk_serial_required_count().to_string(),
        counters.bulk_parallel_legal_count().to_string(),
        counters.bulk_parallel_profitable_count().to_string(),
        counters
            .bulk_parallel_preparation_admitted_count()
            .to_string(),
        counters
            .bulk_parallel_preparation_rejected_count()
            .to_string(),
        counters
            .bulk_parallel_fallback_to_serial_count()
            .to_string(),
    ];
    let counter_refs: Vec<&str> = counter_parts.iter().map(String::as_str).collect();
    digest("bridge-bulk-planning-counters", &counter_refs)
}

fn bulk_planning_failures_digest(failures: &[BridgeBulkPlanningFailure]) -> String {
    let failure_digests: Vec<&str> = failures.iter().map(|failure| failure.digest()).collect();
    digest("bridge-bulk-planning-failures", &failure_digests)
}
