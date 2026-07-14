use std::collections::BTreeMap;

use serde_json::json;

use super::super::certification_bundle::{
    MergeHarnessCertificationBundle, MergeRecordEvidence, MergeSupportMatrix,
};
use super::super::counter_snapshot::MergeHarnessCounterSnapshot;
use super::super::MergeHarnessExecution;

pub(in crate::harness::adapter::adapter_impl) fn summary_json(
    execution: &MergeHarnessExecution,
) -> serde_json::Value {
    let bundle = execution.certification_bundle();
    json!({
        "merge_declaration_identity": bundle.merge_declaration_identity().as_str(),
        "merge_contract_identity": bundle.merge_contract_identity().as_str(),
        "merge_history_digest": bundle.merge_history_digest(),
        "result_bundle_digest": bundle.result_bundle_digest(),
        "continuity_digest": bundle.record_evidence().continuity_digest(),
        "remap_digest": bundle.record_evidence().remap_digest(),
        "explanation_digest": bundle.record_evidence().explanation_digest(),
        "replay_digest": bundle.replay_digest(),
        "diagnostics_digest": bundle.diagnostics_digest(),
        "record_identity": bundle.record_identity().as_str(),
        "outcome_class": format!("{:?}", bundle.support_matrix().outcome_class()),
        "blocked_stage": bundle
            .denial_stage_report()
            .blocked_stage()
            .map(|stage| format!("{stage:?}")),
        "denial_class": bundle
            .denial_stage_report()
            .denial_class()
            .map(|class| format!("{class:?}")),
        "counter_snapshot": counter_snapshot_json(bundle.counter_snapshot()),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn extensions_json(
    execution: &MergeHarnessExecution,
) -> BTreeMap<String, serde_json::Value> {
    let bundle = execution.certification_bundle();
    BTreeMap::from([
        (
            "bridge_merge_certification_bundle".to_string(),
            certification_bundle_json(bundle),
        ),
        merge_record_extension(bundle),
    ])
}

fn merge_record_extension(bundle: &MergeHarnessCertificationBundle) -> (String, serde_json::Value) {
    (
        "bridge_merge_record".to_string(),
        merge_record_json(bundle.record_evidence()),
    )
}

fn certification_bundle_json(bundle: &MergeHarnessCertificationBundle) -> serde_json::Value {
    json!({
        "merge_history_digest": bundle.merge_history_digest(),
        "merge_contract_identity": bundle.merge_contract_identity().as_str(),
        "merge_ontology_mapping_report": {
            "bridge_class": format!("{:?}", bundle.ontology_mapping_report().bridge_class()),
            "ontology_mapping_digest": bundle.ontology_mapping_report().ontology_mapping_digest(),
            "ontology_version": bundle.ontology_mapping_report().ontology_version(),
            "schema_policy_descriptor_version": bundle
                .ontology_mapping_report()
                .schema_policy_descriptor_version(),
        },
        "merge_support_matrix": merge_support_matrix_json(bundle.support_matrix()),
        "merge_denial_stage_report": {
            "blocked_stage": bundle
                .denial_stage_report()
                .blocked_stage()
                .map(|stage| format!("{stage:?}")),
            "denial_class": bundle
                .denial_stage_report()
                .denial_class()
                .map(|class| format!("{class:?}")),
        },
        "result_bundle_digest": bundle.result_bundle_digest(),
        "replay_digest": bundle.replay_digest(),
        "failure_digest": bundle.failure_digest(),
        "diagnostics_digest": bundle.diagnostics_digest(),
        "record_identity": bundle.record_identity().as_str(),
        "counter_snapshot": counter_snapshot_json(bundle.counter_snapshot()),
    })
}

fn merge_support_matrix_json(matrix: &MergeSupportMatrix) -> serde_json::Value {
    json!({
        "outcome_class": format!("{:?}", matrix.outcome_class()),
        "continuity_published": matrix.continuity_published(),
        "remap_published": matrix.remap_published(),
    })
}

fn merge_record_json(record: &MergeRecordEvidence) -> serde_json::Value {
    json!({
        "record_identity": record.record_identity().as_str(),
        "merge_contract_identity": record.merge_contract_identity().as_str(),
        "merge_declaration_identity": record.merge_declaration_identity().as_str(),
        "bundle_digest": record.bundle_digest(),
        "lowered_digest": record.lowered_digest(),
        "reduced_digest": record.reduced_digest(),
        "continuity_digest": record.continuity_digest(),
        "remap_digest": record.remap_digest(),
        "explanation_digest": record.explanation_digest(),
        "outcome_class": format!("{:?}", record.outcome_class()),
        "blocked_stage": record.blocked_stage().map(|stage| format!("{stage:?}")),
        "denial_class": record.denial_class().map(|class| format!("{class:?}")),
    })
}

fn counter_snapshot_json(counter_snapshot: MergeHarnessCounterSnapshot) -> serde_json::Value {
    json!({
        "merge_declaration_count": counter_snapshot.merge_declaration_count(),
        "merge_contract_count": counter_snapshot.merge_contract_count(),
        "merge_parent_count": counter_snapshot.merge_parent_count(),
        "merge_supported_class_count": counter_snapshot.merge_supported_class_count(),
        "merge_unsupported_class_count": counter_snapshot.merge_unsupported_class_count(),
        "merge_parent_order_rejection_count": counter_snapshot.merge_parent_order_rejection_count(),
        "merge_causal_frontier_count": counter_snapshot.merge_causal_frontier_count(),
        "merge_policy_outcome_count": counter_snapshot.merge_policy_outcome_count(),
        "merge_history_packet_count": counter_snapshot.merge_history_packet_count(),
        "merge_routing_result_count": counter_snapshot.merge_routing_result_count(),
        "merge_lineage_resolution_width": counter_snapshot.merge_lineage_resolution_width(),
        "merge_candidate_cohort_width": counter_snapshot.merge_candidate_cohort_width(),
        "merge_structural_consult_width": counter_snapshot.merge_structural_consult_width(),
        "merge_causal_frontier_lookup_count": counter_snapshot.merge_causal_frontier_lookup_count(),
        "merge_history_segment_scan_count": counter_snapshot.merge_history_segment_scan_count(),
        "merge_continuity_count": counter_snapshot.merge_continuity_count(),
        "merge_continuity_denial_count": counter_snapshot.merge_continuity_denial_count(),
        "merge_remap_publication_count": counter_snapshot.merge_remap_publication_count(),
        "merge_deletion_class_count": counter_snapshot.merge_deletion_class_count(),
        "merge_topology_rewire_class_count": counter_snapshot.merge_topology_rewire_class_count(),
        "merge_structural_contradiction_count": counter_snapshot.merge_structural_contradiction_count(),
        "merge_explanation_request_count": counter_snapshot.merge_explanation_request_count(),
        "merge_replay_request_count": counter_snapshot.merge_replay_request_count(),
        "merge_replay_mismatch_count": counter_snapshot.merge_replay_mismatch_count(),
        "merge_widened_scan_count": counter_snapshot.merge_widened_scan_count(),
        "digest_computation_count": counter_snapshot.digest_computation_count(),
        "digest_input_bytes": counter_snapshot.digest_input_bytes(),
    })
}
