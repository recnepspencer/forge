use super::retained_mapping_digest_support::{
    expected_retained_causal_digest, expected_retained_causal_digest_for_basis,
    ExpectedRetainedCausalDigestArtifact, ExpectedRetainedCausalDigestBasis,
};
use super::retained_mapping_support::{
    binding_for, bridge_bulk_planning_reference, bridge_route_reference, missing_bridge_reference,
    query_observation_reference,
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
        .route(crate::facade::TruthCommitIdentity::new(
            "commit-causal-bulk-route",
        ))
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
            query_observation_reference(
                crate::facade::BridgeCausalEvidenceReferenceIdentity::query_observation(
                    "query-observation:bulk-planning",
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_route_reference(routed.result().result_summary()),
            bridge_bulk_planning_reference(&bulk_record),
        ],
    )
    .expect("request should be valid");

    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect("bulk planning record should bind");

    assert_eq!(envelope.counters().bridge_retained_lookup_count(), 2);
    assert_eq!(envelope.counters().retained_bridge_binding_count(), 2);
    assert_eq!(envelope.counters().bridge_record_unindexed_scan_count(), 0);
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
fn causal_envelope_denies_missing_bulk_planning_without_unindexed_scan() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let routed = runtime
        .route(crate::facade::TruthCommitIdentity::new(
            "commit-causal-missing-bulk",
        ))
        .expect("route should succeed");
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:missing-bulk",
            "causal-anchor:missing-bulk",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference(
                crate::facade::BridgeCausalEvidenceReferenceIdentity::query_observation(
                    "query-observation:missing-bulk",
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_route_reference(routed.result().result_summary()),
            missing_bridge_reference(
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
    assert_eq!(denial.counters().bridge_record_unindexed_scan_count(), 0);
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
            .route(crate::facade::TruthCommitIdentity::new(
                "commit-causal-bulk-scale",
            ))
            .expect("route should succeed");
        let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
            crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
                "query-admission:bulk-scale",
                "causal-anchor:bulk-scale",
            )
            .expect("query admission summary should be valid"),
            vec![
                query_observation_reference(
                    crate::facade::BridgeCausalEvidenceReferenceIdentity::query_observation(
                        "query-observation:bulk-scale",
                    )
                    .expect("query observation reference identity should be valid"),
                ),
                bridge_route_reference(routed.result().result_summary()),
                bridge_bulk_planning_reference(&target_record),
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
        assert_eq!(envelope.counters().bridge_record_unindexed_scan_count(), 0);
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
        BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new(format!("commit-bulk-{suffix}-a")),
        )),
        BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit(
            crate::facade::TruthCommitIdentity::new(format!("commit-bulk-{suffix}-b")),
        )),
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
    expected_retained_causal_digest(
        ExpectedRetainedCausalDigestArtifact::BulkPlanningRecord,
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
        counters.bulk_widening_count().to_string(),
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
        counters.bulk_parallel_serial_reduction_count().to_string(),
    ];
    let counter_basis = ExpectedRetainedCausalDigestBasis::from_counter_values(counter_parts);
    expected_retained_causal_digest_for_basis(
        ExpectedRetainedCausalDigestArtifact::BulkPlanningCounters,
        &counter_basis,
    )
}

fn bulk_planning_failures_digest(failures: &[BridgeBulkPlanningFailure]) -> String {
    let failure_basis =
        ExpectedRetainedCausalDigestBasis::from_bulk_planning_failure_records(failures);
    expected_retained_causal_digest_for_basis(
        ExpectedRetainedCausalDigestArtifact::BulkPlanningFailures,
        &failure_basis,
    )
}
